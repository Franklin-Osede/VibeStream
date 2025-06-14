use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum VibeStreamError {
    #[error("Blockchain error: {message}")]
    Blockchain { message: String },
    
    #[error("Database error: {message}")]
    Database { message: String },
    
    #[error("Network error: {message}")]
    Network { message: String },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },
    
    #[error("Transaction not found: {hash}")]
    TransactionNotFound { hash: String },
    
    #[error("Wallet not found: {address}")]
    WalletNotFound { address: String },
    
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
    
    #[error("Internal error: {message}")]
    Internal { message: String },
}

pub type Result<T> = std::result::Result<T, VibeStreamError>; 