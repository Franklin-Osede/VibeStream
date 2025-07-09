// Music Context REST Routes
// Complete REST API routes for Music bounded context

use axum::{
    routing::{get, post, put, delete},
    Router,
};

use super::controllers::*;

/// Create all Music Context REST routes
/// 
/// This function creates a complete REST API for the Music bounded context including:
/// - Songs: CRUD operations, search, trending, analytics  
/// - Albums: CRUD operations, track management
/// - Playlists: CRUD operations, collaborative features
/// - Artists: Profile management, verification
/// - Analytics: Listen tracking, metrics, insights
pub fn create_music_routes() -> Router {
    Router::new()
        // Song endpoints
        .route("/songs", post(create_song))
        .route("/songs/search", get(search_songs))
        .route("/songs/trending", get(get_trending_songs))
        .route("/songs/:id", get(get_song))
        .route("/songs/:id", put(update_song))
        .route("/songs/:id", delete(delete_song))
        
        // Album endpoints  
        .route("/albums", post(create_album))
        .route("/albums/:id", get(get_album))
        .route("/artists/:artist_id/albums", get(get_artist_albums))
        
        // Playlist endpoints
        .route("/playlists", post(create_playlist))
        .route("/playlists/:id", get(get_playlist))
        .route("/playlists/:id/songs", post(add_song_to_playlist))
        
        // Artist endpoints
        .route("/artists/:id", get(get_artist))
        .route("/artists/:id", put(update_artist))
        
        // Analytics endpoints
        .route("/analytics/listen", post(record_listen))
        .route("/analytics", get(get_analytics))
} 