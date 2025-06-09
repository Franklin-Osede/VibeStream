use async_trait::async_trait;
use anyhow::Result;
use std::fmt::Debug;

/// Represents basic blockchain information
#[derive(Debug, Clone)]
pub struct ChainInfo {
    pub chain_id: String,
    pub name: String,
    pub native_token: String,
    pub decimals: u8,
}

/// Chain-agnostic wallet interface
#[async_trait]
pub trait Chain: Send + Sync + Debug {
    /// Get chain information
    fn get_chain_info(&self) -> &ChainInfo;
    
    /// Get wallet address in chain-specific format
    fn get_address(&self) -> String;
    
    /// Get native token balance
    async fn get_balance(&self) -> Result<u64>;
    
    /// Transfer native tokens
    async fn transfer(&mut self, to_address: &str, amount: u64) -> Result<String>;
    
    /// Validate address format
    fn validate_address(&self, address: &str) -> bool;
    
    /// Convert amount to chain's smallest unit
    fn to_base_unit(&self, amount: f64) -> u64 {
        (amount * 10_f64.powi(self.get_chain_info().decimals as i32)) as u64
    }
    
    /// Convert amount from chain's smallest unit to float
    fn from_base_unit(&self, amount: u64) -> f64 {
        amount as f64 / 10_f64.powi(self.get_chain_info().decimals as i32)
    }
}

/// Supported blockchain networks
#[derive(Debug, Clone, Copy)]
pub enum Network {
    Mainnet,
    Testnet,
    Devnet,
    Local,
}

/// Chain configuration
#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub network: Network,
    pub rpc_url: String,
    pub retry_attempts: u32,
    pub timeout: std::time::Duration,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            network: Network::Devnet,
            rpc_url: String::new(),
            retry_attempts: 3,
            timeout: std::time::Duration::from_secs(30),
        }
    }
}

pub mod solana;
// TODO: Add modules for other chains
// pub mod ethereum;
// pub mod polygon;
// pub mod avalanche; 