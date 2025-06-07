use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;

// Response types that match the Solana service
#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransferRequest {
    pub to_address: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct TransferResponse {
    pub signature: String,
    pub amount: f64,
}

// Solana service client
#[derive(Clone)]
pub struct SolanaClient {
    client: Client,
    base_url: String,
}

impl SolanaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    // Get wallet balance
    pub async fn get_balance(&self) -> Result<f64> {
        let response = self.client
            .get(format!("{}/wallet/balance", self.base_url))
            .send()
            .await?;

        let api_response: ApiResponse<f64> = response.json().await?;
        
        match (api_response.success, api_response.data, api_response.error) {
            (true, Some(balance), _) => Ok(balance),
            (false, _, Some(error)) => Err(anyhow::anyhow!(error)),
            _ => Err(anyhow::anyhow!("Invalid response from Solana service")),
        }
    }

    // Transfer SOL
    pub async fn transfer(&self, to_address: String, amount: f64) -> Result<String> {
        let request = TransferRequest {
            to_address,
            amount,
        };

        let response = self.client
            .post(format!("{}/wallet/transfer", self.base_url))
            .json(&request)
            .send()
            .await?;

        let api_response: ApiResponse<TransferResponse> = response.json().await?;
        
        match (api_response.success, api_response.data, api_response.error) {
            (true, Some(transfer), _) => Ok(transfer.signature),
            (false, _, Some(error)) => Err(anyhow::anyhow!(error)),
            _ => Err(anyhow::anyhow!("Invalid response from Solana service")),
        }
    }

    // Health check
    pub async fn health_check(&self) -> Result<bool> {
        let response = self.client
            .get(format!("{}/health", self.base_url))
            .send()
            .await?;

        let api_response: ApiResponse<String> = response.json().await?;
        Ok(api_response.success)
    }
} 