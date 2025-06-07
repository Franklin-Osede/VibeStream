#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    #[allow(dead_code)]
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TransferRequest {
    pub to_address: String,
    pub amount: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TransferResponse {
    pub signature: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceResponse {
    pub balance: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct MintNFTRequest {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NFTResponse {
    pub mint_address: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct TransferNFTRequest {
    pub nft_address: String,
    pub to_address: String,
} 