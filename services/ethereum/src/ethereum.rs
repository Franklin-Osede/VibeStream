use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::{LocalWallet, Signer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vibestream_types::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: U256,
    pub gas_used: Option<U256>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: U256,
}

pub struct EthereumClient {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
}

impl EthereumClient {
    pub fn new(rpc_url: String, private_key: String) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to connect to RPC: {}", e) 
            })?;
        
        let wallet = private_key.parse::<LocalWallet>()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid private key: {}", e) 
            })?;
        
        Ok(Self {
            provider: Arc::new(provider),
            wallet,
        })
    }
    
    pub async fn get_balance(&self, address: &str) -> Result<u64> {
        let address: Address = address.parse()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid address: {}", e) 
            })?;
        
        let balance = self.provider.get_balance(address, None).await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to get balance: {}", e) 
            })?;
        
        // Convert to u64 (this might overflow for very large balances)
        Ok(balance.as_u64())
    }
    
    pub async fn transfer(&self, to: &str, amount: u64) -> Result<TransactionInfo> {
        let to_address: Address = to.parse()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid address: {}", e) 
            })?;
        
        // TODO: Implementar transferencia real
        // Por ahora devolvemos información mock
        Ok(TransactionInfo {
            hash: "0x1234567890abcdef".to_string(),
            from: format!("{:?}", self.wallet.address()),
            to: to.to_string(),
            amount: U256::from(amount),
            gas_used: Some(U256::from(21000)),
            status: "pending".to_string(),
        })
    }
    
    pub async fn get_token_info(&self, token_address: &str) -> Result<TokenInfo> {
        let _address: Address = token_address.parse()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid token address: {}", e) 
            })?;
        
        // TODO: Implementar obtención real de información del token
        // Por ahora devolvemos información mock
        Ok(TokenInfo {
            address: token_address.to_string(),
            name: "Mock Token".to_string(),
            symbol: "MOCK".to_string(),
            decimals: 18,
            total_supply: U256::from(1000000u64),
        })
    }
    
    pub async fn get_token_balance(&self, token_address: &str, owner: &str) -> Result<u64> {
        let _token_addr: Address = token_address.parse()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid token address: {}", e) 
            })?;
        
        let _owner_addr: Address = owner.parse()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid owner address: {}", e) 
            })?;
        
        // TODO: Implementar obtención real del balance del token
        // Por ahora devolvemos un balance mock
        Ok(1000)
    }
    
    pub async fn transfer_token(&self, token_address: &str, to: &str, amount: u64) -> Result<TransactionInfo> {
        let _token_addr: Address = token_address.parse()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid token address: {}", e) 
            })?;
        
        let _to_addr: Address = to.parse()
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid to address: {}", e) 
            })?;
        
        // TODO: Implementar transferencia real del token
        // Por ahora devolvemos información mock
        Ok(TransactionInfo {
            hash: "0xabcdef1234567890".to_string(),
            from: format!("{:?}", self.wallet.address()),
            to: to.to_string(),
            amount: U256::from(amount),
            gas_used: Some(U256::from(45000)),
            status: "pending".to_string(),
        })
    }
} 