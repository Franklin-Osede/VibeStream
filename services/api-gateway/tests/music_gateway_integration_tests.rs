//! Music Gateway Integration Tests - TDD Approach
//! 
//! Verifica que el Music Gateway usa controladores reales conectados a PostgreSQL
//! en lugar de funciones mock que retornan TODO.
//! 
//! Usa testcontainers para levantar PostgreSQL y Redis automáticamente

use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
};
use serde_json::{json, Value};
use tower::ServiceExt;
use uuid::Uuid;

use api_gateway::gateways::create_music_gateway;
use api_gateway::shared::infrastructure::app_state::AppState;
use api_gateway::shared::infrastructure::auth::{JwtService, Claims};

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

/// Helper para crear un request sin autenticación
fn create_unauthenticated_request(method: &str, path: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

/// Helper para configurar testcontainers y crear Music Gateway
async fn setup_test_environment() -> (TestContainersSetup, AppState, axum::Router) {
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    
    let app = create_music_gateway(app_state.clone())
        .await
        .expect("Failed to create music gateway");
    
    (setup, app_state, app)
}

// =============================================================================
// TESTS PARA RUTAS PÚBLICAS - Verificar que usan controladores reales
// =============================================================================

#[tokio::test]
async fn test_get_songs_uses_real_controller() {
    // Arrange: Setup testcontainers
    let (setup, _app_state, app) = setup_test_environment().await;
    
    // Act: GET /songs sin autenticación (ruta pública)
    let request = create_unauthenticated_request("GET", "/songs");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK
    assert_eq!(response.status(), StatusCode::OK);
    
    // Verificar que NO es un mensaje TODO
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();
    
    // No debe contener mensaje TODO
    let message = json_response.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !message.contains("TODO"),
        "GET /songs should use real controller, not TODO handler. Response: {:?}",
        json_response
    );
    
    // Debe tener estructura de respuesta real (songs array, total, etc.)
    assert!(
        json_response.get("songs").is_some() || json_response.get("data").is_some(),
        "Response should have songs or data field. Response: {:?}",
        json_response
    );
}

#[tokio::test]
async fn test_get_albums_uses_real_controller() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: GET /albums
    let request = create_unauthenticated_request("GET", "/albums");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK y no ser TODO
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();
    
    let message = json_response.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !message.contains("TODO"),
        "GET /albums should use real controller. Response: {:?}",
        json_response
    );
}

#[tokio::test]
async fn test_get_playlists_uses_real_controller() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: GET /playlists
    let request = create_unauthenticated_request("GET", "/playlists");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK y no ser TODO
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();
    
    let message = json_response.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !message.contains("TODO"),
        "GET /playlists should use real controller. Response: {:?}",
        json_response
    );
}

// =============================================================================
// TESTS PARA RUTAS PROTEGIDAS - Verificar que usan controladores reales
// =============================================================================

#[tokio::test]
async fn test_create_song_uses_real_controller() {
    // Arrange: Necesitamos un usuario artista y token
    let (_setup, _app_state, app) = setup_test_environment().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id, "artist");
    
    // Act: POST /songs con autenticación
    let song_data = json!({
        "title": "Test Song",
        "artist_id": user_id,
        "duration_seconds": 180,
        "genre": "Electronic",
        "royalty_percentage": 80.0
    });
    
    let request = create_authenticated_request("POST", "/songs", &token, Some(song_data));
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK o 201 CREATED
    assert!(
        response.status().is_success(),
        "POST /songs should succeed. Status: {}",
        response.status()
    );
    
    // Verificar que NO es un mensaje TODO
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();
    
    let message = json_response.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !message.contains("TODO"),
        "POST /songs should use real controller. Response: {:?}",
        json_response
    );
}

#[tokio::test]
async fn test_create_album_uses_real_controller() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id, "artist");
    
    // Act: POST /albums
    let album_data = json!({
        "title": "Test Album",
        "artist_id": user_id,
        "description": "Test album description"
    });
    
    let request = create_authenticated_request("POST", "/albums", &token, Some(album_data));
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert
    assert!(response.status().is_success());
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();
    
    let message = json_response.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !message.contains("TODO"),
        "POST /albums should use real controller. Response: {:?}",
        json_response
    );
}

#[tokio::test]
async fn test_create_playlist_uses_real_controller() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id, "user");
    
    // Act: POST /playlists
    let playlist_data = json!({
        "name": "Test Playlist",
        "description": "Test playlist description",
        "is_public": true
    });
    
    let request = create_authenticated_request("POST", "/playlists", &token, Some(playlist_data));
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert
    assert!(response.status().is_success());
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();
    
    let message = json_response.get("message").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !message.contains("TODO"),
        "POST /playlists should use real controller. Response: {:?}",
        json_response
    );
}

// =============================================================================
// TESTS PARA VERIFICAR QUE ENDPOINTS MOCK RETORNAN TODO (esperado)
// =============================================================================

#[tokio::test]
async fn test_discover_songs_is_mock() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: GET /songs/discover (endpoint mock)
    let request = create_unauthenticated_request("GET", "/songs/discover");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar TODO (esto es esperado hasta que se implemente)
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();
    
    // Este endpoint todavía es mock, así que puede retornar TODO
    // Este test documenta el estado actual
    let message = json_response.get("message").and_then(|v| v.as_str()).unwrap_or("");
    if message.contains("TODO") {
        println!("⚠️  GET /songs/discover is still mock (expected)");
    }
}

// =============================================================================
// TESTS DE INTEGRACIÓN COMPLETA - Flujo completo
// =============================================================================

#[tokio::test]
async fn test_complete_music_flow() {
    // Arrange: Setup completo
    let (_setup, _app_state, app) = setup_test_environment().await;
    let artist_id = Uuid::new_v4();
    let token = create_test_token(artist_id, "artist");
    
    // 1. Crear canción
    let song_data = json!({
        "title": "Integration Test Song",
        "artist_id": artist_id,
        "duration_seconds": 240,
        "genre": "Rock",
        "royalty_percentage": 75.0
    });
    
    let create_request = create_authenticated_request("POST", "/songs", &token, Some(song_data));
    let create_response = app.clone().oneshot(create_request).await.expect("Request failed");
    assert!(create_response.status().is_success());
    
    let create_body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    
    // Obtener ID de la canción creada
    let song_id = create_json["data"]["song_id"]
        .as_str()
        .or_else(|| create_json["song_id"].as_str())
        .and_then(|s| Uuid::parse_str(s).ok());
    
    if let Some(id) = song_id {
        // 2. Obtener canción por ID
        let get_request = create_unauthenticated_request("GET", &format!("/songs/{}", id));
        let get_response = app.oneshot(get_request).await.expect("Request failed");
        assert_eq!(get_response.status(), StatusCode::OK);
        
        let get_body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
        let get_json: Value = serde_json::from_slice(&get_body).unwrap();
        
        // Verificar que la canción existe
        assert!(
            get_json.get("song_id").is_some() || get_json.get("data").is_some(),
            "Song should be retrievable. Response: {:?}",
            get_json
        );
    }
}
