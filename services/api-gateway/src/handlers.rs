use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use vibestream_types::{ApiMessage, Blockchain, WalletAddress, ServiceMessage, MessageBroker};
use crate::services::AppState;
use crate::auth::{Claims, LoginRequest, LoginResponse, UserInfo, hash_password, verify_password};
use sqlx::Row;
use uuid::Uuid;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    service: String,
    timestamp: String,
    redis: String,
}

#[derive(Serialize)]
pub struct DatabaseHealthResponse {
    status: String,
    service: String,
    timestamp: String,
    database: String,
    tables_count: Option<i64>,
}

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    username: String,
    email: String,
    role: String,
    created_at: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
    username: String,
    password: String,
    wallet_address: Option<String>,
    role: Option<String>,
}

#[derive(Serialize)]
pub struct SongResponse {
    id: String,
    title: String,
    artist_id: String,
    duration: Option<i32>,
    file_hash: Option<String>,
    created_at: String,
}

#[derive(Deserialize)]
pub struct CreateSongRequest {
    title: String,
    artist_id: String,
    duration_seconds: Option<i32>,
    genre: Option<String>,
    ipfs_hash: Option<String>,
    royalty_percentage: Option<f64>,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub blockchain: Blockchain,
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub message: String,
    pub request_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub blockchain: String,
    pub balance: Option<u64>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalanceRequest {
    pub blockchain: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalanceResponse {
    pub blockchain: String,
    pub address: String,
    pub balance: u64,
    pub symbol: String,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct OAuthRegisterRequest {
    pub email: String,
    pub username: String,
    pub provider: String,           // "google" | "microsoft"
    pub provider_id: String,        // ID del usuario en el provider
    pub name: String,               // Nombre completo del usuario
    pub profile_picture: Option<String>, // URL de foto de perfil
}

#[axum::debug_handler]
pub async fn health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    let redis_status = match state.message_queue.ping().await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        service: "api-gateway".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        redis: redis_status.to_string(),
    }))
}

#[axum::debug_handler]
pub async fn process_transaction(
    State(state): State<AppState>,
    Json(request): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, StatusCode> {
    // Crear mensaje para el servicio correspondiente
    let api_message = ApiMessage::ProcessTransaction {
        blockchain: request.blockchain.clone(),
        from: request.from,
        to: request.to,
        amount: request.amount,
    };

    let service_message = ServiceMessage::new(api_message);
    let request_id = service_message.id.0.to_string();

    // Determinar la cola correcta según la blockchain
    let queue_name = match request.blockchain {
        Blockchain::Ethereum => "ethereum_queue",
        Blockchain::Solana => "solana_queue",
    };

    // Enviar mensaje a la cola correspondiente
    let serialized = serde_json::to_string(&service_message)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match state.message_queue.send_message(queue_name, &serialized).await {
        Ok(_) => {
            tracing::info!("Transaction request sent to {}: {}", queue_name, request_id);
            Ok(Json(TransactionResponse {
                message: "Transaction request submitted successfully".to_string(),
                request_id,
                status: "pending".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to send transaction request: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
pub async fn get_balance(
    State(state): State<AppState>,
    Path((blockchain, address)): Path<(String, String)>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    // Parsear blockchain
    let blockchain = match blockchain.to_lowercase().as_str() {
        "ethereum" => Blockchain::Ethereum,
        "solana" => Blockchain::Solana,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Crear wallet address
    let wallet = WalletAddress {
        address: address.clone(),
        blockchain: blockchain.clone(),
    };

    // Crear mensaje para obtener balance
    let api_message = ApiMessage::GetBalance { wallet };
    let service_message = ServiceMessage::new(api_message);

    // Determinar la cola correcta
    let queue_name = match blockchain {
        Blockchain::Ethereum => "ethereum_queue",
        Blockchain::Solana => "solana_queue",
    };

    // Enviar mensaje a la cola
    let serialized = serde_json::to_string(&service_message)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match state.message_queue.send_message(queue_name, &serialized).await {
        Ok(_) => {
            tracing::info!("Balance request sent to {}: {}", queue_name, address);
            Ok(Json(BalanceResponse {
                address,
                blockchain: format!("{:?}", blockchain),
                balance: None, // En una implementación real, esperaríamos la respuesta
                status: "request_sent".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to send balance request: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Endpoint para obtener el estado de las colas Redis
#[axum::debug_handler]
pub async fn queue_status(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut status = serde_json::Map::new();
    
    // Verificar conexión Redis
    match state.message_queue.ping().await {
        Ok(_) => {
            status.insert("redis".to_string(), serde_json::Value::String("connected".to_string()));
            status.insert("queues".to_string(), serde_json::json!({
                "ethereum_queue": "available",
                "solana_queue": "available", 
                "zk_queue": "available",
                "response_queue": "available"
            }));
        }
        Err(e) => {
            status.insert("redis".to_string(), serde_json::Value::String("disconnected".to_string()));
            status.insert("error".to_string(), serde_json::Value::String(format!("{:?}", e)));
        }
    }

    Ok(Json(serde_json::Value::Object(status)))
}

// Health check específico para la base de datos
#[axum::debug_handler]
pub async fn database_health_check(State(state): State<AppState>) -> Result<Json<DatabaseHealthResponse>, StatusCode> {
    let database_status = match state.database_pool.health_check().await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    // Contar las tablas en la base de datos
    let tables_count = if database_status == "connected" {
        match sqlx::query("SELECT COUNT(*) as count FROM information_schema.tables WHERE table_schema = 'public'")
            .fetch_one(state.database_pool.get_pool())
            .await
        {
            Ok(row) => Some(row.get::<i64, _>("count")),
            Err(_) => None,
        }
    } else {
        None
    };

    Ok(Json(DatabaseHealthResponse {
        status: if database_status == "connected" { "healthy".to_string() } else { "unhealthy".to_string() },
        service: "api-gateway-database".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: database_status.to_string(),
        tables_count,
    }))
}

// Endpoint para obtener usuarios (TEST)
#[axum::debug_handler]
pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    match sqlx::query("SELECT id, username, email, role, created_at FROM users LIMIT 10")
        .fetch_all(state.database_pool.get_pool())
        .await
    {
        Ok(rows) => {
            let users: Vec<UserResponse> = rows
                .into_iter()
                .map(|row| UserResponse {
                    id: row.get::<uuid::Uuid, _>("id").to_string(),
                    username: row.get("username"),
                    email: row.get("email"),
                    role: row.get("role"),
                    created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                })
                .collect();
            
            tracing::info!("📋 Obtenidos {} usuarios de la base de datos", users.len());
            Ok(Json(users))
        }
        Err(e) => {
            tracing::error!("❌ Error al obtener usuarios: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Endpoint para obtener canciones (TEST)
#[axum::debug_handler]
pub async fn get_songs(State(state): State<AppState>) -> Result<Json<Vec<SongResponse>>, StatusCode> {
    match sqlx::query("SELECT id, title, artist_id, duration_seconds, ipfs_hash, created_at FROM songs LIMIT 10")
        .fetch_all(state.database_pool.get_pool())
        .await
    {
        Ok(rows) => {
            let songs: Vec<SongResponse> = rows
                .into_iter()
                .map(|row| SongResponse {
                    id: row.get::<uuid::Uuid, _>("id").to_string(),
                    title: row.get("title"),
                    artist_id: row.get::<uuid::Uuid, _>("artist_id").to_string(),
                    duration: row.get("duration_seconds"),
                    file_hash: row.get("ipfs_hash"),
                    created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                })
                .collect();
            
            tracing::info!("🎵 Obtenidas {} canciones de la base de datos", songs.len());
            Ok(Json(songs))
        }
        Err(e) => {
            tracing::error!("❌ Error al obtener canciones: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// POST endpoint para crear usuarios
#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Hash the password con bcrypt
    let password_hash = match hash_password(&request.password) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    let user_id = uuid::Uuid::new_v4();
    let role = request.role.unwrap_or_else(|| "user".to_string());
    
    match sqlx::query(
        "INSERT INTO users (id, email, username, password_hash, wallet_address, role) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id, username, email, role, created_at"
    )
    .bind(user_id)
    .bind(&request.email)
    .bind(&request.username)
    .bind(&password_hash)
    .bind(&request.wallet_address)
    .bind(&role)
    .fetch_one(state.database_pool.get_pool())
    .await
    {
        Ok(row) => {
            let user = UserResponse {
                id: row.get::<uuid::Uuid, _>("id").to_string(),
                username: row.get("username"),
                email: row.get("email"),
                role: row.get("role"),
                created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
            };
            
            tracing::info!("✅ Usuario creado: {} ({})", user.username, user.id);
            Ok(Json(user))
        }
        Err(e) => {
            tracing::error!("❌ Error al crear usuario: {:?}", e);
            if e.to_string().contains("duplicate key") {
                Err(StatusCode::CONFLICT) // 409 - Email o username ya existe
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

// POST endpoint para crear canciones
#[axum::debug_handler]
pub async fn create_song(
    State(state): State<AppState>,
    Json(request): Json<CreateSongRequest>,
) -> Result<Json<SongResponse>, StatusCode> {
    let song_id = uuid::Uuid::new_v4();
    let artist_uuid = match uuid::Uuid::parse_str(&request.artist_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match sqlx::query(
        "INSERT INTO songs (id, title, artist_id, duration_seconds, genre, ipfs_hash, royalty_percentage) 
         VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING id, title, artist_id, duration_seconds, ipfs_hash, created_at"
    )
    .bind(song_id)
    .bind(&request.title)
    .bind(artist_uuid)
    .bind(request.duration_seconds)
    .bind(&request.genre)
    .bind(&request.ipfs_hash)
    .bind(request.royalty_percentage.unwrap_or(10.0))
    .fetch_one(state.database_pool.get_pool())
    .await
    {
        Ok(row) => {
            let song = SongResponse {
                id: row.get::<uuid::Uuid, _>("id").to_string(),
                title: row.get("title"),
                artist_id: row.get::<uuid::Uuid, _>("artist_id").to_string(),
                duration: row.get("duration_seconds"),
                file_hash: row.get("ipfs_hash"),
                created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
            };
            
            tracing::info!("🎵 Canción creada: {} ({})", song.title, song.id);
            Ok(Json(song))
        }
        Err(e) => {
            tracing::error!("❌ Error al crear canción: {:?}", e);
            if e.to_string().contains("foreign key") {
                Err(StatusCode::BAD_REQUEST) // Artist ID no existe
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

// POST endpoint para login
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Buscar usuario por email
    match sqlx::query("SELECT id, username, email, password_hash, role FROM users WHERE email = $1")
        .bind(&request.email)
        .fetch_optional(state.database_pool.get_pool())
        .await
    {
        Ok(Some(row)) => {
            let stored_hash: String = row.get("password_hash");
            
            // Verificar password
            match verify_password(&request.password, &stored_hash) {
                Ok(true) => {
                    // Password correcto, generar JWT
                    let user_id: uuid::Uuid = row.get("id");
                    let username: String = row.get("username");
                    let email: String = row.get("email");
                    let role: String = row.get("role");
                    
                    let claims = Claims::new(user_id, username.clone(), email.clone(), role.clone());
                    
                    match claims.to_jwt() {
                        Ok(token) => {
                            let response = LoginResponse {
                                token,
                                user: UserInfo {
                                    id: user_id.to_string(),
                                    username,
                                    email,
                                    role,
                                },
                            };
                            
                            tracing::info!("✅ Login exitoso: {}", request.email);
                            Ok(Json(response))
                        }
                        Err(_) => {
                            tracing::error!("❌ Error generando JWT");
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
                Ok(false) => {
                    tracing::warn!("🚫 Password incorrecto para: {}", request.email);
                    Err(StatusCode::UNAUTHORIZED)
                }
                Err(_) => {
                    tracing::error!("❌ Error verificando password");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => {
            tracing::warn!("🚫 Usuario no encontrado: {}", request.email);
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            tracing::error!("❌ Error en base de datos: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// POST endpoint para register (crear usuario con login automático)
#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Hash the password con bcrypt
    let password_hash = match hash_password(&request.password) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    let user_id = uuid::Uuid::new_v4();
    let role = request.role.unwrap_or_else(|| "user".to_string());
    
    match sqlx::query(
        "INSERT INTO users (id, email, username, password_hash, wallet_address, role) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id, username, email, role"
    )
    .bind(user_id)
    .bind(&request.email)
    .bind(&request.username)
    .bind(&password_hash)
    .bind(&request.wallet_address)
    .bind(&role)
    .fetch_one(state.database_pool.get_pool())
    .await
    {
        Ok(row) => {
            let user_id: uuid::Uuid = row.get("id");
            let username: String = row.get("username");
            let email: String = row.get("email");
            let role: String = row.get("role");
            
            // Generar JWT automáticamente
            let claims = Claims::new(user_id, username.clone(), email.clone(), role.clone());
            
            match claims.to_jwt() {
                Ok(token) => {
                    let response = LoginResponse {
                        token,
                        user: UserInfo {
                            id: user_id.to_string(),
                            username,
                            email,
                            role,
                        },
                    };
                    
                    tracing::info!("✅ Usuario registrado y logueado: {}", request.username);
                    Ok(Json(response))
                }
                Err(_) => {
                    tracing::error!("❌ Error generando JWT después de registro");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            tracing::error!("❌ Error al registrar usuario: {:?}", e);
            if e.to_string().contains("duplicate key") {
                Err(StatusCode::CONFLICT) // 409 - Email o username ya existe
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

// Endpoint protegido de ejemplo - obtener perfil del usuario actual
#[axum::debug_handler]
pub async fn get_profile(claims: Claims) -> Result<Json<UserInfo>, StatusCode> {
    let user_info = UserInfo {
        id: claims.sub,
        username: claims.username,
        email: claims.email,
        role: claims.role,
    };
    
    tracing::info!("📋 Perfil solicitado: {}", user_info.email);
    Ok(Json(user_info))
}

// GET /api/v1/wallet/balance/:blockchain/:address - Obtener balance de wallet (SIMPLIFICADO)
pub async fn get_wallet_balance(
    Path((blockchain, address)): Path<(String, String)>,
    State(_state): State<crate::services::AppState>,
) -> Json<serde_json::Value> {
    tracing::info!("🔍 Balance solicitado para {} en {}", address, blockchain);

    // Respuesta simulada por ahora
    let balance = 1000u64; // Balance simulado
    let symbol = match blockchain.to_lowercase().as_str() {
        "ethereum" => "ETH",
        "solana" => "SOL",
        _ => "UNKNOWN",
    };

    Json(serde_json::json!({
        "blockchain": blockchain.to_uppercase(),
        "address": address,
        "balance": balance,
        "symbol": symbol,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "status": "simulated"
    }))
}

// POST /api/v1/songs/:song_id/purchase - Comprar/pagar una canción (ULTRA SIMPLIFICADO)
pub async fn purchase_song(
    Path(song_id): Path<Uuid>,
    State(state): State<crate::services::AppState>,
    user_claims: Claims,
    Json(payment_data): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let user_id_str = &user_claims.sub;
    tracing::info!("🎵 Procesando compra de canción {} por usuario {}", song_id, user_id_str);

    // Verificar que la canción existe
    let song_exists = sqlx::query("SELECT id FROM songs WHERE id = $1")
        .bind(song_id)
        .fetch_optional(state.database_pool.get_pool())
        .await
        .unwrap_or(None);

    if song_exists.is_none() {
        return Json(serde_json::json!({
            "error": "Song not found",
            "status": "error"
        }));
    }

    let payment_id = Uuid::new_v4();
    let transaction_hash = format!("sim_tx_{}", payment_id);
    
    Json(serde_json::json!({
        "transaction_hash": transaction_hash,
        "payment_id": payment_id,
        "song_id": song_id,
        "user_id": user_id_str,
        "amount_paid": "0.01",
        "artist_royalty": "0.008",
        "platform_fee": "0.002", 
        "blockchain": "ETHEREUM",
        "status": "completed"
    }))
}

// GET /api/v1/blockchain/health - Health check de servicios blockchain (SIMPLIFICADO)
pub async fn blockchain_health_check(
    State(_state): State<crate::services::AppState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "blockchain_services": {
            "ethereum": {
                "status": "simulated",
                "url": "http://localhost:3001"
            },
            "solana": {
                "status": "simulated", 
                "url": "http://localhost:3003"
            }
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// GET /api/v1/user/transactions - Obtener historial de transacciones del usuario (SIMPLIFICADO)
pub async fn get_user_transactions(
    State(_state): State<crate::services::AppState>,
    user_claims: Claims,
) -> Json<Vec<serde_json::Value>> {
    let user_id_str = &user_claims.sub;
    
    // Por ahora retornar lista vacía o simulada
    let transactions = vec![
        serde_json::json!({
            "id": Uuid::new_v4().to_string(),
            "transaction_type": "song_purchase",
            "amount": "0.01",
            "blockchain_network": "ethereum",
            "transaction_hash": "sim_tx_sample",
            "created_at": chrono::Utc::now().to_rfc3339(),
            "status": "completed"
        })
    ];

    tracing::info!("✅ Retrieved {} transactions for user {}", transactions.len(), user_id_str);
    Json(transactions)
}

// POST /api/v1/auth/oauth - Registro/Login via OAuth (VERSIÓN SIMPLIFICADA PARA DEBUG)
#[axum::debug_handler]
pub async fn oauth_register(
    State(_state): State<AppState>,
    Json(request): Json<OAuthRegisterRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::info!("🔐 OAuth request recibido para {} via {}", request.email, request.provider);
    
    // Respuesta simplificada para debugging
    let mock_response = serde_json::json!({
        "status": "success",
        "message": "OAuth endpoint funcionando",
        "received_data": {
            "email": request.email,
            "username": request.username,
            "provider": request.provider,
            "provider_id": request.provider_id,
            "name": request.name
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    tracing::info!("✅ OAuth respuesta enviada");
    Ok(Json(mock_response))
}

// Función para generar wallet custodiada (simplificada por ahora)
fn generate_custodial_wallet(user_id: &uuid::Uuid) -> String {
    // Por ahora generamos una dirección simulada
    // En producción, aquí crearías una wallet real de Ethereum/Solana
    format!("0x{:x}", user_id.as_u128())
} 