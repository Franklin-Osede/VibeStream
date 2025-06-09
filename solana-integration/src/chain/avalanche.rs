use crate::chain::{Chain, ChainInfo, ChainConfig, Network};
use crate::metrics::WalletMetrics;
use async_trait::async_trait;
use anyhow::{Result, anyhow, Context};
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, TransactionRequest, U256, BlockNumber},
};
use std::{
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use log::{info, error, debug, warn};

#[derive(Debug)]
pub struct AvalancheChain {
    wallet: LocalWallet,
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    metrics: WalletMetrics,
    chain_info: ChainInfo,
    config: ChainConfig,
}

impl AvalancheChain {
    pub async fn new(private_key: &str, config: ChainConfig) -> Result<Self> {
        info!("Creating new AvalancheChain instance...");
        
        // Create provider with Avalanche-specific settings
        let provider = Provider::<Http>::try_from(&config.rpc_url)
            .with_context(|| format!("Failed to create provider with URL: {}", config.rpc_url))?;
        let provider = provider.interval(Duration::from_millis(10));
        
        // Create wallet from private key
        let wallet = private_key.parse::<LocalWallet>()
            .with_context(|| "Failed to create wallet from private key")?;
        
        // Create client with signer
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.clone(),
        ));
        
        let chain_info = ChainInfo {
            chain_id: "avalanche".to_string(),
            name: "Avalanche".to_string(),
            native_token: "AVAX".to_string(),
            decimals: 18,
        };
        
        debug!("Avalanche client initialized with config: {:?}", config);
        
        Ok(Self {
            wallet,
            client,
            metrics: WalletMetrics::new(),
            chain_info,
            config,
        })
    }
    
    pub fn get_metrics(&self) -> &WalletMetrics {
        &self.metrics
    }
    
    /// Get the current gas price from the Avalanche network
    pub async fn get_gas_price(&self) -> Result<U256> {
        self.client
            .get_gas_price()
            .await
            .with_context(|| "Failed to get gas price")
    }
    
    /// Get the latest block number
    pub async fn get_block_number(&self) -> Result<U256> {
        self.client
            .get_block(BlockNumber::Latest)
            .await
            .with_context(|| "Failed to get latest block")?
            .ok_or_else(|| anyhow!("Block not found"))?
            .number
            .ok_or_else(|| anyhow!("Block number not available"))
    }
    
    /// Calculate gas limit for Avalanche C-Chain
    async fn estimate_gas(&self, to: Address, amount: U256) -> Result<U256> {
        let tx = TransactionRequest::new()
            .to(to)
            .value(amount);
            
        self.client
            .estimate_gas(&tx, None)
            .await
            .with_context(|| "Failed to estimate gas")
    }
}

#[async_trait]
impl Chain for AvalancheChain {
    fn get_chain_info(&self) -> &ChainInfo {
        &self.chain_info
    }
    
    fn get_address(&self) -> String {
        self.wallet.address().to_string()
    }
    
    async fn get_balance(&self) -> Result<u64> {
        debug!("Querying balance for {}", self.get_address());
        let balance = self.client
            .get_balance(self.wallet.address(), None)
            .await
            .with_context(|| format!("Failed to get balance for {}", self.get_address()))?;
            
        // Convert from nAVAX (u256) to our u64 format
        let balance_u64 = balance
            .as_u64()
            .min(u64::MAX);
            
        info!("Balance retrieved: {} AVAX", self.from_base_unit(balance_u64));
        Ok(balance_u64)
    }
    
    async fn transfer(&mut self, to_address: &str, amount: u64) -> Result<String> {
        info!("Initiating transfer of {} nAVAX to {}", amount, to_address);
        let start_time = Instant::now();
        self.metrics.record_transaction_attempt();
        
        // Validate and parse destination address
        if !self.validate_address(to_address) {
            self.metrics.record_transaction_failure();
            return Err(anyhow!("Invalid Avalanche address format"));
        }
        
        let to_address = Address::from_str(to_address)
            .with_context(|| format!("Failed to parse address: {}", to_address))?;

        // Check balance
        let balance = self.get_balance().await?;
        if balance < amount {
            self.metrics.record_transaction_failure();
            return Err(anyhow!(
                "Insufficient balance. Have {} AVAX, need {} AVAX",
                self.from_base_unit(balance),
                self.from_base_unit(amount)
            ));
        }

        // Get current gas price and estimate gas limit
        let gas_price = self.get_gas_price().await?;
        let amount_u256 = U256::from(amount);
        let gas_limit = self.estimate_gas(to_address, amount_u256).await?;
        
        // Add 10% buffer to gas limit for safety
        let adjusted_gas_limit = gas_limit + (gas_limit / 10);

        // Create transaction request with Avalanche C-Chain specific settings
        let tx = TransactionRequest::new()
            .to(to_address)
            .value(amount_u256)
            .gas(adjusted_gas_limit)
            .gas_price(gas_price);

        // Send transaction
        info!("Sending transaction with gas limit: {}", adjusted_gas_limit);
        let pending_tx = self.client
            .send_transaction(tx, None)
            .await
            .with_context(|| "Failed to send transaction")?;
            
        debug!("Transaction sent, waiting for confirmation...");
        
        // Wait for confirmation - Avalanche typically confirms quickly
        let receipt = pending_tx
            .confirmations(2) // Wait for 2 confirmations on Avalanche
            .await
            .with_context(|| "Failed to get transaction confirmation")?;
            
        let elapsed_time = start_time.elapsed();
        
        match receipt {
            Some(receipt) => {
                if receipt.status.unwrap_or_default().as_u64() == 1 {
                    info!("Transaction confirmed successfully");
                    self.metrics.record_transaction_success(amount, elapsed_time);
                    Ok(receipt.transaction_hash.to_string())
                } else {
                    error!("Transaction failed");
                    self.metrics.record_transaction_failure();
                    Err(anyhow!("Transaction failed"))
                }
            }
            None => {
                warn!("Transaction not found after confirmation wait");
                self.metrics.record_transaction_failure();
                Err(anyhow!("Transaction not found"))
            }
        }
    }
    
    fn validate_address(&self, address: &str) -> bool {
        Address::from_str(address).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    fn create_test_config() -> ChainConfig {
        ChainConfig {
            network: Network::Testnet,
            rpc_url: "https://api.avax-test.network/ext/bc/C/rpc".to_string(),
            retry_attempts: 3,
            timeout: Duration::from_secs(30),
        }
    }
    
    #[tokio::test]
    async fn test_chain_creation() {
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        let chain = AvalancheChain::new(private_key, create_test_config())
            .await
            .unwrap();
        assert_eq!(chain.get_chain_info().chain_id, "avalanche");
        assert_eq!(chain.get_chain_info().decimals, 18);
    }
    
    #[tokio::test]
    async fn test_address_validation() {
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        let chain = AvalancheChain::new(private_key, create_test_config())
            .await
            .unwrap();
        let valid_address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        assert!(chain.validate_address(valid_address));
        assert!(!chain.validate_address("invalid_address"));
    }
    
    #[tokio::test]
    async fn test_amount_conversion() {
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        let chain = AvalancheChain::new(private_key, create_test_config())
            .await
            .unwrap();
        let amount = 1.5; // 1.5 AVAX
        let navax = chain.to_base_unit(amount);
        assert_eq!(chain.from_base_unit(navax), amount);
    }
    
    #[tokio::test]
    async fn test_gas_estimation() {
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        let chain = AvalancheChain::new(private_key, create_test_config())
            .await
            .unwrap();
        let to_address = Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap();
        let amount = U256::from(1_000_000_000); // 1 AVAX
        let gas_limit = chain.estimate_gas(to_address, amount).await.unwrap();
        assert!(gas_limit > U256::zero());
    }
} 