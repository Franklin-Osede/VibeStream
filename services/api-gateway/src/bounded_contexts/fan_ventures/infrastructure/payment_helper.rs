//! Helper functions for creating PaymentCommandHandler
//! 
//! Simplifies the creation of PaymentCommandHandler for use in Fan Ventures handlers.

use std::sync::Arc;
use sqlx::PgPool;

use crate::bounded_contexts::payment::{
    application::{
        handlers::command_handlers::{PaymentCommandHandler, PaymentCommandHandlerImpl},
        services::PaymentApplicationService,
    },
    domain::services::*,
    infrastructure::{
        repositories::PostgreSQLPaymentRepository,
        services::PaymentProcessingServiceImpl,
        gateways::MultiGatewayRouter,
    },
};

/// Create a PaymentCommandHandler with minimal dependencies
/// 
/// This is a simplified version that uses mocks for services that aren't critical
/// for basic payment creation.
pub fn create_payment_command_handler(pool: PgPool) -> Arc<dyn PaymentCommandHandler> {
    // Create payment repository
    let payment_repository = Arc::new(PostgreSQLPaymentRepository::new(pool.clone()));
    
    // Create a simple gateway router (empty for now, payments will be created but not processed immediately)
    let gateway_router = Arc::new(MultiGatewayRouter::new());
    
    // Create payment processing service
    let payment_processing_service = Arc::new(PaymentProcessingServiceImpl::new(
        gateway_router.clone(),
        pool.clone()
    ));
    
    // Use mocks for auxiliary services
    let fraud_detection_service = Arc::new(crate::bounded_contexts::payment::application::services::MockFraudDetectionService {});
    let notification_service = Arc::new(crate::bounded_contexts::payment::application::services::MockNotificationService {});
    
    // Create application service
    let payment_application_service = Arc::new(PaymentApplicationService::new(
        payment_repository.clone(),
        payment_processing_service.clone(),
        fraud_detection_service.clone(),
        notification_service.clone(),
    ));
    
    // Create command handler
    Arc::new(PaymentCommandHandlerImpl::new(
        payment_repository,
        payment_processing_service,
        fraud_detection_service,
        notification_service,
        payment_application_service,
    ))
}

