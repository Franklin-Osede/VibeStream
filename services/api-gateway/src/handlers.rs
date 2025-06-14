use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use vibestream_types::*;
use crate::services::MessageQueue;

#[axum::debug_handler]
pub async fn get_balance(
    Query(params): Query<HashMap<String, String>>,
    State(message_queue): State<Arc<MessageQueue>>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let address = params.get("address").ok_or(StatusCode::BAD_REQUEST)?;
    let blockchain = params.get("blockchain").ok_or(StatusCode::BAD_REQUEST)?;
    
    let wallet = WalletAddress {
        address: address.clone(),
        blockchain: match blockchain.as_str() {
            "ethereum" => Blockchain::Ethereum,
            "solana" => Blockchain::Solana,
            _ => return Err(StatusCode::BAD_REQUEST),
        },
    };
    
    // Enviar mensaje al servicio correspondiente
    let result = match blockchain.as_str() {
        "ethereum" => {
            message_queue.send_ethereum_message(EthereumMessage::GetBalance(wallet)).await
        }
        "solana" => {
            message_queue.send_solana_message(SolanaMessage::GetBalance(wallet)).await
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    if result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // Por ahora devolvemos respuesta inmediata
    // TODO: Implementar polling o WebSockets para respuesta asíncrona
    let response = json!({
        "status": "processing",
        "message": "Balance request sent to blockchain service",
        "address": address,
        "blockchain": blockchain,
        "timestamp": Timestamp::now()
    });
    
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn send_transaction(
    State(message_queue): State<Arc<MessageQueue>>,
    Json(payload): Json<Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let blockchain = payload.get("blockchain")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let from = payload.get("from")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let to = payload.get("to")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let amount = payload.get("amount")
        .and_then(|v| v.as_u64())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // Enviar mensaje al servicio correspondiente
    let result = match blockchain {
        "ethereum" => {
            message_queue.send_ethereum_message(EthereumMessage::SendTransaction {
                from: from.to_string(),
                to: to.to_string(),
                amount,
            }).await
        }
        "solana" => {
            message_queue.send_solana_message(SolanaMessage::SendTransaction {
                from: from.to_string(),
                to: to.to_string(),
                amount,
            }).await
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    if result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let response = json!({
        "status": "processing",
        "message": "Transaction sent to blockchain service",
        "transaction_id": RequestId::new(),
        "blockchain": blockchain,
        "timestamp": Timestamp::now()
    });
    
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn create_stream(
    State(message_queue): State<Arc<MessageQueue>>,
    Json(payload): Json<Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    // TODO: Parsear payload para crear StreamPayment
    let _stream_data = payload;
    
    // Por ahora enviamos un mensaje ZK para generar prueba de solvencia
    let zk_message = ZkMessage::GenerateSolvencyProof {
        balance: 1000, // Mock data
        threshold: 500,
    };
    
    if message_queue.send_zk_message(zk_message).await.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let response = json!({
        "status": "processing",
        "message": "Stream creation request sent to ZK service",
        "stream_id": RequestId::new(),
        "timestamp": Timestamp::now()
    });
    
    Ok(Json(response))
} 