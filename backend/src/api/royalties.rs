use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::{
    AppState,
    error::AppError,
    models::royalty_payment::Model as RoyaltyPayment,
    services::royalty::RoyaltyService,
};

pub fn create_royalty_router() -> Router<AppState> {
    Router::new()
        .route("/royalties", get(list_royalties))
        .route("/royalties/:id", get(get_royalty))
        .route("/royalties/calculate/:song_id", get(calculate_royalties))
        .route("/royalties/distribute/:song_id", post(distribute_royalties))
}

async fn list_royalties(
    state: axum::extract::State<AppState>,
) -> Result<Json<Vec<RoyaltyPayment>>, AppError> {
    let _royalty_service = RoyaltyService::new(state.db.clone());
    // TODO: Implement list royalties
    Err(AppError::NotImplemented)
}

async fn get_royalty(
    state: axum::extract::State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<RoyaltyPayment>, AppError> {
    let _royalty_service = RoyaltyService::new(state.db.clone());
    // TODO: Implement get royalty
    Err(AppError::NotImplemented)
}

async fn calculate_royalties(
    state: axum::extract::State<AppState>,
    Path(_song_id): Path<Uuid>,
) -> Result<Json<f64>, AppError> {
    let _royalty_service = RoyaltyService::new(state.db.clone());
    // TODO: Implement royalty calculation
    Err(AppError::NotImplemented)
}

async fn distribute_royalties(
    state: axum::extract::State<AppState>,
    Path(_song_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let _royalty_service = RoyaltyService::new(state.db.clone());
    // TODO: Implement royalty distribution
    Err(AppError::NotImplemented)
} 