// Music Context REST Routes
// Complete REST API routes for Music bounded context

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use super::controllers::*;
use super::controllers::upload_controller::{
    AudioUploadController, upload_audio, get_upload_progress, 
    get_streaming_url, delete_audio
};
use super::controllers::video_upload_controller::{
    VideoUploadController, upload_video, get_video_streaming, get_video_chunk,
    get_video_metadata, get_video_upload_progress, delete_video
};
use crate::bounded_contexts::music::infrastructure::storage::StorageConfig;

/// Create all Music Context REST routes
/// 
/// This function creates a complete REST API for the Music bounded context including:
/// - Songs: CRUD operations, search, trending, analytics  
/// - Albums: CRUD operations, track management
/// - Playlists: CRUD operations, collaborative features
/// - Artists: Profile management, verification
/// - Analytics: Listen tracking, metrics, insights
/// - Audio Upload: File upload, processing, streaming
pub fn create_music_routes() -> Router {
    // Create upload controllers
    let storage_config = StorageConfig::Local {
        base_path: "./storage/audio".to_string(),
        max_file_size: 100 * 1024 * 1024, // 100MB
    };
    let upload_controller = Arc::new(AudioUploadController::new(storage_config));
    let video_upload_controller = Arc::new(VideoUploadController::new());

    Router::new()
        // Song endpoints
        .route("/songs", post(create_song))
        .route("/songs/search", get(search_songs))
        .route("/songs/trending", get(get_trending_songs))
        .route("/songs/:id", get(get_song))
        .route("/songs/:id", put(update_song))
        .route("/songs/:id", delete(delete_song))
        
        // Audio upload endpoints
        .route("/songs/upload", post(upload_audio))
        .route("/songs/upload/:upload_id/progress", get(get_upload_progress))
        .route("/songs/:song_id/stream", get(get_streaming_url))
        .route("/songs/:song_id/audio", delete(delete_audio))
        
        // Video upload endpoints
        .route("/videos/upload", post(upload_video))
        .route("/videos/upload/:upload_id/progress", get(get_video_upload_progress))
        .route("/videos/:video_id/stream", get(get_video_streaming))
        .route("/videos/:video_id/chunks/:chunk_index", get(get_video_chunk))
        .route("/videos/:video_id/metadata", get(get_video_metadata))
        .route("/videos/:video_id", delete(delete_video))
        
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
        
        // Add upload controllers as state
        .with_state((upload_controller, video_upload_controller))
} 