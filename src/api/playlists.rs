use axum::{
    routing::{get, post},
    Router,
    extract::{Json, Path, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    db::models::playlist::Model as Playlist,
    services::playlist::PlaylistService,
    middleware::auth::AuthUser,
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePlaylistRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 0, max = 500))]
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddSongRequest {
    pub song_id: Uuid,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/playlists", post(create_playlist))
        .route("/playlists/:id", get(get_playlist))
        .route("/playlists/:id/songs", post(add_song))
}

pub async fn create_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<CreatePlaylistRequest>,
) -> Result<Json<Playlist>, AppError> {
    request.validate()?;

    let playlist = state.playlist_service
        .create_playlist(
            &state.db,
            request.name,
            auth_user.user.id,
            request.description,
            request.is_public.unwrap_or(false),
        )
        .await?;

    Ok(Json(playlist))
}

pub async fn get_playlist(
    State(state): State<AppState>,
    auth_user: Option<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<Playlist>, AppError> {
    let playlist = state.playlist_service
        .get_playlist(&state.db, id)
        .await?
        .ok_or(AppError::NotFound("Playlist not found".to_string()))?;

    // Check access permissions
    if !playlist.is_public && auth_user.map(|u| u.user.id) != Some(playlist.user_id) {
        return Err(AppError::Forbidden);
    }

    Ok(Json(playlist))
}

pub async fn add_song(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(playlist_id): Path<Uuid>,
    Json(request): Json<AddSongRequest>,
) -> Result<Json<Playlist>, AppError> {
    // Verify playlist ownership
    let playlist = state.playlist_service
        .get_playlist(&state.db, playlist_id)
        .await?
        .ok_or(AppError::NotFound("Playlist not found".to_string()))?;

    if playlist.user_id != auth_user.user.id {
        return Err(AppError::Forbidden);
    }

    // Add song to playlist
    state.playlist_service
        .add_song(&state.db, playlist_id, request.song_id)
        .await?;

    Ok(Json(playlist))
} 