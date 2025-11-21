# Estrategia Completa de Testing para Rust - VibeStream Backend

> **Objetivo**: Implementar suite completa de tests (Unit, Integration, E2E) para garantizar calidad y confiabilidad del backend antes del frontend.

---

## 📊 Estado Actual de Testing

### Tests Existentes

| Tipo | Archivo | Estado | Cobertura |
|------|---------|--------|-----------|
| Integration | `register_login_integration_tests.rs` | ⚠️ Ignorado | 5 tests |
| Integration | `message_queue_async_tests.rs` | ⚠️ Ignorado | 4 tests |
| Fixtures | `fixtures.rs` | ✅ Activo | - |
| Helpers | `helpers/` | ✅ Activo | - |

### Problemas Identificados

1. **Tests ignorados**: Requieren servicios manuales (Postgres/Redis)
2. **Sin testcontainers**: Tests no son portables
3. **Falta cobertura**: No hay unit tests, E2E tests, contract tests
4. **Sin CI/CD**: Tests no se ejecutan automáticamente
5. **Sin métricas**: Cobertura de código desconocida

---

## 🎯 Objetivos de Testing

### Cobertura Objetivo

| Capa | Cobertura Actual | Cobertura Objetivo |
|------|------------------|-------------------|
| **Domain Services** | 0% | >80% |
| **Repositories** | 10% | >90% |
| **Handlers/Controllers** | 15% | >70% |
| **Middleware** | 0% | >80% |
| **Overall** | ~15% | >70% |

### Tipos de Tests Requeridos

1. ✅ **Unit Tests** - Lógica de negocio aislada
2. ✅ **Integration Tests** - Repositorios y handlers
3. ✅ **E2E Tests** - Flujos completos
4. ✅ **Contract Tests** - Validar OpenAPI spec
5. ✅ **Performance Tests** - Benchmarks y load testing
6. ✅ **Security Tests** - SQL injection, XSS, rate limiting

---

## 🏗️ Arquitectura de Testing

### Estructura de Directorios

```
services/api-gateway/
├── src/
│   └── ... (código fuente)
├── tests/
│   ├── mod.rs                    # Módulo principal
│   ├── helpers/
│   │   ├── mod.rs
│   │   ├── test_setup.rs         # Testcontainers y setup
│   │   ├── test_client.rs        # Cliente HTTP para tests
│   │   └── mocks.rs              # Mocks reutilizables
│   ├── unit/
│   │   ├── mod.rs
│   │   ├── user_service_tests.rs
│   │   ├── payment_service_tests.rs
│   │   ├── music_service_tests.rs
│   │   └── campaign_service_tests.rs
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── repository_tests.rs
│   │   ├── handler_tests.rs
│   │   ├── middleware_tests.rs
│   │   └── database_tests.rs
│   ├── e2e/
│   │   ├── mod.rs
│   │   ├── user_flows_tests.rs
│   │   ├── music_flows_tests.rs
│   │   ├── payment_flows_tests.rs
│   │   └── campaign_flows_tests.rs
│   ├── contract/
│   │   ├── mod.rs
│   │   └── openapi_tests.rs
│   ├── performance/
│   │   ├── mod.rs
│   │   └── benchmarks.rs
│   └── security/
│       ├── mod.rs
│       └── security_tests.rs
└── Cargo.toml
```

---

## 📦 Dependencias de Testing

### Agregar a `Cargo.toml`

```toml
[dev-dependencies]
# Testcontainers para servicios de prueba
testcontainers = "0.15"
testcontainers-modules-postgres = "0.15"
testcontainers-modules-redis = "0.15"

# Mocks
mockall = "0.12"

# HTTP testing
reqwest = { version = "0.11", features = ["json"] }
wiremock = "0.5"  # Para mock HTTP servers

# Benchmarks
criterion = "0.5"

# Testing utilities
tokio-test = "0.4"
assert_matches = "1.5"
pretty_assertions = "1.4"
fake = { version = "2.9", features = ["derive", "chrono", "uuid"] }
serial_test = "3.0"  # Para tests que no pueden ejecutarse en paralelo

# Coverage (opcional)
cargo-tarpaulin = "0.27"
```

---

## 🔧 Implementación Paso a Paso

### Paso 1: Configurar Testcontainers

**Archivo**: `tests/helpers/test_setup.rs`

```rust
use testcontainers::{
    clients::Cli,
    images::{
        postgres::Postgres,
        redis::Redis,
    },
    Container, Docker,
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub struct TestServices {
    _postgres: Container<'_, Postgres>,
    _redis: Container<'_, Redis>,
    pub db_pool: PgPool,
    pub redis_url: String,
    pub postgres_url: String,
}

impl TestServices {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let docker = Cli::default();
        
        // Iniciar PostgreSQL
        let postgres_image = Postgres::default()
            .with_user("postgres")
            .with_password("postgres")
            .with_db_name("vibestream_test");
        
        let postgres = docker.run(postgres_image);
        let postgres_port = postgres.get_host_port_ipv4(5432);
        let postgres_url = format!(
            "postgresql://postgres:postgres@localhost:{}/vibestream_test",
            postgres_port
        );
        
        // Iniciar Redis
        let redis_image = Redis::default();
        let redis = docker.run(redis_image);
        let redis_port = redis.get_host_port_ipv4(6379);
        let redis_url = format!("redis://localhost:{}", redis_port);
        
        // Esperar a que PostgreSQL esté listo
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Crear pool de conexiones
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&postgres_url)
            .await?;
        
        // Ejecutar migraciones
        sqlx::migrate!("../../migrations")
            .run(&db_pool)
            .await?;
        
        Ok(Self {
            _postgres: postgres,
            _redis: redis,
            db_pool,
            redis_url,
            postgres_url,
        })
    }
    
    pub async fn cleanup(&self) -> Result<(), sqlx::Error> {
        // Limpiar datos de test (opcional, testcontainers limpia automáticamente)
        sqlx::query("TRUNCATE TABLE users, songs, campaigns CASCADE")
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}

// Helper para crear AppState de test
impl TestServices {
    pub async fn create_app_state(&self) -> Result<AppState, Box<dyn std::error::Error>> {
        AppState::new(&self.postgres_url, &self.redis_url).await
    }
}
```

### Paso 2: Helper para Cliente HTTP

**Archivo**: `tests/helpers/test_client.rs`

```rust
use axum::Router;
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;

pub struct TestClient {
    client: Client,
    base_url: String,
    default_token: Option<String>,
}

impl TestClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            default_token: None,
        }
    }
    
    pub fn with_token(mut self, token: String) -> Self {
        self.default_token = Some(token);
        self
    }
    
    pub async fn get(&self, path: &str) -> TestResponse {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.get(&url);
        
        if let Some(ref token) = self.default_token {
            request = request.bearer_auth(token);
        }
        
        let response = request.send().await.unwrap();
        TestResponse { response }
    }
    
    pub async fn post(&self, path: &str, body: Value) -> TestResponse {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.post(&url).json(&body);
        
        if let Some(ref token) = self.default_token {
            request = request.bearer_auth(token);
        }
        
        let response = request.send().await.unwrap();
        TestResponse { response }
    }
    
    // Similar para PUT, DELETE, etc.
}

pub struct TestResponse {
    response: reqwest::Response,
}

impl TestResponse {
    pub fn status(&self) -> reqwest::StatusCode {
        self.response.status()
    }
    
    pub async fn json<T: serde::de::DeserializeOwned>(self) -> T {
        self.response.json().await.unwrap()
    }
    
    pub fn assert_success(&self) {
        assert!(
            self.response.status().is_success(),
            "Expected success, got {}",
            self.response.status()
        );
    }
    
    pub fn assert_status(&self, expected: reqwest::StatusCode) {
        assert_eq!(
            self.response.status(),
            expected,
            "Expected status {}, got {}",
            expected,
            self.response.status()
        );
    }
}

// Helper para crear app de test
pub async fn create_test_app(services: &TestServices) -> Router {
    let app_state = services.create_app_state().await.unwrap();
    // Crear router con todas las rutas
    create_unified_gateway(app_state).await.unwrap()
}
```

### Paso 3: Unit Tests - Servicios de Dominio

**Archivo**: `tests/unit/user_service_tests.rs`

```rust
use api_gateway::bounded_contexts::user::application::services::UserApplicationService;
use api_gateway::bounded_contexts::user::application::handlers::CreateUserCommand;
use mockall::predicate::*;
use std::sync::Arc;

// Mock del repositorio
mockall::mock! {
    UserRepository {}
    
    #[async_trait]
    impl UserRepository for UserRepository {
        async fn create(&self, user: User) -> Result<User, AppError>;
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
        // ... otros métodos
    }
}

#[tokio::test]
async fn test_user_service_create_user_success() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();
    let test_user = create_test_user_entity();
    
    mock_repo
        .expect_create()
        .times(1)
        .with(predicate::function(|u: &User| u.email.value() == "test@example.com"))
        .returning(move |_| Ok(test_user.clone()));
    
    let service = UserApplicationService::new(Arc::new(mock_repo));
    
    // Act
    let command = CreateUserCommand {
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        password: "securepass123".to_string(),
        display_name: None,
        bio: None,
    };
    
    let result = service.handle_create_user(command).await;
    
    // Assert
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email.value(), "test@example.com");
    assert_eq!(user.username.value(), "testuser");
}

#[tokio::test]
async fn test_user_service_validate_email() {
    let service = create_test_user_service();
    
    // Email válido
    assert!(service.validate_email("test@example.com").is_ok());
    assert!(service.validate_email("user.name+tag@example.co.uk").is_ok());
    
    // Email inválido
    assert!(service.validate_email("invalid-email").is_err());
    assert!(service.validate_email("@example.com").is_err());
    assert!(service.validate_email("test@").is_err());
}

#[tokio::test]
async fn test_user_service_password_strength() {
    let service = create_test_user_service();
    
    // Password débil
    assert!(service.validate_password("123").is_err());
    assert!(service.validate_password("short").is_err());
    assert!(service.validate_password("nouppercase123").is_err());
    assert!(service.validate_password("NOLOWERCASE123").is_err());
    assert!(service.validate_password("NoNumbers!").is_err());
    
    // Password fuerte
    assert!(service.validate_password("StrongPass123!").is_ok());
    assert!(service.validate_password("MyP@ssw0rd").is_ok());
}

#[tokio::test]
async fn test_user_service_duplicate_email() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();
    mock_repo
        .expect_find_by_email()
        .times(1)
        .returning(|_| Ok(Some(create_test_user_entity())));
    
    let service = UserApplicationService::new(Arc::new(mock_repo));
    
    // Act
    let command = CreateUserCommand {
        email: "existing@example.com".to_string(),
        // ...
    };
    
    let result = service.handle_create_user(command).await;
    
    // Assert
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::Validation { .. }));
}
```

### Paso 4: Integration Tests - Repositorios

**Archivo**: `tests/integration/repository_tests.rs`

```rust
use crate::helpers::TestServices;
use api_gateway::shared::infrastructure::database::postgres::PostgresUserRepository;
use std::sync::Arc;

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
    let found_user = found.unwrap();
    assert_eq!(found_user.email.value(), user.email.value());
    assert_eq!(found_user.username.value(), user.username.value());
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_user_repository_find_by_email() {
    let services = TestServices::new().await.unwrap();
    let repo = PostgresUserRepository::new(Arc::new(services.db_pool.clone()));
    
    let user = create_test_user_entity();
    repo.create(user.clone()).await.unwrap();
    
    let found = repo.find_by_email(user.email.value()).await.unwrap();
    assert!(found.is_some());
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_user_repository_update() {
    let services = TestServices::new().await.unwrap();
    let repo = PostgresUserRepository::new(Arc::new(services.db_pool.clone()));
    
    let mut user = create_test_user_entity();
    let created = repo.create(user.clone()).await.unwrap();
    
    // Actualizar
    user.update_bio(Some("New bio".to_string()));
    repo.update(user.clone()).await.unwrap();
    
    // Verificar
    let updated = repo.find_by_id(created.id.value()).await.unwrap().unwrap();
    assert_eq!(updated.bio, Some("New bio".to_string()));
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_user_repository_follow_user() {
    let services = TestServices::new().await.unwrap();
    let repo = PostgresUserRepository::new(Arc::new(services.db_pool.clone()));
    
    let user1 = repo.create(create_test_user_entity()).await.unwrap();
    let user2 = repo.create(create_test_user_entity()).await.unwrap();
    
    // Seguir
    repo.follow_user(user1.id.value(), user2.id.value()).await.unwrap();
    
    // Verificar followers
    let followers = repo.get_followers(user2.id.value()).await.unwrap();
    assert_eq!(followers.len(), 1);
    assert_eq!(followers[0].id, user1.id);
    
    // Verificar following
    let following = repo.get_following(user1.id.value()).await.unwrap();
    assert_eq!(following.len(), 1);
    assert_eq!(following[0].id, user2.id);
    
    // Dejar de seguir
    repo.unfollow_user(user1.id.value(), user2.id.value()).await.unwrap();
    
    let followers_after = repo.get_followers(user2.id.value()).await.unwrap();
    assert_eq!(followers_after.len(), 0);
    
    services.cleanup().await.unwrap();
}
```

### Paso 5: Integration Tests - Handlers

**Archivo**: `tests/integration/handler_tests.rs`

```rust
use crate::helpers::{TestServices, TestClient, create_test_app};
use serde_json::json;

#[tokio::test]
async fn test_register_user_handler() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(&services).await;
    let client = TestClient::new("http://localhost:3000".to_string());
    
    let request_body = json!({
        "email": "test@example.com",
        "username": "testuser",
        "password": "securepass123",
        "confirm_password": "securepass123",
        "terms_accepted": true
    });
    
    let response = client.post("/api/v1/users/register", request_body).await;
    
    response.assert_status(reqwest::StatusCode::OK);
    
    let body: serde_json::Value = response.json().await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
    assert!(body["data"]["user_id"].is_string());
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_login_user_handler() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(&services).await;
    let client = TestClient::new("http://localhost:3000".to_string());
    
    // Registrar primero
    let register_body = json!({
        "email": "login@example.com",
        "username": "loginuser",
        "password": "pass123",
        "confirm_password": "pass123",
        "terms_accepted": true
    });
    
    client.post("/api/v1/users/register", register_body).await;
    
    // Login
    let login_body = json!({
        "credential": "login@example.com",
        "password": "pass123"
    });
    
    let response = client.post("/api/v1/users/login", login_body).await;
    
    response.assert_status(reqwest::StatusCode::OK);
    let body: serde_json::Value = response.json().await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_get_user_profile_handler() {
    let services = TestServices::new().await.unwrap();
    let app = create_test_app(&services).await;
    let client = TestClient::new("http://localhost:3000".to_string());
    
    // Crear usuario y obtener token
    let token = create_user_and_get_token(&client).await;
    let client = client.with_token(token);
    
    // Obtener perfil
    let response = client.get("/api/v1/users/me").await;
    
    response.assert_status(reqwest::StatusCode::OK);
    let body: serde_json::Value = response.json().await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["email"].is_string());
    
    services.cleanup().await.unwrap();
}
```

### Paso 6: E2E Tests - Flujos Completos

**Archivo**: `tests/e2e/user_flows_tests.rs`

```rust
use crate::helpers::{TestServices, TestClient, create_test_app};
use serde_json::json;

#[tokio::test]
async fn test_complete_user_registration_flow() {
    let services = TestServices::new().await.unwrap();
    let client = TestClient::new("http://localhost:3000".to_string());
    
    // 1. Registrar
    let register_response = client
        .post("/api/v1/users/register", create_register_request())
        .await;
    
    register_response.assert_success();
    let token = extract_token(&register_response).await;
    let client = client.with_token(token);
    
    // 2. Obtener perfil
    let profile_response = client.get("/api/v1/users/me").await;
    profile_response.assert_success();
    
    // 3. Actualizar perfil
    let update_response = client
        .put("/api/v1/users/me", json!({ "bio": "Updated bio" }))
        .await;
    update_response.assert_success();
    
    // 4. Cambiar contraseña
    let change_pass_response = client
        .post("/api/v1/users/me/change-password", json!({
            "current_password": "oldpass",
            "new_password": "newpass123",
            "confirm_new_password": "newpass123"
        }))
        .await;
    change_pass_response.assert_success();
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_complete_music_flow() {
    let services = TestServices::new().await.unwrap();
    let client = TestClient::new("http://localhost:3000".to_string());
    
    // 1. Registrar artista
    let artist_token = create_artist_and_get_token(&client).await;
    let artist_client = client.with_token(artist_token);
    
    // 2. Subir canción
    let song_response = artist_client
        .post("/api/v1/music/songs", create_song_request())
        .await;
    song_response.assert_success();
    let song_id = extract_id(&song_response).await;
    
    // 3. Obtener canción
    let get_response = client.get(&format!("/api/v1/music/songs/{}", song_id)).await;
    get_response.assert_success();
    
    // 4. Registrar fan
    let fan_token = create_fan_and_get_token(&client).await;
    let fan_client = client.with_token(fan_token);
    
    // 5. Reproducir canción
    let listen_response = fan_client
        .post(&format!("/api/v1/music/songs/{}/listen", song_id), json!({
            "duration_seconds": 180
        }))
        .await;
    listen_response.assert_success();
    
    services.cleanup().await.unwrap();
}
```

### Paso 7: Security Tests

**Archivo**: `tests/security/security_tests.rs`

```rust
use crate::helpers::{TestServices, TestClient};

#[tokio::test]
async fn test_sql_injection_prevention() {
    let services = TestServices::new().await.unwrap();
    let client = TestClient::new("http://localhost:3000".to_string());
    
    let malicious_input = "'; DROP TABLE users; --";
    
    let response = client
        .post("/api/v1/users/register", json!({
            "email": malicious_input,
            "username": "test",
            "password": "pass123",
            "confirm_password": "pass123",
            "terms_accepted": true
        }))
        .await;
    
    // Debe rechazar o sanitizar
    assert!(response.status().is_client_error());
    
    // Verificar que la tabla aún existe
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&services.db_pool)
        .await
        .unwrap();
    
    assert!(count >= 0); // Tabla existe
    
    services.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_jwt_token_validation() {
    let services = TestServices::new().await.unwrap();
    let client = TestClient::new("http://localhost:3000".to_string());
    
    // Token inválido
    let response = client
        .with_token("invalid_token".to_string())
        .get("/api/v1/users/me")
        .await;
    
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
    
    // Token expirado
    let expired_token = create_expired_token();
    let response = client
        .with_token(expired_token)
        .get("/api/v1/users/me")
        .await;
    
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_rate_limiting() {
    let services = TestServices::new().await.unwrap();
    let client = TestClient::new("http://localhost:3000".to_string());
    
    // Hacer muchas requests rápidas
    for _ in 0..100 {
        let _ = client.get("/api/v1/users").await;
    }
    
    // La última debe ser rate limited
    let response = client.get("/api/v1/users").await;
    assert_eq!(response.status(), reqwest::StatusCode::TOO_MANY_REQUESTS);
}
```

---

## 🚀 Ejecutar Tests

### Comandos

```bash
# Todos los tests
cargo test

# Solo unit tests
cargo test --lib

# Solo integration tests
cargo test --test '*'

# Solo E2E tests
cargo test --test e2e

# Con coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Benchmarks
cargo bench
```

### CI/CD

Ver `.github/workflows/tests.yml` en el plan de acción para configuración completa.

---

## ✅ Checklist de Implementación

### Infraestructura
- [ ] Testcontainers configurado
- [ ] Helpers de testing creados
- [ ] Mocks configurados
- [ ] CI/CD configurado

### Unit Tests
- [ ] UserService tests (>80% cobertura)
- [ ] PaymentService tests
- [ ] MusicService tests
- [ ] CampaignService tests

### Integration Tests
- [ ] Repository tests (todos los repositorios)
- [ ] Handler tests (todos los handlers críticos)
- [ ] Middleware tests

### E2E Tests
- [ ] User flow completo
- [ ] Music flow completo
- [ ] Payment flow completo

### Otros Tests
- [ ] Contract tests (OpenAPI)
- [ ] Performance tests
- [ ] Security tests

---

## 📊 Métricas de Éxito

- **Cobertura de código**: >70%
- **Tests unitarios**: >100 tests
- **Tests de integración**: >50 tests
- **Tests E2E**: >10 flujos completos
- **Tiempo de ejecución**: <5 minutos en CI

---

> **Tiempo estimado de implementación**: 5-7 días  
> **Prioridad**: ALTA - Necesario antes de empezar frontend

