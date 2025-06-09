use crate::chain::{Chain, ChainConfig};
use crate::chain::solana::SolanaChain;
use crate::chain::ethereum::EthereumChain;
use crate::chain::polygon::PolygonChain;
use crate::chain::avalanche::AvalancheChain;
use anyhow::{Result, anyhow};
use solana_sdk::signature::Keypair;

/// Supported blockchain types
#[derive(Debug, Clone, Copy)]
pub enum ChainType {
    Solana,
    Ethereum,
    Polygon,
    Avalanche,
}

/// Factory for creating chain-specific implementations
pub struct ChainFactory;

impl ChainFactory {
    /// Create a new chain instance based on the specified type
    pub async fn create(chain_type: ChainType, config: ChainConfig) -> Result<Box<dyn Chain>> {
        match chain_type {
            ChainType::Solana => {
                let keypair = Keypair::new(); // In production, this should be loaded from secure storage
                Ok(Box::new(SolanaChain::new(keypair, config)))
            },
            ChainType::Ethereum => {
                // Generate a random private key for testing
                // In production, this should be loaded from secure storage
                let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
                EthereumChain::new(private_key, config)
                    .await
                    .map(|chain| Box::new(chain) as Box<dyn Chain>)
            },
            ChainType::Polygon => {
                // Generate a random private key for testing
                // In production, this should be loaded from secure storage
                let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
                PolygonChain::new(private_key, config)
                    .await
                    .map(|chain| Box::new(chain) as Box<dyn Chain>)
            },
            ChainType::Avalanche => {
                // Generate a random private key for testing
                // In production, this should be loaded from secure storage
                let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
                AvalancheChain::new(private_key, config)
                    .await
                    .map(|chain| Box::new(chain) as Box<dyn Chain>)
            }
        }
    }
    
    /// Create a Solana chain instance from an existing keypair
    pub fn create_with_solana_keypair(
        keypair: Keypair,
        config: ChainConfig
    ) -> Result<Box<dyn Chain>> {
        Ok(Box::new(SolanaChain::new(keypair, config)))
    }
    
    /// Create an Ethereum chain instance from an existing private key
    pub async fn create_with_ethereum_key(
        private_key: &str,
        config: ChainConfig
    ) -> Result<Box<dyn Chain>> {
        EthereumChain::new(private_key, config)
            .await
            .map(|chain| Box::new(chain) as Box<dyn Chain>)
    }
    
    /// Create a Polygon chain instance from an existing private key
    pub async fn create_with_polygon_key(
        private_key: &str,
        config: ChainConfig
    ) -> Result<Box<dyn Chain>> {
        PolygonChain::new(private_key, config)
            .await
            .map(|chain| Box::new(chain) as Box<dyn Chain>)
    }
    
    /// Create an Avalanche chain instance from an existing private key
    pub async fn create_with_avalanche_key(
        private_key: &str,
        config: ChainConfig
    ) -> Result<Box<dyn Chain>> {
        AvalancheChain::new(private_key, config)
            .await
            .map(|chain| Box::new(chain) as Box<dyn Chain>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::chain::Network;
    
    async fn create_test_config(chain_type: ChainType) -> ChainConfig {
        match chain_type {
            ChainType::Solana => ChainConfig {
                network: Network::Devnet,
                rpc_url: "https://api.devnet.solana.com".to_string(),
                retry_attempts: 3,
                timeout: Duration::from_secs(30),
            },
            ChainType::Ethereum => ChainConfig {
                network: Network::Testnet,
                rpc_url: "https://eth-sepolia.g.alchemy.com/v2/your-api-key".to_string(),
                retry_attempts: 3,
                timeout: Duration::from_secs(30),
            },
            ChainType::Polygon => ChainConfig {
                network: Network::Testnet,
                rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/your-api-key".to_string(),
                retry_attempts: 3,
                timeout: Duration::from_secs(30),
            },
            ChainType::Avalanche => ChainConfig {
                network: Network::Testnet,
                rpc_url: "https://api.avax-test.network/ext/bc/C/rpc".to_string(),
                retry_attempts: 3,
                timeout: Duration::from_secs(30),
            },
        }
    }
    
    #[tokio::test]
    async fn test_solana_chain_creation() {
        let config = create_test_config(ChainType::Solana).await;
        let chain = ChainFactory::create(ChainType::Solana, config).await.unwrap();
        assert_eq!(chain.get_chain_info().chain_id, "solana");
    }
    
    #[tokio::test]
    async fn test_ethereum_chain_creation() {
        let config = create_test_config(ChainType::Ethereum).await;
        let chain = ChainFactory::create(ChainType::Ethereum, config).await.unwrap();
        assert_eq!(chain.get_chain_info().chain_id, "ethereum");
    }
    
    #[tokio::test]
    async fn test_polygon_chain_creation() {
        let config = create_test_config(ChainType::Polygon).await;
        let chain = ChainFactory::create(ChainType::Polygon, config).await.unwrap();
        assert_eq!(chain.get_chain_info().chain_id, "polygon");
    }
    
    #[tokio::test]
    async fn test_avalanche_chain_creation() {
        let config = create_test_config(ChainType::Avalanche).await;
        let chain = ChainFactory::create(ChainType::Avalanche, config).await.unwrap();
        assert_eq!(chain.get_chain_info().chain_id, "avalanche");
    }
    
    #[tokio::test]
    async fn test_solana_with_keypair() {
        let config = create_test_config(ChainType::Solana).await;
        let keypair = Keypair::new();
        let expected_address = keypair.pubkey().to_string();
        
        let chain = ChainFactory::create_with_solana_keypair(keypair, config).unwrap();
        assert_eq!(chain.get_address(), expected_address);
    }
    
    #[tokio::test]
    async fn test_ethereum_with_private_key() {
        let config = create_test_config(ChainType::Ethereum).await;
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        
        let chain = ChainFactory::create_with_ethereum_key(private_key, config)
            .await
            .unwrap();
        assert!(chain.validate_address(&chain.get_address()));
    }
    
    #[tokio::test]
    async fn test_polygon_with_private_key() {
        let config = create_test_config(ChainType::Polygon).await;
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        
        let chain = ChainFactory::create_with_polygon_key(private_key, config)
            .await
            .unwrap();
        assert!(chain.validate_address(&chain.get_address()));
    }
    
    #[tokio::test]
    async fn test_avalanche_with_private_key() {
        let config = create_test_config(ChainType::Avalanche).await;
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        
        let chain = ChainFactory::create_with_avalanche_key(private_key, config)
            .await
            .unwrap();
        assert!(chain.validate_address(&chain.get_address()));
    }
} 