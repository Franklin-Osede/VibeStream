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

// Mensajes para ZK Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkMessage {
    GenerateSolvencyProof {
        balance: u64,
        threshold: u64,
    },
    GenerateTransactionProof {
        amount: u64,
        sender_balance: u64,
    },
    VerifyProof {
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
    },
}

// Respuestas de los servicios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceResponse {
    Balance(Balance),
    Transaction(Transaction),
    Stream(StreamPayment),
    ZkProof {
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
        verification_key: Vec<u8>,
    },
    ZkVerification(bool),
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
    GenerateZkProof {
        proof_type: String,
        parameters: serde_json::Value,
    },
}

// Trait abstracto para message brokers
#[async_trait::async_trait]
pub trait MessageBroker: Send + Sync {
    async fn send_message(&self, queue: &str, message: &str) -> crate::Result<()>;
    async fn receive_message(&self, queue: &str) -> crate::Result<Option<String>>;
    async fn receive_message_blocking(&self, queue: &str, timeout_secs: u64) -> crate::Result<Option<String>>;
}

// Nombres de colas estándar
pub struct QueueNames;

impl QueueNames {
    pub const ETHEREUM: &'static str = "ethereum_queue";
    pub const SOLANA: &'static str = "solana_queue";
    pub const ZK: &'static str = "zk_queue";
    pub const RESPONSES: &'static str = "response_queue";
} 