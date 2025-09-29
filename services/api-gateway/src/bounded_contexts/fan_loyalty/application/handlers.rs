use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::{
    FanId, BiometricData, LoyaltyTier, WristbandType, WristbandId, BiometricScore,
    FanLoyaltyAggregate, Fan, NftWristband, VerificationResult, ValidationResult,
    FanLoyaltyEvent, EventGenerationService,
};
use crate::bounded_contexts::fan_loyalty::domain::repository::{
    FanRepository, NftWristbandRepository, FanLoyaltyRepository, 
    BiometricDataRepository, EventRepository, LoyaltyPointsRepository,
};
use crate::bounded_contexts::fan_loyalty::domain::services::{
    BiometricVerificationService, LoyaltyCalculationService, NftWristbandService,
};
use crate::bounded_contexts::fan_loyalty::application::{
    commands::*, queries::*,
};

/// Command handler for fan verification
pub struct VerifyFanCommandHandler {
    fan_repository: Box<dyn FanRepository + Send + Sync>,
    fan_loyalty_repository: Box<dyn FanLoyaltyRepository + Send + Sync>,
    biometric_data_repository: Box<dyn BiometricDataRepository + Send + Sync>,
    event_repository: Box<dyn EventRepository + Send + Sync>,
    biometric_service: BiometricVerificationService,
    event_service: EventGenerationService,
}

impl VerifyFanCommandHandler {
    pub fn new(
        fan_repository: Box<dyn FanRepository + Send + Sync>,
        fan_loyalty_repository: Box<dyn FanLoyaltyRepository + Send + Sync>,
        biometric_data_repository: Box<dyn BiometricDataRepository + Send + Sync>,
        event_repository: Box<dyn EventRepository + Send + Sync>,
    ) -> Self {
        Self {
            fan_repository,
            fan_loyalty_repository,
            biometric_data_repository,
            event_repository,
            biometric_service: BiometricVerificationService,
            event_service: EventGenerationService,
        }
    }
}

#[async_trait]
impl CommandHandler<VerifyFanCommand, VerificationResult> for VerifyFanCommandHandler {
    async fn handle(&self, command: VerifyFanCommand) -> Result<VerificationResult, String> {
        // Validate biometric data
        self.biometric_service.validate_biometric_data(&command.biometric_data)?;
        
        // Save biometric data
        self.biometric_data_repository.save(&command.fan_id, &command.biometric_data).await?;
        
        // Get or create fan loyalty aggregate
        let mut aggregate = self.fan_loyalty_repository
            .find_by_fan_id(&command.fan_id)
            .await?
            .unwrap_or_else(|| FanLoyaltyAggregate::new(command.fan_id.clone()));
        
        // Perform verification
        let result = aggregate.verify_fan(command.biometric_data)?;
        
        // Save updated aggregate
        self.fan_loyalty_repository.save(&aggregate).await?;
        
        // Generate and save events
        match &result {
            VerificationResult::Verified { fan_id, loyalty_tier, biometric_score } => {
                let event = self.event_service.generate_fan_verified_event(
                    fan_id.clone(),
                    loyalty_tier.clone(),
                    biometric_score.clone(),
                );
                self.event_repository.save_event(&event).await?;
            }
            VerificationResult::Failed { .. } => {
                let event = FanLoyaltyEvent::BiometricVerificationFailed {
                    fan_id: command.fan_id,
                    reason: "Insufficient biometric verification".to_string(),
                    timestamp: Utc::now(),
                };
                self.event_repository.save_event(&event).await?;
            }
        }
        
        Ok(result)
    }
}

/// Command handler for creating NFT wristbands
pub struct CreateNftWristbandCommandHandler {
    fan_repository: Box<dyn FanRepository + Send + Sync>,
    wristband_repository: Box<dyn NftWristbandRepository + Send + Sync>,
    event_repository: Box<dyn EventRepository + Send + Sync>,
    wristband_service: NftWristbandService,
    loyalty_service: LoyaltyCalculationService,
    event_service: EventGenerationService,
}

impl CreateNftWristbandCommandHandler {
    pub fn new(
        fan_repository: Box<dyn FanRepository + Send + Sync>,
        wristband_repository: Box<dyn NftWristbandRepository + Send + Sync>,
        event_repository: Box<dyn EventRepository + Send + Sync>,
    ) -> Self {
        Self {
            fan_repository,
            wristband_repository,
            event_repository,
            wristband_service: NftWristbandService,
            loyalty_service: LoyaltyCalculationService,
            event_service: EventGenerationService,
        }
    }
}

#[async_trait]
impl CommandHandler<CreateNftWristbandCommand, NftWristband> for CreateNftWristbandCommandHandler {
    async fn handle(&self, command: CreateNftWristbandCommand) -> Result<NftWristband, String> {
        // Check if fan exists
        let fan = self.fan_repository.find_by_id(&command.fan_id).await?
            .ok_or("Fan not found")?;
        
        // Check fan eligibility
        if !self.loyalty_service.qualifies_for_wristband(&fan.verification_level.into(), &command.wristband_type) {
            return Err("Fan is not eligible for this wristband type".to_string());
        }
        
        // Create wristband
        let wristband = self.wristband_service.create_wristband(
            command.fan_id.clone(),
            command.artist_id,
            command.concert_id,
            command.wristband_type.clone(),
        )?;
        
        // Save wristband
        self.wristband_repository.save(&wristband).await?;
        
        // Generate and save event
        let event = self.event_service.generate_nft_wristband_created_event(
            wristband.id.clone(),
            command.fan_id,
            command.artist_id,
            command.concert_id,
            command.wristband_type,
        );
        self.event_repository.save_event(&event).await?;
        
        Ok(wristband)
    }
}

/// Command handler for adding loyalty points
pub struct AddLoyaltyPointsCommandHandler {
    fan_repository: Box<dyn FanRepository + Send + Sync>,
    loyalty_points_repository: Box<dyn LoyaltyPointsRepository + Send + Sync>,
    event_repository: Box<dyn EventRepository + Send + Sync>,
}

impl AddLoyaltyPointsCommandHandler {
    pub fn new(
        fan_repository: Box<dyn FanRepository + Send + Sync>,
        loyalty_points_repository: Box<dyn LoyaltyPointsRepository + Send + Sync>,
        event_repository: Box<dyn EventRepository + Send + Sync>,
    ) -> Self {
        Self {
            fan_repository,
            loyalty_points_repository,
            event_repository,
        }
    }
}

#[async_trait]
impl CommandHandler<AddLoyaltyPointsCommand, u32> for AddLoyaltyPointsCommandHandler {
    async fn handle(&self, command: AddLoyaltyPointsCommand) -> Result<u32, String> {
        // Add points
        self.loyalty_points_repository.add_points(&command.fan_id, command.points).await?;
        
        // Get total points
        let total_points = self.loyalty_points_repository.get_points(&command.fan_id).await?;
        
        // Generate and save event
        let event = FanLoyaltyEvent::LoyaltyPointsAdded {
            fan_id: command.fan_id,
            points: command.points,
            total_points,
            timestamp: Utc::now(),
        };
        self.event_repository.save_event(&event).await?;
        
        Ok(total_points)
    }
}

/// Query handler for getting fan loyalty
pub struct GetFanLoyaltyQueryHandler {
    fan_repository: Box<dyn FanRepository + Send + Sync>,
    fan_loyalty_repository: Box<dyn FanLoyaltyRepository + Send + Sync>,
    loyalty_points_repository: Box<dyn LoyaltyPointsRepository + Send + Sync>,
}

impl GetFanLoyaltyQueryHandler {
    pub fn new(
        fan_repository: Box<dyn FanRepository + Send + Sync>,
        fan_loyalty_repository: Box<dyn FanLoyaltyRepository + Send + Sync>,
        loyalty_points_repository: Box<dyn LoyaltyPointsRepository + Send + Sync>,
    ) -> Self {
        Self {
            fan_repository,
            fan_loyalty_repository,
            loyalty_points_repository,
        }
    }
}

#[async_trait]
impl QueryHandler<GetFanLoyaltyQuery, FanLoyaltyResponse> for GetFanLoyaltyQueryHandler {
    async fn handle(&self, query: GetFanLoyaltyQuery) -> Result<FanLoyaltyResponse, String> {
        // Get fan
        let fan = self.fan_repository.find_by_id(&query.fan_id).await?
            .ok_or("Fan not found")?;
        
        // Get loyalty aggregate
        let aggregate = self.fan_loyalty_repository.find_by_fan_id(&query.fan_id).await?
            .ok_or("Fan loyalty not found")?;
        
        // Get loyalty points
        let points = self.loyalty_points_repository.get_points(&query.fan_id).await?;
        
        Ok(FanLoyaltyResponse {
            fan_id: query.fan_id,
            loyalty_tier: aggregate.loyalty_tier,
            biometric_score: aggregate.biometric_score,
            loyalty_points: points,
            verification_status: aggregate.verification_status,
        })
    }
}

/// Command handler trait
#[async_trait]
pub trait CommandHandler<C, R> {
    async fn handle(&self, command: C) -> Result<R, String>;
}

/// Query handler trait
#[async_trait]
pub trait QueryHandler<Q, R> {
    async fn handle(&self, query: Q) -> Result<R, String>;
}

/// Fan loyalty response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanLoyaltyResponse {
    pub fan_id: FanId,
    pub loyalty_tier: LoyaltyTier,
    pub biometric_score: BiometricScore,
    pub loyalty_points: u32,
    pub verification_status: crate::bounded_contexts::fan_loyalty::domain::aggregates::VerificationStatus,
}
