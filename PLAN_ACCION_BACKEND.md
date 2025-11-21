# Plan de Acción Estratégico - Backend VibeStream

> **Objetivo**: Preparar el backend para consumo del frontend  
> **Duración estimada**: 4-6 semanas  
> **Prioridad**: CRÍTICA antes de empezar frontend

---

## 🎯 Objetivo Final

Tener un backend con:
- ✅ Gateway unificado en un solo puerto
- ✅ Autenticación completa y funcional
- ✅ Endpoints críticos implementados con lógica real
- ✅ OpenAPI spec completo y validado
- ✅ Base de datos con migraciones completas
- ✅ Tests básicos funcionando

---

## 📅 Fase 1: Fundación (Semana 1-2) - BLOQUEANTE

### Tarea 1.1: Gateway Unificado

**Problema**: 9 puertos diferentes complican el desarrollo del frontend.

**Solución**: Crear un gateway centralizado que enrute todas las peticiones.

#### Pasos de Implementación

1. **Crear nuevo router centralizado** (`services/api-gateway/src/main_unified.rs`):
   ```rust
   // Estructura propuesta:
   Router::new()
       .route("/api/v1/users/*", user_routes)
       .route("/api/v1/music/*", music_routes)
       .route("/api/v1/payments/*", payment_routes)
       // ... etc
       .route("/health", health_check)
       .route("/api-docs/*", openapi_routes)
   ```

2. **Configurar CORS centralizado**:
   ```rust
   use tower_http::cors::CorsLayer;
   
   let cors = CorsLayer::new()
       .allow_origin(Any)
       .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
       .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
   ```

3. **Middleware unificado**:
   - Logging de requests
   - Manejo de errores
   - Rate limiting básico

4. **Health check unificado**:
   - Verificar todos los servicios
   - Retornar estado de cada gateway

**Archivos a modificar**:
- `services/api-gateway/src/main.rs` → Refactorizar a gateway unificado
- Crear `services/api-gateway/src/routing.rs` → Lógica de enrutamiento

**Criterios de éxito**:
- [ ] Un solo puerto (3000)
- [ ] Todos los endpoints accesibles vía `/api/v1/*`
- [ ] CORS funcionando
- [ ] Health check unificado funcionando

**Tiempo estimado**: 2-3 días

---

### Tarea 1.2: Autenticación Completa

**Problema**: Los handlers no extraen `user_id` del JWT, usan UUIDs random.

**Solución**: Extraer claims del JWT en todos los handlers protegidos.

#### Pasos de Implementación

1. **Crear extractor de Claims para Axum**:
   ```rust
   // En shared/infrastructure/auth/middleware.rs
   pub struct AuthenticatedUser {
       pub user_id: Uuid,
       pub username: String,
       pub email: String,
       pub role: String,
       pub tier: String,
   }
   
   impl FromRequestParts<()> for AuthenticatedUser {
       // Extraer de request.extensions()
   }
   ```

2. **Actualizar handlers en `user_controller.rs`**:
   ```rust
   // ❌ ANTES:
   pub async fn follow_user(
       Path(followee_id): Path<Uuid>,
       // ...
   ) {
       let follower_id = Uuid::new_v4(); // Mock
   }
   
   // ✅ DESPUÉS:
   pub async fn follow_user(
       AuthenticatedUser { user_id: follower_id, .. }: AuthenticatedUser,
       Path(followee_id): Path<Uuid>,
       // ...
   ) {
       // Usar follower_id real
   }
   ```

3. **Implementar `change_password`**:
   ```rust
   pub async fn change_password(
       AuthenticatedUser { user_id, .. }: AuthenticatedUser,
       Json(request): Json<ChangePasswordRequest>,
       State(user_service): State<UserAppService>,
   ) {
       // 1. Validar contraseña actual
       // 2. Validar nueva contraseña
       // 3. Hashear nueva contraseña
       // 4. Actualizar en BD
   }
   ```

4. **Implementar `link_wallet`**:
   ```rust
   pub async fn link_wallet(
       AuthenticatedUser { user_id, .. }: AuthenticatedUser,
       Json(request): Json<LinkWalletRequest>,
       State(user_service): State<UserAppService>,
   ) {
       // 1. Verificar firma del wallet
       // 2. Actualizar wallet_address en BD
   }
   ```

5. **Implementar `delete_user`**:
   ```rust
   pub async fn delete_user(
       AuthenticatedUser { user_id, .. }: AuthenticatedUser,
       State(user_service): State<UserAppService>,
   ) {
       // 1. Soft delete o hard delete
       // 2. Limpiar datos relacionados
   }
   ```

6. **Reemplazar datos mock**:
   - `get_user_profile`: Query real a BD
   - `get_user_stats`: Calcular desde BD
   - `get_user_followers`: Query real
   - `get_user_following`: Query real

**Archivos a modificar**:
- `services/api-gateway/src/shared/infrastructure/auth/middleware.rs`
- `services/api-gateway/src/bounded_contexts/user/presentation/controllers/user_controller.rs`
- `services/api-gateway/src/bounded_contexts/user/application/services.rs`

**Criterios de éxito**:
- [ ] Todos los handlers protegidos extraen `user_id` del JWT
- [ ] `change_password` funciona
- [ ] `link_wallet` funciona
- [ ] `delete_user` funciona
- [ ] No hay datos mock en respuestas

**Tiempo estimado**: 3-4 días

---

### Tarea 1.3: OpenAPI Spec Completo

**Problema**: No hay spec completo, el frontend no sabe qué endpoints existen.

**Solución**: Documentar todos los endpoints y generar clientes.

#### Pasos de Implementación

1. **Revisar estructura OpenAPI existente**:
   - `services/api-gateway/src/openapi/mod.rs`
   - Verificar qué está documentado

2. **Documentar endpoints críticos**:
   ```rust
   #[utoipa::path(
       post,
       path = "/api/v1/users/register",
       request_body = RegisterUserRequest,
       responses(
           (status = 200, description = "Usuario registrado", body = RegisterUserResponse),
           (status = 400, description = "Error de validación")
       )
   )]
   pub async fn register_user(...) { ... }
   ```

3. **Generar spec completo**:
   - Asegurar que todos los endpoints estén documentados
   - Validar que los tipos coincidan

4. **Servir Swagger UI real**:
   - Verificar que `/swagger-ui` funcione
   - Verificar que `/redoc` funcione

5. **Generar cliente TypeScript**:
   ```bash
   # Usar openapi-generator o similar
   openapi-generator-cli generate \
       -i http://localhost:3000/api-docs/openapi.json \
       -g typescript-angular \
       -o ../frontend/src/api-client
   ```

**Archivos a modificar**:
- `services/api-gateway/src/openapi/mod.rs`
- Agregar `#[utoipa::path(...)]` a todos los handlers

**Criterios de éxito**:
- [ ] Todos los endpoints documentados
- [ ] Swagger UI funcionando
- [ ] Cliente TypeScript generado
- [ ] Spec validado

**Tiempo estimado**: 2-3 días

---

### Tarea 1.4: Base de Datos - Migraciones y Seed

**Problema**: Migraciones incompletas, falta seed data.

**Solución**: Completar migraciones y crear scripts de seed.

#### Pasos de Implementación

1. **Verificar integridad de migraciones**:
   ```bash
   # Ejecutar todas las migraciones
   sqlx migrate run
   
   # Verificar que no hay errores
   ```

2. **Crear script de seed** (`scripts/seed_dev_data.sql`):
   ```sql
   -- Usuarios de prueba
   INSERT INTO users (email, username, password_hash, role, tier) VALUES
   ('admin@vibestream.com', 'admin', '$2b$...', 'admin', 'diamond'),
   ('artist@vibestream.com', 'artist1', '$2b$...', 'artist', 'gold'),
   ('user@vibestream.com', 'user1', '$2b$...', 'user', 'bronze');
   
   -- Canciones de prueba
   -- Artistas de prueba
   -- etc.
   ```

3. **Automatizar migraciones**:
   ```rust
   // En main.rs o startup
   sqlx::migrate!("./migrations")
       .run(&pool)
       .await?;
   ```

4. **Documentar schema**:
   - Crear diagrama ER
   - Documentar relaciones

**Archivos a crear/modificar**:
- `scripts/seed_dev_data.sql`
- `services/api-gateway/src/main.rs` → Agregar migraciones automáticas

**Criterios de éxito**:
- [ ] Todas las migraciones ejecutan sin errores
- [ ] Seed data disponible para desarrollo
- [ ] Migraciones automatizadas en startup

**Tiempo estimado**: 1-2 días

---

## 📅 Fase 2: Endpoints Críticos (Semana 3-4)

### Tarea 2.1: Music Gateway - Endpoints Básicos

**Problema**: Todos los endpoints son mock.

**Solución**: Implementar endpoints básicos con lógica real.

#### Pasos de Implementación

1. **Conectar repositorios**:
   ```rust
   // En music_gateway.rs
   let pool = app_state.get_db_pool();
   let song_repository = Arc::new(PostgresSongRepository::new(pool.clone()));
   ```

2. **Implementar `GET /api/v1/music/songs`**:
   ```rust
   async fn get_songs(
       Query(params): Query<SongQueryParams>,
       State(repo): State<Arc<PostgresSongRepository>>,
   ) -> Result<Json<SongListResponse>> {
       let songs = repo.find_all(params.page, params.page_size).await?;
       Ok(Json(SongListResponse { songs, ... }))
   }
   ```

3. **Implementar `GET /api/v1/music/songs/:id`**:
   ```rust
   async fn get_song(
       Path(song_id): Path<Uuid>,
       State(repo): State<Arc<PostgresSongRepository>>,
   ) -> Result<Json<SongResponse>> {
       let song = repo.find_by_id(song_id).await?
           .ok_or(StatusCode::NOT_FOUND)?;
       Ok(Json(song.into()))
   }
   ```

4. **Implementar `POST /api/v1/music/songs`** (básico):
   ```rust
   async fn create_song(
       AuthenticatedUser { user_id, .. }: AuthenticatedUser,
       Json(request): Json<CreateSongRequest>,
       State(repo): State<Arc<PostgresSongRepository>>,
   ) -> Result<Json<SongResponse>> {
       // Validar request
       // Crear song entity
       // Guardar en BD
       // Retornar respuesta
   }
   ```

**Archivos a modificar**:
- `services/api-gateway/src/gateways/music_gateway.rs`
- Conectar a repositorios existentes

**Criterios de éxito**:
- [ ] `GET /songs` retorna canciones reales de BD
- [ ] `GET /songs/:id` retorna canción real
- [ ] `POST /songs` crea canción en BD

**Tiempo estimado**: 3-4 días

---

### Tarea 2.2: Payment Gateway - Verificar Implementación

**Problema**: Repositorios existen pero no sabemos si handlers están completos.

**Solución**: Revisar y completar handlers de pagos.

#### Pasos de Implementación

1. **Revisar `PaymentController`**:
   - Verificar qué handlers están implementados
   - Identificar qué falta

2. **Completar handlers faltantes**:
   - Implementar lógica de negocio
   - Conectar a repositorios

3. **Probar endpoints**:
   - Verificar que funcionen
   - Validar respuestas

**Archivos a revisar**:
- `services/api-gateway/src/bounded_contexts/payment/presentation/controllers/payment_controller.rs`

**Criterios de éxito**:
- [ ] Handlers conectados a repositorios
- [ ] Lógica básica funcionando
- [ ] Endpoints probados

**Tiempo estimado**: 2-3 días

---

### Tarea 2.3: Otros Gateways - Mínimo Viable

**Problema**: Todos los gateways son mock.

**Solución**: Implementar al menos un endpoint funcional en cada gateway.

#### Pasos de Implementación

1. **Campaign Gateway**:
   - `GET /api/v1/campaigns` - Listar campañas

2. **Listen Reward Gateway**:
   - `GET /api/v1/listen-rewards/sessions` - Listar sesiones

3. **Fan Ventures Gateway**:
   - `GET /api/v1/fan-ventures/ventures` - Listar ventures

4. **Notification Gateway**:
   - `GET /api/v1/notifications` - Listar notificaciones

5. **Fan Loyalty Gateway**:
   - `GET /api/v1/fan-loyalty/verifications` - Listar verificaciones

**Criterios de éxito**:
- [ ] Al menos un endpoint funcional por gateway
- [ ] Conectado a base de datos
- [ ] Retorna datos reales

**Tiempo estimado**: 3-4 días

---

## 📅 Fase 3: Mejoras y Hardening (Semana 5-6)

### Tarea 3.1: Testing Completo (Unit, Integration, E2E)

**Problema**: Tests existen pero están ignorados, falta cobertura completa.

**Solución**: Implementar suite completa de tests con testcontainers.

#### 3.1.1 Configurar Infraestructura de Testing

**Agregar dependencias a `Cargo.toml`**:
```toml
[dev-dependencies]
# Testcontainers para servicios de prueba
testcontainers = "0.15"
testcontainers-modules-postgres = "0.15"
testcontainers-modules-redis = "0.15"

# Testing adicional
mockall = "0.12"  # Para mocks
wiremock = "0.5"  # Para mock HTTP servers
criterion = "0.5"  # Para benchmarks
```

**Crear helper de testing** (`tests/helpers/test_setup.rs`):
```rust
use testcontainers::{clients, images::postgres::Postgres, images::redis::Redis};
use sqlx::PgPool;

pub struct TestServices {
    pub postgres: Postgres,
    pub redis: Redis,
    pub db_pool: PgPool,
    pub redis_url: String,
}

impl TestServices {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let docker = clients::Cli::default();
        
        // Iniciar PostgreSQL
        let postgres = docker.run(Postgres::default());
        let postgres_url = format!(
            "postgresql://postgres:postgres@localhost:{}/postgres",
            postgres.get_host_port_ipv4(5432)
        );
        
        // Iniciar Redis
        let redis = docker.run(Redis::default());
        let redis_url = format!(
            "redis://localhost:{}",
            redis.get_host_port_ipv4(6379)
        );
        
        // Crear pool de conexiones
        let db_pool = PgPool::connect(&postgres_url).await?;
        
        // Ejecutar migraciones
        sqlx::migrate!("../../migrations")
            .run(&db_pool)
            .await?;
        
        Ok(Self {
            postgres,
            redis,
            db_pool,
            redis_url,
        })
    }
    
    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Limpiar datos de test
        sqlx::query("TRUNCATE TABLE users CASCADE")
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}
```

#### 3.1.2 Unit Tests - Servicios de Dominio

**Crear tests unitarios** (`tests/unit/user_service_tests.rs`):
```rust
use api_gateway::bounded_contexts::user::application::services::UserApplicationService;
use api_gateway::shared::infrastructure::database::postgres::PostgresUserRepository;
use mockall::predicate::*;

#[tokio::test]
async fn test_user_service_create_user() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();
    mock_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(create_test_user()));
    
    let service = UserApplicationService::new(Arc::new(mock_repo));
    
    // Act
    let command = CreateUserCommand {
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        password: "password123".to_string(),
        // ...
    };
    
    let result = service.handle_create_user(command).await;
    
    // Assert
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, "test@example.com");
}

#[tokio::test]
async fn test_user_service_validate_email() {
    let service = UserApplicationService::new(/* ... */);
    
    // Test email válido
    assert!(service.validate_email("test@example.com").is_ok());
    
    // Test email inválido
    assert!(service.validate_email("invalid-email").is_err());
}

#[tokio::test]
async fn test_user_service_password_strength() {
    let service = UserApplicationService::new(/* ... */);
    
    // Password débil
    assert!(service.validate_password("123").is_err());
    
    // Password fuerte
    assert!(service.validate_password("StrongPass123!").is_ok());
}
```

**Tests para otros servicios**:
- `tests/unit/payment_service_tests.rs` - Lógica de pagos
- `tests/unit/music_service_tests.rs` - Lógica de música
- `tests/unit/campaign_service_tests.rs` - Lógica de campañas

#### 3.1.3 Integration Tests - Repositorios

**Crear tests de integración para repositorios** (`tests/integration/repository_tests.rs`):
```rust
use crate::helpers::TestServices;
use api_gateway::shared::infrastructure::database::postgres::PostgresUserRepository;

#[tokio::test]
async fn test_user_repository_create_and_find() {
    let services = TestServices::new().await.unwrap();
    let repo = PostgresUserRepository::new(Arc::new(services.db_pool.clone()));
    
    // Crear usuario
    let user = create_test_user_entity();
    let created = repo.create(user.clone()).await.unwrap();
    
    // Buscar por ID
    let found = repo.find_by_id(created.id.value()).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email.value(), user.email.value());
    
    // Cleanup
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_user_repository_find_by_email() {
    let services = TestServices::new().await.unwrap();
    let repo = PostgresUserRepository::new(Arc::new(services.db_pool.clone()));
    
    // Crear usuario
    let user = create_test_user_entity();
    repo.create(user.clone()).await.unwrap();
    
    // Buscar por email
    let found = repo.find_by_email(user.email.value()).await.unwrap();
    assert!(found.is_some());
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_user_repository_update() {
    let services = TestServices::new().await.unwrap();
    let repo = PostgresUserRepository::new(Arc::new(services.db_pool.clone()));
    
    // Crear y actualizar
    let mut user = create_test_user_entity();
    let created = repo.create(user.clone()).await.unwrap();
    
    user.update_bio("New bio".to_string());
    repo.update(user.clone()).await.unwrap();
    
    // Verificar actualización
    let updated = repo.find_by_id(created.id.value()).await.unwrap().unwrap();
    assert_eq!(updated.bio, Some("New bio".to_string()));
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_user_repository_follow_user() {
    let services = TestServices::new().await.unwrap();
    let repo = PostgresUserRepository::new(Arc::new(services.db_pool.clone()));
    
    // Crear dos usuarios
    let user1 = repo.create(create_test_user_entity()).await.unwrap();
    let user2 = repo.create(create_test_user_entity()).await.unwrap();
    
    // Seguir
    repo.follow_user(user1.id.value(), user2.id.value()).await.unwrap();
    
    // Verificar
    let followers = repo.get_followers(user2.id.value()).await.unwrap();
    assert_eq!(followers.len(), 1);
    assert_eq!(followers[0].id, user1.id);
    
    services.cleanup().await.unwrap();
}
```

**Tests para otros repositorios**:
- `tests/integration/payment_repository_tests.rs`
- `tests/integration/music_repository_tests.rs`
- `tests/integration/campaign_repository_tests.rs`

#### 3.1.4 Integration Tests - Handlers/Controllers

**Crear tests de integración para handlers** (`tests/integration/handler_tests.rs`):
```rust
use crate::helpers::TestServices;
use api_gateway::bounded_contexts::user::presentation::controllers::user_controller;
use axum::http::StatusCode;

#[tokio::test]
async fn test_register_user_handler() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(services.db_pool.clone()).await;
    
    let request_body = json!({
        "email": "test@example.com",
        "username": "testuser",
        "password": "securepass123",
        "confirm_password": "securepass123",
        "terms_accepted": true
    });
    
    let response = app
        .post("/api/v1/users/register")
        .json(&request_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
}

#[tokio::test]
async fn test_login_user_handler() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(services.db_pool.clone()).await;
    
    // Primero registrar
    let register_body = json!({
        "email": "login@example.com",
        "username": "loginuser",
        "password": "pass123",
        "confirm_password": "pass123",
        "terms_accepted": true
    });
    
    app.post("/api/v1/users/register")
        .json(&register_body)
        .send()
        .await
        .unwrap();
    
    // Luego hacer login
    let login_body = json!({
        "credential": "login@example.com",
        "password": "pass123"
    });
    
    let response = app
        .post("/api/v1/users/login")
        .json(&login_body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
}

#[tokio::test]
async fn test_get_user_profile_handler() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(services.db_pool.clone()).await;
    
    // Crear usuario y obtener token
    let token = create_test_user_and_get_token(&app).await;
    
    // Obtener perfil
    let response = app
        .get("/api/v1/users/me")
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["success"], true);
    assert!(body["data"]["email"].is_string());
}

#[tokio::test]
async fn test_follow_user_handler() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(services.db_pool.clone()).await;
    
    // Crear dos usuarios
    let token1 = create_test_user_and_get_token(&app).await;
    let user2_id = create_test_user(&app).await;
    
    // Seguir usuario
    let response = app
        .post(&format!("/api/v1/users/{}/follow", user2_id))
        .bearer_auth(&token1)
        .json(&json!({ "follow": true }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
```

#### 3.1.5 E2E Tests - Flujos Completos

**Crear tests E2E** (`tests/e2e/user_flows_tests.rs`):
```rust
use crate::helpers::TestServices;

#[tokio::test]
async fn test_complete_user_registration_flow() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(services.db_pool.clone()).await;
    
    // 1. Registrar
    let register_response = app
        .post("/api/v1/users/register")
        .json(&create_register_request())
        .send()
        .await
        .unwrap();
    
    assert_eq!(register_response.status(), StatusCode::OK);
    let token = extract_token(&register_response).await;
    
    // 2. Obtener perfil
    let profile_response = app
        .get("/api/v1/users/me")
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    
    assert_eq!(profile_response.status(), StatusCode::OK);
    
    // 3. Actualizar perfil
    let update_response = app
        .put("/api/v1/users/me")
        .bearer_auth(&token)
        .json(&json!({ "bio": "Updated bio" }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(update_response.status(), StatusCode::OK);
    
    // 4. Cambiar contraseña
    let change_pass_response = app
        .post("/api/v1/users/me/change-password")
        .bearer_auth(&token)
        .json(&json!({
            "current_password": "oldpass",
            "new_password": "newpass123",
            "confirm_new_password": "newpass123"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(change_pass_response.status(), StatusCode::OK);
    
    // 5. Login con nueva contraseña
    let login_response = app
        .post("/api/v1/users/login")
        .json(&json!({
            "credential": "test@example.com",
            "password": "newpass123"
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(login_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_complete_music_flow() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(services.db_pool.clone()).await;
    
    // 1. Registrar artista
    let artist_token = create_artist_and_get_token(&app).await;
    
    // 2. Subir canción
    let song_response = app
        .post("/api/v1/music/songs")
        .bearer_auth(&artist_token)
        .json(&create_song_request())
        .send()
        .await
        .unwrap();
    
    let song_id = extract_id(&song_response).await;
    
    // 3. Obtener canción
    let get_response = app
        .get(&format!("/api/v1/music/songs/{}", song_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(get_response.status(), StatusCode::OK);
    
    // 4. Registrar fan
    let fan_token = create_fan_and_get_token(&app).await;
    
    // 5. Reproducir canción
    let listen_response = app
        .post(&format!("/api/v1/music/songs/{}/listen", song_id))
        .bearer_auth(&fan_token)
        .json(&json!({ "duration_seconds": 180 }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(listen_response.status(), StatusCode::OK);
}
```

#### 3.1.6 Contract Tests - OpenAPI

**Crear tests de contrato** (`tests/contract/openapi_tests.rs`):
```rust
use utoipa::OpenApi;

#[tokio::test]
async fn test_openapi_spec_valid() {
    // Cargar spec
    let spec = api_gateway::openapi::ApiDoc::openapi();
    
    // Validar estructura
    assert!(spec.paths.paths.len() > 0, "Debe tener paths definidos");
    assert!(spec.components.is_some(), "Debe tener components");
    
    // Validar que todos los endpoints estén documentados
    let required_paths = vec![
        "/api/v1/users/register",
        "/api/v1/users/login",
        "/api/v1/users/{user_id}",
        "/api/v1/music/songs",
        // ...
    ];
    
    for path in required_paths {
        assert!(
            spec.paths.paths.contains_key(path),
            "Path {} debe estar documentado",
            path
        );
    }
}

#[tokio::test]
async fn test_openapi_schemas_match_handlers() {
    // Validar que los schemas en OpenAPI coincidan con los tipos Rust
    // Esto se puede hacer comparando los tipos serializados
}
```

#### 3.1.7 Performance Tests

**Crear tests de performance** (`tests/performance/load_tests.rs`):
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_user_creation(c: &mut Criterion) {
    c.bench_function("create_user", |b| {
        b.iter(|| {
            // Benchmark creación de usuario
            black_box(create_test_user());
        });
    });
}

fn benchmark_password_hashing(c: &mut Criterion) {
    c.bench_function("hash_password", |b| {
        b.iter(|| {
            PasswordService::hash_password(black_box("testpassword123"))
        });
    });
}

criterion_group!(benches, benchmark_user_creation, benchmark_password_hashing);
criterion_main!(benches);
```

#### 3.1.8 Security Tests

**Crear tests de seguridad** (`tests/security/security_tests.rs`):
```rust
#[tokio::test]
async fn test_sql_injection_prevention() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(services.db_pool.clone()).await;
    
    // Intentar SQL injection
    let malicious_input = "'; DROP TABLE users; --";
    
    let response = app
        .post("/api/v1/users/register")
        .json(&json!({
            "email": malicious_input,
            "username": "test",
            "password": "pass123",
            "confirm_password": "pass123",
            "terms_accepted": true
        }))
        .send()
        .await
        .unwrap();
    
    // Debe rechazar o sanitizar
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_xss_prevention() {
    // Test que los inputs se sanitizan correctamente
}

#[tokio::test]
async fn test_rate_limiting() {
    // Test que el rate limiting funciona
    let app = create_test_app().await;
    
    // Hacer muchas requests rápidas
    for _ in 0..100 {
        let _ = app.get("/api/v1/users").send().await;
    }
    
    // La última debe ser rate limited
    let response = app.get("/api/v1/users").send().await.unwrap();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn test_jwt_token_validation() {
    // Test que tokens inválidos son rechazados
    let app = create_test_app().await;
    
    let response = app
        .get("/api/v1/users/me")
        .header("Authorization", "Bearer invalid_token")
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
```

#### 3.1.9 Configurar CI/CD para Tests

**Crear `.github/workflows/tests.yml`**:
```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_DB: vibestream_test
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      
      redis:
        image: redis:7-alpine
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run unit tests
        run: cargo test --lib -- --test-threads=1
      
      - name: Run integration tests
        run: cargo test --test '*' -- --test-threads=1
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/vibestream_test
          REDIS_URL: redis://localhost:6379
      
      - name: Generate coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

**Criterios de éxito**:
- [ ] Testcontainers configurado y funcionando
- [ ] Unit tests para todos los servicios de dominio (cobertura >80%)
- [ ] Integration tests para todos los repositorios
- [ ] Integration tests para todos los handlers críticos
- [ ] E2E tests para flujos principales (register → login → profile → music)
- [ ] Contract tests validando OpenAPI spec
- [ ] Performance tests para operaciones críticas
- [ ] Security tests (SQL injection, XSS, rate limiting)
- [ ] CI/CD configurado y ejecutando tests automáticamente
- [ ] Cobertura de código >70%

**Tiempo estimado**: 5-7 días

---

### Tarea 3.2: Observabilidad Básica

**Problema**: No hay métricas ni logging estructurado.

**Solución**: Agregar métricas y logging básico.

#### Pasos de Implementación

1. **Logging estructurado**:
   ```rust
   use tracing::{info, error, warn};
   
   // En handlers
   info!(user_id = %user_id, "User registered");
   ```

2. **Métricas básicas**:
   ```rust
   use prometheus::{Counter, Histogram};
   
   // Contadores de requests
   // Histogramas de latencia
   ```

3. **Health checks mejorados**:
   - Verificar BD
   - Verificar Redis
   - Verificar servicios externos

**Criterios de éxito**:
- [ ] Logging estructurado funcionando
- [ ] Métricas básicas expuestas
- [ ] Health checks completos

**Tiempo estimado**: 2 días

---

### Tarea 3.3: Seguridad y Validación

**Problema**: Falta validación de inputs y rate limiting.

**Solución**: Agregar validación y rate limiting.

#### Pasos de Implementación

1. **Validación de inputs**:
   ```rust
   use validator::{Validate, ValidationError};
   
   #[derive(Deserialize, Validate)]
   struct RegisterUserRequest {
       #[validate(email)]
       email: String,
       #[validate(length(min = 3, max = 20))]
       username: String,
       // ...
   }
   ```

2. **Rate limiting**:
   ```rust
   use tower::limit::RateLimitLayer;
   
   // Limitar requests por IP
   ```

3. **Manejo de errores consistente**:
   ```rust
   // Respuestas de error estandarizadas
   struct ErrorResponse {
       error: String,
       message: String,
       code: String,
   }
   ```

**Criterios de éxito**:
- [ ] Validación en todos los endpoints
- [ ] Rate limiting funcionando
- [ ] Errores consistentes

**Tiempo estimado**: 2-3 días

---

## ✅ Checklist Final Pre-Frontend

Antes de que el frontend pueda empezar, verificar:

### Arquitectura
- [ ] Gateway unificado en puerto 3000
- [ ] Todos los endpoints accesibles vía `/api/v1/*`
- [ ] CORS configurado correctamente

### Autenticación
- [ ] Register funciona
- [ ] Login funciona
- [ ] Refresh token funciona
- [ ] Handlers extraen `user_id` del JWT
- [ ] `change_password` funciona
- [ ] `link_wallet` funciona

### Endpoints
- [ ] Al menos 3 endpoints reales en User Gateway
- [ ] Al menos 3 endpoints reales en Music Gateway
- [ ] Al menos 1 endpoint real en otros gateways

### Documentación
- [ ] OpenAPI spec completo
- [ ] Swagger UI funcionando
- [ ] Cliente TypeScript generado

### Base de Datos
- [ ] Migraciones completas
- [ ] Seed data disponible
- [ ] Schema documentado

### Testing
- [ ] Tests de integración funcionando
- [ ] Al menos 10 tests pasando

### Operaciones
- [ ] Health checks funcionando
- [ ] Logging básico funcionando
- [ ] Scripts de desarrollo (docker-compose)

---

## 📊 Métricas de Progreso

### Semana 1-2 (Fase 1)
- Gateway unificado: 0% → 100%
- Autenticación completa: 70% → 100%
- OpenAPI spec: 30% → 100%
- Base de datos: 60% → 100%

### Semana 3-4 (Fase 2)
- Music Gateway: 5% → 40%
- Payment Gateway: 30% → 60%
- Otros Gateways: 5% → 20%

### Semana 5-6 (Fase 3)
- Testing: 15% → 50%
- Observabilidad: 20% → 60%
- Seguridad: 30% → 70%

---

## 🚀 Comenzar Implementación

### Orden Recomendado

1. **Día 1-2**: Gateway unificado
2. **Día 3-5**: Autenticación completa
3. **Día 6-8**: OpenAPI spec
4. **Día 9-10**: Base de datos
5. **Día 11-14**: Music Gateway básico
6. **Día 15-17**: Payment Gateway
7. **Día 18-21**: Otros gateways mínimo viable
8. **Día 22-24**: Testing
9. **Día 25-26**: Observabilidad
10. **Día 27-28**: Seguridad

---

## 📝 Notas Importantes

1. **Priorizar**: Hacer primero lo que bloquea al frontend
2. **Iterar**: No intentar hacer todo perfecto de una vez
3. **Probar**: Probar cada cambio antes de continuar
4. **Documentar**: Documentar decisiones y cambios
5. **Comunicar**: Mantener al equipo informado del progreso

---

> **Última actualización**: Diciembre 2024  
> **Próxima revisión**: Al completar Fase 1

