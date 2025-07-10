/// Integration module for Listen Reward bounded context
/// 
/// This module handles cross-bounded-context communications,
/// particularly with Fractional Ownership for revenue distribution.

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::{
    fractional_ownership::{
        application::{
            commands::DistributeRevenue,
            queries::{GetOwnershipContract, GetOwnershipContractResult},
        },
        integration_service::InMemoryFractionalOwnershipBoundedContext,
        infrastructure::InMemoryOwnershipContractRepository,
    },
    listen_reward::domain::{
        events::ListenSessionCompleted,
        value_objects::RewardAmount,
    },
};

/// Integration event handler that processes listen sessions
/// and triggers revenue distribution to fractional owners
#[derive(Debug, Clone)]
pub struct FractionalOwnershipIntegrationHandler {
    fractional_ownership_context: Arc<InMemoryFractionalOwnershipBoundedContext>,
}

impl FractionalOwnershipIntegrationHandler {
    pub fn new(fractional_ownership_context: Arc<InMemoryFractionalOwnershipBoundedContext>) -> Self {
        Self {
            fractional_ownership_context,
        }
    }

    /// Handles listen session completion and distributes rewards to fractional owners
    pub async fn handle_listen_session_completed(
        &self,
        event: &ListenSessionCompleted,
    ) -> Result<Option<RevenueDistributionTriggered>, AppError> {
        use crate::bounded_contexts::fractional_ownership::application::queries::GetOwnershipContractBySongId;
        
        // 1. Check if the song has fractional ownership
        let song_id_uuid = *event.song_id.value();

        let get_contract_query = GetOwnershipContractBySongId {
            song_id: song_id_uuid,
        };

        // Try to get ownership contract for this song
        let contract_result = match self.fractional_ownership_context
            .get_application_service()
            .get_ownership_contract_by_song_id(get_contract_query)
            .await
        {
            Ok(result) => result,
            Err(AppError::NotFound(_)) => {
                // No fractional ownership for this song, skip distribution
                return Ok(None);
            }
            Err(e) => return Err(e),
        };

        // 2. Calculate revenue amounts
        let total_reward_amount = event.quality_score.score(); // Use score() method

        // 3. Calculate revenue percentage for fractional owners
        let fractional_owner_percentage = 100.0 - contract_result.artist_retained_percentage;
        let fractional_revenue = total_reward_amount * fractional_owner_percentage / 100.0;

        // Only distribute if there's meaningful revenue (min $0.01)
        if fractional_revenue < 0.01 {
            return Ok(None);
        }

        // 4. Trigger revenue distribution
        let distribute_command = DistributeRevenue {
            contract_id: contract_result.contract_id,
            total_revenue: fractional_revenue,
            distribution_period_start: chrono::Utc::now() - chrono::Duration::hours(1),
            distribution_period_end: chrono::Utc::now(),
            platform_fee_percentage: 5.0,
        };

        let distribution_result = self.fractional_ownership_context
            .get_application_service()
            .distribute_revenue(distribute_command)
            .await?;

        // 5. Return success event
        Ok(Some(RevenueDistributionTriggered {
            session_id: event.session_id.value(),
            song_id: song_id_uuid,
            contract_id: contract_result.contract_id,
            total_distributed: fractional_revenue,
            distribution_id: distribution_result.distribution_id,
            shareholder_count: distribution_result.shareholder_count,
            triggered_at: chrono::Utc::now(),
        }))
    }

    /// Calculate how much of the streaming revenue should go to fractional owners vs artist
    pub async fn calculate_revenue_split(
        &self,
        song_id: Uuid,
        total_revenue: f64,
    ) -> Result<RevenueSplit, AppError> {
        use crate::bounded_contexts::fractional_ownership::application::queries::GetOwnershipContractBySongId;
        
        let get_contract_query = GetOwnershipContractBySongId {
            song_id,
        };

        // Try to get ownership contract for this song
        match self.fractional_ownership_context
            .get_application_service()
            .get_ownership_contract_by_song_id(get_contract_query)
            .await
        {
            Ok(contract_result) => {
                // Contract exists - calculate split
                let artist_retained_percentage = contract_result.artist_retained_percentage;
                let fractional_owners_percentage = 100.0 - artist_retained_percentage;
                
                let artist_share = total_revenue * artist_retained_percentage / 100.0;
                let fractional_owners_share = total_revenue * fractional_owners_percentage / 100.0;

                Ok(RevenueSplit {
                    total_revenue,
                    artist_share,
                    fractional_owners_share,
                    artist_retained_percentage,
                    has_fractional_ownership: true,
                })
            }
            Err(AppError::NotFound(_)) => {
                // No fractional ownership - artist gets everything
                Ok(RevenueSplit {
                    total_revenue,
                    artist_share: total_revenue,
                    fractional_owners_share: 0.0,
                    artist_retained_percentage: 100.0,
                    has_fractional_ownership: false,
                })
            }
            Err(e) => Err(e),
        }
    }
}

/// Event emitted when revenue distribution is triggered for fractional owners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistributionTriggered {
    pub session_id: Uuid,
    pub song_id: Uuid,
    pub contract_id: Uuid,
    pub total_distributed: f64,
    pub distribution_id: Uuid,
    pub shareholder_count: u32,
    pub triggered_at: DateTime<Utc>,
}

/// Revenue split calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSplit {
    pub total_revenue: f64,
    pub artist_share: f64,
    pub fractional_owners_share: f64,
    pub artist_retained_percentage: f64,
    pub has_fractional_ownership: bool,
}

/// Metadata included with revenue distributions from streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueMetadata {
    pub listen_session_id: Uuid,
    pub original_listener_id: Uuid,
    pub song_id: Uuid,
    pub listen_duration_seconds: Option<u32>,
    pub quality_score: Option<f64>,
}

/// Factory for creating integration handlers
pub struct ListenRewardIntegrationFactory;

impl ListenRewardIntegrationFactory {
    /// Create fractional ownership integration handler
    pub fn create_fractional_ownership_handler(
        fractional_ownership_context: Arc<InMemoryFractionalOwnershipBoundedContext>,
    ) -> FractionalOwnershipIntegrationHandler {
        FractionalOwnershipIntegrationHandler::new(fractional_ownership_context)
    }
}

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Minimum revenue amount to trigger distribution (prevents micro-transactions)
    pub minimum_distribution_amount: f64,
    /// Whether to enable real-time distribution or batch processing
    pub real_time_distribution: bool,
    /// Batch size for processing distributions
    pub batch_size: u32,
    /// Retry attempts for failed distributions
    pub retry_attempts: u32,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            minimum_distribution_amount: 0.01,
            real_time_distribution: true,
            batch_size: 100,
            retry_attempts: 3,
        }
    }
} 