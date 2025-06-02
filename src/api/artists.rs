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
    db::models::artist::Model as Artist,
    services::artist::ArtistService,
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateArtistRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 0, max = 500))]
    pub bio: Option<String>,
    #[validate(length(min = 0, max = 200))]
    pub website: Option<String>,
    #[validate(length(min = 0, max = 200))]
    pub social_links: Option<Vec<String>>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/artists", post(create_artist))
        .route("/artists/:id", get(get_artist))
}

pub async fn create_artist(
    State(state): State<AppState>,
    Json(request): Json<CreateArtistRequest>,
) -> Result<Json<Artist>, AppError> {
    request.validate()?;
    
    let artist = state.artist_service
        .create_artist(
            &state.db,
            request.name,
            request.bio,
            request.website,
            request.social_links,
        )
        .await?;

    Ok(Json(artist))
}

pub async fn get_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Artist>, AppError> {
    let artist = state.artist_service
        .get_artist(&state.db, id)
        .await?
        .ok_or(AppError::NotFound("Artist not found".to_string()))?;

    Ok(Json(artist))
} 