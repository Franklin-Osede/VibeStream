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
    models::playlist::Model as Playlist,
    services::playlist::PlaylistService,
};

pub fn create_playlist_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_playlist))
        .route("/:id", get(get_playlist))
        .route("/:id/songs", post(add_song))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreatePlaylistRequest {
    pub name: String,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct AddSongRequest {
    pub song_id: Uuid,
}

async fn create_playlist(
    state: axum::extract::State<AppState>,
    Json(_request): Json<CreatePlaylistRequest>,
) -> impl axum::response::IntoResponse {
    let playlist_service = PlaylistService::new(state.db.clone());
    // TODO: Implement playlist creation
    todo!()
}

async fn get_playlist(
    state: axum::extract::State<AppState>,
    Path(_id): Path<Uuid>,
) -> impl axum::response::IntoResponse {
    let playlist_service = PlaylistService::new(state.db.clone());
    // TODO: Implement get playlist
    todo!()
}

async fn add_song(
    state: axum::extract::State<AppState>,
    Path(_playlist_id): Path<Uuid>,
    Json(_request): Json<AddSongRequest>,
) -> impl axum::response::IntoResponse {
    let playlist_service = PlaylistService::new(state.db.clone());
    // TODO: Implement add song to playlist
    todo!()
} 