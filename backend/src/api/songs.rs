use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::{
    AppState,
    error::AppError,
    models::song::Model as Song,
    services::song::SongService,
};

pub fn create_song_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_song))
        .route("/:id", get(get_song))
}

#[derive(Deserialize)]
pub struct CreateSongRequest {
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: i32,
    pub genre: Option<String>,
    pub ipfs_hash: String,
}

async fn create_song(
    state: axum::extract::State<AppState>,
    Json(_request): Json<CreateSongRequest>,
) -> impl axum::response::IntoResponse {
    let song_service = SongService::new(state.db.clone());
    // TODO: Implement song creation
    todo!()
}

async fn get_song(
    state: axum::extract::State<AppState>,
    Path(_id): Path<Uuid>,
) -> impl axum::response::IntoResponse {
    let song_service = SongService::new(state.db.clone());
    // TODO: Implement get song
    todo!()
} 