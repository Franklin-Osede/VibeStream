//! Tests para verificar autenticación en Music Gateway
//! 
//! Siguiendo TDD: Estos tests definen el comportamiento esperado antes de implementar
//! 
//! Usa testcontainers para levantar PostgreSQL y Redis automáticamente

use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
};
use tower::ServiceExt;
use serde_json::json;

use api_gateway::shared::infrastructure::app_state::AppState;
use api_gateway::gateways::create_music_gateway;
use api_gateway::shared::infrastructure::auth::{JwtService, Claims};

// Importar testcontainers setup
use crate::testcontainers_setup::TestContainersSetup;

/// Helper para crear un token JWT válido para testing
fn create_test_token(user_id: uuid::Uuid, role: &str) -> String {
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret_change_in_production".to_string());
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
fn create_authenticated_request(method: &str, path: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(path)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap()
}

/// Helper para crear un request sin autenticación
fn create_unauthenticated_request(method: &str, path: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

/// Helper para configurar testcontainers y crear AppState
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
    
    let app = create_music_gateway(app_state.clone()).await.expect("Failed to create music gateway");
    
    (setup, app_state, app)
}

// =============================================================================
// TESTS PARA RUTAS PÚBLICAS (No requieren autenticación)
// =============================================================================

#[tokio::test]
async fn test_get_songs_public_route() {
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
    let app = create_music_gateway(app_state).await.expect("Failed to create music gateway");
    
    // Act: GET /songs sin autenticación
    let request = create_unauthenticated_request("GET", "/songs");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK (ruta pública)
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_song_by_id_public_route() {
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
    let app = create_music_gateway(app_state).await.expect("Failed to create music gateway");
    
    // Act: GET /songs/:id sin autenticación
    let request = create_unauthenticated_request("GET", "/songs/00000000-0000-0000-0000-000000000001");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK o 404 (ruta pública, pero puede no existir)
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_albums_public_route() {
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
    let app = create_music_gateway(app_state).await.expect("Failed to create music gateway");
    
    // Act: GET /albums sin autenticación
    let request = create_unauthenticated_request("GET", "/albums");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK (ruta pública)
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_playlists_public_route() {
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
    let app = create_music_gateway(app_state).await.expect("Failed to create music gateway");
    
    // Act: GET /playlists sin autenticación
    let request = create_unauthenticated_request("GET", "/playlists");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 200 OK (ruta pública)
    assert_eq!(response.status(), StatusCode::OK);
}

// =============================================================================
// TESTS PARA RUTAS PROTEGIDAS (Requieren autenticación)
// =============================================================================

#[tokio::test]
async fn test_create_song_requires_authentication() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: POST /songs sin autenticación
    let request = Request::builder()
        .method("POST")
        .uri("/songs")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"Test Song","artist_id":"00000000-0000-0000-0000-000000000001","duration_seconds":180,"genre":"Rock","royalty_percentage":80.0}"#))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 401 UNAUTHORIZED
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_song_with_valid_token() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    let user_id = uuid::Uuid::new_v4();
    let token = create_test_token(user_id, "artist");
    
    // Act: POST /songs con autenticación válida
    let request = Request::builder()
        .method("POST")
        .uri("/songs")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{"title":"Test Song","artist_id":"{}","duration_seconds":180,"genre":"Rock","royalty_percentage":80.0}}"#,
            user_id
        )))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 201 CREATED o 400 BAD_REQUEST (depende de validación)
    assert!(
        response.status() == StatusCode::CREATED || 
        response.status() == StatusCode::BAD_REQUEST ||
        response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn test_update_song_requires_authentication() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: PUT /songs/:id sin autenticación
    let request = Request::builder()
        .method("PUT")
        .uri("/songs/00000000-0000-0000-0000-000000000001")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"Updated Song"}"#))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 401 UNAUTHORIZED
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_song_requires_authentication() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: DELETE /songs/:id sin autenticación
    let request = create_unauthenticated_request("DELETE", "/songs/00000000-0000-0000-0000-000000000001");
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 401 UNAUTHORIZED
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_album_requires_authentication() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: POST /albums sin autenticación
    let request = Request::builder()
        .method("POST")
        .uri("/albums")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"Test Album","artist_id":"00000000-0000-0000-0000-000000000001"}"#))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 401 UNAUTHORIZED
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_playlist_requires_authentication() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: POST /playlists sin autenticación
    let request = Request::builder()
        .method("POST")
        .uri("/playlists")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"Test Playlist","is_public":true}"#))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 401 UNAUTHORIZED
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_add_song_to_playlist_requires_authentication() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Act: POST /playlists/:id/songs sin autenticación
    let request = Request::builder()
        .method("POST")
        .uri("/playlists/00000000-0000-0000-0000-000000000001/songs")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"song_id":"00000000-0000-0000-0000-000000000002"}"#))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 401 UNAUTHORIZED
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// =============================================================================
// TESTS PARA VALIDACIÓN DE PERMISOS
// =============================================================================

#[tokio::test]
async fn test_create_song_only_allows_artists() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    let user_id = uuid::Uuid::new_v4();
    let token = create_test_token(user_id, "user"); // Rol "user", no "artist"
    
    // Act: POST /songs con token de usuario normal
    let request = Request::builder()
        .method("POST")
        .uri("/songs")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{"title":"Test Song","artist_id":"{}","duration_seconds":180,"genre":"Rock","royalty_percentage":80.0}}"#,
            user_id
        )))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 403 FORBIDDEN (solo artistas pueden crear canciones)
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_song_only_allows_owner() {
    // Arrange
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    let owner_id = uuid::Uuid::new_v4();
    let other_user_id = uuid::Uuid::new_v4();
    
    // Primero crear una canción como owner
    let owner_token = create_test_token(owner_id, "artist");
    // ... crear canción ...
    
    // Intentar actualizar como otro usuario
    let other_token = create_test_token(other_user_id, "artist");
    let request = Request::builder()
        .method("PUT")
        .uri("/songs/00000000-0000-0000-0000-000000000001")
        .header(AUTHORIZATION, format!("Bearer {}", other_token))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"Hacked Song"}"#))
        .unwrap();
    
    let response = app.oneshot(request).await.expect("Request failed");
    
    // Assert: Debe retornar 403 FORBIDDEN (solo owner puede actualizar)
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

