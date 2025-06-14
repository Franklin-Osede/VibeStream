use axum::{
    extract::{Query, State, Path},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use vibestream_types::*;
use crate::services::MessageQueue;
use serde::{Deserialize, Serialize};
use crate::services::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    service: String,
    timestamp: String,
    redis: String,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub blockchain: Blockchain,
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub message: String,
    pub request_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub blockchain: String,
    pub balance: Option<u64>,
    pub status: String,
}

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

#[axum::debug_handler]
pub async fn health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    let redis_status = match state.message_queue.ping().await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        service: "api-gateway".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        redis: redis_status.to_string(),
    }))
}

#[axum::debug_handler]
pub async fn process_transaction(
    State(state): State<AppState>,
    Json(request): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, StatusCode> {
    // Crear mensaje para el servicio correspondiente
    let api_message = ApiMessage::ProcessTransaction {
        blockchain: request.blockchain.clone(),
        from: request.from,
        to: request.to,
        amount: request.amount,
    };

    let service_message = ServiceMessage::new(api_message);
    let request_id = service_message.id.0.to_string();

    // Determinar la cola correcta según la blockchain
    let queue_name = match request.blockchain {
        Blockchain::Ethereum => "ethereum_queue",
        Blockchain::Solana => "solana_queue",
    };

    // Enviar mensaje a la cola correspondiente
    match state.message_queue.send_message(
        queue_name,
        &serde_json::to_string(&service_message).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ).await {
        Ok(_) => {
            tracing::info!("Transaction request sent to {}: {}", queue_name, request_id);
            Ok(Json(TransactionResponse {
                message: "Transaction request submitted successfully".to_string(),
                request_id,
                status: "pending".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to send transaction request: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
pub async fn get_balance(
    State(state): State<AppState>,
    Path((blockchain, address)): Path<(String, String)>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    // Parsear blockchain
    let blockchain = match blockchain.to_lowercase().as_str() {
        "ethereum" => Blockchain::Ethereum,
        "solana" => Blockchain::Solana,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Crear wallet address
    let wallet = WalletAddress {
        address: address.clone(),
        blockchain: blockchain.clone(),
    };

    // Crear mensaje para obtener balance
    let api_message = ApiMessage::GetBalance { wallet };
    let service_message = ServiceMessage::new(api_message);

    // Determinar la cola correcta
    let queue_name = match blockchain {
        Blockchain::Ethereum => "ethereum_queue",
        Blockchain::Solana => "solana_queue",
    };

    // Enviar mensaje a la cola
    match state.message_queue.send_message(
        queue_name,
        &serde_json::to_string(&service_message).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ).await {
        Ok(_) => {
            tracing::info!("Balance request sent to {}: {}", queue_name, address);
            Ok(Json(BalanceResponse {
                address,
                blockchain: format!("{:?}", blockchain),
                balance: None, // En una implementación real, esperaríamos la respuesta
                status: "request_sent".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to send balance request: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Endpoint para obtener el estado de las colas Redis
#[axum::debug_handler]
pub async fn queue_status(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut status = serde_json::Map::new();
    
    // Verificar conexión Redis
    match state.message_queue.ping().await {
        Ok(_) => {
            status.insert("redis".to_string(), serde_json::Value::String("connected".to_string()));
            status.insert("queues".to_string(), serde_json::json!({
                "ethereum_queue": "available",
                "solana_queue": "available", 
                "zk_queue": "available",
                "response_queue": "available"
            }));
        }
        Err(e) => {
            status.insert("redis".to_string(), serde_json::Value::String("disconnected".to_string()));
            status.insert("error".to_string(), serde_json::Value::String(format!("{:?}", e)));
        }
    }

    Ok(Json(serde_json::Value::Object(status)))
} 