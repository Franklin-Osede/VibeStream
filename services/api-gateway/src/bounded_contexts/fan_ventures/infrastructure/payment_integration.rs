//! Payment Integration for Fan Ventures
//! 
//! Handles integration between Fan Ventures and Payment system.
//! Creates payments automatically when investments are made and updates
//! venture funding when payments are confirmed.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use tracing::{info, error, warn};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::{
    payment::{
        application::{
            commands::{InitiatePaymentCommand, PaymentPurposeDto, PaymentMetadataDto},
            handlers::command_handlers::PaymentCommandHandler,
        },
        domain::value_objects::Currency,
    },
    fan_ventures::{
        domain::entities::{FanInvestment, InvestmentStatus},
        infrastructure::postgres_repository::PostgresFanVenturesRepository,
    },
};

/// Service to integrate Fan Ventures with Payment system
pub struct FanVenturesPaymentIntegration {
    payment_handler: Arc<dyn PaymentCommandHandler>,
    venture_repository: Arc<PostgresFanVenturesRepository>,
}

impl FanVenturesPaymentIntegration {
    pub fn new(
        payment_handler: Arc<dyn PaymentCommandHandler>,
        venture_repository: Arc<PostgresFanVenturesRepository>,
    ) -> Self {
        Self {
            payment_handler,
            venture_repository,
        }
    }

    /// Create a payment for a venture investment
    pub async fn create_investment_payment(
        &self,
        investment: &FanInvestment,
        venture_id: Uuid,
        artist_id: Uuid,
    ) -> Result<Uuid, AppError> {
        info!(
            "Creating payment for investment {} in venture {}",
            investment.id, venture_id
        );

        // Create payment purpose - using SharePurchase as it's semantically similar
        // In the future, we could add a specific VentureInvestment purpose type
        let purpose = PaymentPurposeDto {
            purpose_type: "SharePurchase".to_string(), // Reusing SharePurchase for now
            campaign_id: None,
            nft_quantity: None,
            contract_id: Some(venture_id), // Using venture_id as contract_id
            ownership_percentage: None, // Could calculate based on investment amount
            share_id: None,
            from_user: None,
            to_user: None,
            song_id: None,
            artist_id: Some(artist_id),
            session_id: None,
            listen_duration: None,
            distribution_id: None,
            original_payment_id: None,
            reason: None,
        };

        // Create payment metadata with investment details
        let metadata = PaymentMetadataDto {
            user_ip: None,
            user_agent: None,
            platform_version: env!("CARGO_PKG_VERSION").to_string(),
            reference_id: Some(investment.id.to_string()),
            additional_data: serde_json::json!({
                "investment_id": investment.id,
                "venture_id": venture_id,
                "investment_type": format!("{:?}", investment.investment_type),
            }),
        };

        // Create payment command
        let command = InitiatePaymentCommand {
            payer_id: investment.fan_id,
            payee_id: artist_id,
            amount_value: investment.investment_amount,
            amount_currency: Currency::USD, // TODO: Make configurable
            payment_method: crate::bounded_contexts::payment::application::commands::PaymentMethodDto {
                method_type: "PlatformBalance".to_string(), // Default for now
                card_details: None,
                crypto_details: None,
                bank_details: None,
            },
            purpose,
            metadata,
            idempotency_key: Some(format!("venture_investment_{}", investment.id)),
        };

        // Initiate payment
        let result = self.payment_handler.handle_initiate_payment(command).await?;

        info!(
            "Payment {} created for investment {} in venture {}",
            result.payment_id, investment.id, venture_id
        );

        Ok(result.payment_id)
    }

    /// Update investment and venture funding when payment is confirmed
    pub async fn handle_payment_confirmed(
        &self,
        payment_id: Uuid,
        investment_id: Uuid,
        venture_id: Uuid,
        amount: f64,
    ) -> Result<(), AppError> {
        info!(
            "Payment {} confirmed for investment {} in venture {}",
            payment_id, investment_id, venture_id
        );

        // Get investment
        let investments = self.venture_repository.get_fan_investments_by_venture(venture_id).await?;
        let investment = investments
            .iter()
            .find(|inv| inv.id == investment_id)
            .ok_or_else(|| AppError::NotFound(format!("Investment {} not found", investment_id)))?;

        // Update investment status to Active
        let mut updated_investment = investment.clone();
        updated_investment.status = InvestmentStatus::Active;
        updated_investment.updated_at = Utc::now();

        // Update investment status
        self.venture_repository.update_fan_investment(&updated_investment).await?;

        // Update venture funding
        let mut venture = self.venture_repository.get_venture(venture_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Venture {} not found", venture_id)))?;

        venture.current_funding += amount;
        venture.updated_at = Utc::now();

        self.venture_repository.create_venture(&venture).await?;

        info!(
            "Updated venture {} funding to ${} after payment confirmation",
            venture_id, venture.current_funding
        );

        Ok(())
    }

    /// Handle payment failure - revert investment
    pub async fn handle_payment_failed(
        &self,
        payment_id: Uuid,
        investment_id: Uuid,
        venture_id: Uuid,
    ) -> Result<(), AppError> {
        warn!(
            "Payment {} failed for investment {} in venture {}",
            payment_id, investment_id, venture_id
        );

        // Update investment status to Cancelled
        let investments = self.venture_repository.get_fan_investments_by_venture(venture_id).await?;
        let investment = investments
            .iter()
            .find(|inv| inv.id == investment_id)
            .ok_or_else(|| AppError::NotFound(format!("Investment {} not found", investment_id)))?;

        let mut updated_investment = investment.clone();
        updated_investment.status = InvestmentStatus::Cancelled;
        updated_investment.updated_at = Utc::now();

        self.venture_repository.update_fan_investment(&updated_investment).await?;

        info!(
            "Cancelled investment {} due to payment failure",
            investment_id
        );

        Ok(())
    }
}

