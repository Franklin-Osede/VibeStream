use axum::{
    routing::{get, post},
    Router,
    extract::{Json, Path, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

use crate::{
    error::AppError,
    db::models::royalty::Model as Royalty,
    services::royalty::RoyaltyService,
    middleware::auth::AuthUser,
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoyaltyRequest {
    pub nft_id: Uuid,
    #[validate(range(min = 0.0))]
    pub amount: Decimal,
    #[validate(length(min = 1, max = 100))]
    pub transaction_hash: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/royalties", post(create_royalty))
        .route("/royalties/:id", get(get_royalty))
        .route("/royalties/nft/:nft_id", get(get_nft_royalties))
        .route("/royalties/nft/:nft_id/total", get(get_total_royalties))
}

pub async fn create_royalty(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<CreateRoyaltyRequest>,
) -> Result<Json<Royalty>, AppError> {
    request.validate()?;

    // Verify NFT ownership
    let nft = state.nft_service
        .get_nft(&state.db, request.nft_id)
        .await?
        .ok_or(AppError::NotFound("NFT not found".to_string()))?;

    if nft.owner_id != auth_user.user.id {
        return Err(AppError::Forbidden);
    }

    let royalty = state.royalty_service
        .create_royalty_payment(
            &state.db,
            request.nft_id,
            request.amount,
            request.transaction_hash,
        )
        .await?;

    Ok(Json(royalty))
}

pub async fn get_royalty(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Royalty>, AppError> {
    let royalty = state.royalty_service
        .get_royalty_payment(&state.db, id)
        .await?
        .ok_or(AppError::NotFound("Royalty payment not found".to_string()))?;

    Ok(Json(royalty))
}

pub async fn get_nft_royalties(
    State(state): State<AppState>,
    Path(nft_id): Path<Uuid>,
) -> Result<Json<Vec<Royalty>>, AppError> {
    let royalties = state.royalty_service
        .get_nft_royalties(&state.db, nft_id)
        .await?;

    Ok(Json(royalties))
}

pub async fn get_total_royalties(
    State(state): State<AppState>,
    Path(nft_id): Path<Uuid>,
) -> Result<Json<Decimal>, AppError> {
    let total = state.royalty_service
        .get_total_royalties(&state.db, nft_id)
        .await?;

    Ok(Json(total))
} 