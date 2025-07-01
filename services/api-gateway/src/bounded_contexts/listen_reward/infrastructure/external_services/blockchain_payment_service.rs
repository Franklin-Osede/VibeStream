// Blockchain Payment Service (Stub)
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{ExternalServiceHealth, ExternalServiceHealthCheck};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResult {
    pub transaction_hash: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentError {
    InsufficientFunds,
    NetworkError(String),
}

pub type TransactionHash = String;

#[async_trait]
pub trait BlockchainPaymentService: Send + Sync {
    async fn send_payment(&self, amount: f64, recipient: &str) -> Result<PaymentResult, PaymentError>;
}

pub struct MockBlockchainPaymentService;

#[async_trait]
impl BlockchainPaymentService for MockBlockchainPaymentService {
    async fn send_payment(&self, _amount: f64, _recipient: &str) -> Result<PaymentResult, PaymentError> {
        Ok(PaymentResult {
            transaction_hash: "0x123".to_string(),
            success: true,
        })
    }
}

#[async_trait]
impl ExternalServiceHealthCheck for MockBlockchainPaymentService {
    async fn health_check(&self) -> ExternalServiceHealth {
        ExternalServiceHealth::healthy("blockchain_payment".to_string(), 50)
    }
} 