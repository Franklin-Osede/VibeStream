use std::sync::Arc;
use axum::{
    Json,
    extract::Extension,
};
use serde::{Deserialize, Serialize};
use crate::models::{ApiResponse, TransferRequest, TransferResponse};
use solana_integration::SolanaClient;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// Request types
#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub to_address: String,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub signature: String,
    pub amount: f64,
}

// Health check endpoint
pub async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse {
        success: true,
        data: Some("Service is healthy"),
        error: None,
    })
}

// Get wallet balance endpoint
pub async fn get_wallet_balance(
    Extension(client): Extension<Arc<SolanaClient>>,
) -> Json<ApiResponse<f64>> {
    match client.get_wallet_balance().await {
        Ok(balance) => Json(ApiResponse {
            success: true,
            data: Some(balance),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: Some(0.0),
            error: Some(e.to_string()),
        }),
    }
}

// Transfer SOL endpoint
pub async fn transfer_sol(
    Extension(client): Extension<Arc<SolanaClient>>,
    Json(request): Json<TransferRequest>,
) -> Json<ApiResponse<TransferResponse>> {
    match client.transfer_sol(&request.to_address, request.amount).await {
        Ok(signature) => Json(ApiResponse {
            success: true,
            data: Some(TransferResponse {
                signature,
                amount: request.amount,
            }),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
} 