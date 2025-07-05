use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::{
    domain::{
        value_objects::*,
        repository::*,
    },
    application::queries::*,
};

/// Query Handler for Payment Read Operations
#[async_trait]
pub trait PaymentQueryHandler: Send + Sync {
    async fn handle_get_payment(&self, query: GetPaymentQuery) -> Result<PaymentQueryResult, AppError>;
    async fn handle_get_payment_by_transaction(&self, query: GetPaymentByTransactionQuery) -> Result<PaymentQueryResult, AppError>;
    async fn handle_get_user_payment_history(&self, query: GetUserPaymentHistoryQuery) -> Result<PaymentHistoryResult, AppError>;
    async fn handle_get_payment_statistics(&self, query: GetPaymentStatisticsQuery) -> Result<PaymentStatisticsResult, AppError>;
}

/// Query Handler for Analytics Operations
#[async_trait]
pub trait PaymentAnalyticsQueryHandler: Send + Sync {
    async fn handle_get_payment_analytics(&self, query: GetPaymentAnalyticsQuery) -> Result<PaymentAnalyticsResult, AppError>;
    async fn handle_get_user_payment_summary(&self, query: GetUserPaymentSummaryQuery) -> Result<UserPaymentSummaryResult, AppError>;
    async fn handle_get_artist_revenue_summary(&self, query: GetArtistRevenueSummaryQuery) -> Result<ArtistRevenueSummaryResult, AppError>;
    async fn handle_get_shareholder_portfolio(&self, query: GetShareholderPortfolioQuery) -> Result<ShareholderPortfolioResult, AppError>;
}

/// Query Handler for Royalty Operations
#[async_trait]
pub trait RoyaltyQueryHandler: Send + Sync {
    async fn handle_get_royalty_distributions(&self, query: GetRoyaltyDistributionsQuery) -> Result<Vec<RoyaltyDistributionQueryResult>, AppError>;
}

/// Query Handler for Fraud Operations
#[async_trait]
pub trait FraudQueryHandler: Send + Sync {
    async fn handle_get_fraud_alerts(&self, query: GetFraudAlertsQuery) -> Result<Vec<FraudAlertQueryResult>, AppError>;
    async fn handle_get_pending_transactions(&self, query: GetPendingTransactionsQuery) -> Result<Vec<PaymentQueryResult>, AppError>;
    async fn handle_get_failed_transactions_analysis(&self, query: GetFailedTransactionsAnalysisQuery) -> Result<HashMap<String, u64>, AppError>;
}

/// Implementation of Payment Query Handler
pub struct PaymentQueryHandlerImpl {
    payment_repository: Arc<dyn PaymentRepository>,
    payment_analytics_repository: Arc<dyn PaymentAnalyticsRepository>,
}

impl PaymentQueryHandlerImpl {
    pub fn new(
        payment_repository: Arc<dyn PaymentRepository>,
        payment_analytics_repository: Arc<dyn PaymentAnalyticsRepository>,
    ) -> Self {
        Self {
            payment_repository,
            payment_analytics_repository,
        }
    }
}

#[async_trait]
impl PaymentQueryHandler for PaymentQueryHandlerImpl {
    async fn handle_get_payment(&self, query: GetPaymentQuery) -> Result<PaymentQueryResult, AppError> {
        // 1. Find payment by ID
        let payment_id = PaymentId::from_uuid(query.payment_id);
        let payment_aggregate = self.payment_repository
            .find_by_id(&payment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Payment not found".to_string()))?;
        
        // 2. Get events if requested
        let events = if query.include_events {
            Some(self.payment_repository.get_payment_events(&payment_id).await?)
        } else {
            None
        };
        
        // 3. Convert to query result
        Ok(self.convert_to_payment_query_result(payment_aggregate, events))
    }
    
    async fn handle_get_payment_by_transaction(&self, query: GetPaymentByTransactionQuery) -> Result<PaymentQueryResult, AppError> {
        // 1. Find payment by transaction ID
        let transaction_id = TransactionId::from_uuid(query.transaction_id);
        let payment_aggregate = self.payment_repository
            .find_by_transaction_id(&transaction_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Payment not found".to_string()))?;
        
        // 2. Convert to query result
        Ok(self.convert_to_payment_query_result(payment_aggregate, None))
    }
    
    async fn handle_get_user_payment_history(&self, query: GetUserPaymentHistoryQuery) -> Result<PaymentHistoryResult, AppError> {
        // 1. Validate query
        query.validate()?;
        
        // 2. Build filter criteria
        let filter = PaymentFilter {
            user_id: Some(query.user_id),
            role: query.role,
            status: query.status,
            purpose_type: query.purpose_type,
            start_date: query.start_date,
            end_date: query.end_date,
        };
        
        // 3. Get payments with pagination
        let (offset, limit) = query.pagination.to_offset_limit();
        let payments = self.payment_repository
            .find_by_filter(filter.clone(), offset, limit)
            .await?;
        
        // 4. Get total count
        let total_count = self.payment_repository
            .count_by_filter(filter.clone())
            .await?;
        
        // 5. Calculate pagination info
        let total_pages = ((total_count as f64) / (query.pagination.page_size as f64)).ceil() as u32;
        
        // 6. Get summary statistics
        let summary = self.payment_analytics_repository
            .get_user_payment_summary(query.user_id, query.start_date, query.end_date)
            .await?;
        
        // 7. Convert payments to query results
        let payment_results = payments.into_iter()
            .map(|p| self.convert_to_payment_query_result(p, None))
            .collect();
        
        Ok(PaymentHistoryResult {
            payments: payment_results,
            total_count,
            total_pages,
            current_page: query.pagination.page,
            summary,
        })
    }
    
    async fn handle_get_payment_statistics(&self, query: GetPaymentStatisticsQuery) -> Result<PaymentStatisticsResult, AppError> {
        // 1. Get basic statistics
        let stats = self.payment_analytics_repository
            .get_payment_statistics(
                query.start_date,
                query.end_date,
                query.currency,
                query.purpose_type,
            )
            .await?;
        
        // 2. Get trends if requested
        let trends = if let Some(group_by) = query.group_by {
            Some(self.payment_analytics_repository
                .get_payment_trends(
                    query.start_date,
                    query.end_date,
                    &group_by,
                )
                .await?)
        } else {
            None
        };
        
        // 3. Get payment breakdown
        let breakdown = self.payment_analytics_repository
            .get_payment_breakdown(query.start_date, query.end_date)
            .await?;
        
        Ok(PaymentStatisticsResult {
            period_start: query.start_date,
            period_end: query.end_date,
            total_payments: stats.total_payments,
            successful_payments: stats.successful_payments,
            failed_payments: stats.failed_payments,
            total_volume: stats.total_volume,
            total_fees: stats.total_fees,
            success_rate: stats.success_rate,
            average_payment_amount: stats.average_payment_amount,
            payment_trends: trends,
            payment_breakdown: breakdown,
        })
    }
}

#[async_trait]
impl PaymentAnalyticsQueryHandler for PaymentQueryHandlerImpl {
    async fn handle_get_payment_analytics(&self, query: GetPaymentAnalyticsQuery) -> Result<PaymentAnalyticsResult, AppError> {
        // 1. Get overview data
        let overview = self.payment_analytics_repository
            .get_payment_overview(query.start_date, query.end_date)
            .await?;
        
        // 2. Get trends if requested
        let trends = if query.include_trends {
            Some(self.payment_analytics_repository
                .get_payment_trends(query.start_date, query.end_date, "day")
                .await?)
        } else {
            None
        };
        
        // 3. Get top users if requested
        let (top_payers, top_payees) = if query.include_top_users {
            let payers = self.payment_analytics_repository
                .get_top_payers(query.start_date, query.end_date, 10)
                .await?;
            let payees = self.payment_analytics_repository
                .get_top_payees(query.start_date, query.end_date, 10)
                .await?;
            (Some(payers), Some(payees))
        } else {
            (None, None)
        };
        
        // 4. Get fraud metrics if requested
        let fraud_metrics = if query.include_fraud_metrics {
            Some(self.payment_analytics_repository
                .get_fraud_metrics(query.start_date, query.end_date)
                .await?)
        } else {
            None
        };
        
        // 5. Get revenue breakdown
        let revenue_breakdown = self.payment_analytics_repository
            .get_revenue_breakdown(query.start_date, query.end_date)
            .await?;
        
        Ok(PaymentAnalyticsResult {
            period_start: query.start_date,
            period_end: query.end_date,
            overview,
            trends,
            top_payers,
            top_payees,
            fraud_metrics,
            revenue_breakdown,
        })
    }
    
    async fn handle_get_user_payment_summary(&self, query: GetUserPaymentSummaryQuery) -> Result<UserPaymentSummaryResult, AppError> {
        // 1. Get user payment summary
        let summary = self.payment_analytics_repository
            .get_detailed_user_payment_summary(
                query.user_id,
                query.start_date,
                query.end_date,
            )
            .await?;
        
        // 2. Get payment method usage
        let payment_methods = self.payment_analytics_repository
            .get_user_payment_method_usage(query.user_id, query.start_date, query.end_date)
            .await?;
        
        // 3. Get recent activity
        let recent_activity = self.payment_analytics_repository
            .get_user_recent_activity(query.user_id, 10)
            .await?;
        
        Ok(UserPaymentSummaryResult {
            user_id: query.user_id,
            period_start: query.start_date.unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30)),
            period_end: query.end_date.unwrap_or_else(|| chrono::Utc::now()),
            as_payer: summary.as_payer,
            as_payee: summary.as_payee,
            total_summary: summary.total_summary,
            favorite_payment_methods: payment_methods,
            recent_activity,
        })
    }
    
    async fn handle_get_artist_revenue_summary(&self, query: GetArtistRevenueSummaryQuery) -> Result<ArtistRevenueSummaryResult, AppError> {
        // 1. Get artist revenue summary
        let summary = self.payment_analytics_repository
            .get_artist_revenue_summary(
                query.artist_id,
                query.start_date,
                query.end_date,
            )
            .await?;
        
        // 2. Get song breakdown if requested
        let song_breakdown = if query.include_song_breakdown {
            Some(self.payment_analytics_repository
                .get_artist_song_revenue_breakdown(
                    query.artist_id,
                    query.start_date,
                    query.end_date,
                )
                .await?)
        } else {
            None
        };
        
        // 3. Get revenue trends
        let revenue_trends = self.payment_analytics_repository
            .get_artist_revenue_trends(
                query.artist_id,
                query.start_date,
                query.end_date,
            )
            .await?;
        
        Ok(ArtistRevenueSummaryResult {
            artist_id: query.artist_id,
            period_start: query.start_date.unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30)),
            period_end: query.end_date.unwrap_or_else(|| chrono::Utc::now()),
            total_revenue: summary.total_revenue,
            platform_fees_paid: summary.platform_fees_paid,
            net_revenue: summary.net_revenue,
            royalty_distributions: summary.royalty_distributions,
            song_breakdown,
            revenue_trends,
        })
    }
    
    async fn handle_get_shareholder_portfolio(&self, query: GetShareholderPortfolioQuery) -> Result<ShareholderPortfolioResult, AppError> {
        // 1. Get shareholder portfolio
        let portfolio = self.payment_analytics_repository
            .get_shareholder_portfolio(query.shareholder_id, query.include_pending)
            .await?;
        
        // 2. Get detailed holdings
        let holdings = self.payment_analytics_repository
            .get_shareholder_holdings(query.shareholder_id, query.include_historical)
            .await?;
        
        Ok(ShareholderPortfolioResult {
            shareholder_id: query.shareholder_id,
            total_investments: portfolio.total_investments,
            current_value: portfolio.current_value,
            total_returns: portfolio.total_returns,
            return_rate: portfolio.return_rate,
            active_contracts: portfolio.active_contracts,
            pending_distributions: portfolio.pending_distributions,
            holdings,
        })
    }
}

/// Implementation of Royalty Query Handler
pub struct RoyaltyQueryHandlerImpl {
    royalty_repository: Arc<dyn RoyaltyDistributionRepository>,
}

impl RoyaltyQueryHandlerImpl {
    pub fn new(royalty_repository: Arc<dyn RoyaltyDistributionRepository>) -> Self {
        Self { royalty_repository }
    }
}

#[async_trait]
impl RoyaltyQueryHandler for RoyaltyQueryHandlerImpl {
    async fn handle_get_royalty_distributions(&self, query: GetRoyaltyDistributionsQuery) -> Result<Vec<RoyaltyDistributionQueryResult>, AppError> {
        // 1. Build filter
        let filter = RoyaltyDistributionFilter {
            artist_id: query.artist_id,
            song_id: query.song_id,
            status: query.status,
            start_date: query.start_date,
            end_date: query.end_date,
        };
        
        // 2. Get distributions
        let (offset, limit) = query.pagination.to_offset_limit();
        let distributions = self.royalty_repository
            .find_by_filter(filter, offset, limit)
            .await?;
        
        // 3. Convert to query results
        let results = distributions.into_iter()
            .map(|d| self.convert_to_royalty_query_result(d))
            .collect();
        
        Ok(results)
    }
}

impl RoyaltyQueryHandlerImpl {
    fn convert_to_royalty_query_result(&self, distribution: RoyaltyDistributionAggregate) -> RoyaltyDistributionQueryResult {
        RoyaltyDistributionQueryResult {
            distribution_id: distribution.distribution().id(),
            song_id: distribution.distribution().song_id(),
            artist_id: distribution.distribution().artist_id(),
            total_revenue: AmountDto {
                value: distribution.distribution().total_revenue().value(),
                currency: distribution.distribution().total_revenue().currency(),
            },
            artist_amount: AmountDto {
                value: distribution.distribution().artist_amount().value(),
                currency: distribution.distribution().artist_amount().currency(),
            },
            platform_fee: AmountDto {
                value: distribution.distribution().platform_fee().value(),
                currency: distribution.distribution().platform_fee().currency(),
            },
            artist_share_percentage: distribution.distribution().artist_share_percentage(),
            platform_fee_percentage: distribution.distribution().platform_fee_percentage(),
            period_start: distribution.distribution().period_start(),
            period_end: distribution.distribution().period_end(),
            status: format!("{:?}", distribution.distribution().status()),
            created_at: distribution.distribution().created_at,
            distributed_at: distribution.distribution().distributed_at,
            payment_ids: distribution.payments().iter().map(|p| *p.payment().id().value()).collect(),
        }
    }
}

/// Implementation of Fraud Query Handler
pub struct FraudQueryHandlerImpl {
    fraud_repository: Arc<dyn FraudRepository>,
}

impl FraudQueryHandlerImpl {
    pub fn new(fraud_repository: Arc<dyn FraudRepository>) -> Self {
        Self { fraud_repository }
    }
}

#[async_trait]
impl FraudQueryHandler for FraudQueryHandlerImpl {
    async fn handle_get_fraud_alerts(&self, query: GetFraudAlertsQuery) -> Result<Vec<FraudAlertQueryResult>, AppError> {
        // 1. Build filter
        let filter = FraudAlertFilter {
            user_id: query.user_id,
            payment_id: query.payment_id,
            risk_score_min: query.risk_score_min,
            risk_score_max: query.risk_score_max,
            status: query.status,
            start_date: query.start_date,
            end_date: query.end_date,
        };
        
        // 2. Get alerts
        let (offset, limit) = query.pagination.to_offset_limit();
        let alerts = self.fraud_repository
            .find_alerts_by_filter(filter, offset, limit)
            .await?;
        
        // 3. Convert to query results
        let results = alerts.into_iter()
            .map(|alert| self.convert_to_fraud_alert_query_result(alert))
            .collect();
        
        Ok(results)
    }
    
    async fn handle_get_pending_transactions(&self, query: GetPendingTransactionsQuery) -> Result<Vec<PaymentQueryResult>, AppError> {
        // Implementation for getting pending transactions
        // This would typically involve finding payments that are in "Processing" state
        // for longer than the specified time
        
        let filter = PaymentFilter {
            user_id: None,
            role: None,
            status: Some("Processing".to_string()),
            purpose_type: None,
            start_date: None,
            end_date: None,
        };
        
        // Get payments that are stuck in processing
        let (offset, limit) = query.pagination.to_offset_limit();
        let payments = self.fraud_repository
            .find_stuck_payments(query.older_than_minutes, offset, limit)
            .await?;
        
        // Convert to query results
        let results = payments.into_iter()
            .map(|p| PaymentQueryResult {
                payment_id: *p.payment().id().value(),
                transaction_id: p.payment().transaction_id().map(|t| *t.value()),
                payer_id: p.payment().payer_id(),
                payee_id: p.payment().payee_id(),
                amount: AmountDto {
                    value: p.payment().amount().value(),
                    currency: p.payment().amount().currency(),
                },
                net_amount: AmountDto {
                    value: p.payment().net_amount().value(),
                    currency: p.payment().net_amount().currency(),
                },
                platform_fee: p.payment().platform_fee().map(|f| AmountDto {
                    value: f.value(),
                    currency: f.currency(),
                }),
                payment_method: PaymentMethodDto {
                    method_type: format!("{:?}", p.payment().payment_method()),
                    display_name: "".to_string(),
                    last_four: None,
                    details: serde_json::Value::Null,
                },
                purpose: PaymentPurposeDto {
                    purpose_type: format!("{:?}", p.payment().purpose()),
                    description: "".to_string(),
                    details: serde_json::Value::Null,
                },
                status: format!("{:?}", p.payment().status()),
                blockchain_hash: p.payment().blockchain_hash().map(|h| h.value().to_string()),
                created_at: p.payment().created_at(),
                updated_at: p.payment().updated_at(),
                completed_at: p.payment().completed_at(),
                failure_reason: p.payment().failure_reason().map(|r| r.to_string()),
                metadata: PaymentMetadataDto {
                    platform_version: "".to_string(),
                    user_ip: None,
                    reference_id: None,
                    additional_data: serde_json::Value::Null,
                },
                events: None,
            })
            .collect();
        
        Ok(results)
    }
    
    async fn handle_get_failed_transactions_analysis(&self, query: GetFailedTransactionsAnalysisQuery) -> Result<HashMap<String, u64>, AppError> {
        // Get failed transaction analysis
        let analysis = self.fraud_repository
            .get_failed_transactions_analysis(
                query.start_date,
                query.end_date,
                query.group_by_error_code,
                query.group_by_payment_method,
            )
            .await?;
        
        Ok(analysis)
    }
}

impl FraudQueryHandlerImpl {
    fn convert_to_fraud_alert_query_result(&self, alert: FraudAlert) -> FraudAlertQueryResult {
        FraudAlertQueryResult {
            alert_id: alert.id(),
            payment_id: alert.payment_id(),
            user_id: alert.user_id(),
            risk_score: alert.risk_score(),
            fraud_indicators: alert.fraud_indicators(),
            action_taken: alert.action_taken(),
            review_status: format!("{:?}", alert.review_status()),
            reviewed_by: alert.reviewed_by(),
            reviewed_at: alert.reviewed_at(),
            review_notes: alert.review_notes(),
            created_at: alert.created_at(),
            payment_details: None, // Would need to fetch separately if needed
        }
    }
}

// Helper implementation for PaymentQueryHandlerImpl
impl PaymentQueryHandlerImpl {
    fn convert_to_payment_query_result(
        &self,
        payment_aggregate: PaymentAggregate,
        events: Option<Vec<PaymentEvent>>,
    ) -> PaymentQueryResult {
        let payment = payment_aggregate.payment();
        
        PaymentQueryResult {
            payment_id: *payment.id().value(),
            transaction_id: payment.transaction_id().map(|t| *t.value()),
            payer_id: payment.payer_id(),
            payee_id: payment.payee_id(),
            amount: AmountDto {
                value: payment.amount().value(),
                currency: payment.amount().currency(),
            },
            net_amount: AmountDto {
                value: payment.net_amount().value(),
                currency: payment.net_amount().currency(),
            },
            platform_fee: payment.platform_fee().map(|f| AmountDto {
                value: f.value(),
                currency: f.currency(),
            }),
            payment_method: PaymentMethodDto {
                method_type: format!("{:?}", payment.payment_method()),
                display_name: "".to_string(),
                last_four: None,
                details: serde_json::Value::Null,
            },
            purpose: PaymentPurposeDto {
                purpose_type: format!("{:?}", payment.purpose()),
                description: "".to_string(),
                details: serde_json::Value::Null,
            },
            status: format!("{:?}", payment.status()),
            blockchain_hash: payment.blockchain_hash().map(|h| h.value().to_string()),
            created_at: payment.created_at(),
            updated_at: payment.updated_at(),
            completed_at: payment.completed_at(),
            failure_reason: payment.failure_reason().map(|r| r.to_string()),
            metadata: PaymentMetadataDto {
                platform_version: "".to_string(),
                user_ip: None,
                reference_id: None,
                additional_data: serde_json::Value::Null,
            },
            events: events.map(|e| e.into_iter().map(|event| PaymentEventDto {
                event_type: format!("{:?}", event.event_type()),
                event_data: serde_json::Value::Null,
                occurred_at: event.occurred_at(),
            }).collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pagination_calculation() {
        let query = GetUserPaymentHistoryQuery {
            user_id: uuid::Uuid::new_v4(),
            role: None,
            status: None,
            purpose_type: None,
            start_date: None,
            end_date: None,
            pagination: PaginationQuery {
                page: 2,
                page_size: 20,
                sort_by: None,
                sort_order: None,
            },
        };
        
        let (offset, limit) = query.pagination.to_offset_limit();
        assert_eq!(offset, 40);
        assert_eq!(limit, 20);
    }
} 