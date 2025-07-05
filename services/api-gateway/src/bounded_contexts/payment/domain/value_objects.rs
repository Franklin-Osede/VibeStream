 use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;

/// Payment ID Value Object
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentId(Uuid);

impl PaymentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }
    
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Transaction ID Value Object
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(Uuid);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }
    
    pub fn value(&self) -> Uuid {
        self.0
    }
}

/// Money Amount Value Object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Amount {
    value: f64,
    currency: Currency,
}

impl Amount {
    pub fn new(value: f64, currency: Currency) -> Result<Self, AppError> {
        if value < 0.0 {
            return Err(AppError::InvalidInput("Amount cannot be negative".to_string()));
        }
        
        if value > 1_000_000_000.0 {
            return Err(AppError::InvalidInput("Amount exceeds maximum limit".to_string()));
        }
        
        Ok(Self { value, currency })
    }
    
    pub fn value(&self) -> f64 {
        self.value
    }
    
    pub fn currency(&self) -> &Currency {
        &self.currency
    }
    
    pub fn add(&self, other: &Amount) -> Result<Amount, AppError> {
        if self.currency != other.currency {
            return Err(AppError::InvalidInput("Cannot add amounts with different currencies".to_string()));
        }
        
        Amount::new(self.value + other.value, self.currency.clone())
    }
    
    pub fn subtract(&self, other: &Amount) -> Result<Amount, AppError> {
        if self.currency != other.currency {
            return Err(AppError::InvalidInput("Cannot subtract amounts with different currencies".to_string()));
        }
        
        Amount::new(self.value - other.value, self.currency.clone())
    }
    
    pub fn multiply(&self, factor: f64) -> Result<Amount, AppError> {
        Amount::new(self.value * factor, self.currency.clone())
    }
    
    pub fn percentage_of(&self, percentage: f64) -> Result<Amount, AppError> {
        if percentage < 0.0 || percentage > 100.0 {
            return Err(AppError::InvalidInput("Percentage must be between 0 and 100".to_string()));
        }
        
        Amount::new(self.value * percentage / 100.0, self.currency.clone())
    }
}

/// Currency Enumeration
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    USD,
    ETH,
    SOL,
    USDC,
    VIBES, // Platform token
}

impl Currency {
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::ETH => "Ξ",
            Currency::SOL => "◎",
            Currency::USDC => "USDC",
            Currency::VIBES => "VIBES",
        }
    }
    
    pub fn is_cryptocurrency(&self) -> bool {
        matches!(self, Currency::ETH | Currency::SOL | Currency::USDC | Currency::VIBES)
    }
}

/// Payment Method Value Object
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard {
        last_four_digits: String,
        card_type: CardType,
    },
    Cryptocurrency {
        blockchain: Blockchain,
        wallet_address: WalletAddress,
    },
    PlatformBalance,
    BankTransfer {
        bank_name: String,
        account_ending: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Visa,
    Mastercard,
    AmericanExpress,
    Discover,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Blockchain {
    Ethereum,
    Solana,
    Polygon,
    Binance,
}

impl Blockchain {
    pub fn native_currency(&self) -> Currency {
        match self {
            Blockchain::Ethereum => Currency::ETH,
            Blockchain::Solana => Currency::SOL,
            Blockchain::Polygon => Currency::ETH, // MATIC on Polygon
            Blockchain::Binance => Currency::ETH, // BNB on BSC
        }
    }
    
    pub fn supports_currency(&self, currency: &Currency) -> bool {
        match (self, currency) {
            (Blockchain::Ethereum, Currency::ETH | Currency::USDC) => true,
            (Blockchain::Solana, Currency::SOL | Currency::USDC) => true,
            (_, Currency::VIBES) => true, // Platform token on all chains
            _ => false,
        }
    }
}

/// Wallet Address Value Object
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletAddress(String);

impl WalletAddress {
    pub fn new(address: String) -> Result<Self, AppError> {
        if address.trim().is_empty() {
            return Err(AppError::InvalidInput("Wallet address cannot be empty".to_string()));
        }
        
        // Basic validation - in production you'd want blockchain-specific validation
        if address.len() < 26 || address.len() > 62 {
            return Err(AppError::InvalidInput("Invalid wallet address format".to_string()));
        }
        
        Ok(Self(address.trim().to_string()))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn is_ethereum_format(&self) -> bool {
        self.0.starts_with("0x") && self.0.len() == 42
    }
    
    pub fn is_solana_format(&self) -> bool {
        self.0.len() >= 32 && self.0.len() <= 44 && !self.0.starts_with("0x")
    }
}

/// Fee Percentage Value Object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeePercentage(f64);

impl FeePercentage {
    pub fn new(percentage: f64) -> Result<Self, AppError> {
        if percentage < 0.0 {
            return Err(AppError::InvalidInput("Fee percentage cannot be negative".to_string()));
        }
        
        if percentage > 100.0 {
            return Err(AppError::InvalidInput("Fee percentage cannot exceed 100%".to_string()));
        }
        
        Ok(Self(percentage))
    }
    
    pub fn value(&self) -> f64 {
        self.0
    }
    
    pub fn calculate_fee(&self, amount: &Amount) -> Result<Amount, AppError> {
        amount.percentage_of(self.0)
    }
    
    pub fn calculate_net_amount(&self, gross_amount: &Amount) -> Result<Amount, AppError> {
        let fee = self.calculate_fee(gross_amount)?;
        gross_amount.subtract(&fee)
    }
}

/// Transaction Hash Value Object
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionHash(String);

impl TransactionHash {
    pub fn new(hash: String) -> Result<Self, AppError> {
        if hash.trim().is_empty() {
            return Err(AppError::InvalidInput("Transaction hash cannot be empty".to_string()));
        }
        
        // Basic validation for common hash formats
        let clean_hash = hash.trim();
        if clean_hash.len() < 32 || clean_hash.len() > 128 {
            return Err(AppError::InvalidInput("Invalid transaction hash format".to_string()));
        }
        
        Ok(Self(clean_hash.to_string()))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Payment Purpose - why the payment is being made
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentPurpose {
    /// Purchase of campaign NFT
    NFTPurchase {
        campaign_id: Uuid,
        nft_quantity: u32,
    },
    /// Purchase of fractional shares
    SharePurchase {
        contract_id: Uuid,
        ownership_percentage: f64,
    },
    /// Trading shares between users
    ShareTrade {
        share_id: Uuid,
        from_user: Uuid,
        to_user: Uuid,
    },
    /// Royalty payment to artist
    RoyaltyDistribution {
        song_id: Uuid,
        artist_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    },
    /// Listen reward payment to user
    ListenReward {
        session_id: Uuid,
        song_id: Uuid,
        listen_duration: u32,
    },
    /// Revenue distribution to shareholders
    RevenueDistribution {
        contract_id: Uuid,
        distribution_id: Uuid,
    },
    /// Platform fee collection
    PlatformFee {
        related_payment_id: Uuid,
        fee_type: String,
    },
    /// Refund of previous payment
    Refund {
        original_payment_id: Uuid,
        reason: String,
    },
}

impl PaymentPurpose {
    pub fn description(&self) -> String {
        match self {
            PaymentPurpose::NFTPurchase { campaign_id, nft_quantity } => {
                format!("Purchase of {} NFT(s) from campaign {}", nft_quantity, campaign_id)
            }
            PaymentPurpose::SharePurchase { contract_id, ownership_percentage } => {
                format!("Purchase of {:.2}% ownership in contract {}", ownership_percentage, contract_id)
            }
            PaymentPurpose::ShareTrade { share_id, from_user: _, to_user: _ } => {
                format!("Trade of share {}", share_id)
            }
            PaymentPurpose::RoyaltyDistribution { song_id, artist_id: _, period_start: _, period_end: _ } => {
                format!("Royalty distribution for song {}", song_id)
            }
            PaymentPurpose::ListenReward { session_id, song_id: _, listen_duration } => {
                format!("Listen reward for session {} ({} seconds)", session_id, listen_duration)
            }
            PaymentPurpose::RevenueDistribution { contract_id, distribution_id } => {
                format!("Revenue distribution {} for contract {}", distribution_id, contract_id)
            }
            PaymentPurpose::PlatformFee { related_payment_id, fee_type } => {
                format!("{} fee for payment {}", fee_type, related_payment_id)
            }
            PaymentPurpose::Refund { original_payment_id, reason } => {
                format!("Refund for payment {} - {}", original_payment_id, reason)
            }
        }
    }
    
    pub fn category(&self) -> PaymentCategory {
        match self {
            PaymentPurpose::NFTPurchase { .. } => PaymentCategory::Purchase,
            PaymentPurpose::SharePurchase { .. } => PaymentCategory::Investment,
            PaymentPurpose::ShareTrade { .. } => PaymentCategory::Trade,
            PaymentPurpose::RoyaltyDistribution { .. } => PaymentCategory::Payout,
            PaymentPurpose::ListenReward { .. } => PaymentCategory::Reward,
            PaymentPurpose::RevenueDistribution { .. } => PaymentCategory::Distribution,
            PaymentPurpose::PlatformFee { .. } => PaymentCategory::Fee,
            PaymentPurpose::Refund { .. } => PaymentCategory::Refund,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentCategory {
    Purchase,
    Investment,
    Trade,
    Payout,
    Reward,
    Distribution,
    Fee,
    Refund,
}

/// Payment Status Value Object
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    /// Payment has been initiated but not yet processed
    Pending,
    /// Payment is being processed
    Processing,
    /// Payment completed successfully
    Completed,
    /// Payment failed
    Failed { 
        error_code: String,
        error_message: String,
    },
    /// Payment was cancelled by user or system
    Cancelled {
        reason: String,
    },
    /// Payment is being refunded
    Refunding,
    /// Payment has been refunded
    Refunded {
        refund_amount: f64,
        refund_date: DateTime<Utc>,
    },
}

impl PaymentStatus {
    pub fn is_final(&self) -> bool {
        matches!(self, 
            PaymentStatus::Completed | 
            PaymentStatus::Failed { .. } | 
            PaymentStatus::Cancelled { .. } |
            PaymentStatus::Refunded { .. }
        )
    }
    
    pub fn is_successful(&self) -> bool {
        matches!(self, PaymentStatus::Completed)
    }
    
    pub fn can_be_refunded(&self) -> bool {
        matches!(self, PaymentStatus::Completed)
    }
}

/// Pricing strategy based on platform growth phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformPhase {
    Launch,    // 0-1K users - Freemium aggressive
    Growth,    // 1K-10K users - Introduce fees gradually
    Scale,     // 10K+ users - Competitive fees
    Mature,    // 100K+ users - Premium features
}

/// Dynamic fee configuration based on platform phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicFeeConfig {
    pub phase: PlatformPhase,
    pub user_count: u64,
    pub fees: PlatformFees,
    pub effective_date: DateTime<Utc>,
    pub grandfathered_users: Vec<Uuid>, // Users who keep old pricing
}

/// Platform fees structure with growth-based pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformFees {
    // Music streaming fees
    pub streaming_fee_percentage: f64,
    
    // NFT marketplace fees
    pub nft_marketplace_fee_percentage: f64,
    
    // Fractional ownership fees
    pub ownership_transaction_fee_percentage: f64,
    pub revenue_sharing_fee_percentage: f64,
    
    // Listen rewards fees
    pub reward_processing_fee_percentage: f64,
    
    // Payment processing fees
    pub payment_processing_fee_percentage: f64,
    pub payment_fixed_fee: f64,
    
    // Premium features
    pub premium_analytics_fee: Option<f64>,
    pub premium_marketing_tools_fee: Option<f64>,
}

impl PlatformFees {
    /// Get fees for launch phase (aggressive freemium)
    pub fn launch_phase() -> Self {
        Self {
            streaming_fee_percentage: 0.0,        // FREE to attract users
            nft_marketplace_fee_percentage: 2.5,  // 50% discount vs OpenSea
            ownership_transaction_fee_percentage: 1.0,  // Very low for adoption
            revenue_sharing_fee_percentage: 5.0,  // Low
            reward_processing_fee_percentage: 0.0, // FREE for engagement
            payment_processing_fee_percentage: 2.9, // Only real costs
            payment_fixed_fee: 0.30,
            premium_analytics_fee: None,           // Not available yet
            premium_marketing_tools_fee: None,    // Not available yet
        }
    }
    
    /// Get fees for growth phase (introduce fees gradually)
    pub fn growth_phase() -> Self {
        Self {
            streaming_fee_percentage: 5.0,        // Very low vs competition (Spotify 30%)
            nft_marketplace_fee_percentage: 3.5,  // Still competitive
            ownership_transaction_fee_percentage: 1.5,  // Gradual increase
            revenue_sharing_fee_percentage: 7.5,  // Gradual increase
            reward_processing_fee_percentage: 2.0, // Low but cover costs
            payment_processing_fee_percentage: 2.9,
            payment_fixed_fee: 0.30,
            premium_analytics_fee: Some(9.99),    // Monthly fee for advanced analytics
            premium_marketing_tools_fee: None,    // Not available yet
        }
    }
    
    /// Get fees for scale phase (competitive fees)
    pub fn scale_phase() -> Self {
        Self {
            streaming_fee_percentage: 15.0,       // Final target, still better than Spotify
            nft_marketplace_fee_percentage: 5.0,  // Competitive with OpenSea
            ownership_transaction_fee_percentage: 2.5,  // Final target
            revenue_sharing_fee_percentage: 10.0, // Final target
            reward_processing_fee_percentage: 5.0, // Final target
            payment_processing_fee_percentage: 2.9,
            payment_fixed_fee: 0.30,
            premium_analytics_fee: Some(19.99),   // Premium analytics
            premium_marketing_tools_fee: Some(29.99), // Marketing automation tools
        }
    }
    
    /// Get fees for mature phase (premium platform)
    pub fn mature_phase() -> Self {
        Self {
            streaming_fee_percentage: 15.0,       // Maintain competitive edge
            nft_marketplace_fee_percentage: 5.0,
            ownership_transaction_fee_percentage: 2.5,
            revenue_sharing_fee_percentage: 10.0,
            reward_processing_fee_percentage: 5.0,
            payment_processing_fee_percentage: 2.9,
            payment_fixed_fee: 0.30,
            premium_analytics_fee: Some(29.99),   // Advanced analytics suite
            premium_marketing_tools_fee: Some(49.99), // Full marketing automation
        }
    }
    
    /// Calculate platform fee for a given amount
    pub fn calculate_platform_fee(&self, amount: &Amount, fee_type: FeeType) -> Result<Amount, String> {
        let fee_percentage = match fee_type {
            FeeType::Streaming => self.streaming_fee_percentage,
            FeeType::NFTMarketplace => self.nft_marketplace_fee_percentage,
            FeeType::OwnershipTransaction => self.ownership_transaction_fee_percentage,
            FeeType::RevenueSharing => self.revenue_sharing_fee_percentage,
            FeeType::RewardProcessing => self.reward_processing_fee_percentage,
            FeeType::PaymentProcessing => self.payment_processing_fee_percentage,
        };
        
        let fee_amount = amount.value() * (fee_percentage / 100.0);
        let total_fee = fee_amount + self.payment_fixed_fee;
        
        Amount::new(total_fee, amount.currency())
    }
    
    /// Calculate net amount after platform fees
    pub fn calculate_net_amount(&self, gross_amount: &Amount, fee_type: FeeType) -> Result<Amount, String> {
        let platform_fee = self.calculate_platform_fee(gross_amount, fee_type)?;
        let net_value = gross_amount.value() - platform_fee.value();
        
        if net_value < 0.0 {
            return Err("Platform fee exceeds gross amount".to_string());
        }
        
        Amount::new(net_value, gross_amount.currency())
    }
}

/// Type of fee being calculated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeType {
    Streaming,
    NFTMarketplace,
    OwnershipTransaction,
    RevenueSharing,
    RewardProcessing,
    PaymentProcessing,
}

/// Fee calculation with detailed breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeCalculation {
    pub gross_amount: Amount,
    pub platform_fee: Amount,
    pub processing_fee: Amount,
    pub net_amount: Amount,
    pub fee_percentage: f64,
    pub fee_type: FeeType,
    pub platform_phase: PlatformPhase,
    pub breakdown: FeeBreakdown,
}

/// Detailed fee breakdown for transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeBreakdown {
    pub platform_commission: Amount,
    pub payment_processing: Amount,
    pub fixed_fee: Amount,
    pub total_fees: Amount,
    pub user_receives: Amount,
}

impl DynamicFeeConfig {
    /// Create new fee configuration for current platform state
    pub fn new(user_count: u64) -> Self {
        let (phase, fees) = match user_count {
            0..=999 => (PlatformPhase::Launch, PlatformFees::launch_phase()),
            1000..=9999 => (PlatformPhase::Growth, PlatformFees::growth_phase()),
            10000..=99999 => (PlatformPhase::Scale, PlatformFees::scale_phase()),
            _ => (PlatformPhase::Mature, PlatformFees::mature_phase()),
        };
        
        Self {
            phase,
            user_count,
            fees,
            effective_date: Utc::now(),
            grandfathered_users: Vec::new(),
        }
    }
    
    /// Add grandfathered user who keeps current pricing
    pub fn add_grandfathered_user(&mut self, user_id: Uuid) {
        if !self.grandfathered_users.contains(&user_id) {
            self.grandfathered_users.push(user_id);
        }
    }
    
    /// Check if user has grandfathered pricing
    pub fn is_grandfathered(&self, user_id: &Uuid) -> bool {
        self.grandfathered_users.contains(user_id)
    }
    
    /// Get appropriate fees for a specific user
    pub fn get_user_fees(&self, user_id: &Uuid) -> &PlatformFees {
        if self.is_grandfathered(user_id) {
            // Return previous phase fees for grandfathered users
            match self.phase {
                PlatformPhase::Growth => &PlatformFees::launch_phase(),
                PlatformPhase::Scale => &PlatformFees::growth_phase(),
                PlatformPhase::Mature => &PlatformFees::scale_phase(),
                _ => &self.fees,
            }
        } else {
            &self.fees
        }
    }
    
    /// Update to next phase if user count threshold is met
    pub fn maybe_advance_phase(&mut self) -> bool {
        let new_config = Self::new(self.user_count);
        
        if std::mem::discriminant(&self.phase) != std::mem::discriminant(&new_config.phase) {
            // Granfather existing users before advancing
            // This would be populated from database
            self.phase = new_config.phase;
            self.fees = new_config.fees;
            self.effective_date = Utc::now();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_creation() {
        let amount = Amount::new(100.0, Currency::USD).unwrap();
        assert_eq!(amount.value(), 100.0);
        assert_eq!(amount.currency(), &Currency::USD);
    }
    
    #[test]
    fn test_amount_addition() {
        let amount1 = Amount::new(100.0, Currency::USD).unwrap();
        let amount2 = Amount::new(50.0, Currency::USD).unwrap();
        let result = amount1.add(&amount2).unwrap();
        assert_eq!(result.value(), 150.0);
    }
    
    #[test]
    fn test_different_currency_addition_fails() {
        let amount1 = Amount::new(100.0, Currency::USD).unwrap();
        let amount2 = Amount::new(50.0, Currency::ETH).unwrap();
        assert!(amount1.add(&amount2).is_err());
    }
    
    #[test]
    fn test_fee_calculation() {
        let fee = FeePercentage::new(2.5).unwrap();
        let amount = Amount::new(100.0, Currency::USD).unwrap();
        let fee_amount = fee.calculate_fee(&amount).unwrap();
        assert_eq!(fee_amount.value(), 2.5);
    }
    
    #[test]
    fn test_wallet_address_validation() {
        // Valid Ethereum address
        let eth_addr = WalletAddress::new("0x742D35Cc6551C8B04E3B6c8D8F4A7C4E8D2F9E1A".to_string()).unwrap();
        assert!(eth_addr.is_ethereum_format());
        
        // Invalid empty address
        assert!(WalletAddress::new("".to_string()).is_err());
    }
} 