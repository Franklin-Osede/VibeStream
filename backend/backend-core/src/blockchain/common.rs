use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ethers::types::H256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub status: TransactionStatus,
}

#[async_trait]
pub trait BlockchainClient: Send + Sync {
    async fn get_balance(&self, address: &str) -> anyhow::Result<u64>;
    async fn transfer(&self, to: &str, amount: u64) -> anyhow::Result<TransactionInfo>;
    async fn get_transaction_status(&self, tx_hash: &str) -> anyhow::Result<TransactionStatus>;
}

#[async_trait]
pub trait TokenClient: Send + Sync {
    async fn get_token_info(&self, token_address: &str) -> anyhow::Result<TokenInfo>;
    async fn transfer_token(&self, token_address: &str, to: &str, amount: u64) -> anyhow::Result<TransactionInfo>;
    async fn get_token_balance(&self, token_address: &str, address: &str) -> anyhow::Result<u64>;
}

pub type BlockchainResult<T> = anyhow::Result<T>;
pub type TransactionHash = H256; 