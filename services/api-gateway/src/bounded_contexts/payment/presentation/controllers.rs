use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::{
    application::{
        commands::*,
        queries::*,
        handlers::*,
        dto::*,
    },
};

/// Payment Controller
pub struct PaymentController {
    payment_command_handler: Arc<dyn PaymentCommandHandler>,
    payment_query_handler: Arc<dyn PaymentQueryHandler>,
    payment_analytics_handler: Arc<dyn PaymentAnalyticsQueryHandler>,
}

impl PaymentController {
    pub fn new(
        payment_command_handler: Arc<dyn PaymentCommandHandler>,
        payment_query_handler: Arc<dyn PaymentQueryHandler>,
        payment_analytics_handler: Arc<dyn PaymentAnalyticsQueryHandler>,
    ) -> Self {
        Self {
            payment_command_handler,
            payment_query_handler,
            payment_analytics_handler,
        }
    }
    
    /// POST /api/v1/payments - Initiate a new payment
    pub async fn initiate_payment(
        State(controller): State<Arc<PaymentController>>,
        Json(command): Json<InitiatePaymentCommand>,
    ) -> Result<Json<SuccessResponseDTO<InitiatePaymentResult>>, AppError> {
        let result = controller
            .payment_command_handler
            .handle_initiate_payment(command)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: Some("Payment initiated successfully".to_string()),
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// POST /api/v1/payments/{id}/process - Start processing a payment
    pub async fn process_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
        Json(command): Json<StartPaymentProcessingCommand>,
    ) -> Result<Json<SuccessResponseDTO<ProcessPaymentResult>>, AppError> {
        let result = controller
            .payment_command_handler
            .handle_start_processing(command)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: Some("Payment processing started".to_string()),
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// POST /api/v1/payments/{id}/complete - Complete a payment
    pub async fn complete_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
        Json(command): Json<CompletePaymentCommand>,
    ) -> Result<Json<SuccessResponseDTO<ProcessPaymentResult>>, AppError> {
        let result = controller
            .payment_command_handler
            .handle_complete_payment(command)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: Some("Payment completed successfully".to_string()),
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// POST /api/v1/payments/{id}/cancel - Cancel a payment
    pub async fn cancel_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
        Json(command): Json<CancelPaymentCommand>,
    ) -> Result<Json<SuccessResponseDTO<ProcessPaymentResult>>, AppError> {
        let result = controller
            .payment_command_handler
            .handle_cancel_payment(command)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: Some("Payment cancelled successfully".to_string()),
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// POST /api/v1/payments/refund - Initiate a refund
    pub async fn initiate_refund(
        State(controller): State<Arc<PaymentController>>,
        Json(command): Json<InitiateRefundCommand>,
    ) -> Result<Json<SuccessResponseDTO<RefundResult>>, AppError> {
        let result = controller
            .payment_command_handler
            .handle_initiate_refund(command)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: Some("Refund initiated successfully".to_string()),
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/payments/{id} - Get payment by ID
    pub async fn get_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
        Query(params): Query<GetPaymentParams>,
    ) -> Result<Json<SuccessResponseDTO<PaymentQueryResult>>, AppError> {
        let query = GetPaymentQuery {
            payment_id,
            include_events: params.include_events.unwrap_or(false),
        };
        
        let result = controller
            .payment_query_handler
            .handle_get_payment(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/payments/transaction/{transaction_id} - Get payment by transaction ID
    pub async fn get_payment_by_transaction(
        State(controller): State<Arc<PaymentController>>,
        Path(transaction_id): Path<Uuid>,
    ) -> Result<Json<SuccessResponseDTO<PaymentQueryResult>>, AppError> {
        let query = GetPaymentByTransactionQuery { transaction_id };
        
        let result = controller
            .payment_query_handler
            .handle_get_payment_by_transaction(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/payments/user/{user_id}/history - Get user payment history
    pub async fn get_user_payment_history(
        State(controller): State<Arc<PaymentController>>,
        Path(user_id): Path<Uuid>,
        Query(params): Query<UserPaymentHistoryParams>,
    ) -> Result<Json<SuccessResponseDTO<PaymentHistoryResult>>, AppError> {
        let query = GetUserPaymentHistoryQuery {
            user_id,
            role: params.role,
            status: params.status,
            purpose_type: params.purpose_type,
            start_date: params.start_date,
            end_date: params.end_date,
            pagination: PaginationQuery {
                page: params.page.unwrap_or(0),
                page_size: params.page_size.unwrap_or(20),
                sort_by: params.sort_by,
                sort_order: params.sort_order,
            },
        };
        
        let result = controller
            .payment_query_handler
            .handle_get_user_payment_history(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/payments/statistics - Get payment statistics
    pub async fn get_payment_statistics(
        State(controller): State<Arc<PaymentController>>,
        Query(params): Query<PaymentStatisticsParams>,
    ) -> Result<Json<SuccessResponseDTO<PaymentStatisticsResult>>, AppError> {
        let query = GetPaymentStatisticsQuery {
            start_date: params.start_date,
            end_date: params.end_date,
            group_by: params.group_by,
            currency: params.currency,
            purpose_type: params.purpose_type,
        };
        
        let result = controller
            .payment_query_handler
            .handle_get_payment_statistics(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/payments/analytics - Get payment analytics dashboard
    pub async fn get_payment_analytics(
        State(controller): State<Arc<PaymentController>>,
        Query(params): Query<PaymentAnalyticsParams>,
    ) -> Result<Json<SuccessResponseDTO<PaymentAnalyticsResult>>, AppError> {
        let query = GetPaymentAnalyticsQuery {
            start_date: params.start_date,
            end_date: params.end_date,
            include_trends: params.include_trends.unwrap_or(false),
            include_top_users: params.include_top_users.unwrap_or(false),
            include_fraud_metrics: params.include_fraud_metrics.unwrap_or(false),
        };
        
        let result = controller
            .payment_analytics_handler
            .handle_get_payment_analytics(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/payments/user/{user_id}/summary - Get user payment summary
    pub async fn get_user_payment_summary(
        State(controller): State<Arc<PaymentController>>,
        Path(user_id): Path<Uuid>,
        Query(params): Query<UserSummaryParams>,
    ) -> Result<Json<SuccessResponseDTO<UserPaymentSummaryResult>>, AppError> {
        let query = GetUserPaymentSummaryQuery {
            user_id,
            start_date: params.start_date,
            end_date: params.end_date,
        };
        
        let result = controller
            .payment_analytics_handler
            .handle_get_user_payment_summary(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
}

/// Royalty Controller
pub struct RoyaltyController {
    royalty_command_handler: Arc<dyn RoyaltyCommandHandler>,
    royalty_query_handler: Arc<dyn RoyaltyQueryHandler>,
}

impl RoyaltyController {
    pub fn new(
        royalty_command_handler: Arc<dyn RoyaltyCommandHandler>,
        royalty_query_handler: Arc<dyn RoyaltyQueryHandler>,
    ) -> Self {
        Self {
            royalty_command_handler,
            royalty_query_handler,
        }
    }
    
    /// POST /api/v1/royalties/distribute - Create royalty distribution
    pub async fn create_royalty_distribution(
        State(controller): State<Arc<RoyaltyController>>,
        Json(command): Json<CreateRoyaltyDistributionCommand>,
    ) -> Result<Json<SuccessResponseDTO<RoyaltyDistributionResult>>, AppError> {
        let result = controller
            .royalty_command_handler
            .handle_create_distribution(command)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: Some("Royalty distribution created successfully".to_string()),
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// POST /api/v1/royalties/{id}/process - Process royalty distribution
    pub async fn process_royalty_distribution(
        State(controller): State<Arc<RoyaltyController>>,
        Path(distribution_id): Path<Uuid>,
        Json(command): Json<ProcessRoyaltyDistributionCommand>,
    ) -> Result<Json<SuccessResponseDTO<RoyaltyDistributionResult>>, AppError> {
        let result = controller
            .royalty_command_handler
            .handle_process_distribution(command)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: Some("Royalty distribution processed successfully".to_string()),
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/royalties - Get royalty distributions
    pub async fn get_royalty_distributions(
        State(controller): State<Arc<RoyaltyController>>,
        Query(params): Query<RoyaltyDistributionsParams>,
    ) -> Result<Json<SuccessResponseDTO<Vec<RoyaltyDistributionQueryResult>>>, AppError> {
        let query = GetRoyaltyDistributionsQuery {
            artist_id: params.artist_id,
            song_id: params.song_id,
            status: params.status,
            start_date: params.start_date,
            end_date: params.end_date,
            pagination: PaginationQuery {
                page: params.page.unwrap_or(0),
                page_size: params.page_size.unwrap_or(20),
                sort_by: params.sort_by,
                sort_order: params.sort_order,
            },
        };
        
        let result = controller
            .royalty_query_handler
            .handle_get_royalty_distributions(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
    
    /// GET /api/v1/royalties/artist/{artist_id}/summary - Get artist revenue summary
    pub async fn get_artist_revenue_summary(
        State(controller): State<Arc<PaymentController>>,
        Path(artist_id): Path<Uuid>,
        Query(params): Query<ArtistSummaryParams>,
    ) -> Result<Json<SuccessResponseDTO<ArtistRevenueSummaryResult>>, AppError> {
        let query = GetArtistRevenueSummaryQuery {
            artist_id,
            start_date: params.start_date,
            end_date: params.end_date,
            include_song_breakdown: params.include_song_breakdown.unwrap_or(false),
        };
        
        let result = controller
            .payment_analytics_handler
            .handle_get_artist_revenue_summary(query)
            .await?;
        
        Ok(Json(SuccessResponseDTO {
            success: true,
            data: result,
            message: None,
            correlation_id: Some(Uuid::new_v4().to_string()),
            timestamp: chrono::Utc::now(),
        }))
    }
}

// Parameter structs for query parameters

#[derive(Debug, Deserialize)]
pub struct GetPaymentParams {
    pub include_events: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UserPaymentHistoryParams {
    pub role: Option<String>,
    pub status: Option<String>,
    pub purpose_type: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentStatisticsParams {
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub group_by: Option<String>,
    pub currency: Option<Currency>,
    pub purpose_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentAnalyticsParams {
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub include_trends: Option<bool>,
    pub include_top_users: Option<bool>,
    pub include_fraud_metrics: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UserSummaryParams {
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct RoyaltyDistributionsParams {
    pub artist_id: Option<Uuid>,
    pub song_id: Option<Uuid>,
    pub status: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistSummaryParams {
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub include_song_breakdown: Option<bool>,
}

use crate::bounded_contexts::payment::domain::value_objects::Currency; 