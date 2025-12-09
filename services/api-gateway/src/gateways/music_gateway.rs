// =============================================================================
// MUSIC GATEWAY - GESTIÓN DE MÚSICA INDEPENDIENTE
// =============================================================================
//
// Este gateway maneja todas las operaciones relacionadas con música:
// - Subida y gestión de canciones
// - Streaming de audio
// - Gestión de álbumes y playlists
// - Búsqueda y descubrimiento musical

use axum::{
    Router,
    routing::{get, post, put, delete},
    response::Json as ResponseJson,
    middleware,
};
use serde_json::json;
use crate::shared::infrastructure::app_state::{AppState, AppStateFactory};
use crate::shared::infrastructure::auth::middleware::jwt_auth_middleware;
use crate::bounded_contexts::music::presentation::controllers::{
    SongController, AlbumController, PlaylistController, ArtistController
};

// =============================================================================
// GATEWAY CREATION
// =============================================================================

/// Crear el gateway de música con todas las rutas y middleware
/// 
/// Conecta a controllers reales que usan repositorios PostgreSQL
pub async fn create_music_gateway(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    // Crear MusicAppState desde AppState usando el factory
    let music_app_state = AppStateFactory::create_music_state(app_state).await
        .map_err(|e| -> Box<dyn std::error::Error> {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))
        })?;
    
    // =============================================================================
    // RUTAS PÚBLICAS (No requieren autenticación)
    // =============================================================================
    let public_routes = Router::new()
        // Health & Info
        .route("/health", get(health_check))
        .route("/info", get(gateway_info))
        
        // Songs - Lectura pública
        .route("/songs", get(SongController::get_songs))
        .route("/songs/:id", get(SongController::get_song))
        
        // Albums - Lectura pública
        .route("/albums", get(AlbumController::get_albums))
        .route("/albums/:id", get(AlbumController::get_album))
        
        // Playlists - Lectura pública
        .route("/playlists", get(PlaylistController::get_playlists))
        .route("/playlists/:id", get(PlaylistController::get_playlist))
        
        // Artists - Lectura pública
        .route("/artists/:id", get(ArtistController::get_artist))
        .route("/artists/:id/albums", get(ArtistController::get_artist_albums))
        
        // Endpoints temporales (públicos por ahora)
        .route("/songs/discover", get(SongController::discover_songs))
        .route("/songs/trending", get(SongController::get_trending_songs))
        .route("/artists", get(get_artists))
        .route("/artists/:id/songs", get(get_artist_songs))
        .route("/search", get(search_music))
        .route("/discover", get(discover_music))
        .route("/genres", get(get_genres))
        .route("/moods", get(get_moods))
        .route("/genres/:genre/songs", get(get_songs_by_genre))
        .route("/moods/:mood/songs", get(get_songs_by_mood))
        .route("/analytics/songs/:id", get(get_song_analytics))
        .route("/analytics/albums/:id", get(get_album_analytics))
        .route("/analytics/artists/:id", get(get_artist_analytics))
        .route("/analytics/playlists/:id", get(get_playlist_analytics))
        .route("/analytics/trending", get(get_trending_analytics))
        .route("/analytics/genres", get(get_genre_analytics));
    
    // =============================================================================
    // RUTAS PROTEGIDAS (Requieren autenticación JWT)
    // =============================================================================
    let protected_routes = Router::new()
        // Songs - Escritura (requiere auth)
        .route("/songs", post(SongController::create_song))
        .route("/songs/:id", put(SongController::update_song))
        .route("/songs/:id", delete(SongController::delete_song))
        
        // Albums - Escritura (requiere auth)
        .route("/albums", post(AlbumController::create_album))
        .route("/albums/:id", put(AlbumController::update_album))
        .route("/albums/:id", delete(AlbumController::delete_album))
        
        // Playlists - Escritura (requiere auth)
        .route("/playlists", post(PlaylistController::create_playlist))
        .route("/playlists/:id/songs", post(PlaylistController::add_song_to_playlist))
        .route("/playlists/:id/songs/:song_id", delete(PlaylistController::remove_song_from_playlist))
        
        // Artists - Escritura (requiere auth)
        // TODO: Implementar ArtistController::update_artist
        // .route("/artists/:id", put(ArtistController::update_artist))
        
        // Endpoints temporales protegidos
        .route("/songs/:id/like", post(like_song))
        .route("/songs/:id/unlike", post(unlike_song))
        .route("/songs/:id/share", post(share_song))
        .route("/admin/songs", get(get_all_songs_admin))
        .route("/admin/songs/:id", put(update_song_admin))
        .route("/admin/songs/:id", delete(delete_song_admin))
        .route("/admin/albums", get(get_all_albums_admin))
        .route("/admin/artists", get(get_all_artists_admin))
        
        // Aplicar middleware de autenticación a todas las rutas protegidas
        .layer(middleware::from_fn(jwt_auth_middleware));
    
    // =============================================================================
    // COMBINAR RUTAS PÚBLICAS Y PROTEGIDAS
    // =============================================================================
    let router = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(music_app_state);

    Ok(router)
}

// =============================================================================
// HEALTH & INFO HANDLERS
// =============================================================================

async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "music-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn gateway_info() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "gateway": "music",
        "description": "Music streaming and management",
        "endpoints": {
            "health": "/health",
            "songs": "/songs",
            "albums": "/albums",
            "playlists": "/playlists",
            "artists": "/artists",
            "search": "/search, /discover",
            "analytics": "/analytics/*",
            "admin": "/admin/*"
        }
    }))
}

// =============================================================================
// TEMPORARY HANDLERS - These endpoints use mock handlers until real controllers are implemented
// =============================================================================
// NOTE: Main CRUD endpoints (songs, albums, playlists) already use real controllers
// These handlers are for discovery, analytics, and admin features that are not yet implemented



// =============================================================================
// SONG INTERACTION HANDLERS
// =============================================================================

async fn like_song() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Like song endpoint - TODO: Implement with real service"
    }))
}

async fn unlike_song() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Unlike song endpoint - TODO: Implement with real service"
    }))
}

async fn share_song() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Share song endpoint - TODO: Implement with real service"
    }))
}

// NOTE: Album and Playlist CRUD handlers removed - these endpoints now use real controllers:
// - AlbumController::get_albums, create_album, get_album, update_album, delete_album
// - PlaylistController::get_playlists, create_playlist, get_playlist, add_song_to_playlist, remove_song_from_playlist

// =============================================================================
// ARTIST MANAGEMENT HANDLERS
// =============================================================================

async fn get_artists() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get artists endpoint - TODO: Implement with real service"
    }))
}

async fn get_artist() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get artist endpoint - TODO: Implement with real service"
    }))
}

async fn get_artist_songs() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get artist songs endpoint - TODO: Implement with real service"
    }))
}

async fn get_artist_albums() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get artist albums endpoint - TODO: Implement with real service"
    }))
}

// =============================================================================
// SEARCH & DISCOVERY HANDLERS
// =============================================================================

async fn search_music() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Search music endpoint - TODO: Implement with real service"
    }))
}

async fn discover_music() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Discover music endpoint - TODO: Implement with real service"
    }))
}

// =============================================================================
// GENRE & MOOD HANDLERS
// =============================================================================

async fn get_genres() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "genres": ["Rock", "Pop", "Jazz", "Classical"],
        "message": "Get genres endpoint - TODO: Implement with real service"
    }))
}

async fn get_moods() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "moods": ["Happy", "Sad", "Energetic", "Calm"],
        "message": "Get moods endpoint - TODO: Implement with real service"
    }))
}

async fn get_songs_by_genre() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get songs by genre endpoint - TODO: Implement with real service"
    }))
}

async fn get_songs_by_mood() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get songs by mood endpoint - TODO: Implement with real service"
    }))
}

// =============================================================================
// ANALYTICS HANDLERS
// =============================================================================

async fn get_song_analytics() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get song analytics endpoint - TODO: Implement with real service"
    }))
}

async fn get_album_analytics() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get album analytics endpoint - TODO: Implement with real service"
    }))
}

async fn get_artist_analytics() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get artist analytics endpoint - TODO: Implement with real service"
    }))
}

async fn get_playlist_analytics() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get playlist analytics endpoint - TODO: Implement with real service"
    }))
}

async fn get_trending_analytics() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get trending analytics endpoint - TODO: Implement with real service"
    }))
}

async fn get_genre_analytics() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get genre analytics endpoint - TODO: Implement with real service"
    }))
}

// =============================================================================
// ADMIN HANDLERS
// =============================================================================

async fn get_all_songs_admin() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get all songs admin endpoint - TODO: Implement with real service"
    }))
}

async fn update_song_admin() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Update song admin endpoint - TODO: Implement with real service"
    }))
}

async fn delete_song_admin() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Delete song admin endpoint - TODO: Implement with real service"
    }))
}

async fn get_all_albums_admin() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get all albums admin endpoint - TODO: Implement with real service"
    }))
}

async fn get_all_artists_admin() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "message": "Get all artists admin endpoint - TODO: Implement with real service"
    }))
}
