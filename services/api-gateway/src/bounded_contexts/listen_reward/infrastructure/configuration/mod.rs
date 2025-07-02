// Configuration module for Listen & Reward bounded context
//
// This module provides configuration and dependency injection
// for the Listen & Reward bounded context infrastructure components.

use std::sync::Arc;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use crate::bounded_contexts::listen_reward::{
    application::{
        ListenRewardApplicationService,
        use_cases::{
            StartListenSessionUseCase,
            CompleteListenSessionUseCase,
            ProcessRewardDistributionUseCase,
        },
    },
    infrastructure::{
        repositories::{
            PostgresListenSessionRepository,
            PostgresRewardDistributionRepository,
            PostgresRewardAnalyticsRepository,
        },
        event_publishers::EventPublisherFactory,
        external_services::MockZkProofVerificationService,
    },
    presentation::controllers::{
        ListenSessionController,
        RewardController,
        AnalyticsController,
        ListenRewardController,
    },
};

/// Configuración para el bounded context de Listen Reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenRewardConfig {
    /// Configuración de la base de datos
    pub database: DatabaseConfig,
    
    /// Configuración del servicio ZK Proof
    pub zk_proof: ZkProofConfig,
    
    /// Configuración de los publicadores de eventos
    pub event_publishers: EventPublishersConfig,
    
    /// Configuración de recompensas
    pub rewards: RewardsConfig,
}

/// Configuración de la base de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Cadena de conexión a PostgreSQL
    pub connection_string: String,
    
    /// Número máximo de conexiones en el pool
    pub max_connections: u32,
}

/// Configuración del servicio ZK Proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofConfig {
    /// URL del servicio ZK Proof
    pub service_url: String,
    
    /// Timeout en segundos para las solicitudes al servicio
    pub timeout_seconds: u64,
    
    /// Número máximo de reintentos
    pub max_retries: u32,
}

/// Configuración de los publicadores de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPublishersConfig {
    /// Tipo de publicador principal ("postgres", "redis_stream", etc.)
    pub primary_publisher: String,
    
    /// Configuración para el publicador de PostgreSQL
    pub postgres: Option<PostgresPublisherConfig>,
    
    /// Configuración para el publicador de Redis Stream
    pub redis_stream: Option<RedisStreamPublisherConfig>,
}

/// Configuración para el publicador de PostgreSQL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresPublisherConfig {
    /// Cadena de conexión a PostgreSQL (puede ser la misma que la de la base de datos principal)
    pub connection_string: String,
}

/// Configuración para el publicador de Redis Stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisStreamPublisherConfig {
    /// URL de conexión a Redis
    pub redis_url: String,
    
    /// Clave del stream para los eventos
    pub stream_key: String,
}

/// Configuración de recompensas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardsConfig {
    /// Duración mínima de escucha en segundos para recibir recompensa
    pub min_listen_duration_seconds: u32,
    
    /// Multiplicador de recompensa base
    pub base_reward_multiplier: f64,
    
    /// Multiplicadores por tier de usuario
    pub tier_multipliers: TierMultipliers,
    
    /// Límite diario de recompensas por usuario
    pub daily_reward_limit_per_user: f64,
}

/// Multiplicadores por tier de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMultipliers {
    pub basic: f64,
    pub premium: f64,
    pub vip: f64,
    pub artist: f64,
}

/// Proveedor de configuración
pub struct ConfigProvider {
    config: ListenRewardConfig,
}

impl ConfigProvider {
    /// Crea un nuevo proveedor de configuración
    pub fn new(config: ListenRewardConfig) -> Self {
        Self { config }
    }
    
    /// Obtiene la configuración completa
    pub fn get_config(&self) -> &ListenRewardConfig {
        &self.config
    }
    
    /// Crea un pool de conexiones a la base de datos
    pub async fn create_db_pool(&self) -> Result<PgPool, sqlx::Error> {
        // Crear el pool de conexiones de manera simple
        let pool = PgPool::connect(&self.config.database.connection_string).await?;
        Ok(pool)
    }
    
    /// Obtiene la configuración del publicador de eventos principal
    pub fn get_event_publisher_config(&self) -> Result<(String, serde_json::Value), String> {
        let publisher_type = &self.config.event_publishers.primary_publisher;
        
        let config = match publisher_type.as_str() {
            "postgres" => {
                let pg_config = self.config.event_publishers.postgres
                    .as_ref()
                    .ok_or("Postgres publisher configuration not found")?;
                
                serde_json::to_value(pg_config)
                    .map_err(|e| format!("Failed to serialize Postgres config: {}", e))?
            },
            "redis_stream" => {
                let redis_config = self.config.event_publishers.redis_stream
                    .as_ref()
                    .ok_or("Redis Stream publisher configuration not found")?;
                
                serde_json::to_value(redis_config)
                    .map_err(|e| format!("Failed to serialize Redis Stream config: {}", e))?
            },
            _ => return Err(format!("Unsupported event publisher type: {}", publisher_type)),
        };
        
        Ok((publisher_type.clone(), config))
    }
}

/// Configuración completa de infraestructura para Listen Reward
pub struct ListenRewardInfrastructureConfig {
    pub db_pool: PgPool,
    pub listen_session_repository: Arc<PostgresListenSessionRepository>,
    pub reward_distribution_repository: Arc<PostgresRewardDistributionRepository>,
    pub analytics_repository: Arc<PostgresRewardAnalyticsRepository>,
    pub zk_proof_service: Arc<MockZkProofVerificationService>,
    pub event_publisher: Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::event_publishers::EventPublisher>,
    pub application_service: Arc<ListenRewardApplicationService>,
    pub listen_session_controller: Arc<ListenSessionController>,
    pub reward_controller: Arc<RewardController>,
    pub analytics_controller: Arc<AnalyticsController>,
    pub listen_reward_controller: Arc<ListenRewardController>,
}

impl ListenRewardInfrastructureConfig {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Crear pool de base de datos
        let db_pool = PgPool::connect(database_url).await?;

        // Crear repositorios
        let listen_session_repository = Arc::new(PostgresListenSessionRepository::new(db_pool.clone()));
        let reward_distribution_repository = Arc::new(PostgresRewardDistributionRepository::new(db_pool.clone()));
        let analytics_repository = Arc::new(PostgresRewardAnalyticsRepository::new(db_pool.clone()));

        // Crear servicios externos
        let zk_proof_service = Arc::new(MockZkProofVerificationService::new_always_valid());

        // Crear event publisher
        let event_publisher_factory = EventPublisherFactory::new(db_pool.clone()).await?;
        let event_publisher1 = event_publisher_factory.create_postgres_publisher().await?;
        let event_publisher2 = event_publisher_factory.create_postgres_publisher().await?;

        // Crear use cases
        let start_session_use_case = Arc::new(StartListenSessionUseCase::new());
        let complete_session_use_case = Arc::new(CompleteListenSessionUseCase::new());
        let process_distribution_use_case = Arc::new(ProcessRewardDistributionUseCase::new());

        // Crear application service
        let application_service = Arc::new(ListenRewardApplicationService::new(
            start_session_use_case,
            complete_session_use_case,
            process_distribution_use_case,
            listen_session_repository.clone() as Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::ListenSessionRepository>,
            reward_distribution_repository.clone() as Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::RewardDistributionRepository>,
            analytics_repository.clone() as Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::RewardAnalyticsRepository>,
            Arc::from(event_publisher1),
            zk_proof_service.clone() as Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::external_services::ZkProofVerificationService>,
        ));

        // Crear controllers
        let listen_session_controller = Arc::new(ListenSessionController::new());
        
        let reward_controller = Arc::new(RewardController::new());
        
        let analytics_controller = Arc::new(AnalyticsController::new(
            application_service.clone(),
        ));
        
        let listen_reward_controller = Arc::new(ListenRewardController::new(
            application_service.clone(),
        ));

        Ok(Self {
            db_pool,
            listen_session_repository,
            reward_distribution_repository,
            analytics_repository,
            zk_proof_service,
            event_publisher: Arc::from(event_publisher2),
            application_service,
            listen_session_controller,
            reward_controller,
            analytics_controller,
            listen_reward_controller,
        })
    }
} 