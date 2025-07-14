use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::Path,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use vibestream_types::*;
use tokio::net::TcpListener;

mod ethereum;
use ethereum::{EthereumClient, TransactionInfo, TokenInfo};

#[derive(Debug, Serialize, Deserialize)]
struct TransferRequest {
    to: String,
    amount: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = EthereumClient::new(
        std::env::var("ETH_RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string()),
        std::env::var("ETH_PRIVATE_KEY").unwrap_or_else(|_| "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()),
    )?;

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/balance/:address", get(get_balance))
        .route("/transfer", post(transfer))
        .route("/token/:address/info", get(get_token_info))
        .route("/token/:address/balance/:owner", get(get_token_balance))
        .route("/token/:address/transfer", post(transfer_token));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Ethereum service listening on {}", addr);

    let listener = TcpListener::bind(addr).await.map_err(|e| VibeStreamError::Network { 
        message: format!("Failed to bind to address: {}", e) 
    })?;

    axum::serve(listener, app)
        .await
        .map_err(|e| VibeStreamError::Network { 
            message: format!("Server error: {}", e) 
        })?;
    
    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "ethereum-service",
        "timestamp": Timestamp::now()
    }))
}

async fn get_balance(Path(address): Path<String>) -> std::result::Result<Json<u64>, StatusCode> {
    let client = get_client().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let balance = client.get_balance(&address).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(balance))
}

async fn transfer(Json(request): Json<TransferRequest>) -> std::result::Result<Json<TransactionInfo>, StatusCode> {
    let client = get_client().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tx_info = client.transfer(&request.to, request.amount).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(tx_info))
}

async fn get_token_info(Path(address): Path<String>) -> std::result::Result<Json<TokenInfo>, StatusCode> {
    let client = get_client().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let token_info = client.get_token_info(&address).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(token_info))
}

async fn get_token_balance(
    Path((token_address, owner)): Path<(String, String)>
) -> std::result::Result<Json<u64>, StatusCode> {
    let client = get_client().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let balance = client.get_token_balance(&token_address, &owner).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(balance))
}

async fn transfer_token(
    Path(token_address): Path<String>,
    Json(request): Json<TransferRequest>
) -> std::result::Result<Json<TransactionInfo>, StatusCode> {
    let client = get_client().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tx_info = client.transfer_token(&token_address, &request.to, request.amount).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(tx_info))
}

fn get_client() -> Result<EthereumClient> {
    EthereumClient::new(
        std::env::var("ETH_RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string()),
        std::env::var("ETH_PRIVATE_KEY").unwrap_or_else(|_| "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()),
    )
} 