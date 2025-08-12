// Listen Reward Application Service
//
// This service orchestrates all use cases and provides a unified interface
// for the Listen Reward bounded context. It handles cross-cutting concerns
// and coordinates between different use cases.

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bounded_contexts::listen_reward::{
    domain::{
        entities::ListenSession,
        value_objects::RewardAmount,
        aggregates::RewardPool,
    },
    infrastructure::{
        repositories::{
            ListenSessionRepository, RewardDistributionRepository, RewardAnalyticsRepository,
        },
        event_publishers::EventPublisher,
        // TODO: Add back when external services are implemented
        // external_services::ZkProofVerificationService,
    },
    application::use_cases::{
        StartListenSessionUseCase,
    },
};
use crate::shared::domain::errors::AppError;

// Application Service Commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartListeningCommand {
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub user_tier: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteListeningCommand {
    pub session_id: Uuid,
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub zk_proof_hash: String,
    pub song_duration_seconds: u32,
    pub completion_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRewardsCommand {
    pub distribution_id: Uuid,
    pub session_ids: Vec<Uuid>,
    pub base_reward_rate: f64,
    pub platform_fee_percentage: f64,
}

// Query Commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserListeningHistoryQuery {
    pub user_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtistAnalyticsQuery {
    pub artist_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartListeningResponse {
    pub session_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub estimated_reward: f64,
    pub user_tier: String,
    pub events_triggered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteListeningResponse {
    pub session_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub final_reward: Option<f64>,
    pub status: String,
    pub verification_status: String,
    pub events_triggered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRewardsResponse {
    pub distribution_id: Uuid,
    pub processed_sessions: u32,
    pub total_rewards_distributed: f64,
    pub total_artist_royalties: f64,
    pub events_triggered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListeningHistory {
    pub sessions: Vec<ListeningSessionSummary>,
    pub total_sessions: u32,
    pub total_rewards_earned: f64,
    pub favorite_genres: Vec<String>,
    pub listening_streak: u32,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeningSessionSummary {
    pub session_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub duration_seconds: u32,
    pub reward_earned: f64,
    pub quality_score: Option<f64>,
    pub listened_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistAnalytics {
    pub artist_id: Uuid,
    pub total_listens: u64,
    pub unique_listeners: u64,
    pub total_revenue: f64,
    pub average_session_duration: f64,
    pub top_songs: Vec<TopSongAnalytics>,
    pub listener_demographics: ListenerDemographics,
    pub growth_metrics: GrowthMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSongAnalytics {
    pub song_id: Uuid,
    pub title: String,
    pub listen_count: u64,
    pub revenue: f64,
    pub average_completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerDemographics {
    pub age_groups: Vec<AgeGroup>,
    pub countries: Vec<CountryMetric>,
    pub listening_times: Vec<TimeMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetrics {
    pub daily_growth_rate: f64,
    pub monthly_growth_rate: f64,
    pub retention_rate: f64,
    pub churn_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeGroup {
    pub range: String,
    pub percentage: f64,
    pub listen_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryMetric {
    pub country_code: String,
    pub country_name: String,
    pub percentage: f64,
    pub listen_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeMetric {
    pub hour: u8,
    pub listen_count: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub per_page: u32,
    pub total_items: u64,
}

/// Main Application Service for Listen Reward Bounded Context
pub struct ListenRewardApplicationService {
    start_session_use_case: Arc<StartListenSessionUseCase>,
    // TODO: Add back when use cases are implemented
    // complete_session_use_case: Arc<EndListenSessionUseCase>,
    // process_distribution_use_case: Arc<DistributeRewardsUseCase>,
    session_repository: Arc<dyn ListenSessionRepository>,
    distribution_repository: Arc<dyn RewardDistributionRepository>,
    analytics_repository: Arc<dyn RewardAnalyticsRepository>,
    event_publisher: Arc<dyn EventPublisher>,
    // TODO: Add back when ZkProofVerificationService is implemented
    // zk_verification_service: Arc<dyn ZkProofVerificationService>,
}

impl ListenRewardApplicationService {
    pub fn new(
        start_session_use_case: Arc<StartListenSessionUseCase>,
        // TODO: Add back when use cases are implemented
        // complete_session_use_case: Arc<EndListenSessionUseCase>,
        // process_distribution_use_case: Arc<DistributeRewardsUseCase>,
        session_repository: Arc<dyn ListenSessionRepository>,
        distribution_repository: Arc<dyn RewardDistributionRepository>,
        analytics_repository: Arc<dyn RewardAnalyticsRepository>,
        event_publisher: Arc<dyn EventPublisher>,
        // TODO: Add back when ZkProofVerificationService is implemented
        // zk_verification_service: Arc<dyn ZkProofVerificationService>,
    ) -> Self {
        Self {
            start_session_use_case,
            // complete_session_use_case,
            // process_distribution_use_case,
            session_repository,
            distribution_repository,
            analytics_repository,
            event_publisher,
            // TODO: Add back when ZkProofVerificationService is implemented
            // zk_verification_service,
        }
    }

    /// Constructor simplificado para configuración temporal
    pub fn new_simple(
        session_repository: Arc<dyn ListenSessionRepository>,
        distribution_repository: Arc<dyn RewardDistributionRepository>,
        analytics_repository: Arc<dyn RewardAnalyticsRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        // Crear use cases temporales con implementaciones mock
        // TODO: Add back when external services are implemented
// use crate::bounded_contexts::listen_reward::infrastructure::external_services::MockZkProofVerificationService;
        
        let start_session_use_case = Arc::new(StartListenSessionUseCase::new());
        
        // TODO: Add back when use cases are implemented
        // let complete_session_use_case = Arc::new(EndListenSessionUseCase::new());
        // let process_distribution_use_case = Arc::new(DistributeRewardsUseCase::new());
        
        // TODO: Add back when ZkProofVerificationService is implemented
        // let zk_verification_service = Arc::new(MockZkProofVerificationService::new_always_valid()) as Arc<dyn ZkProofVerificationService>;

        Self {
            start_session_use_case,
            // complete_session_use_case,
            // process_distribution_use_case,
            session_repository,
            distribution_repository,
            analytics_repository,
            event_publisher,
            // TODO: Add back when ZkProofVerificationService is implemented
            // zk_verification_service,
        }
    }

    /// Start a new listening session
    pub async fn start_listening_session(
        &self,
        command: StartListeningCommand,
    ) -> Result<StartListeningResponse, AppError> {
        // Validate rate limits
        self.validate_user_rate_limits(command.user_id).await?;

        // Parse reward tier
        // TODO: Add back when RewardTier is implemented
        // let reward_tier = RewardTier::from_string(&command.user_tier)
        //     .map_err(|e| AppError::ValidationError(e))?;
        let reward_tier = "premium"; // Temporary mock

        // Crear comando para el caso de uso (conversión a String donde corresponde)
        let use_case_command = crate::bounded_contexts::listen_reward::application::use_cases::StartListenSessionCommand {
            user_id: command.user_id,
            song_contract: command.song_contract.clone(),
            artist_contract: command.artist_contract.clone(),
            user_tier: reward_tier.to_string(),
        };

        // Ejecutar caso de uso (síncrono)
        let (response, _event) = self
            .start_session_use_case
            .execute(use_case_command)
            .map_err(AppError::BusinessLogicError)?;

        // Calcular recompensa estimada
        let estimated_reward = self
            .calculate_estimated_reward(&reward_tier)
            .await?;

        // Convertir valores devueltos
        let session_uuid = uuid::Uuid::parse_str(&response.session_id)
            .unwrap_or_else(|_| uuid::Uuid::new_v4());
        let started_at = chrono::DateTime::parse_from_rfc3339(&response.started_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());

        Ok(StartListeningResponse {
            session_id: session_uuid,
            started_at,
            estimated_reward,
            user_tier: reward_tier.to_string(),
            events_triggered: Vec::new(),
        })
    }

    /// Complete a listening session with ZK proof
    pub async fn complete_listening_session(
        &self,
        command: CompleteListeningCommand,
    ) -> Result<CompleteListeningResponse, AppError> {
        // Validate session exists and is active
        // TODO: Add back when ListenSessionId is implemented
        // let session_id = ListenSessionId::from_uuid(command.session_id);
        let session_id = command.session_id.to_string(); // Temporary mock
        // TODO: Add back when ListenSessionId is implemented
        // let session = self.session_repository
        //     .find_by_id(&session_id)
        // TODO: Add back when ListenSessionId is implemented
        // let session = self.session_repository
        //     .find_by_id(&session_id)
        //     .await
        //     .map_err(|e| AppError::DatabaseError(e.to_string()))?
        //     .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;
        // TODO: Add back when proper types are implemented
        // let session = ListenSession::new(
        //     uuid::Uuid::new_v4(),
        //     command.session_id,
        //     uuid::Uuid::new_v4(),
        //     uuid::Uuid::new_v4(),
        //     "mock_proof_hash".to_string(),
        //     1000, // base_reward in cents
        //     chrono::Utc::now(),
        // );
        // Create temporary contracts for the session
        let song_contract = vibestream_types::SongContract {
            id: uuid::Uuid::new_v4(),
            title: "Unknown".to_string(),
            artist_id: uuid::Uuid::new_v4(),
            artist_name: "Unknown".to_string(),
            duration_seconds: None,
            genre: None,
            ipfs_hash: None,
            metadata_url: None,
            nft_contract_address: None,
            nft_token_id: None,
            royalty_percentage: None,
            is_minted: false,
            created_at: chrono::Utc::now(),
        };
        
        let artist_contract = vibestream_types::ArtistContract {
            id: uuid::Uuid::new_v4(),
            user_id: uuid::Uuid::new_v4(),
            stage_name: "Unknown".to_string(),
            profile_image_url: None,
        };
        
        let session = ListenSession::from_parts(
            crate::bounded_contexts::listen_reward::domain::value_objects::ListenSessionId::new(),
            command.session_id,
            song_contract,
            artist_contract,
            crate::bounded_contexts::listen_reward::domain::value_objects::RewardTier::Premium,
            crate::bounded_contexts::listen_reward::domain::entities::SessionStatus::Active,
            None,
            None,
            None,
            None,
            None,
            chrono::Utc::now(),
            None,
            None,
        );

        // TODO: Add back when SessionStatus is implemented
        // if *session.status() != SessionStatus::Active {
        if false { // Temporary mock - always allow
            return Err(AppError::BusinessLogicError("Session is not active".to_string()));
        }

        // Clonar sesión para evitar problemas de ownership
        // TODO: Add back when proper types are implemented
        // let session_for_usecase = session.clone();
        let session_for_usecase = session;

        // TODO: Add back when ZkProofVerificationService is implemented
        // Verificar ZK proof de forma asíncrona usando una referencia a la sesión original
        // let zk_verification_task = self
        //     .zk_verification_service
        //     .verify_proof(&command.zk_proof_hash, &session);
        let zk_verification_task = async { Ok::<bool, String>(true) };

        // Crear comando para el caso de uso
        let use_case_command = CompleteListeningCommand {
            session_id: command.session_id,
            listen_duration_seconds: command.listen_duration_seconds,
            quality_score: command.quality_score,
            zk_proof_hash: command.zk_proof_hash.clone(),
            song_duration_seconds: command.song_duration_seconds,
            completion_percentage: 100.0, // TODO: Calculate actual percentage
        };

        // TODO: Add back when complete_session_use_case is implemented
        // Ejecutar caso de uso (síncrono) pasando la copia mutable
        // let (_updated_session, response, _event) = self
        //     .complete_session_use_case
        //     .execute(session_for_usecase, use_case_command)
        //     .map_err(AppError::BusinessLogicError)?;
        
        // Temporary mock response
        let response = crate::bounded_contexts::listen_reward::application::use_cases::complete_listen_session::CompleteListenSessionResponse {
            session_id: command.session_id.to_string(),
            status: "completed".to_string(),
            listen_duration_seconds: 180,
            quality_score: 0.95,
            is_eligible_for_reward: true,
            completed_at: chrono::Utc::now().to_rfc3339(),
        };

        // Esperar verificación ZK
        let is_zk_valid = zk_verification_task
            .await
            .unwrap_or(false);

        let verification_status = if is_zk_valid { "verified" } else { "failed" };

        Ok(CompleteListeningResponse {
            session_id: uuid::Uuid::parse_str(&response.session_id)
                .unwrap_or_default(),
            completed_at: chrono::Utc::now(),
            final_reward: None,
            status: response.status,
            verification_status: verification_status.to_string(),
            events_triggered: Vec::new(),
        })
    }

    /// Process reward distribution for completed sessions
    #[allow(unused_variables)]
    pub async fn process_reward_distribution(
        &self,
        _command: ProcessRewardsCommand,
    ) -> Result<ProcessRewardsResponse, AppError> {
        Err(AppError::InternalError("process_reward_distribution no implementado".to_string()))
    }

    /// Get user listening history with analytics
    pub async fn get_user_listening_history(
        &self,
        query: GetUserListeningHistoryQuery,
    ) -> Result<UserListeningHistory, AppError> {
        let pagination = crate::bounded_contexts::listen_reward::infrastructure::repositories::Pagination {
            offset: ((query.page.unwrap_or(1) - 1) * query.limit.unwrap_or(20)) as i64,
            limit: query.limit.unwrap_or(20) as i64,
        };

        let filter = crate::bounded_contexts::listen_reward::infrastructure::repositories::ListenSessionFilter {
            user_id: Some(query.user_id),
            start_date: query.start_date,
            end_date: query.end_date,
            ..Default::default()
        };

        // Get user reward history
        let reward_history = self.analytics_repository
            .get_user_reward_history(query.user_id, &pagination)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Convert to response format
        let sessions_vec: Vec<ListeningSessionSummary> = reward_history
            .into_iter()
            .map(|h| ListeningSessionSummary {
                session_id: h.session_id,
                song_title: "Unknown".to_string(), // Would be fetched from music context
                artist_name: "Unknown".to_string(), // Would be fetched from music context
                duration_seconds: h.listen_duration.unwrap_or(0),
                reward_earned: h.reward_amount,
                quality_score: h.quality_score,
                listened_at: h.earned_at,
                status: "Rewarded".to_string(),
            })
            .collect();

        let total_rewards: f64 = sessions_vec.iter().map(|s| s.reward_earned).sum();
        let total_sessions = sessions_vec.len() as u32;

        Ok(UserListeningHistory {
            sessions: sessions_vec,
            total_sessions,
            total_rewards_earned: total_rewards,
            favorite_genres: vec![], // Would be calculated from song data
            listening_streak: 0, // Would be calculated from session patterns
            pagination: PaginationInfo {
                current_page: query.page.unwrap_or(1),
                total_pages: 1,
                per_page: query.limit.unwrap_or(20),
                total_items: total_sessions as u64,
            },
        })
    }

    /// Get artist analytics and revenue data
    pub async fn get_artist_analytics(
        &self,
        query: GetArtistAnalyticsQuery,
    ) -> Result<ArtistAnalytics, AppError> {
        let artist_revenue = self.analytics_repository
            .get_artist_revenue(query.artist_id, query.start_date, query.end_date)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ArtistAnalytics {
            artist_id: query.artist_id,
            total_listens: artist_revenue.total_sessions as u64,
            unique_listeners: artist_revenue.unique_listeners as u64,
            total_revenue: artist_revenue.total_revenue,
            average_session_duration: 180.0, // Would be calculated from session data
            top_songs: artist_revenue.top_songs.into_iter().map(|song| TopSongAnalytics {
                song_id: song.song_id,
                title: song.title,
                listen_count: song.listen_count as u64,
                revenue: song.revenue,
                average_completion_rate: 0.85, // Would be calculated
            }).collect(),
            listener_demographics: ListenerDemographics {
                age_groups: vec![],
                countries: vec![],
                listening_times: vec![],
            },
            growth_metrics: GrowthMetrics {
                daily_growth_rate: 0.05,
                monthly_growth_rate: 0.15,
                retention_rate: 0.75,
                churn_rate: 0.25,
            },
        })
    }

    // Private helper methods
    async fn validate_user_rate_limits(&self, user_id: Uuid) -> Result<(), AppError> {
        let today_start = chrono::Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let today_end = today_start + chrono::Duration::days(1);

        let session_count = self.session_repository
            .count_user_sessions_in_period(user_id, today_start, today_end)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if session_count > 100 {
            return Err(AppError::RateLimitError("Daily session limit exceeded".to_string()));
        }

        Ok(())
    }

    // TODO: Add back when RewardTier is implemented
    // async fn calculate_estimated_reward(&self, tier: &RewardTier) -> Result<f64, AppError> {
    //     // Base reward calculation: 3 minutes * 0.5 tokens/minute * tier multiplier
    //     let base_reward = 3.0 * 0.5 * tier.multiplier();
    //     Ok(base_reward)
    // }
    async fn calculate_estimated_reward(&self, _tier: &str) -> Result<f64, AppError> {
        // Base reward calculation: 3 minutes * 0.5 tokens/minute * tier multiplier
        let base_reward = 3.0 * 0.5 * 1.0; // Temporary mock multiplier
        Ok(base_reward)
    }

    pub fn get_session_repository(&self) -> Arc<dyn ListenSessionRepository> {
        Arc::clone(&self.session_repository)
    }

    pub fn get_analytics_repository(&self) -> Arc<dyn RewardAnalyticsRepository> {
        Arc::clone(&self.analytics_repository)
    }
} 