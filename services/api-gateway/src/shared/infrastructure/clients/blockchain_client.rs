use ethers::prelude::*;
use std::sync::Arc;
use std::convert::TryFrom;
use crate::shared::domain::errors::AppError;

/// Configuration for BlockchainClient
#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    pub rpc_url: String,
    pub chain_id: u64,
    pub private_key: Option<String>,
}

/// Chain-agnostic Blockchain Client
/// Wraps ethers-rs to provide a unified interface for EVM chains
#[derive(Clone)]
pub struct BlockchainClient {
    pub provider: Arc<Provider<Http>>,
    pub wallet: Option<LocalWallet>,
    pub chain_id: u64,
}

impl BlockchainClient {
    /// Create a new BlockchainClient
    pub async fn new(config: BlockchainConfig) -> Result<Self, AppError> {
        let provider = Provider::<Http>::try_from(config.rpc_url.clone())
            .map_err(|e| AppError::InternalError(format!("Invalid RPC URL: {}", e)))?;

        let provider = Arc::new(provider);

        let wallet = if let Some(pk) = config.private_key {
            let wallet = pk.parse::<LocalWallet>()
                .map_err(|e| AppError::InternalError(format!("Invalid private key: {}", e)))?
                .with_chain_id(config.chain_id);
            Some(wallet)
        } else {
            None
        };

        Ok(Self {
            provider,
            wallet,
            chain_id: config.chain_id,
        })
    }

    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64, AppError> {
        let block = self.provider.get_block_number().await
            .map_err(|e| AppError::ExternalServiceError(format!("Failed to get block number: {}", e)))?;
        Ok(block.as_u64())
    }

    /// Convert to Provider (read-only middleware)
    pub fn get_middleware(&self) -> Arc<Provider<Http>> {
        self.provider.clone()
    }

    /// Get a client capable of signing transactions (if wallet is configured)
    pub fn get_signer_middleware(&self) -> Result<Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>, AppError> {
        if let Some(wallet) = &self.wallet {
            let client = SignerMiddleware::new(self.provider.clone(), wallet.clone());
            Ok(Arc::new(client))
        } else {
            Err(AppError::ConfigurationError("No private key configured for signing".to_string()))
        }
    }

    /// Send a transaction (transfer ETH/MATIC/etc)
    pub async fn send_transaction(&self, to: &str, value_wei: u64) -> Result<String, AppError> {
        let signer_middleware = self.get_signer_middleware()?;
        
        let to_address: Address = to.parse()
            .map_err(|e| AppError::ValidationError(format!("Invalid to address: {}", e)))?;

        let tx = TransactionRequest::new()
            .to(to_address)
            .value(value_wei);

        let pending_tx = signer_middleware.send_transaction(tx, None).await
            .map_err(|e| AppError::ExternalServiceError(format!("Failed to send transaction: {}", e)))?;

        let receipt = pending_tx.await
            .map_err(|e| AppError::ExternalServiceError(format!("Failed to await receipt: {}", e)))?
            .ok_or_else(|| AppError::ExternalServiceError("Transaction dropped".to_string()))?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }
}
