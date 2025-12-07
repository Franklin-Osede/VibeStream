// =============================================================================
// USER GATEWAY INTEGRATION TESTS - TDD APPROACH
// =============================================================================
// 
// RED PHASE: Tests que deben fallar hasta que conectemos el gateway a controllers reales
// GREEN PHASE: Implementar conexión mínima para pasar tests
// REFACTOR PHASE: Mejorar implementación
// 
// Usa testcontainers para levantar PostgreSQL y Redis automáticamente

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt;
use api_gateway::gateways::create_user_gateway;
use api_gateway::shared::infrastructure::app_state::AppState;

// Importar testcontainers setup
use crate::testcontainers_setup::TestContainersSetup;

// =============================================================================
// TEST 1: User Gateway debe usar controllers reales para /register
// =============================================================================

#[tokio::test]
async fn test_user_gateway_register_uses_real_controller() {
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

    // Act: Hacer POST a /register con datos válidos
    let request_body = json!({
        "email": "test@example.com",
        "username": "testuser",
        "password": "securepass123",
        "confirm_password": "securepass123",
        "display_name": "Test User",
        "terms_accepted": true
    });

    let request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Assert: Debe retornar 200 OK con datos reales (no TODO)
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();

    // Verificar que NO es un mensaje TODO
    assert!(
        !json_response["message"].as_str().unwrap_or("").contains("TODO"),
        "Gateway should use real controller, not TODO handler"
    );

    // Verificar que tiene datos reales
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["user_id"].is_string());
    assert_eq!(json_response["data"]["username"], "testuser");
    assert_eq!(json_response["data"]["email"], "test@example.com");
    assert!(json_response["data"]["token"].is_string());
}

// =============================================================================
// TEST 2: User Gateway debe usar controllers reales para /login
// =============================================================================

#[tokio::test]
async fn test_user_gateway_login_uses_real_controller() {
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

    // Primero crear un usuario
    let register_body = json!({
        "email": "login@example.com",
        "username": "loginuser",
        "password": "securepass123",
        "confirm_password": "securepass123",
        "terms_accepted": true
    });

    let register_request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();

    let _register_response = app.clone().oneshot(register_request).await.unwrap();

    // Act: Hacer login
    let login_body = json!({
        "credential": "login@example.com",
        "password": "securepass123"
    });

    let login_request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&login_body).unwrap()))
        .unwrap();

    let response = app.oneshot(login_request).await.unwrap();

    // Assert: Debe retornar token JWT real
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();

    assert!(
        !json_response["message"].as_str().unwrap_or("").contains("TODO"),
        "Gateway should use real controller for login"
    );

    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["token"].is_string());
    assert!(json_response["data"]["user_id"].is_string());
}

// =============================================================================
// TEST 3: Health check debe seguir funcionando
// =============================================================================

#[tokio::test]
async fn test_user_gateway_health_check_still_works() {
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

    // Act
    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json_response["status"], "healthy");
    assert_eq!(json_response["service"], "user-gateway");
}


