use axum::{
    routing::{get, post},
    Router,
    extract::{Json, Path},
    response::IntoResponse,
    http::StatusCode,
};
use uuid::Uuid;
use serde::Deserialize;
use crate::{
    AppState,
    error::AppError,
    models::song_nft::Model as SongNft,
    services::nft::NftService,
};

#[derive(Debug, serde::Deserialize)]
pub struct CreateNftRequest {
    pub song_id: Uuid,
    pub contract_id: Uuid,
    pub token_id: i32,
    pub royalty_percentage: f64,
    pub owner_address: String,
}

pub fn create_nft_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/nfts", get(list_nfts))
        .route("/nfts", post(create_nft))
        .route("/nfts/:id", get(get_nft))
        .with_state(state)
}

pub async fn list_nfts() -> Result<Json<Vec<SongNft>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn create_nft(
    state: axum::extract::State<AppState>,
    Json(_request): Json<CreateNftRequest>,
) -> Result<(StatusCode, Json<SongNft>), AppError> {
    let _nft_service = NftService::new(state.db.clone());
    // TODO: Implement NFT creation
    Err(AppError::NotImplemented)
}

pub async fn get_nft(
    state: axum::extract::State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SongNft>, AppError> {
    let _nft_service = NftService::new(state.db.clone());
    // TODO: Implement get NFT
    Err(AppError::NotImplemented)
} 