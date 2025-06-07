use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub to_address: String,
    pub amount: u64,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub signature: String,
    pub amount: u64,
} 