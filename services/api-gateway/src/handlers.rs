use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use vibestream_types::{ApiMessage, Blockchain, WalletAddress, ServiceMessage, MessageBroker};
use crate::services::AppState;
use sqlx::Row;

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