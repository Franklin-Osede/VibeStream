use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FractionalOwnershipError {
    #[error("Insufficient shares available: requested {requested}, available {available}")]
    InsufficientSharesAvailable { requested: u32, available: u32 },
    
    #[error("Invalid ownership percentage: {percentage}%. Must be between 0.01% and 100%")]
    InvalidOwnershipPercentage { percentage: f64 },
    
    #[error("Share price must be greater than zero: {price}")]
    InvalidSharePrice { price: f64 },
    
    #[error("Total ownership cannot exceed 100%: current {current}%, attempting to add {additional}%")]
    OwnershipExceedsLimit { current: f64, additional: f64 },
    
    #[error("Song is not available for fractional ownership")]
    SongNotAvailableForOwnership,
    
    #[error("User already owns maximum allowed shares")]
    MaximumSharesPerUserExceeded,
    
    #[error("Cannot purchase shares in own song")]
    CannotPurchaseOwnSong,
    
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: f64, available: f64 },
    
    #[error("Revenue distribution failed: {reason}")]
    RevenueDistributionFailed { reason: String },
    
    #[error("Share trading is currently disabled for this song")]
    TradingDisabled,
} 