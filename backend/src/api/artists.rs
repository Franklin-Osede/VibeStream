use axum::{
    routing::{get, post},
    Router,
    extract::{Json, Path},
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::{
    AppState,
    error::AppError,
    models::artist::Model as Artist,
    services::artist::ArtistService,
};

#[derive(Debug, Deserialize)]
pub struct CreateArtistRequest {
    pub name: String,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
}

pub fn create_artist_router() -> Router<AppState> {
    Router::new()
        .route("/artists", get(list_artists))
        .route("/artists", post(create_artist))
        .route("/artists/:id", get(get_artist))
}

pub async fn list_artists() -> Result<Json<Vec<Artist>>, AppError> {
    Err(AppError::NotImplemented)
}

async fn create_artist(
    state: axum::extract::State<AppState>,
    Json(_request): Json<CreateArtistRequest>,
) -> Result<(StatusCode, Json<Artist>), AppError> {
    let _artist_service = ArtistService::new(state.db.clone());
    // TODO: Implement artist creation
    Err(AppError::NotImplemented)
}

async fn get_artist(
    state: axum::extract::State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<Artist>, AppError> {
    let _artist_service = ArtistService::new(state.db.clone());
    // TODO: Implement get artist
    Err(AppError::NotImplemented)
} 