// =============================================================================
// AUTH MIDDLEWARE INTEGRATION TESTS - TDD APPROACH
// =============================================================================
// 
// RED PHASE: Tests que verifican que las rutas protegidas requieren auth
// GREEN PHASE: Aplicar middleware a rutas protegidas
// 
// Usa testcontainers para levantar PostgreSQL y Redis automáticamente

use axum::{
    body::Body,
    http::{Request, StatusCode, HeaderValue},
};
use tower::ServiceExt;
use api_gateway::gateways::create_user_gateway;
use api_gateway::shared::infrastructure::app_state::AppState;
use api_gateway::shared::infrastructure::auth::JwtService;

// Importar testcontainers setup
use crate::testcontainers_setup::TestContainersSetup;

// =============================================================================
// TEST 1: Rutas protegidas deben requerir JWT token
// =============================================================================

#[tokio::test]
async fn test_protected_route_requires_auth() {
    // Arrange: Setup testcontainers y crear gateway
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    let app = create_user_gateway(app_state)
        .await
        .expect("Failed to create user gateway");

    // Act: Intentar acceder a ruta protegida sin token
    let request = Request::builder()
        .method("GET")
        .uri("/test_user_id/profile") // Ruta protegida
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Assert: Debe retornar 401 Unauthorized
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Protected route should require authentication"
    );
}

// =============================================================================
// TEST 2: Rutas públicas NO deben requerir JWT token
// =============================================================================

#[tokio::test]
async fn test_public_routes_no_auth_required() {
    // Arrange: Setup testcontainers
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    let app = create_user_gateway(app_state)
        .await
        .expect("Failed to create user gateway");

    // Act: Acceder a ruta pública sin token
    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Assert: Debe permitir acceso (200 OK)
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Public routes should not require authentication"
    );
}

// =============================================================================
// TEST 3: Rutas protegidas con token válido deben permitir acceso
// =============================================================================

#[tokio::test]
async fn test_protected_route_with_valid_token() {
    // Arrange: Setup testcontainers
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    let app = create_user_gateway(app_state)
        .await
        .expect("Failed to create user gateway");

    // Generar token válido
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret_change_in_production".to_string());
    let jwt_service = JwtService::new(&jwt_secret).expect("Failed to create JWT service");
    
    let token = jwt_service.generate_access_token(
        uuid::Uuid::new_v4(),
        "testuser",
        "test@example.com",
        "user",
        "free",
    ).expect("Failed to generate token");

    // Act: Acceder a ruta protegida con token válido
    let mut request = Request::builder()
        .method("GET")
        .uri("/test_user_id/profile")
        .body(Body::empty())
        .unwrap();
    
    request.headers_mut().insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap()
    );

    let response = app.oneshot(request).await.unwrap();

    // Assert: Debe permitir acceso (no 401)
    assert_ne!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Protected route with valid token should allow access"
    );
}

// =============================================================================
// TEST 4: Rutas protegidas con token inválido deben rechazar
// =============================================================================

#[tokio::test]
async fn test_protected_route_with_invalid_token() {
    // Arrange: Setup testcontainers
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    let app = create_user_gateway(app_state)
        .await
        .expect("Failed to create user gateway");

    // Act: Acceder con token inválido
    let mut request = Request::builder()
        .method("GET")
        .uri("/test_user_id/profile")
        .body(Body::empty())
        .unwrap();
    
    request.headers_mut().insert(
        "authorization",
        HeaderValue::from_str("Bearer invalid_token_12345").unwrap()
    );

    let response = app.oneshot(request).await.unwrap();

    // Assert: Debe retornar 401
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Protected route with invalid token should reject"
    );
}

// =============================================================================
// TEST 5: Register y Login deben ser rutas públicas
// =============================================================================

#[tokio::test]
async fn test_register_and_login_are_public() {
    // Arrange: Setup testcontainers
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    let app = create_user_gateway(app_state)
        .await
        .expect("Failed to create user gateway");

    // Test register (sin token)
    let register_request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"email":"test@test.com","username":"test","password":"pass123","confirm_password":"pass123","terms_accepted":true}"#))
        .unwrap();

    let register_response = app.clone().oneshot(register_request).await.unwrap();
    
    // Register puede fallar por validación, pero NO debe ser 401
    assert_ne!(
        register_response.status(),
        StatusCode::UNAUTHORIZED,
        "Register endpoint should be public (not require auth)"
    );

    // Test login (sin token)
    let login_request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"credential":"test@test.com","password":"pass123"}"#))
        .unwrap();

    let login_response = app.oneshot(login_request).await.unwrap();
    
    // Login puede fallar por credenciales, pero NO debe ser 401
    assert_ne!(
        login_response.status(),
        StatusCode::UNAUTHORIZED,
        "Login endpoint should be public (not require auth)"
    );
}

