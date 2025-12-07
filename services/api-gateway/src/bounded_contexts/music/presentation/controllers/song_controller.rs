use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json as ResponseJson,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::shared::infrastructure::app_state::MusicAppState;
use crate::shared::infrastructure::auth::AuthenticatedUser;
use crate::bounded_contexts::music::domain::entities::Song;
use crate::bounded_contexts::music::domain::value_objects::{SongTitle, SongDuration, Genre, RoyaltyPercentage};
use crate::bounded_contexts::music::domain::repositories::SongRepository;
use crate::bounded_contexts::orchestrator::DomainEvent;
use crate::shared::domain::errors::AppError;

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateSongRequest {
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub royalty_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct CreateSongResponse {
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub royalty_percentage: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct SongResponse {
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub royalty_percentage: f64,
    pub listen_count: u64,
    pub revenue_generated: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSongRequest {
    pub title: Option<String>,
    pub genre: Option<String>,
    pub royalty_percentage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct SongQuery {
    pub genre: Option<String>,
    pub artist_id: Option<Uuid>,
    pub q: Option<String>, // Search query for title
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SongListResponse {
    pub songs: Vec<SongResponse>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

// =============================================================================
// SONG CONTROLLER
// =============================================================================

pub struct SongController;

impl SongController {
    /// GET /api/v1/music/songs - List songs with optional filters
    /// 
    /// OpenAPI documentation is in `openapi/paths.rs::_get_songs_doc`
    pub async fn get_songs(
        State(state): State<MusicAppState>,
        Query(query): Query<SongQuery>,
    ) -> Result<ResponseJson<SongListResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        let limit = query.limit.unwrap_or(20);
        let offset = query.offset.unwrap_or(0);
        
        // Get songs from repository based on filters
        let songs = if let Some(search_query) = &query.q {
            // Search by title
            state.song_repository
                .search_by_title(search_query, Some(limit + offset))
                .await
                .map_err(|e| {
                    tracing::error!("Error searching songs: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                        "error": "Failed to search songs",
                        "message": format!("{:?}", e)
                    })))
                })?
        } else if let Some(artist_id) = query.artist_id {
            // Filter by artist
            use crate::bounded_contexts::music::domain::ArtistId;
            let artist_id_vo = ArtistId::from_uuid(artist_id);
            state.song_repository
                .find_by_artist(&artist_id_vo)
                .await
                .map_err(|e| {
                    tracing::error!("Error fetching songs by artist: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                        "error": "Failed to fetch songs",
                        "message": format!("{:?}", e)
                    })))
                })?
        } else if let Some(genre_str) = &query.genre {
            // Filter by genre
            use crate::bounded_contexts::music::domain::Genre;
            let genre = Genre::new(genre_str.clone())
                .map_err(|e| {
                    tracing::error!("Invalid genre: {:?}", e);
                    (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                        "error": "Invalid genre",
                        "message": format!("{:?}", e)
                    })))
                })?;
            state.song_repository
                .find_by_genre(&genre)
                .await
                .map_err(|e| {
                    tracing::error!("Error fetching songs by genre: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                        "error": "Failed to fetch songs",
                        "message": format!("{:?}", e)
                    })))
                })?
        } else {
            // Get all songs with pagination
            state.song_repository
                .find_all(limit, offset)
                .await
                .map_err(|e| {
                    tracing::error!("Error fetching songs: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                        "error": "Failed to fetch songs",
                        "message": format!("{:?}", e)
                    })))
                })?
        };
        
        // Apply pagination to filtered results if needed
        let (paginated_songs, total) = if query.q.is_some() || query.artist_id.is_some() || query.genre.is_some() {
            // For filtered results, apply pagination manually
            let total = songs.len();
            let end = (offset + limit).min(total);
            let paginated = songs.into_iter().skip(offset).take(limit).collect();
            (paginated, total)
        } else {
            // For find_all, get total count
            let total = state.song_repository
                .count()
                .await
                .map_err(|e| {
                    tracing::error!("Error counting songs: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                        "error": "Failed to count songs",
                        "message": format!("{:?}", e)
                    })))
                })?;
            (songs, total)
        };
        
        // Convert to response DTOs
        let song_responses: Vec<SongResponse> = paginated_songs
            .into_iter()
            .map(|song| SongResponse {
                song_id: song.id().to_uuid(),
                title: song.title().to_string(),
                artist_id: song.artist_id().to_uuid(),
                duration_seconds: song.duration().seconds(),
                genre: song.genre().to_string(),
                royalty_percentage: song.royalty_percentage().value(),
                listen_count: song.listen_count().value(),
                revenue_generated: song.revenue_generated(),
                created_at: song.created_at(),
                updated_at: song.updated_at(),
            })
            .collect();
        
        let response = SongListResponse {
            songs: song_responses,
            total,
            limit,
            offset,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// POST /api/v1/music/songs - Create a new song
    /// 
    /// OpenAPI documentation is in `openapi/paths.rs::_create_song_doc`
    /// Requires authentication - only artists can create songs
    pub async fn create_song(
        AuthenticatedUser { user_id, role, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        Json(request): Json<CreateSongRequest>,
    ) -> Result<ResponseJson<CreateSongResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Validate that user is an artist or admin
        if role != "artist" && role != "admin" {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "Only artists can create songs"
                })),
            ));
        }

        // Validate that the artist_id matches the authenticated user (unless admin)
        if role != "admin" && request.artist_id != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "You can only create songs for yourself"
                })),
            ));
        }
        // Validate input
        let title = SongTitle::new(request.title.clone())
            .map_err(|e| {
                tracing::error!("Invalid song title: {}", e);
                (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                    "error": "Invalid song title",
                    "message": e
                })))
            })?;
        
        let duration = SongDuration::new(request.duration_seconds)
            .map_err(|e| {
                tracing::error!("Invalid song duration: {}", e);
                (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                    "error": "Invalid song duration",
                    "message": e
                })))
            })?;
        
        let genre = Genre::new(request.genre.clone())
            .map_err(|e| {
                tracing::error!("Invalid genre: {}", e);
                (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                    "error": "Invalid genre",
                    "message": e
                })))
            })?;
        
        let royalty_percentage = RoyaltyPercentage::new(request.royalty_percentage)
            .map_err(|e| {
                tracing::error!("Invalid royalty percentage: {}", e);
                (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                    "error": "Invalid royalty percentage",
                    "message": e
                })))
            })?;
        
        // Create song entity
        let song = Song::new(
            title,
            crate::bounded_contexts::music::domain::value_objects::ArtistId::from_uuid(request.artist_id),
            duration,
            genre,
            royalty_percentage,
        );
        
        // Save to repository
        state.song_repository
            .save(&song)
            .await
            .map_err(|e| {
                tracing::error!("Error saving song: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to save song",
                    "message": format!("{:?}", e)
                })))
            })?;
        
        // Publish domain event
        let event = DomainEvent::SongListened {
            user_id, // Use authenticated user ID
            song_id: song.id().to_uuid(),
            artist_id: song.artist_id().to_uuid(),
            duration_seconds: song.duration().seconds(),
            occurred_at: chrono::Utc::now(),
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish song created event: {:?}", e);
        }
        
        let response = CreateSongResponse {
            song_id: song.id().to_uuid(),
            title: song.title().to_string(),
            artist_id: song.artist_id().to_uuid(),
            duration_seconds: song.duration().seconds(),
            genre: song.genre().to_string(),
            royalty_percentage: song.royalty_percentage().value(),
            created_at: song.created_at(),
        };
        
        Ok(ResponseJson(response))
    }
    
    /// GET /api/v1/music/songs/:id - Get song by ID
    /// 
    /// OpenAPI documentation is in `openapi/paths.rs::_get_song_doc`
    pub async fn get_song(
        State(state): State<MusicAppState>,
        Path(song_id): Path<Uuid>,
    ) -> Result<ResponseJson<SongResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get song from repository
        let song = state.song_repository
            .find_by_id(&crate::bounded_contexts::music::domain::value_objects::SongId::from_uuid(song_id))
            .await
            .map_err(|e| {
                tracing::error!("Error fetching song: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch song",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Song not found",
                    "message": format!("Song with ID {} not found", song_id)
                })))
            })?;
        
        let response = SongResponse {
            song_id: song.id().to_uuid(),
            title: song.title().to_string(),
            artist_id: song.artist_id().to_uuid(),
            duration_seconds: song.duration().seconds(),
            genre: song.genre().to_string(),
            royalty_percentage: song.royalty_percentage().value(),
            listen_count: song.listen_count().value(),
            revenue_generated: song.revenue_generated(),
            created_at: song.created_at(),
            updated_at: song.updated_at(),
        };
        
        Ok(ResponseJson(response))
    }
    
    /// PUT /api/v1/music/songs/:id - Update song
    /// 
    /// OpenAPI documentation is in `openapi/paths.rs::_update_song_doc`
    /// Requires authentication - only song owner or admin can update
    pub async fn update_song(
        AuthenticatedUser { user_id, role, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        Path(song_id): Path<Uuid>,
        Json(request): Json<UpdateSongRequest>,
    ) -> Result<ResponseJson<SongResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get existing song
        let mut song = state.song_repository
            .find_by_id(&crate::bounded_contexts::music::domain::value_objects::SongId::from_uuid(song_id))
            .await
            .map_err(|e| {
                tracing::error!("Error fetching song: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch song",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Song not found",
                    "message": format!("Song with ID {} not found", song_id)
                })))
            })?;

        // Validate permissions: only song owner (artist) or admin can update
        if role != "admin" && song.artist_id().to_uuid() != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "You can only update your own songs"
                })),
            ));
        }
        
        // Update fields if provided
        if let Some(title) = request.title {
            let new_title = SongTitle::new(title)
                .map_err(|e| {
                    tracing::error!("Invalid song title: {}", e);
                    (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                        "error": "Invalid song title",
                        "message": e
                    })))
                })?;
            // TODO: Implement set_title method in Song entity
        }
        
        if let Some(genre) = request.genre {
            let new_genre = Genre::new(genre)
                .map_err(|e| {
                    tracing::error!("Invalid genre: {}", e);
                    (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                        "error": "Invalid genre",
                        "message": e
                    })))
                })?;
            // TODO: Implement set_genre method in Song entity
        }
        
        if let Some(royalty_percentage) = request.royalty_percentage {
            let new_royalty = RoyaltyPercentage::new(royalty_percentage)
                .map_err(|e| {
                    tracing::error!("Invalid royalty percentage: {}", e);
                    (StatusCode::BAD_REQUEST, ResponseJson(serde_json::json!({
                        "error": "Invalid royalty percentage",
                        "message": e
                    })))
                })?;
            // TODO: Implement set_royalty_percentage method in Song entity
        }
        
        // Save updated song
        state.song_repository
            .save(&song)
            .await
            .map_err(|e| {
                tracing::error!("Error updating song: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to update song",
                    "message": format!("{:?}", e)
                })))
            })?;
        
        let response = SongResponse {
            song_id: song.id().to_uuid(),
            title: song.title().to_string(),
            artist_id: song.artist_id().to_uuid(),
            duration_seconds: song.duration().seconds(),
            genre: song.genre().to_string(),
            royalty_percentage: song.royalty_percentage().value(),
            listen_count: song.listen_count().value(),
            revenue_generated: song.revenue_generated(),
            created_at: song.created_at(),
            updated_at: song.updated_at(),
        };
        
        Ok(ResponseJson(response))
    }
    
    /// DELETE /api/v1/music/songs/:id - Delete song
    /// 
    /// OpenAPI documentation is in `openapi/paths.rs::_delete_song_doc`
    /// Requires authentication - only song owner or admin can delete
    pub async fn delete_song(
        AuthenticatedUser { user_id, role, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        Path(song_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Check if song exists
        let song = state.song_repository
            .find_by_id(&crate::bounded_contexts::music::domain::value_objects::SongId::from_uuid(song_id))
            .await
            .map_err(|e| {
                tracing::error!("Error fetching song: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch song",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Song not found",
                    "message": format!("Song with ID {} not found", song_id)
                })))
            })?;

        // Validate permissions: only song owner (artist) or admin can delete
        if role != "admin" && song.artist_id().to_uuid() != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "You can only delete your own songs"
                })),
            ));
        }
        
        // Delete from repository
        state.song_repository
            .delete(&crate::bounded_contexts::music::domain::value_objects::SongId::from_uuid(song_id))
            .await
            .map_err(|e| {
                tracing::error!("Error deleting song: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to delete song",
                    "message": format!("{:?}", e)
                })))
            })?;
        
        Ok(ResponseJson(serde_json::json!({
            "message": "Song deleted successfully",
            "song_id": song_id
        })))
    }
    
    /// GET /api/v1/music/songs/discover - Discover songs
    pub async fn discover_songs(
        State(state): State<MusicAppState>,
        Query(query): Query<SongQuery>,
    ) -> Result<ResponseJson<SongListResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // For now, return all songs as "discovered"
        // TODO: Implement actual discovery algorithm
        Self::get_songs(State(state), Query(query)).await
    }
    
    /// GET /api/v1/music/songs/trending - Get trending songs
    pub async fn get_trending_songs(
        State(state): State<MusicAppState>,
        Query(query): Query<SongQuery>,
    ) -> Result<ResponseJson<SongListResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // For now, return all songs as "trending"
        // TODO: Implement actual trending algorithm based on listen count, likes, etc.
        Self::get_songs(State(state), Query(query)).await
    }
    
    /// POST /api/v1/music/songs/:id/like - Like a song
    pub async fn like_song(
        State(state): State<MusicAppState>,
        Path(song_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement like functionality
        // For now, just publish an event
        let event = DomainEvent::SongLiked {
            user_id: Uuid::new_v4(), // TODO: Get from auth context
            song_id,
            occurred_at: chrono::Utc::now(),
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish song liked event: {:?}", e);
        }
        
        Ok(ResponseJson(serde_json::json!({
            "message": "Song liked successfully",
            "song_id": song_id
        })))
    }
    
    /// POST /api/v1/music/songs/:id/unlike - Unlike a song
    pub async fn unlike_song(
        State(state): State<MusicAppState>,
        Path(song_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement unlike functionality
        Ok(ResponseJson(serde_json::json!({
            "message": "Song unliked successfully",
            "song_id": song_id
        })))
    }
    
    /// POST /api/v1/music/songs/:id/share - Share a song
    pub async fn share_song(
        State(state): State<MusicAppState>,
        Path(song_id): Path<Uuid>,
        Json(share_data): Json<HashMap<String, String>>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        let platform = share_data.get("platform").unwrap_or(&"unknown".to_string()).clone();
        
        let event = DomainEvent::SongShared {
            user_id: Uuid::new_v4(), // TODO: Get from auth context
            song_id,
            platform,
            occurred_at: chrono::Utc::now(),
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish song shared event: {:?}", e);
        }
        
        Ok(ResponseJson(serde_json::json!({
            "message": "Song shared successfully",
            "song_id": song_id
        })))
    }
}
