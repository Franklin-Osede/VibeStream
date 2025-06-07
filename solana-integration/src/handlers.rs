use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use solana_sdk::pubkey::Pubkey;
use solana_integration::{SolanaClient, ProofRequest, VerifyRequest};

use crate::models::{
    ApiResponse,
    TransferRequest,
};

pub async fn get_balance(
    State(client): State<Arc<SolanaClient>>,
) -> impl IntoResponse {
    match client.wallet_client.get_balance().await {
        Ok(balance) => (StatusCode::OK, Json(ApiResponse::success(balance))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn transfer_sol(
    State(client): State<Arc<SolanaClient>>,
    Json(request): Json<TransferRequest>,
) -> impl IntoResponse {
    match client.transfer_sol(&request.to_address, request.amount).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct MintNFTRequest {
    pub metadata_uri: String,
}

pub async fn mint_nft(
    State(client): State<Arc<SolanaClient>>,
    Json(request): Json<MintNFTRequest>,
) -> impl IntoResponse {
    match client.nft_client.mint_nft(&request.metadata_uri).await {
        Ok(pubkey) => (StatusCode::OK, Json(ApiResponse::success(pubkey.to_string()))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct TransferNFTRequest {
    pub nft_address: String,
    pub recipient: String,
}

pub async fn transfer_nft(
    State(client): State<Arc<SolanaClient>>,
    Json(request): Json<TransferNFTRequest>,
) -> impl IntoResponse {
    let nft_address = match request.nft_address.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let recipient = match request.recipient.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    match client.nft_client.transfer_nft(&nft_address, &recipient).await {
        Ok(_) => (StatusCode::OK, Json(ApiResponse::success("NFT transferred successfully"))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn generate_proof(
    State(client): State<Arc<SolanaClient>>,
    Json(request): Json<ProofRequest>,
) -> impl IntoResponse {
    match client.generate_proof(request).await {
        Ok(proof) => {
            match serde_json::to_string(&proof) {
                Ok(proof_str) => (StatusCode::OK, Json(ApiResponse::success(proof_str))).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn verify_proof(
    State(client): State<Arc<SolanaClient>>,
    Json(request): Json<VerifyRequest>,
) -> impl IntoResponse {
    match client.verify_proof(request).await {
        Ok(is_valid) => (StatusCode::OK, Json(ApiResponse::success(is_valid))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
} 