use crate::chain::{Chain, ChainInfo, ChainConfig, Network};
use crate::metrics::WalletMetrics;
use async_trait::async_trait;
use anyhow::{Result, anyhow, Context};
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    commitment_config::{CommitmentConfig, CommitmentLevel},
};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};
use std::time::{Duration, Instant};
use std::str::FromStr;
use log::{info, error, debug, warn};

#[derive(Debug)]
pub struct SolanaChain {
    keypair: Keypair,
    client: RpcClient,
    metrics: WalletMetrics,
    chain_info: ChainInfo,
    config: ChainConfig,
}

impl SolanaChain {
    pub fn new(keypair: Keypair, config: ChainConfig) -> Self {
        info!("Creating new SolanaChain instance...");
        
        let client = RpcClient::new_with_commitment(
            config.rpc_url.clone(),
            CommitmentConfig::finalized()
        );
        
        let chain_info = ChainInfo {
            chain_id: "solana".to_string(),
            name: "Solana".to_string(),
            native_token: "SOL".to_string(),
            decimals: 9,
        };
        
        debug!("RPC client initialized with config: {:?}", config);
        
        Self {
            keypair,
            client,
            metrics: WalletMetrics::new(),
            chain_info,
            config,
        }
    }
    
    pub fn get_metrics(&self) -> &WalletMetrics {
        &self.metrics
    }
}

#[async_trait]
impl Chain for SolanaChain {
    fn get_chain_info(&self) -> &ChainInfo {
        &self.chain_info
    }
    
    fn get_address(&self) -> String {
        self.keypair.pubkey().to_string()
    }
    
    async fn get_balance(&self) -> Result<u64> {
        debug!("Querying balance for {}", self.get_address());
        let balance = self.client
            .get_balance_with_commitment(
                &self.keypair.pubkey(),
                CommitmentConfig::finalized()
            )
            .with_context(|| format!("Failed to get balance for {}", self.get_address()))?
            .value;
        
        info!("Balance retrieved: {} SOL", self.from_base_unit(balance));
        Ok(balance)
    }
    
    async fn transfer(&mut self, to_address: &str, amount: u64) -> Result<String> {
        info!("Initiating transfer of {} lamports to {}", amount, to_address);
        let start_time = Instant::now();
        self.metrics.record_transaction_attempt();
        
        // Validate and parse destination address
        if !self.validate_address(to_address) {
            self.metrics.record_transaction_failure();
            return Err(anyhow!("Invalid address format"));
        }
        
        let to_pubkey = Pubkey::from_str(to_address)
            .with_context(|| format!("Failed to parse address: {}", to_address))?;

        // Check balance before transfer
        let balance = self.get_balance().await?;
        if balance < amount {
            self.metrics.record_transaction_failure();
            return Err(anyhow!(
                "Insufficient balance. Have {} SOL, need {} SOL",
                self.from_base_unit(balance),
                self.from_base_unit(amount)
            ));
        }

        // Create transfer instruction
        let instruction = system_instruction::transfer(
            &self.keypair.pubkey(),
            &to_pubkey,
            amount,
        );
        debug!("Transfer instruction created");

        // Get latest blockhash
        let recent_blockhash = self.client
            .get_latest_blockhash()
            .with_context(|| "Failed to get latest blockhash")?;
        debug!("Blockhash obtained: {}", recent_blockhash);

        // Create and sign transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            recent_blockhash,
        );
        debug!("Transaction created and signed");

        // Send and confirm transaction
        info!("Sending transaction...");
        let signature = self.client
            .send_and_confirm_transaction_with_spinner_and_config(
                &transaction,
                CommitmentConfig::finalized(),
                RpcSendTransactionConfig {
                    skip_preflight: false,
                    preflight_commitment: Some(CommitmentLevel::Finalized),
                    encoding: None,
                    max_retries: Some(self.config.retry_attempts),
                    min_context_slot: None,
                },
            )
            .with_context(|| "Failed to send transaction")?;
        
        info!("Transaction sent successfully. Signature: {}", signature);

        // Additional confirmation wait
        std::thread::sleep(Duration::from_secs(1));
        debug!("Waiting for additional confirmation...");

        // Verify transaction status
        let confirmation = self.client
            .get_signature_status_with_commitment(&signature, CommitmentConfig::finalized())
            .with_context(|| format!("Failed to verify transaction: {}", signature))?;

        let elapsed_time = start_time.elapsed();

        match confirmation {
            Some(Ok(_)) => {
                info!("Transaction confirmed successfully");
                self.metrics.record_transaction_success(amount, elapsed_time);
                Ok(signature.to_string())
            },
            Some(Err(e)) => {
                error!("Transaction failed: {:?}", e);
                self.metrics.record_transaction_failure();
                Err(anyhow!("Transaction failed: {:?}", e))
            },
            None => {
                warn!("Transaction not found after confirmation");
                self.metrics.record_transaction_failure();
                Err(anyhow!("Transaction not found"))
            }
        }
    }
    
    fn validate_address(&self, address: &str) -> bool {
        Pubkey::from_str(address).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_config() -> ChainConfig {
        ChainConfig {
            network: Network::Devnet,
            rpc_url: "https://api.devnet.solana.com".to_string(),
            retry_attempts: 3,
            timeout: Duration::from_secs(30),
        }
    }
    
    #[tokio::test]
    async fn test_chain_creation() {
        let chain = SolanaChain::new(Keypair::new(), create_test_config());
        assert_eq!(chain.get_chain_info().chain_id, "solana");
        assert_eq!(chain.get_chain_info().decimals, 9);
        assert_eq!(chain.get_metrics().get_success_rate(), 0.0);
    }
    
    #[tokio::test]
    async fn test_address_validation() {
        let chain = SolanaChain::new(Keypair::new(), create_test_config());
        let valid_address = Keypair::new().pubkey().to_string();
        assert!(chain.validate_address(&valid_address));
        assert!(!chain.validate_address("invalid_address"));
    }
    
    #[tokio::test]
    async fn test_amount_conversion() {
        let chain = SolanaChain::new(Keypair::new(), create_test_config());
        let amount = 1.5; // 1.5 SOL
        let lamports = chain.to_base_unit(amount);
        assert_eq!(lamports, 1_500_000_000);
        assert_eq!(chain.from_base_unit(lamports), amount);
    }
} 