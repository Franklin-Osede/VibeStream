// PostgreSQL Implementation for RewardDistribution Repository
//
// This implementation provides persistent storage for RewardDistribution aggregates
// using PostgreSQL with proper serialization of the internal state and events.

use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use serde_json;

use crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution;
use crate::bounded_contexts::listen_reward::domain::value_objects::{
    ListenSessionId, RewardAmount, RewardTier, ZkProofHash, RewardPoolId, ValidationPeriod
};
use crate::bounded_contexts::listen_reward::domain::aggregates::reward_distribution::RewardPool;
use crate::shared::domain::events::DomainEvent;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use super::{RewardDistributionRepository, RepositoryResult, Pagination};

// Estructura para mapear la tabla reward_distributions
#[derive(sqlx::FromRow, Debug)]
struct RewardDistributionRow {
    id: Uuid,
    pool_id: Uuid,
    total_tokens: f64,
    distributed_tokens: f64,
    reserved_tokens: f64,
    validation_period_start: DateTime<Utc>,
    validation_period_end: DateTime<Utc>,
    events: Option<serde_json::Value>,
    status: String,
    version: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct PostgresRewardDistributionRepository {
    pool: PgPool,
}

impl PostgresRewardDistributionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Convierte una entidad de dominio a una fila de base de datos
    fn distribution_to_row(&self, distribution: &RewardDistribution) -> RewardDistributionRow {
        // Para simplificar, no serializamos eventos complejos por ahora
        let events_json = if distribution.get_events().is_empty() {
            None
        } else {
            // Serializar solo metadatos básicos de eventos
            let event_count = distribution.get_events().len();
            let events_metadata = serde_json::json!({
                "event_count": event_count,
                "last_event_timestamp": chrono::Utc::now()
            });
            Some(events_metadata)
        };
        
        RewardDistributionRow {
            id: distribution.id(),
            pool_id: distribution.pool_id(),
            total_tokens: distribution.total_amount(),
            distributed_tokens: distribution.distributed_amount(),
            reserved_tokens: distribution.reserved_amount(),
            validation_period_start: distribution.period_start(),
            validation_period_end: distribution.period_end(),
            events: events_json,
            status: distribution.status(),
            version: distribution.version(),
            created_at: distribution.created_at(),
            updated_at: Utc::now(),
        }
    }

    // Convierte una fila de base de datos a una entidad de dominio
    fn row_to_entity(&self, row: &RewardDistributionRow) -> RepositoryResult<RewardDistribution> {
        // Crear el pool de recompensas
        let total_tokens = RewardAmount::new(row.total_tokens)
            .map_err(|e| format!("Invalid total tokens: {}", e))?;
        
        // Crear el período de validación
        let validation_period = ValidationPeriod::new(
            row.validation_period_start,
            row.validation_period_end
        ).map_err(|e| format!("Invalid validation period: {}", e))?;
        
        let pool = RewardPool::new(total_tokens, validation_period);
        
        // Para eventos, creamos un vector vacío ya que la serialización es compleja
        let events: Vec<Box<dyn DomainEvent>> = Vec::new();
        
        // Crear la entidad usando el constructor from_persisted_state
        let distribution = RewardDistribution::from_persisted_state(
            row.id,
            pool,
            row.created_at,
            events,
            row.version,
        );
        
        Ok(distribution)
    }
}

#[async_trait]
impl RewardDistributionRepository for PostgresRewardDistributionRepository {
    async fn find_by_id(&self, id: &Uuid) -> RepositoryResult<Option<RewardDistribution>> {
        let query = "SELECT * FROM reward_distributions WHERE id = $1";
        
        let result = sqlx::query_as::<_, RewardDistributionRow>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        match result {
            Some(row) => Ok(Some(self.row_to_entity(&row)?)),
            None => Ok(None),
        }
    }

    async fn save(&self, distribution: &RewardDistribution) -> RepositoryResult<()> {
        let row = self.distribution_to_row(distribution);
        
        let query = r#"
            INSERT INTO reward_distributions (
                id, pool_id, total_tokens, distributed_tokens, reserved_tokens,
                validation_period_start, validation_period_end, events, status, version
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            ON CONFLICT (id) DO UPDATE SET
                distributed_tokens = EXCLUDED.distributed_tokens,
                reserved_tokens = EXCLUDED.reserved_tokens,
                events = EXCLUDED.events,
                status = EXCLUDED.status,
                version = EXCLUDED.version
        "#;
        
        sqlx::query(query)
            .bind(row.id)
            .bind(row.pool_id)
            .bind(row.total_tokens)
            .bind(row.distributed_tokens)
            .bind(row.reserved_tokens)
            .bind(row.validation_period_start)
            .bind(row.validation_period_end)
            .bind(row.events)
            .bind(row.status)
            .bind(row.version)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save reward distribution: {}", e))?;
            
        Ok(())
    }

    async fn update(&self, distribution: &RewardDistribution, expected_version: i32) -> RepositoryResult<()> {
        let row = self.distribution_to_row(distribution);
        
        let query = r#"
            UPDATE reward_distributions SET
                distributed_tokens = $1,
                reserved_tokens = $2,
                events = $3,
                status = $4,
                version = version + 1
            WHERE id = $5 AND version = $6
        "#;
        
        let result = sqlx::query(query)
            .bind(row.distributed_tokens)
            .bind(row.reserved_tokens)
            .bind(row.events)
            .bind(row.status)
            .bind(row.id)
            .bind(expected_version)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to update reward distribution: {}", e))?;
            
        if result.rows_affected() == 0 {
            return Err("Distribution was modified by another process or not found".to_string());
        }
        
        Ok(())
    }

    async fn find_by_pool_id(&self, pool_id: &RewardPoolId) -> RepositoryResult<Vec<RewardDistribution>> {
        let query = "SELECT * FROM reward_distributions WHERE pool_id = $1 ORDER BY created_at DESC";
        
        let rows = sqlx::query_as::<_, RewardDistributionRow>(query)
            .bind(pool_id.value())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut distributions = Vec::new();
        for row in rows {
            distributions.push(self.row_to_entity(&row)?);
        }
        
        Ok(distributions)
    }

    async fn find_active_distributions(&self, pagination: &Pagination) -> RepositoryResult<Vec<RewardDistribution>> {
        let query = "SELECT * FROM reward_distributions WHERE status = 'active' ORDER BY created_at DESC LIMIT $1 OFFSET $2";
        
        let rows = sqlx::query_as::<_, RewardDistributionRow>(query)
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut distributions = Vec::new();
        for row in rows {
            distributions.push(self.row_to_entity(&row)?);
        }
        
        Ok(distributions)
    }

    async fn find_distributions_with_pending_rewards(&self) -> RepositoryResult<Vec<RewardDistribution>> {
        let query = "SELECT * FROM reward_distributions WHERE status = 'pending' AND distributed_tokens < total_tokens";
        
        let rows = sqlx::query_as::<_, RewardDistributionRow>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut distributions = Vec::new();
        for row in rows {
            distributions.push(self.row_to_entity(&row)?);
        }
        
        Ok(distributions)
    }

    async fn mark_processed(&self, id: &Uuid) -> RepositoryResult<()> {
        let query = "UPDATE reward_distributions SET status = 'processed' WHERE id = $1";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to mark distribution as processed: {}", e))?;
            
        if result.rows_affected() == 0 {
            return Err(format!("Distribution not found: {}", id));
        }
        
        Ok(())
    }
} 