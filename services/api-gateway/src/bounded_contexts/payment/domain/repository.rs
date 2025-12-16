use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shared::domain::errors::AppError;
use super::aggregates::*;
use super::entities::*;
use super::value_objects::*;
use super::events::*;

pub type PaymentRepositoryResult<T> = Result<T, AppError>;

/// Repository for Payment Aggregates
#[async_trait]
pub trait PaymentRepository: Send + Sync {
    /// Save a payment aggregate
    async fn save(&self, aggregate: &PaymentAggregate) -> PaymentRepositoryResult<()>;
    
    /// Find payment by ID
    async fn find_by_id(&self, id: &PaymentId) -> PaymentRepositoryResult<Option<PaymentAggregate>>;
    
    /// Find payment by transaction ID
    async fn find_by_transaction_id(&self, transaction_id: &TransactionId) -> PaymentRepositoryResult<Option<PaymentAggregate>>;
    
    /// Find payments by payer ID
    async fn find_by_payer_id(&self, payer_id: Uuid, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find payments by payee ID
    async fn find_by_payee_id(&self, payee_id: Uuid, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find payments by status
    async fn find_by_status(&self, status: &PaymentStatus, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find payments by purpose category
    async fn find_by_purpose_category(&self, category: &PaymentCategory, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find payments within date range
    async fn find_by_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find payments by amount range
    async fn find_by_amount_range(&self, min_amount: &Amount, max_amount: &Amount, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find payments that can be refunded
    async fn find_refundable_payments(&self, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find pending payments older than specified duration
    async fn find_stale_pending_payments(&self, older_than: DateTime<Utc>) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Update payment aggregate
    async fn update(&self, aggregate: &PaymentAggregate) -> PaymentRepositoryResult<()>;
    
    /// Delete payment (soft delete)
    async fn delete(&self, id: &PaymentId) -> PaymentRepositoryResult<()>;
    
    /// Check if payment exists
    async fn exists(&self, id: &PaymentId) -> PaymentRepositoryResult<bool>;
    
    /// Find payment by idempotency key
    async fn find_by_idempotency_key(&self, key: &str) -> PaymentRepositoryResult<Option<PaymentAggregate>>;
    
    /// Find payments by filter
    async fn find_by_filter(&self, filter: PaymentFilter, offset: u64, limit: u64) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Save a payment batch
    async fn save_batch(&self, batch: &PaymentBatch) -> PaymentRepositoryResult<()>;
    
    /// Get payment count by status
    async fn count_by_status(&self, status: &PaymentStatus) -> PaymentRepositoryResult<u64>;
    
    /// Get payment count by filter
    async fn count_by_filter(&self, filter: PaymentFilter) -> PaymentRepositoryResult<u64>;
    
    /// Get total payment volume for date range
    async fn get_total_volume(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<Currency, f64>>;
    
    /// Get payment events
    async fn get_payment_events(&self, payment_id: &PaymentId) -> PaymentRepositoryResult<Vec<PaymentEvent>>;
    
    /// Find payment batch by ID
    async fn find_batch_by_id(&self, batch_id: Uuid) -> PaymentRepositoryResult<Option<PaymentBatch>>;
}

/// Repository for Royalty Distribution Aggregates
#[async_trait]
pub trait RoyaltyDistributionRepository: Send + Sync {
    /// Save a royalty distribution aggregate
    async fn save(&self, aggregate: &RoyaltyDistributionAggregate) -> PaymentRepositoryResult<()>;
    
    /// Find distribution by ID
    async fn find_by_id(&self, id: Uuid) -> PaymentRepositoryResult<Option<RoyaltyDistributionAggregate>>;
    
    /// Find distributions by song ID
    async fn find_by_song_id(&self, song_id: Uuid, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RoyaltyDistributionAggregate>>;
    
    /// Find distributions by artist ID
    async fn find_by_artist_id(&self, artist_id: Uuid, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RoyaltyDistributionAggregate>>;
    
    /// Find distributions by status
    async fn find_by_status(&self, status: &DistributionStatus, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RoyaltyDistributionAggregate>>;
    
    /// Find distributions within date range
    async fn find_by_period(&self, start: DateTime<Utc>, end: DateTime<Utc>, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RoyaltyDistributionAggregate>>;
    
    /// Find pending distributions
    async fn find_pending_distributions(&self) -> PaymentRepositoryResult<Vec<RoyaltyDistributionAggregate>>;
    
    /// Update distribution aggregate
    async fn update(&self, aggregate: &RoyaltyDistributionAggregate) -> PaymentRepositoryResult<()>;
    
    /// Delete distribution
    async fn delete(&self, id: Uuid) -> PaymentRepositoryResult<()>;
    
    /// Get total distributions for artist in period
    async fn get_artist_total_distributions(&self, artist_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<Currency, f64>>;
}

/// Repository for Revenue Sharing Aggregates
#[async_trait]
pub trait RevenueSharingRepository: Send + Sync {
    /// Save a revenue sharing aggregate
    async fn save(&self, aggregate: &RevenueSharingAggregate) -> PaymentRepositoryResult<()>;
    
    /// Find distribution by ID
    async fn find_by_id(&self, id: Uuid) -> PaymentRepositoryResult<Option<RevenueSharingAggregate>>;
    
    /// Find distributions by contract ID
    async fn find_by_contract_id(&self, contract_id: Uuid, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RevenueSharingAggregate>>;
    
    /// Find distributions by song ID
    async fn find_by_song_id(&self, song_id: Uuid, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RevenueSharingAggregate>>;
    
    /// Find distributions by shareholder ID
    async fn find_by_shareholder_id(&self, shareholder_id: Uuid, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RevenueSharingAggregate>>;
    
    /// Find distributions by status
    async fn find_by_status(&self, status: &RevenueSharingStatus, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RevenueSharingAggregate>>;
    
    /// Find distributions within period
    async fn find_by_period(&self, start: DateTime<Utc>, end: DateTime<Utc>, pagination: &Pagination) -> PaymentRepositoryResult<Vec<RevenueSharingAggregate>>;
    
    /// Find pending distributions
    async fn find_pending_distributions(&self) -> PaymentRepositoryResult<Vec<RevenueSharingAggregate>>;
    
    /// Update distribution aggregate
    async fn update(&self, aggregate: &RevenueSharingAggregate) -> PaymentRepositoryResult<()>;
    
    /// Delete distribution
    async fn delete(&self, id: Uuid) -> PaymentRepositoryResult<()>;
    
    /// Get shareholder total distributions
    async fn get_shareholder_total_distributions(&self, shareholder_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<Currency, f64>>;
}

/// Query repository for payment analytics and reporting
#[async_trait]
pub trait PaymentAnalyticsRepository: Send + Sync {
    /// Get payment statistics for a period
    async fn get_payment_statistics(&self, start: DateTime<Utc>, end: DateTime<Utc>, currency: Option<Currency>, purpose_type: Option<String>) -> PaymentRepositoryResult<PaymentStatistics>;
    
    /// Get payment volume by currency
    async fn get_volume_by_currency(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<Currency, f64>>;
    
    /// Get payment volume by purpose
    async fn get_volume_by_purpose(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<PaymentCategory, f64>>;
    
    /// Get payment success rate
    async fn get_success_rate(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<f64>;
    
    /// Get average payment amount by purpose
    async fn get_average_payment_by_purpose(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<PaymentCategory, f64>>;
    
    /// Get top payers by volume
    async fn get_top_payers(&self, start: DateTime<Utc>, end: DateTime<Utc>, limit: u32) -> PaymentRepositoryResult<Vec<UserPaymentSummary>>;
    
    /// Get top payees by volume
    async fn get_top_payees(&self, start: DateTime<Utc>, end: DateTime<Utc>, limit: u32) -> PaymentRepositoryResult<Vec<UserPaymentSummary>>;
    
    /// Get payment trends (daily, weekly, monthly)
    async fn get_payment_trends(&self, start: DateTime<Utc>, end: DateTime<Utc>, granularity: &str) -> PaymentRepositoryResult<Vec<PaymentTrend>>;
    
    /// Get failed payment analysis
    async fn get_failed_payment_analysis(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<FailedPaymentAnalysis>;
    
    /// Get fraud detection metrics
    async fn get_fraud_metrics(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<FraudMetrics>;
    
    /// Get shareholder portfolio
    async fn get_shareholder_portfolio(&self, shareholder_id: Uuid, include_pending: bool) -> PaymentRepositoryResult<ShareholderPortfolio>;
    
    /// Get shareholder holdings
    async fn get_shareholder_holdings(&self, shareholder_id: Uuid, include_historical: bool) -> PaymentRepositoryResult<Vec<ShareholderHolding>>;

    // Missing methods required by handlers
    async fn get_payment_breakdown(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<String, f64>>;
    async fn get_payment_overview(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<PaymentStatistics>;
    async fn get_revenue_breakdown(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<HashMap<String, f64>>;
    async fn get_detailed_user_payment_summary(&self, user_id: Uuid, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> PaymentRepositoryResult<DetailedUserPaymentSummary>;
    async fn get_user_payment_method_usage(&self, user_id: Uuid, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> PaymentRepositoryResult<HashMap<String, u64>>;
    async fn get_user_recent_activity(&self, user_id: Uuid, limit: u32) -> PaymentRepositoryResult<Vec<PaymentActivity>>;
    async fn get_user_payment_summary(&self, user_id: Uuid, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> PaymentRepositoryResult<UserPaymentSummary>;
    async fn get_artist_revenue_summary(&self, artist_id: Uuid, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> PaymentRepositoryResult<ArtistRevenueSummary>;
    async fn get_artist_song_revenue_breakdown(&self, artist_id: Uuid, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> PaymentRepositoryResult<Vec<SongRevenueBreakdown>>;
    async fn get_artist_revenue_trends(&self, artist_id: Uuid, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> PaymentRepositoryResult<Vec<RevenueTrend>>;
}


/// Repository for Fraud Detection
#[async_trait]
pub trait FraudRepository: Send + Sync {
    /// Find alerts by filter
    async fn find_alerts_by_filter(&self, filter: FraudAlertFilter, offset: u64, limit: u64) -> PaymentRepositoryResult<Vec<super::entities::FraudAlert>>;
    
    /// Find stuck payments
    async fn find_stuck_payments(&self, older_than_minutes: u32, offset: u64, limit: u64) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Get failed transactions analysis
    async fn get_failed_transactions_analysis(
        &self, 
        start: Option<DateTime<Utc>>, 
        end: Option<DateTime<Utc>>,
        group_by_error: Option<bool>,
        group_by_method: Option<bool>
    ) -> PaymentRepositoryResult<HashMap<String, u64>>;
}

/// Payment Event Store Repository
#[async_trait]
pub trait PaymentEventRepository: Send + Sync {
    /// Save domain events
    async fn save_events(&self, aggregate_id: &PaymentId, events: &[Box<dyn crate::shared::domain::events::DomainEvent>], expected_version: u64) -> PaymentRepositoryResult<()>;
    
    /// Load events for aggregate
    async fn load_events(&self, aggregate_id: &PaymentId) -> PaymentRepositoryResult<Vec<Box<dyn crate::shared::domain::events::DomainEvent>>>;
    
    /// Load events since version
    async fn load_events_since(&self, aggregate_id: &PaymentId, version: u64) -> PaymentRepositoryResult<Vec<Box<dyn crate::shared::domain::events::DomainEvent>>>;
    
    /// Get all events of specific type
    async fn get_events_by_type(&self, event_type: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<Vec<Box<dyn crate::shared::domain::events::DomainEvent>>>;
    
    /// Get events for user
    async fn get_user_events(&self, user_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<Vec<Box<dyn crate::shared::domain::events::DomainEvent>>>;
}

/// Repository for Wallet Aggregates
#[async_trait]
pub trait WalletRepository: Send + Sync {
    async fn save(&self, wallet: &Wallet) -> PaymentRepositoryResult<()>;
    async fn find_by_user_id(&self, user_id: Uuid) -> PaymentRepositoryResult<Option<Wallet>>;
}

// Supporting data structures for analytics

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatistics {
    pub total_payments: u64,
    pub successful_payments: u64,
    pub failed_payments: u64,
    pub cancelled_payments: u64,
    pub pending_payments: u64,
    pub total_volume: HashMap<Currency, f64>,
    pub total_fees_collected: HashMap<Currency, f64>,
    pub success_rate: f64,
    pub average_payment_amount: HashMap<Currency, f64>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPaymentSummary {
    pub user_id: Uuid,
    pub total_volume: HashMap<Currency, f64>,
    pub payment_count: u64,
    pub average_payment_amount: HashMap<Currency, f64>,
    pub first_payment_date: DateTime<Utc>,
    pub last_payment_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTrend {
    pub period: DateTime<Utc>,
    pub payment_count: u64,
    pub total_volume: HashMap<Currency, f64>,
    pub success_rate: f64,
    pub average_amount: HashMap<Currency, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendGranularity {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedPaymentAnalysis {
    pub total_failed_payments: u64,
    pub failure_reasons: HashMap<String, u64>,
    pub failure_rate_by_payment_method: HashMap<String, f64>,
    pub failure_rate_by_currency: HashMap<Currency, f64>,
    pub most_common_error_codes: Vec<(String, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudMetrics {
    pub total_fraud_alerts: u64,
    pub high_risk_transactions: u64,
    pub blocked_transactions: u64,
    pub fraud_indicators: HashMap<String, u64>,
    pub fraud_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderPortfolio {
    pub total_investments: f64,
    pub current_value: f64,
    pub total_returns: f64,
    pub return_rate: f64,
    pub active_contracts: u32,
    pub pending_distributions: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderHolding {
    pub contract_id: Uuid,
    pub asset_name: String,
    pub symbol: String,
    pub shares_owned: f64,
    pub ownership_percentage: f64,
    pub initial_investment: f64,
    pub total_returns: f64,
    pub purchased_at: DateTime<Utc>,
    pub last_distribution_at: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedUserPaymentSummary {
    pub as_payer: UserRoleSummary,
    pub as_payee: UserRoleSummary,
    pub total_summary: UserRoleSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleSummary {
    pub total_volume: HashMap<Currency, f64>,
    pub count: u64,
    pub average_amount: HashMap<Currency, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentActivity {
    pub payment_id: Uuid,
    pub amount: f64,
    pub currency: Currency,
    pub activity_type: String, // "Sent", "Received"
    pub date: DateTime<Utc>,
    pub status: crate::bounded_contexts::payment::domain::value_objects::PaymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRevenueSummary {
    pub total_revenue: HashMap<Currency, f64>,
    pub platform_fees_paid: HashMap<Currency, f64>,
    pub net_revenue: HashMap<Currency, f64>,
    pub royalty_distributions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongRevenueBreakdown {
    pub song_id: Uuid,
    pub revenue: HashMap<Currency, f64>,
    pub percentage_of_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueTrend {
    pub period: DateTime<Utc>,
    pub revenue: HashMap<Currency, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: u64,
    pub limit: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 50,
        }
    }
}

impl Pagination {
    pub fn new(offset: u64, limit: u64) -> Self {
        Self {
            offset,
            limit: limit.min(1000), // Cap at 1000 for performance
        }
    }
    
    pub fn page(page: u64, page_size: u64) -> Self {
        Self {
            offset: page * page_size,
            limit: page_size.min(1000),
        }
    }
}

/// Payment specifications for complex queries
pub struct PaymentSpecification {
    pub payer_id: Option<Uuid>,
    pub payee_id: Option<Uuid>,
    pub status: Option<PaymentStatus>,
    pub purpose_category: Option<PaymentCategory>,
    pub currency: Option<Currency>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub payment_method_type: Option<String>,
    pub blockchain: Option<Blockchain>,
}

impl Default for PaymentSpecification {
    fn default() -> Self {
        Self {
            payer_id: None,
            payee_id: None,
            status: None,
            purpose_category: None,
            currency: None,
            min_amount: None,
            max_amount: None,
            start_date: None,
            end_date: None,
            payment_method_type: None,
            blockchain: None,
        }
    }
}

impl PaymentSpecification {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_payer_id(mut self, payer_id: Uuid) -> Self {
        self.payer_id = Some(payer_id);
        self
    }
    
    pub fn with_payee_id(mut self, payee_id: Uuid) -> Self {
        self.payee_id = Some(payee_id);
        self
    }
    
    pub fn with_status(mut self, status: PaymentStatus) -> Self {
        self.status = Some(status);
        self
    }
    
    pub fn with_purpose_category(mut self, category: PaymentCategory) -> Self {
        self.purpose_category = Some(category);
        self
    }
    
    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }
    
    pub fn with_amount_range(mut self, min_amount: f64, max_amount: f64) -> Self {
        self.min_amount = Some(min_amount);
        self.max_amount = Some(max_amount);
        self
    }
    
    pub fn with_date_range(mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self.end_date = Some(end_date);
        self
    }
    
    pub fn with_blockchain(mut self, blockchain: Blockchain) -> Self {
        self.blockchain = Some(blockchain);
        self
    }
}

/// Extended payment repository with complex queries
#[async_trait]
pub trait PaymentQueryRepository: Send + Sync {
    /// Find payments by specification
    async fn find_by_specification(&self, spec: &PaymentSpecification, pagination: &Pagination) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Count payments by specification
    async fn count_by_specification(&self, spec: &PaymentSpecification) -> PaymentRepositoryResult<u64>;
    
    /// Find similar payments (for fraud detection)
    async fn find_similar_payments(&self, payment: &PaymentAggregate, similarity_threshold: f64) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
    
    /// Find payments with related events
    async fn find_payments_with_events(&self, event_types: &[String], start: DateTime<Utc>, end: DateTime<Utc>) -> PaymentRepositoryResult<Vec<PaymentAggregate>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_creation() {
        let pagination = Pagination::new(0, 50);
        assert_eq!(pagination.offset, 0);
        assert_eq!(pagination.limit, 50);
        
        let page_pagination = Pagination::page(2, 20);
        assert_eq!(page_pagination.offset, 40);
        assert_eq!(page_pagination.limit, 20);
    }
    
    #[test]
    fn test_payment_specification_builder() {
        let spec = PaymentSpecification::new()
            .with_payer_id(Uuid::new_v4())
            .with_currency(Currency::USD)
            .with_amount_range(100.0, 1000.0);
        
        assert!(spec.payer_id.is_some());
        assert_eq!(spec.currency, Some(Currency::USD));
        assert_eq!(spec.min_amount, Some(100.0));
        assert_eq!(spec.max_amount, Some(1000.0));
    }
} 