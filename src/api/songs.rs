use axum::{
    routing::{get, post},
    Router,
    extract::{Json, Path, State, Multipart},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    db::models::song::Model as Song,
    services::song::SongService,
    middleware::auth::AuthUser,
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSongRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    pub artist_id: Uuid,
    #[validate(range(min = 1, max = 3600))]
    pub duration: i32,
    #[validate(length(min = 0, max = 50))]
    pub genre: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/songs", post(create_song))
        .route("/songs/:id", get(get_song))
}

pub async fn create_song(
    State(state): State<AppState>,
    auth_user: AuthUser,
    mut multipart: Multipart,
    Json(request): Json<CreateSongRequest>,
) -> Result<Json<Song>, AppError> {
    request.validate()?;

    // Ensure user has permission to create songs for this artist
    state.artist_service
        .verify_artist_ownership(&state.db, request.artist_id, auth_user.user.id)
        .await?;

    // Handle file upload
    let file_data = match multipart.next_field().await? {
        Some(field) => {
            if field.name() != Some("file") {
                return Err(AppError::ValidationError("Missing song file".to_string()));
            }
            field.bytes().await?
        }
        None => return Err(AppError::ValidationError("Missing song file".to_string())),
    };

    // Create song with file
    let song = state.song_service
        .create_song(
            &state.db,
            request.title,
            request.artist_id,
            request.duration,
            request.genre,
            file_data,
        )
        .await?;

    Ok(Json(song))
}

pub async fn get_song(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Song>, AppError> {
    let song = state.song_service
        .get_song(&state.db, id)
        .await?
        .ok_or(AppError::NotFound("Song not found".to_string()))?;

    Ok(Json(song))
} 