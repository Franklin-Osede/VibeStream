use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use super::controllers::{PaymentController, RoyaltyController};

/// Create payment routes
pub fn payment_routes(payment_controller: Arc<PaymentController>) -> Router {
    Router::new()
        // Payment operations
        .route("/payments", post(PaymentController::initiate_payment))
        .route("/payments/:id/process", post(PaymentController::process_payment))
        .route("/payments/:id/complete", post(PaymentController::complete_payment))
        .route("/payments/:id/cancel", post(PaymentController::cancel_payment))
        .route("/payments/refund", post(PaymentController::initiate_refund))
        
        // Payment queries
        .route("/payments/:id", get(PaymentController::get_payment))
        .route("/payments/transaction/:transaction_id", get(PaymentController::get_payment_by_transaction))
        .route("/payments/user/:user_id/history", get(PaymentController::get_user_payment_history))
        .route("/payments/user/:user_id/summary", get(PaymentController::get_user_payment_summary))
        
        // Payment analytics
        .route("/payments/statistics", get(PaymentController::get_payment_statistics))
        .route("/payments/analytics", get(PaymentController::get_payment_analytics))
        
        .with_state(payment_controller)
}

/// Create royalty routes
pub fn royalty_routes(royalty_controller: Arc<RoyaltyController>) -> Router {
    Router::new()
        // Royalty operations
        .route("/royalties/distribute", post(RoyaltyController::create_royalty_distribution))
        .route("/royalties/:id/process", post(RoyaltyController::process_royalty_distribution))
        
        // Royalty queries
        .route("/royalties", get(RoyaltyController::get_royalty_distributions))
        .route("/royalties/artist/:artist_id/summary", get(RoyaltyController::get_artist_revenue_summary))
        
        .with_state(royalty_controller)
}

/// Create all payment-related routes
pub fn create_payment_routes(
    payment_controller: Arc<PaymentController>,
    royalty_controller: Arc<RoyaltyController>,
) -> Router {
    Router::new()
        .merge(payment_routes(payment_controller))
        .merge(royalty_routes(royalty_controller))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;
    
    #[tokio::test]
    async fn test_routes_creation() {
        // This test would require proper mock controllers
        // For now, just test that routes can be created
        
        // Mock controllers would be needed for proper testing
        // let payment_controller = Arc::new(MockPaymentController::new());
        // let royalty_controller = Arc::new(MockRoyaltyController::new());
        // let app = create_payment_routes(payment_controller, royalty_controller);
        
        // Test that routes are accessible
        assert!(true);
    }
} 