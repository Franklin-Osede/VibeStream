use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::Path,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod ethereum;
use ethereum::EthereumClient;

#[derive(Debug, Serialize, Deserialize)]
struct TransferRequest {
    to: String,
    amount: u64,
}

#[tokio::main]
async fn main() {
    let client = EthereumClient::new(
        std::env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set"),
        std::env::var("ETH_PRIVATE_KEY").expect("ETH_PRIVATE_KEY must be set"),
    ).expect("Failed to create Ethereum client");

    let app = Router::new()
        .route("/balance/:address", get(get_balance))
        .route("/transfer", post(transfer))
        .route("/token/:address/info", get(get_token_info))
        .route("/token/:address/balance/:owner", get(get_token_balance))
        .route("/token/:address/transfer", post(transfer_token))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Ethereum service listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_balance(Path(address): Path<String>) -> Json<u64> {
    let client = get_client();
    let balance = client.get_balance(&address).await.unwrap();
    Json(balance)
}

async fn transfer(Json(request): Json<TransferRequest>) -> Json<ethereum::TransactionInfo> {
    let client = get_client();
    let tx_info = client.transfer(&request.to, request.amount).await.unwrap();
    Json(tx_info)
}

async fn get_token_info(Path(address): Path<String>) -> Json<ethereum::TokenInfo> {
    let client = get_client();
    let token_info = client.get_token_info(&address).await.unwrap();
    Json(token_info)
}

async fn get_token_balance(
    Path((token_address, owner)): Path<(String, String)>
) -> Json<u64> {
    let client = get_client();
    let balance = client.get_token_balance(&token_address, &owner).await.unwrap();
    Json(balance)
}

async fn transfer_token(
    Path(token_address): Path<String>,
    Json(request): Json<TransferRequest>
) -> Json<ethereum::TransactionInfo> {
    let client = get_client();
    let tx_info = client.transfer_token(&token_address, &request.to, request.amount).await.unwrap();
    Json(tx_info)
}

fn get_client() -> EthereumClient {
    EthereumClient::new(
        std::env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set"),
        std::env::var("ETH_PRIVATE_KEY").expect("ETH_PRIVATE_KEY must be set"),
    ).expect("Failed to create Ethereum client")
} 