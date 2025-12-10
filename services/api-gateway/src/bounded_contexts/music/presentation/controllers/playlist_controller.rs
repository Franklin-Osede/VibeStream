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
use crate::bounded_contexts::music::domain::repositories::PlaylistRepository;

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct CreatePlaylistRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddSongToPlaylistRequest {
    pub song_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct PlaylistResponse {
    pub playlist_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub song_count: u32,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct PlaylistListResponse {
    pub playlists: Vec<PlaylistResponse>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

// =============================================================================
// PLAYLIST CONTROLLER
// =============================================================================

pub struct PlaylistController;

impl PlaylistController {
    /// GET /api/v1/music/playlists - List playlists with pagination
    pub async fn get_playlists(
        State(state): State<MusicAppState>,
        Query(query): Query<PlaylistQuery>,
    ) -> Result<ResponseJson<PlaylistListResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        let limit = query.limit.unwrap_or(20).min(100) as u32; // Max 100 per page
        let offset = query.offset.unwrap_or(0) as u32;
        let page = (offset / limit) + 1;
        let page_size = limit;
        
        // Get playlists from repository
        let playlists = state.playlist_repository
            .find_all(page, page_size)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching playlists: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch playlists",
                    "message": format!("{:?}", e)
                })))
            })?;
        
        // Get total count for pagination
        let total = state.playlist_repository
            .count()
            .await
            .map_err(|e| {
                tracing::error!("Error counting playlists: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to count playlists",
                    "message": format!("{:?}", e)
                })))
            })? as usize;
        
        // Convert to response DTOs
        let playlist_responses: Vec<PlaylistResponse> = playlists
            .into_iter()
            .map(|playlist| PlaylistResponse {
                playlist_id: playlist.id,
                name: playlist.name,
                description: playlist.description,
                is_public: playlist.is_public,
                song_count: playlist.song_count,
                created_by: playlist.created_by,
                created_at: playlist.created_at,
                updated_at: playlist.updated_at,
            })
            .collect();
        
        let response = PlaylistListResponse {
            playlists: playlist_responses,
            total,
            limit: limit as usize,
            offset: offset as usize,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// POST /api/v1/music/playlists - Create a new playlist
    /// Requires authentication - uses AuthenticatedUser to get creator ID
    pub async fn create_playlist(
        AuthenticatedUser { user_id, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        axum::extract::Json(request): axum::extract::Json<CreatePlaylistRequest>,
    ) -> Result<ResponseJson<PlaylistResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Validate request
        if request.name.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": "Invalid request",
                    "message": "Playlist name cannot be empty"
                })),
            ));
        }

        // Get created_by from authenticated user
        let created_by = user_id;

        // Create new playlist entity
        let playlist_id = Uuid::new_v4();
        let playlist = crate::bounded_contexts::music::domain::repositories::playlist_repository::Playlist::new(
            playlist_id,
            request.name,
            request.description,
            request.is_public,
            created_by,
        );

        // Save to repository
        state.playlist_repository
            .save(&playlist)
            .await
            .map_err(|e| {
                tracing::error!("Error creating playlist: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to create playlist",
                    "message": format!("{:?}", e)
                })))
            })?;

        // Return response
        let response = PlaylistResponse {
            playlist_id: playlist.id,
            name: playlist.name,
            description: playlist.description,
            is_public: playlist.is_public,
            song_count: playlist.song_count,
            created_by: playlist.created_by,
            created_at: playlist.created_at,
            updated_at: playlist.updated_at,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// GET /api/v1/music/playlists/:id - Get playlist by ID
    pub async fn get_playlist(
        State(state): State<MusicAppState>,
        Path(playlist_id): Path<Uuid>,
    ) -> Result<ResponseJson<PlaylistResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get playlist from repository
        let playlist = state.playlist_repository
            .find_by_id(&playlist_id)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching playlist: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch playlist",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Playlist not found",
                    "message": format!("Playlist with ID {} not found", playlist_id)
                })))
            })?;
        
        let response = PlaylistResponse {
            playlist_id: playlist.id,
            name: playlist.name,
            description: playlist.description,
            is_public: playlist.is_public,
            song_count: playlist.song_count,
            created_by: playlist.created_by,
            created_at: playlist.created_at,
            updated_at: playlist.updated_at,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// POST /api/v1/music/playlists/:id/songs - Add song to playlist
    /// Requires authentication - only playlist owner can add songs
    pub async fn add_song_to_playlist(
        AuthenticatedUser { user_id, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        Path(playlist_id): Path<Uuid>,
        axum::extract::Json(request): axum::extract::Json<AddSongToPlaylistRequest>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get playlist to verify ownership
        let playlist = state.playlist_repository
            .find_by_id(&playlist_id)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching playlist: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch playlist",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Playlist not found",
                    "message": format!("Playlist with ID {} not found", playlist_id)
                })))
            })?;

        // Verify ownership (only creator can add songs)
        if playlist.created_by != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "Only the playlist owner can add songs"
                })),
            ));
        }

        // Add song to playlist
        state.playlist_repository
            .add_song(&playlist_id, &request.song_id)
            .await
            .map_err(|e| {
                tracing::error!("Error adding song to playlist: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to add song to playlist",
                    "message": format!("{:?}", e)
                })))
            })?;

        // Update song count
        let mut updated_playlist = playlist;
        updated_playlist.song_count += 1;
        updated_playlist.updated_at = Utc::now();
        
        state.playlist_repository
            .update(&updated_playlist)
            .await
            .map_err(|e| {
                tracing::error!("Error updating playlist song count: {:?}", e);
                // Don't fail the request if count update fails, just log it
                tracing::warn!("Song added but count update failed");
            })
            .ok();

        Ok(ResponseJson(serde_json::json!({
            "success": true,
            "message": "Song added to playlist successfully",
            "playlist_id": playlist_id,
            "song_id": request.song_id
        })))
    }
    
    /// DELETE /api/v1/music/playlists/:id/songs/:song_id - Remove song from playlist
    /// Requires authentication - only playlist owner can remove songs
    pub async fn remove_song_from_playlist(
        AuthenticatedUser { user_id, .. }: AuthenticatedUser,
        State(state): State<MusicAppState>,
        Path((playlist_id, song_id)): Path<(Uuid, Uuid)>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get playlist to verify ownership
        let playlist = state.playlist_repository
            .find_by_id(&playlist_id)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching playlist: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch playlist",
                    "message": format!("{:?}", e)
                })))
            })?
            .ok_or_else(|| {
                (StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({
                    "error": "Playlist not found",
                    "message": format!("Playlist with ID {} not found", playlist_id)
                })))
            })?;

        // Verify ownership (only creator can remove songs)
        if playlist.created_by != user_id {
            return Err((
                StatusCode::FORBIDDEN,
                ResponseJson(serde_json::json!({
                    "error": "Forbidden",
                    "message": "Only the playlist owner can remove songs"
                })),
            ));
        }

        // Verify song exists in playlist
        let playlist_songs = state.playlist_repository
            .get_songs(&playlist_id)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching playlist songs: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to fetch playlist songs",
                    "message": format!("{:?}", e)
                })))
            })?;

        if !playlist_songs.contains(&song_id) {
            return Err((
                StatusCode::NOT_FOUND,
                ResponseJson(serde_json::json!({
                    "error": "Song not found in playlist",
                    "message": format!("Song with ID {} is not in playlist {}", song_id, playlist_id)
                })),
            ));
        }

        // Remove song from playlist
        state.playlist_repository
            .remove_song(&playlist_id, &song_id)
            .await
            .map_err(|e| {
                tracing::error!("Error removing song from playlist: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({
                    "error": "Failed to remove song from playlist",
                    "message": format!("{:?}", e)
                })))
            })?;

        // Update song count
        let mut updated_playlist = playlist;
        if updated_playlist.song_count > 0 {
            updated_playlist.song_count -= 1;
        }
        updated_playlist.updated_at = Utc::now();
        
        state.playlist_repository
            .update(&updated_playlist)
            .await
            .map_err(|e| {
                tracing::error!("Error updating playlist song count: {:?}", e);
                // Don't fail the request if count update fails, just log it
                tracing::warn!("Song removed but count update failed");
            })
            .ok();

        Ok(ResponseJson(serde_json::json!({
            "success": true,
            "message": "Song removed from playlist successfully",
            "playlist_id": playlist_id,
            "song_id": song_id
        })))
    }
}
