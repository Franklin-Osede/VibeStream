//! Fan Ventures Handlers HTTP Tests
//! 
//! Tests de integración HTTP para los handlers de Fan Ventures.
//! Verifica que los endpoints funcionan correctamente con requests HTTP reales.

use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
};
use serde_json::{json, Value};
use tower::ServiceExt;
use uuid::Uuid;
use chrono::Utc;

use api_gateway::bounded_contexts::fan_ventures::presentation::venture_routes::create_venture_routes;
use axum::extract::FromRequestParts;
use api_gateway::shared::infrastructure::app_state::AppState;
use api_gateway::shared::infrastructure::auth::JwtService;

// Importar testcontainers setup
use crate::testcontainers_setup::TestContainersSetup;

/// Helper para crear un token JWT válido para testing
fn create_test_token(user_id: Uuid, role: &str) -> String {
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_for_testing_only".to_string());
    let jwt_service = JwtService::new(&jwt_secret).expect("Failed to create JWT service");
    
    jwt_service.generate_token_pair(
        user_id,
        "testuser",
        "test@example.com",
        role,
        "bronze",
    ).expect("Failed to generate token").access_token
}

/// Helper para crear un request con autenticación
fn create_authenticated_request(method: &str, path: &str, token: &str, body: Option<Value>) -> Request<Body> {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header(AUTHORIZATION, format!("Bearer {}", token));
    
    if let Some(body_value) = body {
        builder = builder
            .header("content-type", "application/json");
        let body_str = serde_json::to_string(&body_value).unwrap();
        builder.body(Body::from(body_str)).unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    }
}

/// Helper para configurar testcontainers y crear AppState
async fn setup_test_environment() -> (TestContainersSetup, AppState) {
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    
    (setup, app_state)
}

// =============================================================================
// TESTS PARA LIST VENTURES
// =============================================================================

#[tokio::test]
async fn test_list_ventures_success() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id, "user");
    
    let request = create_authenticated_request("GET", "/", &token, None);
    let response = app.oneshot(request).await.expect("Request failed");
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json.get("data").is_some());
    assert!(json["data"].get("ventures").is_some());
    assert!(json["data"].get("total").is_some());
}

#[tokio::test]
async fn test_list_ventures_with_filters() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id, "user");
    
    // Test with category filter
    let request = Request::builder()
        .method("GET")
        .uri("/?category=music")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.expect("Request failed");
    assert_eq!(response.status(), StatusCode::OK);
    
    // Test with status filter
    let request = Request::builder()
        .method("GET")
        .uri("/?status=open")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_ventures_requires_auth() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let request = Request::builder()
        .method("GET")
        .uri("/")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// =============================================================================
// TESTS PARA CREATE VENTURE
// =============================================================================

#[tokio::test]
async fn test_create_venture_success() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let artist_id = Uuid::new_v4();
    let token = create_test_token(artist_id, "artist");
    
    let venture_data = json!({
        "title": "Test Venture",
        "description": "A test venture",
        "category": "music",
        "funding_goal": 10000.0,
        "min_investment": 50.0,
        "max_investment": 1000.0,
        "end_date": Utc::now().to_rfc3339(),
        "tags": ["pop", "album"]
    });
    
    let request = create_authenticated_request("POST", "/", &token, Some(venture_data));
    let response = app.oneshot(request).await.expect("Request failed");
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json.get("data").is_some());
    assert!(json["data"].get("venture_id").is_some());
    assert_eq!(json["data"]["title"], "Test Venture");
}

#[tokio::test]
async fn test_create_venture_requires_auth() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let venture_data = json!({
        "title": "Test Venture",
        "description": "A test venture",
        "funding_goal": 10000.0,
        "min_investment": 50.0
    });
    
    let request = Request::builder()
        .method("POST")
        .uri("/")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&venture_data).unwrap()))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// =============================================================================
// TESTS PARA UPDATE VENTURE
// =============================================================================

#[tokio::test]
async fn test_update_venture_success() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let artist_id = Uuid::new_v4();
    let token = create_test_token(artist_id, "artist");
    
    // First, create a venture
    let venture_data = json!({
        "title": "Original Title",
        "description": "Original description",
        "category": "music",
        "funding_goal": 10000.0,
        "min_investment": 50.0
    });
    
    let create_request = create_authenticated_request("POST", "/", &token, Some(venture_data));
    let create_response = app.clone().oneshot(create_request).await.expect("Request failed");
    assert_eq!(create_response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let venture_id = json["data"]["venture_id"].as_str().unwrap();
    
    // Now update it
    let update_data = json!({
        "title": "Updated Title",
        "description": "Updated description"
    });
    
    let update_request = create_authenticated_request("PUT", &format!("/{}", venture_id), &token, Some(update_data));
    let update_response = app.oneshot(update_request).await.expect("Request failed");
    
    assert_eq!(update_response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(update_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["data"]["title"], "Updated Title");
    assert_eq!(json["data"]["description"], "Updated description");
}

#[tokio::test]
async fn test_update_venture_forbidden_not_owner() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let artist_id = Uuid::new_v4();
    let other_user_id = Uuid::new_v4();
    let artist_token = create_test_token(artist_id, "artist");
    let other_token = create_test_token(other_user_id, "user");
    
    // Create venture as artist
    let venture_data = json!({
        "title": "Artist's Venture",
        "description": "Description",
        "category": "music",
        "funding_goal": 10000.0,
        "min_investment": 50.0
    });
    
    let create_request = create_authenticated_request("POST", "/", &artist_token, Some(venture_data));
    let create_response = app.clone().oneshot(create_request).await.expect("Request failed");
    
    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let venture_id = json["data"]["venture_id"].as_str().unwrap();
    
    // Try to update as different user
    let update_data = json!({
        "title": "Hacked Title"
    });
    
    let update_request = create_authenticated_request("PUT", &format!("/{}", venture_id), &other_token, Some(update_data));
    let update_response = app.oneshot(update_request).await.expect("Request failed");
    
    assert_eq!(update_response.status(), StatusCode::FORBIDDEN);
}

// =============================================================================
// TESTS PARA DELETE VENTURE
// =============================================================================

#[tokio::test]
async fn test_delete_venture_success() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let artist_id = Uuid::new_v4();
    let token = create_test_token(artist_id, "artist");
    
    // Create a venture
    let venture_data = json!({
        "title": "To Delete",
        "description": "Will be deleted",
        "category": "music",
        "funding_goal": 10000.0,
        "min_investment": 50.0
    });
    
    let create_request = create_authenticated_request("POST", "/", &token, Some(venture_data));
    let create_response = app.clone().oneshot(create_request).await.expect("Request failed");
    
    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let venture_id = json["data"]["venture_id"].as_str().unwrap();
    
    // Delete it
    let delete_request = create_authenticated_request("DELETE", &format!("/{}", venture_id), &token, None);
    let delete_response = app.oneshot(delete_request).await.expect("Request failed");
    
    assert_eq!(delete_response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(delete_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json.get("data").is_some());
    assert_eq!(json["data"]["message"], "Venture deleted successfully");
}

#[tokio::test]
async fn test_delete_venture_forbidden_not_owner() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let artist_id = Uuid::new_v4();
    let other_user_id = Uuid::new_v4();
    let artist_token = create_test_token(artist_id, "artist");
    let other_token = create_test_token(other_user_id, "user");
    
    // Create venture as artist
    let venture_data = json!({
        "title": "Artist's Venture",
        "description": "Description",
        "category": "music",
        "funding_goal": 10000.0,
        "min_investment": 50.0
    });
    
    let create_request = create_authenticated_request("POST", "/", &artist_token, Some(venture_data));
    let create_response = app.clone().oneshot(create_request).await.expect("Request failed");
    
    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let venture_id = json["data"]["venture_id"].as_str().unwrap();
    
    // Try to delete as different user
    let delete_request = create_authenticated_request("DELETE", &format!("/{}", venture_id), &other_token, None);
    let delete_response = app.oneshot(delete_request).await.expect("Request failed");
    
    assert_eq!(delete_response.status(), StatusCode::FORBIDDEN);
}

// =============================================================================
// TESTS PARA GET ARTIST VENTURES
// =============================================================================

#[tokio::test]
async fn test_get_artist_ventures_success() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let artist_id = Uuid::new_v4();
    let token = create_test_token(artist_id, "artist");
    
    // Create a couple of ventures
    for i in 0..2 {
        let venture_data = json!({
            "title": format!("Venture {}", i),
            "description": format!("Description {}", i),
            "category": "music",
            "funding_goal": 10000.0,
            "min_investment": 50.0
        });
        
        let create_request = create_authenticated_request("POST", "/", &token, Some(venture_data));
        let _response = app.clone().oneshot(create_request).await.expect("Request failed");
    }
    
    // Get artist ventures
    let get_request = create_authenticated_request("GET", &format!("/artists/{}/ventures", artist_id), &token, None);
    let get_response = app.oneshot(get_request).await.expect("Request failed");
    
    assert_eq!(get_response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(get_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json.get("data").is_some());
    assert!(json["data"].get("ventures").is_some());
    assert!(json["data"].get("total").is_some());
    assert!(json["data"]["total"].as_u64().unwrap() >= 2);
}

#[tokio::test]
async fn test_get_artist_ventures_empty() {
    let (_setup, app_state) = setup_test_environment().await;
    let app = create_venture_routes().with_state(app_state);
    
    let artist_id = Uuid::new_v4();
    let token = create_test_token(artist_id, "artist");
    
    // Get ventures for artist with no ventures
    let get_request = create_authenticated_request("GET", &format!("/artists/{}/ventures", artist_id), &token, None);
    let get_response = app.oneshot(get_request).await.expect("Request failed");
    
    assert_eq!(get_response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(get_response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["data"]["total"], 0);
    assert!(json["data"]["ventures"].as_array().unwrap().is_empty());
}

