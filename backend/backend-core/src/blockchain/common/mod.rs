use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub status: TransactionStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[async_trait]
pub trait BlockchainClient {
    async fn get_balance(&self, address: &str) -> Result<u64>;
    async fn transfer(&self, to: &str, amount: u64) -> Result<TransactionInfo>;
    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus>;
}

#[async_trait]
pub trait TokenClient {
    async fn get_token_info(&self, token_address: &str) -> Result<TokenInfo>;
    async fn transfer_token(&self, token_address: &str, to: &str, amount: u64) -> Result<TransactionInfo>;
    async fn get_token_balance(&self, token_address: &str, address: &str) -> Result<u64>;
} 