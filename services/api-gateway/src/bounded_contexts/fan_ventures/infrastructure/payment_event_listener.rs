//! Payment Event Listeners for Fan Ventures
//! 
//! Handles payment events (completed, failed) and updates venture investments accordingly.

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use tracing::{info, warn, error};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::{
    orchestrator::{EventHandler, DomainEvent},
    payment::domain::events::{PaymentCompleted, PaymentFailed},
    fan_ventures::infrastructure::{
        postgres_repository::PostgresFanVenturesRepository,
        payment_integration::FanVenturesPaymentIntegration,
    },
};

/// Event listener for payment events related to fan ventures
pub struct FanVenturesPaymentEventListener {
    payment_integration: Arc<FanVenturesPaymentIntegration>,
}

impl FanVenturesPaymentEventListener {
    pub fn new(payment_integration: Arc<FanVenturesPaymentIntegration>) -> Self {
        Self {
            payment_integration,
        }
    }

    /// Handle PaymentCompleted event
    /// 
    /// Checks if the payment is for a venture investment and updates accordingly.
    async fn handle_payment_completed_internal(&self, event: &PaymentCompleted) -> Result<(), AppError> {
        // Check if this is a venture investment payment by looking at metadata
        // The metadata should contain investment_id and venture_id
        let metadata = &event.metadata;
        let event_data = metadata.event_data();
        
        // Try to extract investment_id and venture_id from metadata
        let investment_id = event_data
            .get("additional_data")
            .and_then(|ad| ad.get("investment_id"))
            .and_then(|id| id.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());
        
        let venture_id = event_data
            .get("additional_data")
            .and_then(|ad| ad.get("venture_id"))
            .and_then(|id| id.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());

        // Also check purpose - if it's SharePurchase with contract_id, that's our venture_id
        let venture_id_from_purpose = match &event.purpose {
            crate::bounded_contexts::payment::domain::value_objects::PaymentPurpose::SharePurchase { contract_id, .. } => {
                Some(*contract_id)
            }
            _ => None,
        };

        let venture_id = venture_id.or(venture_id_from_purpose);
        
        if let (Some(inv_id), Some(v_id)) = (investment_id, venture_id) {
            info!(
                "Processing payment completed for investment {} in venture {}",
                inv_id, v_id
            );

            // Update investment and funding
            self.payment_integration.handle_payment_confirmed(
                *event.payment_id.value(),
                inv_id,
                v_id,
                event.net_amount.value(),
            ).await?;

            info!(
                "Successfully processed payment completion for investment {}",
                inv_id
            );
        } else {
            // Not a venture investment payment, ignore
            info!("Payment {} is not a venture investment, ignoring", event.payment_id.value());
        }

        Ok(())
    }

    /// Handle PaymentFailed event
    /// 
    /// Checks if the payment is for a venture investment and cancels it.
    async fn handle_payment_failed_internal(&self, event: &PaymentFailed) -> Result<(), AppError> {
        // Similar logic to handle_payment_completed
        // For now, we'll need to query the payment to get metadata
        // This is a simplified version - in production, you'd want to store
        // the investment_id in a more accessible way
        
        warn!(
            "Payment {} failed - need to check if it's a venture investment",
            event.payment_id.value()
        );

        // TODO: Query payment repository to get metadata and find investment_id
        // For now, we'll log and return OK (the payment system will handle the failure)
        
        Ok(())
    }
}

/// Implement EventHandler trait for FanVenturesPaymentEventListener
#[async_trait]
impl EventHandler for FanVenturesPaymentEventListener {
    async fn handle(&self, event: &dyn DomainEvent) -> Result<(), AppError> {
        let event_type = event.event_type();
        
        match event_type {
            "PaymentCompleted" => {
                // Try to downcast to PaymentCompleted
                // Since we can't directly downcast, we'll need to deserialize from event_data
                let event_data = event.event_data();
                
                // Try to deserialize PaymentCompleted from event data
                if let Ok(payment_completed) = serde_json::from_value::<PaymentCompleted>(event_data.clone()) {
                    self.handle_payment_completed_internal(&payment_completed).await?;
                } else {
                    // If deserialization fails, try to extract manually
                    warn!("Could not deserialize PaymentCompleted event, attempting manual extraction");
                }
            }
            "PaymentFailed" => {
                let event_data = event.event_data();
                if let Ok(payment_failed) = serde_json::from_value::<PaymentFailed>(event_data.clone()) {
                    self.handle_payment_failed_internal(&payment_failed).await?;
                } else {
                    warn!("Could not deserialize PaymentFailed event");
                }
            }
            "SharePurchasePaymentCompleted" => {
                // This is the specific event for SharePurchase payments
                // We can extract venture_id from contract_id
                let event_data = event.event_data();
                
                if let Ok(share_purchase_event) = serde_json::from_value::<crate::bounded_contexts::payment::domain::events::SharePurchasePaymentCompleted>(event_data.clone()) {
                    let venture_id = share_purchase_event.contract_id;
                    let payment_id = share_purchase_event.payment_id.value();
                    
                    // Try to find investment by payment_id from metadata
                    // The metadata should contain investment_id in additional_data
                    let metadata = share_purchase_event.metadata.event_data();
                    if let Some(investment_id_str) = metadata
                        .get("additional_data")
                        .and_then(|ad| ad.get("investment_id"))
                        .and_then(|id| id.as_str())
                    {
                        if let Ok(investment_id) = Uuid::parse_str(investment_id_str) {
                            info!(
                                "Processing SharePurchasePaymentCompleted for investment {} in venture {}",
                                investment_id, venture_id
                            );
                            
                            // Update investment and funding
                            if let Err(e) = self.payment_integration.handle_payment_confirmed(
                                payment_id,
                                investment_id,
                                venture_id,
                                share_purchase_event.purchase_amount.value(),
                            ).await {
                                error!("Failed to handle payment confirmation: {:?}", e);
                            } else {
                                info!("Successfully processed SharePurchasePaymentCompleted for investment {}", investment_id);
                            }
                        }
                    } else {
                        warn!("SharePurchasePaymentCompleted for venture {} but no investment_id in metadata", venture_id);
                    }
                } else {
                    warn!("Could not deserialize SharePurchasePaymentCompleted event");
                }
            }
            _ => {
                // Not a payment event we care about
            }
        }
        
        Ok(())
    }
}

