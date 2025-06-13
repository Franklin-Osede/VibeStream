use serde::{Deserialize, Serialize};
use crate::{RequestId, Timestamp, Transaction, WalletAddress, Balance, StreamPayment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMessage<T> {
    pub id: RequestId,
    pub timestamp: Timestamp,
    pub payload: T,
}

impl<T> ServiceMessage<T> {
    pub fn new(payload: T) -> Self {
        Self {
            id: RequestId::new(),
            timestamp: Timestamp::now(),
            payload,
        }
    }
}

// Mensajes para Ethereum Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthereumMessage {
    GetBalance(WalletAddress),
    SendTransaction {
        from: String,
        to: String,
        amount: u64,
    },
    GetTransactionStatus(String), // transaction hash
    CreateStream(StreamPayment),
}

// Mensajes para Solana Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SolanaMessage {
    GetBalance(WalletAddress),
    SendTransaction {
        from: String,
        to: String,
        amount: u64,
    },
    GetTransactionStatus(String), // transaction hash
    CreateStream(StreamPayment),
}

// Respuestas de los servicios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceResponse {
    Balance(Balance),
    Transaction(Transaction),
    Stream(StreamPayment),
    Error(String),
}

// Mensajes del API Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiMessage {
    ProcessTransaction {
        blockchain: crate::Blockchain,
        from: String,
        to: String,
        amount: u64,
    },
    GetBalance {
        wallet: WalletAddress,
    },
    CreateStream {
        stream: StreamPayment,
    },
} 