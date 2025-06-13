use serde::{Deserialize, Serialize};
use crate::{RequestId, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Blockchain {
    Ethereum,
    Solana,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: RequestId,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub blockchain: Blockchain,
    pub timestamp: Timestamp,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub blockchain: Blockchain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub wallet: WalletAddress,
    pub amount: u64,
    pub token_symbol: String,
    pub last_updated: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamPayment {
    pub id: RequestId,
    pub from: WalletAddress,
    pub to: WalletAddress,
    pub amount_per_second: u64,
    pub duration_seconds: u64,
    pub total_amount: u64,
    pub started_at: Timestamp,
    pub status: StreamStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
} 