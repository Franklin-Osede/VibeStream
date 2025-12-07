use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::infrastructure::app_state::MusicAppState;
use crate::shared::infrastructure::auth::AuthenticatedUser;

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateAlbumRequest {
    pub title: String,
    pub artist_id: Uuid,
    pub description: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAlbumRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AlbumResponse {
    pub album_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub description: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub song_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct AlbumListResponse {
    pub albums: Vec<AlbumResponse>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

// =============================================================================
// ALBUM CONTROLLER
// =============================================================================

pub struct AlbumController;

impl AlbumController {
    /// GET /api/v1/music/albums - List albums with pagination
    pub async fn get_albums(
        State(state): State<MusicAppState>,
        Query(query): Query<AlbumQuery>,
    ) -> Result<ResponseJson<AlbumListResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        let limit = query.limit.unwrap_or(20).min(100) as u32; // Max 100 per page
        let offset = query.offset.unwrap_or(0) as u32;
        let page = (offset / limit) + 1;
        let page_size = limit;
        
        // Get albums from repository
        let albums = state.album_repository
            .find_all(page, page_size)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching albums: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch albums",
                    "message": format!("{:?}", e)
                })))
            })?;
        
        // Get total count for pagination
        let total = state.album_repository
            .count()
            .await
            .map_err(|e| {
                tracing::error!("Error counting albums: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to count albums",
                    "message": format!("{:?}", e)
                })))
            })? as usize;
        
        // Convert to response DTOs
        let album_responses: Vec<AlbumResponse> = albums
            .into_iter()
            .map(|album| AlbumResponse {
                album_id: album.id,
                title: album.title,
                artist_id: album.artist_id,
                description: album.description,
                release_date: album.release_date,
                song_count: album.song_count,
                created_at: album.created_at,
                updated_at: album.updated_at,
            })
            .collect();
        
        let response = AlbumListResponse {
            albums: album_responses,
            total,
            limit: limit as usize,
            offset: offset as usize,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// POST /api/v1/music/albums - Create a new album
    /// Requires authentication - only artists can create albums
    pub async fn create_album(
        AuthenticatedUser { user_id, role, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        axum::extract::Json(request): axum::extract::Json<CreateAlbumRequest>,
    ) -> Result<ResponseJson<AlbumResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Validate that user is an artist or admin
        if role != "artist" && role != "admin" {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "Only artists can create albums"
                })),
            ));
        }

        // Validate that the artist_id matches the authenticated user (unless admin)
        if role != "admin" && request.artist_id != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "You can only create albums for yourself"
                })),
            ));
        }

        // Validate request
        if request.title.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": "Invalid request",
                    "message": "Album title cannot be empty"
                })),
            ));
        }

        // Create new album entity
        let album_id = Uuid::new_v4();
        let album = crate::bounded_contexts::music::domain::repositories::album_repository::Album::new(
            album_id,
            request.title,
            request.artist_id,
            request.description,
            request.release_date,
        );

        // Save to repository
        state.album_repository
            .save(&album)
            .await
            .map_err(|e| {
                tracing::error!("Error creating album: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to create album",
                    "message": format!("{:?}", e)
                })))
            })?;

        // Return response
        let response = AlbumResponse {
            album_id: album.id,
            title: album.title,
            artist_id: album.artist_id,
            description: album.description,
            release_date: album.release_date,
            song_count: album.song_count,
            created_at: album.created_at,
            updated_at: album.updated_at,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// GET /api/v1/music/albums/:id - Get album by ID
    pub async fn get_album(
        State(state): State<MusicAppState>,
        Path(album_id): Path<Uuid>,
    ) -> Result<ResponseJson<AlbumResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get album from repository
        let album = state.album_repository
            .find_by_id(&album_id)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching album: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch album",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Album not found",
                    "message": format!("Album with ID {} not found", album_id)
                })))
            })?;
        
        let response = AlbumResponse {
            album_id: album.id,
            title: album.title,
            artist_id: album.artist_id,
            description: album.description,
            release_date: album.release_date,
            song_count: album.song_count,
            created_at: album.created_at,
            updated_at: album.updated_at,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// PUT /api/v1/music/albums/:id - Update album by ID
    /// Requires authentication - only album owner or admin can update
    pub async fn update_album(
        AuthenticatedUser { user_id, role, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        Path(album_id): Path<Uuid>,
        axum::extract::Json(request): axum::extract::Json<UpdateAlbumRequest>,
    ) -> Result<ResponseJson<AlbumResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get existing album
        let mut album = state.album_repository
            .find_by_id(&album_id)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching album: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch album",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Album not found",
                    "message": format!("Album with ID {} not found", album_id)
                })))
            })?;

        // Validate permissions: only album owner (artist) or admin can update
        if role != "admin" && album.artist_id != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "You can only update your own albums"
                })),
            ));
        }

        // Update fields if provided
        if let Some(title) = request.title {
            if title.trim().is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    ResponseJson(serde_json::json!({
                        "error": "Invalid request",
                        "message": "Album title cannot be empty"
                    })),
                ));
            }
            album.title = title;
        }

        if request.description.is_some() {
            album.description = request.description;
        }

        if request.release_date.is_some() {
            album.release_date = request.release_date;
        }

        album.updated_at = Utc::now();

        // Save updated album
        state.album_repository
            .update(&album)
            .await
            .map_err(|e| {
                tracing::error!("Error updating album: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to update album",
                    "message": format!("{:?}", e)
                })))
            })?;

        let response = AlbumResponse {
            album_id: album.id,
            title: album.title,
            artist_id: album.artist_id,
            description: album.description,
            release_date: album.release_date,
            song_count: album.song_count,
            created_at: album.created_at,
            updated_at: album.updated_at,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// DELETE /api/v1/music/albums/:id - Delete album by ID
    /// Requires authentication - only album owner or admin can delete
    pub async fn delete_album(
        AuthenticatedUser { user_id, role, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        Path(album_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Verify album exists
        let album = state.album_repository
            .find_by_id(&album_id)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching album: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch album",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Album not found",
                    "message": format!("Album with ID {} not found", album_id)
                })))
            })?;

        // Validate permissions: only album owner (artist) or admin can delete
        if role != "admin" && album.artist_id != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "You can only delete your own albums"
                })),
            ));
        }

        // Check if album has songs (optional validation)
        if album.song_count > 0 {
            tracing::warn!("Deleting album {} with {} songs", album_id, album.song_count);
        }

        // Delete album
        state.album_repository
            .delete(&album_id)
            .await
            .map_err(|e| {
                tracing::error!("Error deleting album: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to delete album",
                    "message": format!("{:?}", e)
                })))
            })?;

        Ok(ResponseJson(serde_json::json!({
            "success": true,
            "message": "Album deleted successfully",
            "album_id": album_id
        })))
    }
}
