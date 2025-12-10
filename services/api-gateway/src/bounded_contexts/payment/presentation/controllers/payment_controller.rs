use axum::{
    extract::{Query, Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router, Extension,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use std::sync::Arc;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::payment::application::{
    commands::{
        InitiatePaymentCommand, ProcessPaymentCommand, CompletePaymentCommand, 
        CancelPaymentCommand, InitiateRefundCommand, 
        CreateRoyaltyDistributionCommand, ProcessRoyaltyDistributionCommand,
    },
    handlers::{
        command_handlers::{
            PaymentCommandHandler, PaymentCommandHandlerImpl,
            RoyaltyCommandHandler, RoyaltyCommandHandlerImpl,
        },
        query_handlers::*,
    },
    queries::{
        GetPaymentQuery, SearchPaymentsQuery, SearchPaymentsRequest, SearchPaymentsResult,
        GetPaymentByTransactionQuery, GetUserPaymentHistoryQuery, GetUserPaymentSummaryQuery,
        GetPaymentStatisticsQuery, GetPaymentAnalyticsQuery, 
        GetRoyaltyDistributionsQuery, GetArtistRevenueSummaryQuery,
        GetWalletQuery, GetWalletBalanceQuery,
    },
    dto::*, 
    dto::{PaymentStatistics, PaymentAnalytics, FraudDetectionStats},
    services::{
        PaymentApplicationService, RoyaltyDistributionApplicationService,
        MockPaymentProcessingService, MockFraudDetectionService, MockNotificationService,
    },
};

use crate::bounded_contexts::payment::infrastructure::repositories::{
    PostgreSQLPaymentRepository, PostgresRoyaltyRepository, PostgresWalletRepository,
};
use crate::bounded_contexts::payment::infrastructure::webhooks::{
    WebhookRouter, StripeWebhookHandler, PayPalWebhookHandler, CoinbaseWebhookHandler,
    WebhookQueueProcessor, ReconciliationResult,
};
use crate::services::MessageQueue;
use crate::bounded_contexts::payment::infrastructure::gateways::{
    StripeGateway, PayPalGateway, CoinbaseGateway,
};

use crate::shared::domain::errors::AppError;

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

// Payment DTOs are now imported from application/dto.rs


// API Response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// PAYMENT CONTROLLER
// =============================================================================

pub struct PaymentController {
    payment_repository: Arc<PostgreSQLPaymentRepository>,
    royalty_repository: Arc<PostgresRoyaltyRepository>,
    wallet_repository: Arc<PostgresWalletRepository>,
    webhook_router: Arc<WebhookRouter>,
    webhook_queue_processor: Option<Arc<WebhookQueueProcessor>>,
    command_handler: Arc<PaymentCommandHandlerImpl>,
}

impl PaymentController {
    pub fn new(
        payment_repository: Arc<PostgreSQLPaymentRepository>,
        royalty_repository: Arc<PostgresRoyaltyRepository>,
        wallet_repository: Arc<PostgresWalletRepository>,
        webhook_router: Arc<WebhookRouter>,
        webhook_queue_processor: Option<Arc<WebhookQueueProcessor>>,
        command_handler: Arc<PaymentCommandHandlerImpl>,
    ) -> Self {
        Self {
            payment_repository,
            royalty_repository,
            wallet_repository,
            webhook_router,
            webhook_queue_processor,
            command_handler,
        }
    }

    pub fn routes(controller: Arc<Self>) -> Router {
        Router::new()
            // Payment operations
            .route("/payments", post(initiate_payment))
            .route("/payments/:payment_id/process", post(process_payment))
            .route("/payments/:payment_id/complete", post(complete_payment))
            .route("/payments/:payment_id/cancel", post(cancel_payment))
            .route("/payments/refund", post(initiate_refund))
            
            // Payment queries
            .route("/payments/:payment_id", get(get_payment))
            .route("/payments/transaction/:transaction_id", get(get_payment_by_transaction))
            .route("/payments/search", get(search_payments))
            .route("/payments/user/:user_id/history", get(get_user_payment_history))
            .route("/payments/user/:user_id/summary", get(get_user_payment_summary))
            
            // Payment analytics
            .route("/payments/statistics", get(get_payment_statistics))
            .route("/payments/analytics", get(get_payment_analytics))
            
            // Royalty operations
            .route("/royalties/distribute", post(distribute_royalties))
            .route("/royalties/:distribution_id/process", post(process_royalty_distribution))
            
            // Royalty queries
            .route("/royalties", get(get_royalty_distributions))
            .route("/royalties/artist/:artist_id/summary", get(get_artist_revenue_summary))
            
            // Wallet operations
            .route("/wallets", get(list_wallets).post(create_wallet))
            .route("/wallets/:wallet_id", get(get_wallet).put(update_wallet))
            .route("/wallets/:wallet_id/balance", get(get_wallet_balance))
            
            // Payment gateway operations
            .route("/gateways", get(list_payment_gateways))
            .route("/gateways/:gateway_id/process", post(process_gateway_payment))
            
            // Webhook endpoints
            .route("/webhooks/stripe", post(stripe_webhook))
            .route("/webhooks/paypal", post(paypal_webhook))
            .route("/webhooks/coinbase", post(coinbase_webhook))
            .route("/webhooks/:gateway", post(generic_webhook))
            
            // Reconciliation endpoints
            .route("/reconcile/:payment_id", post(reconcile_payment))
            
            .with_state(controller)
    }
}

    // =============================================================================
    // PAYMENT OPERATIONS
    // =============================================================================

    #[utoipa::path(
        post,
        path = "/api/v1/payments",
        request_body = InitiatePaymentRequest,
        responses(
            (status = 200, description = "Payment initiated successfully", body = ApiResponse<InitiatePaymentResponse>),
            (status = 400, description = "Invalid input"),
            (status = 402, description = "Insufficient funds"),
            (status = 403, description = "Fraud detected")
        ),
        tag = "payments"
    )]
    pub async fn initiate_payment(
        State(controller): State<Arc<PaymentController>>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<InitiatePaymentRequest>,
    ) -> Result<Json<ApiResponse<InitiatePaymentResponse>>, StatusCode> {
        // Construct purpose DTO (simplified mapping)
        let purpose = PaymentPurposeDto {
            purpose_type: request.payment_type.clone(),
            campaign_id: if request.payment_type == "NFTPurchase" { Some(request.related_entity_id) } else { None },
            contract_id: if request.payment_type == "SharePurchase" { Some(request.related_entity_id) } else { None },
            song_id: if request.payment_type == "RoyaltyDistribution" { Some(request.related_entity_id) } else { None },
            nft_quantity: None,
            ownership_percentage: None,
            share_id: None,
            from_user: None,
            to_user: None,
            artist_id: None,
            session_id: None,
            listen_duration: None,
            distribution_id: None,
            original_payment_id: None,
            reason: None,
        };

        let command = InitiatePaymentCommand {
            payer_id: request.payer_id,
            payee_id: request.payee_id,
            amount_value: request.amount,
            amount_currency: request.currency,
            payment_method: request.payment_method,
            purpose,
            metadata: request.metadata,
            idempotency_key: None, // Could come from headers
        };

        match controller.command_handler.handle_initiate_payment(command).await {
            Ok(result) => {
                let response = InitiatePaymentResponse {
                    payment_id: result.payment_id,
                    status: result.status,
                    amount: result.net_amount, // Using net amount 
                    currency: request.currency.to_string(), // Returning requested currency
                    payment_url: None, // URL generated later or by specific gateway logic
                    expires_at: None, // Default expiry
                    created_at: result.created_at,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(err) => {
                eprintln!("Initiate payment error: {:?}", err);
                match err {
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    AppError::InsufficientFundsError(_) => Err(StatusCode::PAYMENT_REQUIRED), // Fixed enum variant name
                    AppError::FraudDetected(_) => Err(StatusCode::FORBIDDEN),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    #[utoipa::path(
        post,
        path = "/api/v1/payments/{payment_id}/process",
        request_body = ProcessPaymentRequest,
        params(
            ("payment_id" = Uuid, Path, description = "Payment ID to process")
        ),
        responses(
            (status = 200, description = "Payment processed successfully", body = ApiResponse<PaymentDTO>),
            (status = 404, description = "Payment not found"),
            (status = 502, description = "Payment gateway error")
        ),
        tag = "payments"
    )]
    pub async fn process_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<ProcessPaymentRequest>,
    ) -> Result<Json<ApiResponse<PaymentDTO>>, StatusCode> { // Start returning proper result type or DTO? 
        // Note: Controller returns PaymentDTO but Command returns ProcessPaymentResult. 
        // We should map ProcessPaymentResult to PaymentDTO if possible, or return ProcessPaymentResult directly if API allows.
        // Assuming we return ProcessPaymentResult for now as it contains status etc.
        
        let command = StartPaymentProcessingCommand {
            payment_id,
            processor_id: "System".to_string(), // Default or derive
            external_transaction_id: request.gateway_transaction_id,
        };

        match controller.command_handler.handle_start_processing(command).await {
            Ok(_) => {
                // We need to return PaymentDTO, but handle_start_processing returns ProcessPaymentResult.
                // We might need to fetch the payment again or construct DTO from result.
                // For now, let's return a success response with what we have.
                // This assumes PaymentDTO can be constructed partialy or we change response type.
                // Re-fetching is safest.
                let query = GetPaymentQuery { payment_id };
                let query_handler = GetPaymentQueryHandler::new(controller.payment_repository.clone());
                 match query_handler.handle(query).await {
                    Ok(Some(payment)) => Ok(Json(ApiResponse::success(payment))),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR)
                 }
            }, 
            Err(err) => {
                eprintln!("Process payment error: {:?}", err);
                match err {
                    AppError::NotFound(_) => Err(StatusCode::NOT_FOUND),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    // AppError::PaymentGatewayError(_) => Err(StatusCode::BAD_GATEWAY), // If exists
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    #[utoipa::path(
        post,
        path = "/api/v1/payments/{payment_id}/complete",
        params(
            ("payment_id" = Uuid, Path, description = "Payment ID to complete")
        ),
        responses(
            (status = 200, description = "Payment completed successfully", body = ApiResponse<PaymentDTO>),
            (status = 404, description = "Payment not found"),
            (status = 400, description = "Invalid request")
        ),
        tag = "payments"
    )]
    pub async fn complete_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<PaymentDTO>>, StatusCode> {
        let command = CompletePaymentCommand {
            payment_id,
            blockchain_hash: None, // Need to get from request if manual completion
            external_transaction_id: None,
            gateway_response: None,
            processing_fee: None,
        };

        match controller.command_handler.handle_complete_payment(command).await {
            Ok(_) => {
                // Re-fetch for DTO
                let query = GetPaymentQuery { payment_id };
                let query_handler = GetPaymentQueryHandler::new(controller.payment_repository.clone());
                 match query_handler.handle(query).await {
                    Ok(Some(payment)) => Ok(Json(ApiResponse::success(payment))),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR)
                 }
             }
            Err(err) => {
                eprintln!("Complete payment error: {:?}", err);
                match err {
                    AppError::NotFound(_) => Err(StatusCode::NOT_FOUND),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    #[utoipa::path(
        post,
        path = "/api/v1/payments/{payment_id}/cancel",
        params(
            ("payment_id" = Uuid, Path, description = "Payment ID to cancel")
        ),
        responses(
            (status = 200, description = "Payment cancelled successfully", body = ApiResponse<PaymentDTO>),
            (status = 404, description = "Payment not found"),
            (status = 400, description = "Invalid request")
        ),
        tag = "payments"
    )]
    pub async fn cancel_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<PaymentDTO>>, StatusCode> {
        let command = CancelPaymentCommand {
            payment_id,
            reason: "User requested cancellation".to_string(),
            cancelled_by: current_user_id,
        };

        match controller.command_handler.handle_cancel_payment(command).await {
            Ok(_) => {
                // Re-fetch for DTO
                let query = GetPaymentQuery { payment_id };
                let query_handler = GetPaymentQueryHandler::new(controller.payment_repository.clone());
                 match query_handler.handle(query).await {
                    Ok(Some(payment)) => Ok(Json(ApiResponse::success(payment))),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR)
                 }
             }
            Err(err) => {
                eprintln!("Cancel payment error: {:?}", err);
                match err {
                    AppError::NotFound(_) => Err(StatusCode::NOT_FOUND),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    #[utoipa::path(
        post,
        path = "/api/v1/payments/refund",
        request_body = InitiateRefundRequest,
        responses(
            (status = 200, description = "Refund initiated successfully", body = ApiResponse<crate::bounded_contexts::payment::application::commands::RefundResult>),
            (status = 400, description = "Invalid input"),
            (status = 404, description = "Original payment not found")
        ),
        tag = "payments"
    )]
    pub async fn initiate_refund(
        State(controller): State<Arc<PaymentController>>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<InitiateRefundRequest>,
    ) -> Result<Json<ApiResponse<crate::bounded_contexts::payment::application::commands::RefundResult>>, StatusCode> {
        let original_payment_id = request.original_payment_id.ok_or(StatusCode::BAD_REQUEST)?;
        
        let command = InitiateRefundCommand {
            original_payment_id,
            refund_amount: request.refund_amount.unwrap_or(0.0),
            refund_currency: Currency::USD, // TODO: Get from request or original payment
            reason: request.reason.unwrap_or_else(|| "User requested refund".to_string()),
            initiated_by: current_user_id,
        };

        match controller.payment_command_handler.handle_initiate_refund(command).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(err) => {
                eprintln!("Initiate refund error: {:?}", err);
                match err {
                   AppError::NotFound(_) => Err(StatusCode::NOT_FOUND),
                   AppError::InvalidInput(_) => Err(StatusCode::BAD_REQUEST),
                   AppError::InvalidState(_) => Err(StatusCode::CONFLICT),
                   _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    // =============================================================================
    // PAYMENT QUERIES
    // =============================================================================

    #[utoipa::path(
        get,
        path = "/api/v1/payments/{payment_id}",
        params(
            ("payment_id" = Uuid, Path, description = "Payment ID")
        ),
        responses(
            (status = 200, description = "Payment details", body = ApiResponse<PaymentDTO>),
            (status = 404, description = "Payment not found")
        ),
        tag = "payments"
    )]
    pub async fn get_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<PaymentDTO>>, StatusCode> {
        let query = GetPaymentQuery { payment_id };
        let handler = GetPaymentQueryHandler::new(controller.payment_repository.clone());

        match handler.handle(query).await {
            Ok(Some(payment)) => Ok(Json(ApiResponse::success(payment))),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(err) => {
                eprintln!("Get payment error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn get_payment_by_transaction(
        State(controller): State<Arc<PaymentController>>,
        Path(transaction_id): Path<String>,
    ) -> Result<Json<ApiResponse<PaymentDTO>>, StatusCode> {
        let query = GetPaymentByTransactionQuery { transaction_id };
        
        // Handler would be implemented
        // For now, return not found
        Err(StatusCode::NOT_FOUND)
    }

    async fn search_payments(
        State(controller): State<Arc<PaymentController>>,
        Query(params): Query<SearchPaymentsRequest>,
    ) -> Result<Json<ApiResponse<SearchPaymentsResult>>, StatusCode> {
        let query = SearchPaymentsQuery {
            user_id: params.user_id,
            payment_type: params.payment_type,
            status: params.status,
            currency: params.currency,
            min_amount: params.min_amount,
            max_amount: params.max_amount,
            date_from: params.date_from,
            date_to: params.date_to,
            limit: params.limit,
            offset: params.offset,
        };

        // Handler would be implemented
        let result = SearchPaymentsResult {
            payments: vec![],
            total_count: 0,
            has_more: false,
        };

        Ok(Json(ApiResponse::success(result)))
    }

    async fn get_user_payment_history(
        State(_controller): State<Arc<PaymentController>>,
        Path(user_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<SearchPaymentsResult>>, StatusCode> {
        // Check authorization
        if user_id != current_user_id {
            return Err(StatusCode::FORBIDDEN);
        }

        let result = SearchPaymentsResult {
            payments: vec![],
            total_count: 0,
            has_more: false,
        };

        Ok(Json(ApiResponse::success(result)))
    }

    async fn get_user_payment_summary(
        State(_controller): State<Arc<PaymentController>>,
        Path(user_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        // Check authorization
        if user_id != current_user_id {
            return Err(StatusCode::FORBIDDEN);
        }

        Ok(Json(ApiResponse::success(())))
    }

    // =============================================================================
    // PAYMENT ANALYTICS
    // =============================================================================

    async fn get_payment_statistics(
        State(_controller): State<Arc<PaymentController>>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<PaymentStatistics>>, StatusCode> {
        let stats = PaymentStatistics {
            total_payments: 1000,
            total_volume: 50000.0,
            successful_payments: 950,
            failed_payments: 30,
            refunded_payments: 20,
            average_amount: 50.0,
            currencies: std::collections::HashMap::new(),
            payment_methods: std::collections::HashMap::new(),
        };

        Ok(Json(ApiResponse::success(stats)))
    }

    async fn get_payment_analytics(
        State(_controller): State<Arc<PaymentController>>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<PaymentAnalytics>>, StatusCode> {
        let analytics = PaymentAnalytics {
            time_range: params.get("time_range").cloned().unwrap_or("7d".to_string()),
            daily_volume: vec![],
            top_payment_types: vec![],
            gateway_performance: vec![],
            fraud_detection_stats: FraudDetectionStats {
                total_checks: 1000,
                flagged_transactions: 5,
                false_positives: 1,
                prevented_fraud_amount: 500.0,
            },
        };

        Ok(Json(ApiResponse::success(analytics)))
    }

    // =============================================================================
    // ROYALTY OPERATIONS
    // =============================================================================

    async fn distribute_royalties(
        State(controller): State<Arc<PaymentController>>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<DistributeRoyaltiesRequest>,
    ) -> Result<Json<ApiResponse<DistributeRoyaltiesResponse>>, StatusCode> {
        let command = DistributeRoyaltiesCommand {
            song_id: request.song_id,
            artist_id: request.artist_id,
            album_id: request.album_id,
            period_start: request.period_start,
            period_end: request.period_end,
            total_revenue: request.total_revenue,
            currency: request.currency,
            distribution_rules: request.distribution_rules,
            initiated_by: current_user_id,
        };

        let handler = DistributeRoyaltiesCommandHandler::new(controller.royalty_repository.clone());

        match handler.handle(command).await {
            Ok(result) => {
                let response = DistributeRoyaltiesResponse {
                    distribution_id: result.distribution_id,
                    total_amount: result.total_amount,
                    recipient_count: result.recipient_count,
                    distributions: result.distributions,
                    status: result.status,
                    created_at: result.created_at,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(err) => {
                eprintln!("Distribute royalties error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn process_royalty_distribution(
        State(_controller): State<Arc<PaymentController>>,
        Path(distribution_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        // Process royalty distribution logic
        Ok(Json(ApiResponse::success(())))
    }

    async fn get_royalty_distributions(
        State(_controller): State<Arc<PaymentController>>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        Ok(Json(ApiResponse::success(())))
    }

    async fn get_artist_revenue_summary(
        State(_controller): State<Arc<PaymentController>>,
        Path(artist_id): Path<Uuid>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        Ok(Json(ApiResponse::success(())))
    }

    // =============================================================================
    // WALLET OPERATIONS
    // =============================================================================

    async fn create_wallet(
        State(controller): State<Arc<PaymentController>>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<CreateWalletRequest>,
    ) -> Result<Json<ApiResponse<CreateWalletResponse>>, StatusCode> {
        let command = CreateWalletCommand {
            user_id: request.user_id,
            wallet_type: request.wallet_type,
            currency: request.currency,
            is_primary: request.is_primary,
            created_by: current_user_id,
        };

        let handler = CreateWalletCommandHandler::new(controller.wallet_repository.clone());

        match handler.handle(command).await {
            Ok(result) => {
                let response = CreateWalletResponse {
                    wallet_id: result.wallet_id,
                    address: result.address,
                    wallet_type: result.wallet_type,
                    currency: result.currency,
                    balance: result.balance,
                    is_primary: result.is_primary,
                    created_at: result.created_at,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(err) => {
                eprintln!("Create wallet error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn list_wallets(
        State(_controller): State<Arc<PaymentController>>,
        Query(params): Query<std::collections::HashMap<String, String>>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<Vec<CreateWalletResponse>>>, StatusCode> {
        // List user's wallets
        Ok(Json(ApiResponse::success(vec![])))
    }

    async fn get_wallet(
        State(_controller): State<Arc<PaymentController>>,
        Path(wallet_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<CreateWalletResponse>>, StatusCode> {
        // Get wallet details
        Err(StatusCode::NOT_FOUND)
    }

    async fn update_wallet(
        State(_controller): State<Arc<PaymentController>>,
        Path(wallet_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<CreateWalletResponse>>, StatusCode> {
        // Update wallet
        Err(StatusCode::NOT_FOUND)
    }

    async fn get_wallet_balance(
        State(_controller): State<Arc<PaymentController>>,
        Path(wallet_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<WalletBalanceResponse>>, StatusCode> {
        let response = WalletBalanceResponse {
            wallet_id,
            balance: 100.0,
            currency: "USD".to_string(),
            available_balance: 100.0,
            pending_balance: 0.0,
            last_updated: Utc::now(),
        };

        Ok(Json(ApiResponse::success(response)))
    }

    // =============================================================================
    // PAYMENT GATEWAY OPERATIONS
    // =============================================================================

    async fn list_payment_gateways(
        State(_controller): State<Arc<PaymentController>>,
    ) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
        let gateways = vec![
            "stripe".to_string(),
            "paypal".to_string(),
            "ethereum".to_string(),
            "solana".to_string(),
        ];

        Ok(Json(ApiResponse::success(gateways)))
    }

    async fn process_gateway_payment(
        State(_controller): State<Arc<PaymentController>>,
        Path(gateway_id): Path<String>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        // Process payment through specific gateway
        Ok(Json(ApiResponse::success(())))
    }

    // =============================================================================
    // WEBHOOK HANDLERS
    // =============================================================================

    async fn stripe_webhook(
        State(controller): State<Arc<PaymentController>>,
        headers: axum::http::HeaderMap,
        body: String,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        let signature = headers
            .get("stripe-signature")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::BAD_REQUEST)?;

        // If queue processor is available, enqueue for async processing
        if let Some(ref queue_processor) = controller.webhook_queue_processor {
            match queue_processor.enqueue_webhook("stripe", &body, signature).await {
                Ok(webhook_id) => {
                    tracing::info!("Enqueued Stripe webhook: {}", webhook_id);
                    return Ok(Json(ApiResponse::success(())));
                }
                Err(e) => {
                    tracing::error!("Failed to enqueue Stripe webhook: {:?}", e);
                    // Fallback to synchronous processing
                }
            }
        }

        // Fallback to synchronous processing
        match controller.webhook_router.route_webhook("stripe", &body, signature).await {
            Ok(result) => {
                if result.success {
                    Ok(Json(ApiResponse::success(())))
                } else {
                    Ok(Json(ApiResponse::error(result.error_message.unwrap_or_else(|| "Webhook processing failed".to_string()))))
                }
            }
            Err(e) => {
                tracing::error!("Stripe webhook error: {:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }

    async fn paypal_webhook(
        State(controller): State<Arc<PaymentController>>,
        headers: axum::http::HeaderMap,
        body: String,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        let signature = headers
            .get("paypal-transmission-sig")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::BAD_REQUEST)?;

        // If queue processor is available, enqueue for async processing
        if let Some(ref queue_processor) = controller.webhook_queue_processor {
            match queue_processor.enqueue_webhook("paypal", &body, signature).await {
                Ok(webhook_id) => {
                    tracing::info!("Enqueued PayPal webhook: {}", webhook_id);
                    return Ok(Json(ApiResponse::success(())));
                }
                Err(e) => {
                    tracing::error!("Failed to enqueue PayPal webhook: {:?}", e);
                    // Fallback to synchronous processing
                }
            }
        }

        // Fallback to synchronous processing
        match controller.webhook_router.route_webhook("paypal", &body, signature).await {
            Ok(result) => {
                if result.success {
                    Ok(Json(ApiResponse::success(())))
                } else {
                    Ok(Json(ApiResponse::error(result.error_message.unwrap_or_else(|| "Webhook processing failed".to_string()))))
                }
            }
            Err(e) => {
                tracing::error!("PayPal webhook error: {:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }

    async fn coinbase_webhook(
        State(controller): State<Arc<PaymentController>>,
        headers: axum::http::HeaderMap,
        body: String,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        let signature = headers
            .get("x-cc-webhook-signature")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::BAD_REQUEST)?;

        // If queue processor is available, enqueue for async processing
        if let Some(ref queue_processor) = controller.webhook_queue_processor {
            match queue_processor.enqueue_webhook("coinbase", &body, signature).await {
                Ok(webhook_id) => {
                    tracing::info!("Enqueued Coinbase webhook: {}", webhook_id);
                    return Ok(Json(ApiResponse::success(())));
                }
                Err(e) => {
                    tracing::error!("Failed to enqueue Coinbase webhook: {:?}", e);
                    // Fallback to synchronous processing
                }
            }
        }

        match controller.webhook_router.route_webhook("coinbase", &body, signature).await {
            Ok(result) => {
                if result.success {
                    Ok(Json(ApiResponse::success(())))
                } else {
                    Ok(Json(ApiResponse::error(result.error_message.unwrap_or_else(|| "Webhook processing failed".to_string()))))
                }
            }
            Err(e) => {
                tracing::error!("Coinbase webhook error: {:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }

    /// POST /payments/reconcile/:payment_id
    /// Reconcile payment with webhook events
    async fn reconcile_payment(
        State(controller): State<Arc<PaymentController>>,
        Path(payment_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<ReconciliationResult>>, StatusCode> {
        if let Some(ref queue_processor) = controller.webhook_queue_processor {
            match queue_processor.reconcile_payment(payment_id).await {
                Ok(result) => Ok(Json(ApiResponse::success(result))),
                Err(e) => {
                    tracing::error!("Reconciliation error: {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        } else {
            Err(StatusCode::NOT_IMPLEMENTED)
        }
    }

    async fn generic_webhook(
        State(controller): State<Arc<PaymentController>>,
        Path(gateway): Path<String>,
        headers: axum::http::HeaderMap,
        body: String,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        // Try to find signature in common header names
        let signature = headers
            .get("x-webhook-signature")
            .or_else(|| headers.get("signature"))
            .or_else(|| headers.get("x-signature"))
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::BAD_REQUEST)?;

        match controller.webhook_router.route_webhook(&gateway, &body, signature).await {
            Ok(result) => {
                if result.success {
                    Ok(Json(ApiResponse::success(())))
                } else {
                    Ok(Json(ApiResponse::error(result.error_message.unwrap_or_else(|| "Webhook processing failed".to_string()))))
                }
            }
            Err(e) => {
                tracing::error!("Generic webhook error for gateway {}: {:?}", gateway, e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }


// Factory functions
pub fn create_payment_controller(
    payment_repository: Arc<PostgresPaymentRepository>,
    royalty_repository: Arc<PostgresRoyaltyRepository>,
    wallet_repository: Arc<PostgresWalletRepository>,
    webhook_router: Arc<WebhookRouter>,
) -> Arc<PaymentController> {
    Arc::new(PaymentController::new(
        payment_repository,
        royalty_repository,
        wallet_repository,
        webhook_router,
    ))
}

pub fn create_payment_routes(
    payment_repository: Arc<PostgresPaymentRepository>,
    royalty_repository: Arc<PostgresRoyaltyRepository>,
    wallet_repository: Arc<PostgresWalletRepository>,
    webhook_router: Arc<WebhookRouter>,
) -> Router {
    let controller = create_payment_controller(
        payment_repository,
        royalty_repository,
        wallet_repository,
        webhook_router,
    );
    
    PaymentController::routes(controller)
} 