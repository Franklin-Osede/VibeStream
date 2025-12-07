// =============================================================================
// REGISTER/LOGIN INTEGRATION TESTS - TDD APPROACH
// =============================================================================
// 
// Tests de integración end-to-end para flujo de autenticación
// Verifica que register y login funcionen correctamente con la base de datos real
// 
// Usa testcontainers para levantar PostgreSQL y Redis automáticamente

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use serde_json::{json, Value};
use api_gateway::gateways::create_user_gateway;
use api_gateway::shared::infrastructure::app_state::AppState;

// Importar testcontainers setup
use crate::testcontainers_setup::TestContainersSetup;

// =============================================================================
// TEST 1: Register debe crear usuario y retornar token
// =============================================================================

#[tokio::test]
async fn test_register_creates_user_and_returns_token() {
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

    // Generar email único para evitar conflictos
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let email = format!("test_{}@example.com", timestamp);
    let username = format!("testuser_{}", timestamp);

    let request_body = json!({
        "email": email,
        "username": username,
        "password": "securepass123",
        "confirm_password": "securepass123",
        "display_name": "Test User",
        "terms_accepted": true
    });

    // Act: Registrar usuario
    let request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Assert: Debe retornar 200 OK con token
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Register should return 200 OK"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");

    // Verificar estructura de respuesta
    assert_eq!(
        json_response["success"],
        true,
        "Response should indicate success"
    );

    // Verificar que tiene datos de usuario
    assert!(
        json_response["data"]["user_id"].is_string(),
        "Response should contain user_id"
    );
    
    assert_eq!(
        json_response["data"]["email"],
        email,
        "Response should contain correct email"
    );
    
    assert_eq!(
        json_response["data"]["username"],
        username,
        "Response should contain correct username"
    );

    // Verificar que tiene token JWT
    assert!(
        json_response["data"]["token"].is_string(),
        "Response should contain JWT token"
    );
    
    let token = json_response["data"]["token"].as_str().unwrap();
    assert!(
        !token.is_empty(),
        "Token should not be empty"
    );
    assert!(
        token.len() > 20,
        "Token should be a valid JWT (longer than 20 chars)"
    );
}

// =============================================================================
// TEST 2: Login debe autenticar usuario existente y retornar token
// =============================================================================

#[tokio::test]
async fn test_login_authenticates_user_and_returns_token() {
    // Arrange: Setup testcontainers y crear un usuario
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

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let email = format!("login_{}@example.com", timestamp);
    let username = format!("loginuser_{}", timestamp);
    let password = "securepass123";

    // Registrar usuario primero
    let register_body = json!({
        "email": email.clone(),
        "username": username.clone(),
        "password": password,
        "confirm_password": password,
        "terms_accepted": true
    });

    let register_request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();

    let register_response = app.clone().oneshot(register_request).await.unwrap();
    assert_eq!(register_response.status(), StatusCode::OK);

    // Act: Hacer login
    let login_body = json!({
        "credential": email.clone(), // Puede ser email o username
        "password": password
    });

    let login_request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&login_body).unwrap()))
        .unwrap();

    let response = app.oneshot(login_request).await.unwrap();

    // Assert: Debe retornar 200 OK con token
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Login should return 200 OK"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");

    assert_eq!(
        json_response["success"],
        true,
        "Login should indicate success"
    );

    // Verificar token
    assert!(
        json_response["data"]["token"].is_string(),
        "Login response should contain JWT token"
    );
    
    assert!(
        json_response["data"]["user_id"].is_string(),
        "Login response should contain user_id"
    );
}

// =============================================================================
// TEST 3: Login con credenciales incorrectas debe fallar
// =============================================================================

#[tokio::test]
async fn test_login_with_wrong_password_fails() {
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

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let email = format!("wrongpass_{}@example.com", timestamp);
    let username = format!("wrongpassuser_{}", timestamp);

    // Registrar usuario
    let register_body = json!({
        "email": email.clone(),
        "username": username.clone(),
        "password": "correctpass123",
        "confirm_password": "correctpass123",
        "terms_accepted": true
    });

    let register_request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();

    let _register_response = app.clone().oneshot(register_request).await.unwrap();

    // Act: Intentar login con password incorrecto
    let login_body = json!({
        "credential": email,
        "password": "wrongpassword"
    });

    let login_request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&login_body).unwrap()))
        .unwrap();

    let response = app.oneshot(login_request).await.unwrap();

    // Assert: Debe retornar 401 Unauthorized
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Login with wrong password should return 401"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");

    assert_eq!(
        json_response["success"],
        false,
        "Login should indicate failure"
    );
}

// =============================================================================
// TEST 4: Register con email duplicado debe fallar
// =============================================================================

#[tokio::test]
async fn test_register_duplicate_email_fails() {
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

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let email = format!("duplicate_{}@example.com", timestamp);
    let username1 = format!("user1_{}", timestamp);
    let username2 = format!("user2_{}", timestamp);

    // Registrar primer usuario
    let register_body1 = json!({
        "email": email.clone(),
        "username": username1,
        "password": "pass123",
        "confirm_password": "pass123",
        "terms_accepted": true
    });

    let register_request1 = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body1).unwrap()))
        .unwrap();

    let _register_response1 = app.clone().oneshot(register_request1).await.unwrap();

    // Act: Intentar registrar con mismo email
    let register_body2 = json!({
        "email": email.clone(),
        "username": username2,
        "password": "pass123",
        "confirm_password": "pass123",
        "terms_accepted": true
    });

    let register_request2 = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body2).unwrap()))
        .unwrap();

    let response = app.oneshot(register_request2).await.unwrap();

    // Assert: Debe retornar 409 Conflict o 400 Bad Request
    assert!(
        response.status() == StatusCode::CONFLICT || response.status() == StatusCode::BAD_REQUEST,
        "Register with duplicate email should return 409 or 400, got {}",
        response.status()
    );
}

// =============================================================================
// TEST 5: Register con passwords que no coinciden debe fallar
// =============================================================================

#[tokio::test]
async fn test_register_password_mismatch_fails() {
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

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let email = format!("mismatch_{}@example.com", timestamp);
    let username = format!("mismatchuser_{}", timestamp);

    // Act: Registrar con passwords que no coinciden
    let register_body = json!({
        "email": email,
        "username": username,
        "password": "pass123",
        "confirm_password": "differentpass",
        "terms_accepted": true
    });

    let request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Assert: Debe retornar 400 Bad Request
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "Register with password mismatch should return 400"
    );
}

