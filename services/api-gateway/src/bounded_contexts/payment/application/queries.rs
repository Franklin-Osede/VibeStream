use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::bounded_contexts::payment::domain::value_objects::*;

/// Query to get payment by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentQuery {
    pub payment_id: Uuid,
    pub include_events: bool,
}

/// Query to get payment by transaction ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentByTransactionQuery {
    pub transaction_id: Uuid,
}

/// Query to get user payment history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPaymentHistoryQuery {
    pub user_id: Uuid,
    pub role: Option<String>, // "payer", "payee", or "both"
    pub status: Option<String>,
    pub purpose_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub pagination: PaginationQuery,
}

/// Query to get payment statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentStatisticsQuery {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub group_by: Option<String>, // "day", "week", "month"
    pub currency: Option<Currency>,
    pub purpose_type: Option<String>,
}

/// Query to get royalty distributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoyaltyDistributionsQuery {
    pub artist_id: Option<Uuid>,
    pub song_id: Option<Uuid>,
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub pagination: PaginationQuery,
}

/// Query to get revenue sharing distributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRevenueSharingQuery {
    pub contract_id: Option<Uuid>,
    pub shareholder_id: Option<Uuid>,
    pub song_id: Option<Uuid>,
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub pagination: PaginationQuery,
}

/// Query to get payment batches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentBatchesQuery {
    pub batch_type: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub pagination: PaginationQuery,
}

/// Query to get fraud alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFraudAlertsQuery {
    pub user_id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub risk_score_min: Option<f64>,
    pub risk_score_max: Option<f64>,
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub pagination: PaginationQuery,
}

/// Query for payment analytics dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentAnalyticsQuery {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub include_trends: bool,
    pub include_top_users: bool,
    pub include_fraud_metrics: bool,
}

/// Query for user payment summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPaymentSummaryQuery {
    pub user_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Query for artist revenue summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtistRevenueSummaryQuery {
    pub artist_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub include_song_breakdown: bool,
}

/// Query for shareholder portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShareholderPortfolioQuery {
    pub shareholder_id: Uuid,
    pub include_pending: bool,
    pub include_historical: bool,
}

/// Query for pending transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPendingTransactionsQuery {
    pub older_than_minutes: Option<u32>,
    pub payment_method_type: Option<String>,
    pub min_amount: Option<f64>,
    pub pagination: PaginationQuery,
}

/// Query for failed transactions analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFailedTransactionsAnalysisQuery {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub group_by_error_code: bool,
    pub group_by_payment_method: bool,
}

/// Request for searching payments (API level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPaymentsRequest {
    pub user_id: Option<Uuid>,
    pub payment_type: Option<String>,
    pub status: Option<PaymentStatus>,
    pub currency: Option<Currency>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Query for searching payments (Domain level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPaymentsQuery {
    pub user_id: Option<Uuid>,
    pub payment_type: Option<String>,
    pub status: Option<PaymentStatus>,
    pub currency: Option<Currency>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Result of payment search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPaymentsResult {
    pub payments: Vec<PaymentDTO>,
    pub total_count: u64,
    pub has_more: bool,
}

use crate::bounded_contexts::payment::application::dto::PaymentDTO; // Ensure PaymentDTO is available

// Supporting types for queries

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>, // "asc" or "desc"
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 20,
            sort_by: Some("created_at".to_string()),
            sort_order: Some("desc".to_string()),
        }
    }
}

// Query Results

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentQueryResult {
    pub payment_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub payer_id: Uuid,
    pub payee_id: Uuid,
    pub amount: AmountDto,
    pub net_amount: AmountDto,
    pub platform_fee: Option<AmountDto>,
    pub payment_method: PaymentMethodDto,
    pub purpose: PaymentPurposeDto,
    pub status: String,
    pub blockchain_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    pub metadata: PaymentMetadataDto,
    pub events: Option<Vec<PaymentEventDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentHistoryResult {
    pub payments: Vec<PaymentQueryResult>,
    pub total_count: u64,
    pub total_pages: u32,
    pub current_page: u32,
    pub summary: PaymentHistorySummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentHistorySummary {
    pub total_volume: HashMap<Currency, f64>,
    pub total_payments: u64,
    pub successful_payments: u64,
    pub failed_payments: u64,
    pub pending_payments: u64,
    pub total_fees_paid: HashMap<Currency, f64>,
    pub average_payment_amount: HashMap<Currency, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatisticsResult {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_payments: u64,
    pub successful_payments: u64,
    pub failed_payments: u64,
    pub total_volume: HashMap<Currency, f64>,
    pub total_fees: HashMap<Currency, f64>,
    pub success_rate: f64,
    pub average_payment_amount: HashMap<Currency, f64>,
    pub payment_trends: Option<Vec<PaymentTrendData>>,
    pub payment_breakdown: PaymentBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTrendData {
    pub period: DateTime<Utc>,
    pub payment_count: u64,
    pub total_volume: HashMap<Currency, f64>,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBreakdown {
    pub by_purpose: HashMap<String, PaymentBreakdownItem>,
    pub by_payment_method: HashMap<String, PaymentBreakdownItem>,
    pub by_currency: HashMap<Currency, PaymentBreakdownItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBreakdownItem {
    pub count: u64,
    pub volume: f64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyDistributionQueryResult {
    pub distribution_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_revenue: AmountDto,
    pub artist_amount: AmountDto,
    pub platform_fee: AmountDto,
    pub artist_share_percentage: f64,
    pub platform_fee_percentage: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub distributed_at: Option<DateTime<Utc>>,
    pub payment_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSharingQueryResult {
    pub distribution_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub total_revenue: AmountDto,
    pub platform_fee_percentage: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub shareholders: Vec<ShareholderDistributionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderDistributionDto {
    pub shareholder_id: Uuid,
    pub ownership_percentage: f64,
    pub distribution_amount: AmountDto,
    pub payment_status: String,
    pub payment_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBatchQueryResult {
    pub batch_id: Uuid,
    pub batch_type: String,
    pub total_amount: AmountDto,
    pub payment_count: u32,
    pub successful_payments: u32,
    pub failed_payments: u32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub processing_duration_ms: Option<u64>,
    pub payments: Option<Vec<PaymentBatchItemDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBatchItemDto {
    pub payment_id: Uuid,
    pub amount: AmountDto,
    pub status: String,
    pub error_message: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAlertQueryResult {
    pub alert_id: Uuid,
    pub payment_id: Uuid,
    pub user_id: Uuid,
    pub risk_score: f64,
    pub fraud_indicators: Vec<String>,
    pub action_taken: String,
    pub review_status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub payment_details: Option<PaymentQueryResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAnalyticsResult {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub overview: PaymentOverview,
    pub trends: Option<Vec<PaymentTrendData>>,
    pub top_payers: Option<Vec<TopUserData>>,
    pub top_payees: Option<Vec<TopUserData>>,
    pub fraud_metrics: Option<FraudMetricsData>,
    pub revenue_breakdown: RevenueBreakdownData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentOverview {
    pub total_payments: u64,
    pub total_volume: HashMap<Currency, f64>,
    pub total_fees: HashMap<Currency, f64>,
    pub success_rate: f64,
    pub average_payment_amount: HashMap<Currency, f64>,
    pub growth_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopUserData {
    pub user_id: Uuid,
    pub total_volume: HashMap<Currency, f64>,
    pub payment_count: u64,
    pub average_amount: HashMap<Currency, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudMetricsData {
    pub total_alerts: u64,
    pub high_risk_transactions: u64,
    pub blocked_transactions: u64,
    pub fraud_rate: f64,
    pub top_indicators: Vec<(String, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueBreakdownData {
    pub platform_fees: HashMap<Currency, f64>,
    pub processing_fees: HashMap<Currency, f64>,
    pub artist_royalties: HashMap<Currency, f64>,
    pub shareholder_distributions: HashMap<Currency, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPaymentSummaryResult {
    pub user_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub as_payer: UserRoleSummary,
    pub as_payee: UserRoleSummary,
    pub total_summary: UserRoleSummary,
    pub favorite_payment_methods: Vec<PaymentMethodUsage>,
    pub recent_activity: Vec<RecentPaymentActivity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleSummary {
    pub total_payments: u64,
    pub total_volume: HashMap<Currency, f64>,
    pub successful_payments: u64,
    pub failed_payments: u64,
    pub average_amount: HashMap<Currency, f64>,
    pub first_payment: Option<DateTime<Utc>>,
    pub last_payment: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodUsage {
    pub method_type: String,
    pub usage_count: u64,
    pub total_volume: HashMap<Currency, f64>,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentPaymentActivity {
    pub payment_id: Uuid,
    pub amount: AmountDto,
    pub purpose_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRevenueSummaryResult {
    pub artist_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_revenue: HashMap<Currency, f64>,
    pub platform_fees_paid: HashMap<Currency, f64>,
    pub net_revenue: HashMap<Currency, f64>,
    pub royalty_distributions: u64,
    pub song_breakdown: Option<Vec<SongRevenueBreakdown>>,
    pub revenue_trends: Vec<RevenueTimeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongRevenueBreakdown {
    pub song_id: Uuid,
    pub song_title: String,
    pub total_revenue: HashMap<Currency, f64>,
    pub listen_count: u64,
    pub revenue_per_listen: HashMap<Currency, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueTimeData {
    pub period: DateTime<Utc>,
    pub revenue: HashMap<Currency, f64>,
    pub distributions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderPortfolioResult {
    pub shareholder_id: Uuid,
    pub total_investments: HashMap<Currency, f64>,
    pub current_value: HashMap<Currency, f64>,
    pub total_returns: HashMap<Currency, f64>,
    pub return_rate: f64,
    pub active_contracts: u32,
    pub pending_distributions: HashMap<Currency, f64>,
    pub holdings: Vec<ShareholderHolding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderHolding {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub song_title: String,
    pub ownership_percentage: f64,
    pub initial_investment: AmountDto,
    pub current_value: AmountDto,
    pub total_distributions: AmountDto,
    pub last_distribution: Option<DateTime<Utc>>,
}

// Supporting DTOs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmountDto {
    pub value: f64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDto {
    pub method_type: String,
    pub display_name: String,
    pub last_four: Option<String>,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPurposeDto {
    pub purpose_type: String,
    pub description: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMetadataDto {
    pub platform_version: String,
    pub user_ip: Option<String>,
    pub reference_id: Option<String>,
    pub additional_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentEventDto {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
}

// Query validation and helpers

impl GetUserPaymentHistoryQuery {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if let (Some(start), Some(end)) = (&self.start_date, &self.end_date) {
            if end <= start {
                errors.push("End date must be after start date".to_string());
            }
        }
        
        if self.pagination.page_size > 100 {
            errors.push("Page size cannot exceed 100".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl PaginationQuery {
    pub fn to_offset_limit(&self) -> (u64, u64) {
        let offset = (self.page as u64) * (self.page_size as u64);
        (offset, self.page_size as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset_calculation() {
        let pagination = PaginationQuery {
            page: 2,
            page_size: 20,
            sort_by: None,
            sort_order: None,
        };
        
        let (offset, limit) = pagination.to_offset_limit();
        assert_eq!(offset, 40);
        assert_eq!(limit, 20);
    }
    
    #[test]
    fn test_user_payment_history_query_validation() {
        let query = GetUserPaymentHistoryQuery {
            user_id: Uuid::new_v4(),
            role: None,
            status: None,
            purpose_type: None,
            start_date: Some(Utc::now()),
            end_date: Some(Utc::now() + chrono::Duration::days(30)),
            pagination: PaginationQuery::default(),
        };
        
        assert!(query.validate().is_ok());
    }
} 