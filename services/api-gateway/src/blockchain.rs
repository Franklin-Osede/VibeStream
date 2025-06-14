use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use vibestream_types::VibeStreamError;

#[derive(Debug, Clone)]
pub struct BlockchainClients {
    pub ethereum_client: BlockchainClient,
    pub solana_client: BlockchainClient,
    http_client: Client,
}

impl BlockchainClients {
    pub fn new() -> Self {
        let http_client = Client::new();
        
        Self {
            ethereum_client: BlockchainClient::new(
                http_client.clone(),
                std::env::var("ETHEREUM_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:3001".to_string())
            ),
            solana_client: BlockchainClient::new(
                http_client.clone(),
                std::env::var("SOLANA_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:3003".to_string())
            ),
            http_client,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockchainClient {
    http_client: Client,
    base_url: String,
}

impl BlockchainClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn get_balance(&self, address: &str) -> Result<u64, VibeStreamError> {
        let url = format!("{}/balance/{}", self.base_url, address);
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to get balance: {}", e) 
            })?;

        if !response.status().is_success() {
            return Err(VibeStreamError::Network {
                message: format!("Balance request failed with status: {}", response.status())
            });
        }

        let balance: u64 = response
            .json()
            .await
            .map_err(|e| VibeStreamError::Serialization { 
                message: format!("Failed to parse balance response: {}", e) 
            })?;

        Ok(balance)
    }

    pub async fn transfer(&self, to: &str, amount: u64) -> Result<TransactionInfo, VibeStreamError> {
        let url = format!("{}/transfer", self.base_url);
        
        let request = TransferRequest {
            to: to.to_string(),
            amount,
        };

        let response = self.http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to send transfer: {}", e) 
            })?;

        if !response.status().is_success() {
            return Err(VibeStreamError::Network {
                message: format!("Transfer request failed with status: {}", response.status())
            });
        }

        let tx_info: TransactionInfo = response
            .json()
            .await
            .map_err(|e| VibeStreamError::Serialization { 
                message: format!("Failed to parse transfer response: {}", e) 
            })?;

        Ok(tx_info)
    }

    pub async fn health_check(&self) -> Result<bool, VibeStreamError> {
        let url = format!("{}/health", self.base_url);
        
        match self.http_client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub to: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub gas_fee: u64,
    pub block_number: Option<u64>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub song_id: Uuid,
    pub user_wallet: String,
    pub blockchain: String, // "ethereum" or "solana"
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub transaction_hash: String,
    pub payment_id: Uuid,
    pub song_id: Uuid,
    pub user_id: Uuid,
    pub artist_id: Uuid,
    pub amount_paid: Decimal,
    pub artist_royalty: Decimal,
    pub platform_fee: Decimal,
    pub blockchain: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoyaltyDistribution {
    pub artist_wallet: String,
    pub artist_amount: Decimal,
    pub platform_wallet: String,
    pub platform_amount: Decimal,
}

// Funciones de utilidad para cálculos de regalías
pub fn calculate_royalty_distribution(
    total_amount: Decimal,
    royalty_percentage: Decimal,
    platform_fee_percentage: Decimal,
) -> RoyaltyDistribution {
    let artist_amount = total_amount * (royalty_percentage / Decimal::from(100));
    let platform_amount = total_amount * (platform_fee_percentage / Decimal::from(100));
    
    RoyaltyDistribution {
        artist_wallet: String::new(), // Se llenará con la dirección real
        artist_amount,
        platform_wallet: std::env::var("PLATFORM_WALLET_ADDRESS")
            .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000".to_string()),
        platform_amount,
    }
} 