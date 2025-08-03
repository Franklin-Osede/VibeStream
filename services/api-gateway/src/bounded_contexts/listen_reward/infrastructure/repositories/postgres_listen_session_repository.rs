use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;

use crate::bounded_contexts::listen_reward::domain::{
    entities::{ListenSession, listen_session::SessionStatus},
    value_objects::{
        ListenSessionId, RewardAmount, RewardTier, ZkProofHash, ListenDuration, QualityScore
    },
};
// These imports are used in the file

use super::{
    ListenSessionRepository, ListenSessionQueryRepository,
    RepositoryResult, Pagination, ListenSessionFilter,
};

// Estructura para mapear la tabla listen_sessions
#[derive(sqlx::FromRow, Debug)]
struct ListenSessionRow {
    id: Uuid,
    user_id: Uuid,
    song_id: Uuid,
    artist_id: Uuid,
    user_tier: String,
    status: String,
    listen_duration_seconds: Option<i32>,
    quality_score: Option<f64>,
    zk_proof_hash: Option<String>,
    base_reward_tokens: Option<f64>,
    final_reward_tokens: Option<f64>,
    started_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    verified_at: Option<DateTime<Utc>>,
    version: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct PostgresListenSessionRepository {
    pool: PgPool,
}

impl PostgresListenSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Convierte una entidad de dominio a una fila de base de datos
    fn session_to_row(&self, session: &ListenSession) -> ListenSessionRow {
        ListenSessionRow {
            id: session.id().value(),
            user_id: session.user_id(),
            song_id: *session.song_id().value(),
            artist_id: *session.artist_id().value(),
            user_tier: session.user_tier().to_string(),
            status: session.status().to_string(),
            listen_duration_seconds: session.listen_duration().map(|d| d.seconds() as i32),
            quality_score: session.quality_score().map(|q| q.score()),
            zk_proof_hash: session.zk_proof().map(|p| p.value()),
            base_reward_tokens: session.base_reward().map(|r| r.tokens()),
            final_reward_tokens: session.final_reward().map(|r| r.tokens()),
            started_at: session.started_at(),
            completed_at: session.completed_at(),
            verified_at: session.verified_at(),
            version: session.version(),
            created_at: session.started_at(), // Usamos started_at como created_at
            updated_at: Utc::now(),
        }
    }

    // Convierte una fila de base de datos a una entidad de dominio
    fn row_to_entity(&self, row: &ListenSessionRow) -> RepositoryResult<ListenSession> {
        // Convertir value objects
        let session_id = ListenSessionId::from_uuid(row.id);
        let song_id = row.song_id;
        let artist_id = row.artist_id;
        
        // Convertir enums
        let user_tier = RewardTier::from_string(&row.user_tier)
            .map_err(|e| format!("Invalid user tier: {}", e))?;
        
        let status = SessionStatus::from_string(&row.status)
            .map_err(|e| format!("Invalid status: {}", e))?;
        
        // Convertir opcionales
        let listen_duration = row.listen_duration_seconds
            .map(|s| ListenDuration::new(s as u32))
            .transpose()
            .map_err(|e| format!("Invalid duration: {}", e))?;
        
        let quality_score = row.quality_score
            .map(|s| QualityScore::new(s))
            .transpose()
            .map_err(|e| format!("Invalid quality score: {}", e))?;
        
        let zk_proof = row.zk_proof_hash
            .clone()
            .map(|h| ZkProofHash::new(h))
            .transpose()
            .map_err(|e| format!("Invalid ZK proof: {}", e))?;
        
        let base_reward = row.base_reward_tokens
            .map(|t| RewardAmount::new(t))
            .transpose()
            .map_err(|e| format!("Invalid base reward: {}", e))?;
        
        let final_reward = row.final_reward_tokens
            .map(|t| RewardAmount::new(t))
            .transpose()
            .map_err(|e| format!("Invalid final reward: {}", e))?;
        
        // Crear contratos temporales para la entidad
        let song_contract = SongContract {
            id: song_id,
            title: "Unknown".to_string(), // Placeholder
            artist_id,
            artist_name: "Unknown".to_string(), // Placeholder
            duration_seconds: None,
            genre: None,
            ipfs_hash: None,
            metadata_url: None,
            nft_contract_address: None,
            nft_token_id: None,
            royalty_percentage: None,
            is_minted: false,
            created_at: Utc::now(),
        };
        
        let artist_contract = ArtistContract {
            id: artist_id,
            name: "Unknown".to_string(), // Placeholder
            verified: false,
            bio: None,
            avatar_url: None,
            social_links: None,
            genres: vec![],
            total_streams: 0,
            monthly_listeners: 0,
            created_at: Utc::now(),
        };
        
        // Crear la entidad usando el constructor from_parts
        let session = ListenSession::from_parts(
            session_id,
            row.user_id,
            song_contract,
            artist_contract,
            user_tier,
            status,
            listen_duration,
            quality_score,
            zk_proof,
            base_reward,
            final_reward,
            row.started_at,
            row.completed_at,
            row.verified_at,
        );
        
        Ok(session)
    }
}

#[async_trait]
impl ListenSessionRepository for PostgresListenSessionRepository {
    async fn find_by_id(&self, id: &ListenSessionId) -> RepositoryResult<Option<ListenSession>> {
        let query = "SELECT * FROM listen_sessions WHERE id = $1 AND status != 'deleted'";
        
        let result = sqlx::query_as::<_, ListenSessionRow>(query)
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        match result {
            Some(row) => Ok(Some(self.row_to_entity(&row)?)),
            None => Ok(None),
        }
    }

    async fn save(&self, session: &ListenSession) -> RepositoryResult<()> {
        let row = self.session_to_row(session);
        
        let query = r#"
            INSERT INTO listen_sessions (
                id, user_id, song_id, artist_id, user_tier, status, 
                listen_duration_seconds, quality_score, zk_proof_hash,
                base_reward_tokens, final_reward_tokens, started_at,
                completed_at, verified_at, version
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
            )
            ON CONFLICT (id) DO NOTHING
        "#;
        
        sqlx::query(query)
            .bind(row.id)
            .bind(row.user_id)
            .bind(row.song_id)
            .bind(row.artist_id)
            .bind(row.user_tier)
            .bind(row.status)
            .bind(row.listen_duration_seconds)
            .bind(row.quality_score)
            .bind(row.zk_proof_hash)
            .bind(row.base_reward_tokens)
            .bind(row.final_reward_tokens)
            .bind(row.started_at)
            .bind(row.completed_at)
            .bind(row.verified_at)
            .bind(row.version)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save listen session: {}", e))?;
            
        Ok(())
    }

    async fn update(&self, session: &ListenSession, expected_version: i32) -> RepositoryResult<()> {
        let row = self.session_to_row(session);
        
        let query = r#"
            UPDATE listen_sessions SET
                user_tier = $1,
                status = $2,
                listen_duration_seconds = $3,
                quality_score = $4,
                zk_proof_hash = $5,
                base_reward_tokens = $6,
                final_reward_tokens = $7,
                completed_at = $8,
                verified_at = $9,
                version = version + 1
            WHERE id = $10 AND version = $11
        "#;
        
        let result = sqlx::query(query)
            .bind(row.user_tier)
            .bind(row.status)
            .bind(row.listen_duration_seconds)
            .bind(row.quality_score)
            .bind(row.zk_proof_hash)
            .bind(row.base_reward_tokens)
            .bind(row.final_reward_tokens)
            .bind(row.completed_at)
            .bind(row.verified_at)
            .bind(row.id)
            .bind(expected_version)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to update listen session: {}", e))?;
            
        if result.rows_affected() == 0 {
            return Err("Session was modified by another process or not found".to_string());
        }
        
        Ok(())
    }

    async fn delete(&self, id: &ListenSessionId) -> RepositoryResult<()> {
        // Soft delete marcando como 'deleted'
        let query = "UPDATE listen_sessions SET status = 'deleted' WHERE id = $1";
        
        let result = sqlx::query(query)
            .bind(id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete listen session: {}", e))?;
            
        if result.rows_affected() == 0 {
            return Err(format!("Session not found: {}", id.value()));
        }
        
        Ok(())
    }

    async fn exists(&self, id: &ListenSessionId) -> RepositoryResult<bool> {
        let query = "SELECT COUNT(*) FROM listen_sessions WHERE id = $1 AND status != 'deleted'";
        
        let count: i64 = sqlx::query_scalar(query)
            .bind(id.value())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        Ok(count > 0)
    }

    async fn find_active_sessions_for_user(&self, user_id: Uuid) -> RepositoryResult<Vec<ListenSession>> {
        let query = "SELECT * FROM listen_sessions WHERE user_id = $1 AND status = 'active'";
        
        let rows = sqlx::query_as::<_, ListenSessionRow>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(self.row_to_entity(&row)?);
        }
        
        Ok(sessions)
    }

    async fn count_user_sessions_in_period(&self, user_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> RepositoryResult<i64> {
        let query = "SELECT COUNT(*) FROM listen_sessions WHERE user_id = $1 AND started_at >= $2 AND started_at <= $3";
        
        let count: i64 = sqlx::query_scalar(query)
            .bind(user_id)
            .bind(start)
            .bind(end)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        Ok(count)
    }
}

#[async_trait]
impl ListenSessionQueryRepository for PostgresListenSessionRepository {
    async fn find_sessions(&self, _filter: &ListenSessionFilter, pagination: &Pagination) -> RepositoryResult<Vec<ListenSession>> {
        // Implementación simplificada - solo paginación básica
        let query = "SELECT * FROM listen_sessions WHERE status != 'deleted' ORDER BY started_at DESC LIMIT $1 OFFSET $2";
        
        let rows = sqlx::query_as::<_, ListenSessionRow>(query)
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(self.row_to_entity(&row)?);
        }
        
        Ok(sessions)
    }

    async fn count_sessions(&self, _filter: &ListenSessionFilter) -> RepositoryResult<i64> {
        let query = "SELECT COUNT(*) FROM listen_sessions WHERE status != 'deleted'";
        
        let count: i64 = sqlx::query_scalar(query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        Ok(count)
    }

    async fn find_sessions_by_song(&self, song_id: Uuid, pagination: &Pagination) -> RepositoryResult<Vec<ListenSession>> {
        let query = "SELECT * FROM listen_sessions WHERE song_id = $1 AND status != 'deleted' ORDER BY started_at DESC LIMIT $2 OFFSET $3";
        
        let rows = sqlx::query_as::<_, ListenSessionRow>(query)
            .bind(song_id)
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(self.row_to_entity(&row)?);
        }
        
        Ok(sessions)
    }

    async fn find_sessions_by_artist(&self, artist_id: Uuid, pagination: &Pagination) -> RepositoryResult<Vec<ListenSession>> {
        let query = "SELECT * FROM listen_sessions WHERE artist_id = $1 AND status != 'deleted' ORDER BY started_at DESC LIMIT $2 OFFSET $3";
        
        let rows = sqlx::query_as::<_, ListenSessionRow>(query)
            .bind(artist_id)
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(self.row_to_entity(&row)?);
        }
        
        Ok(sessions)
    }

    async fn find_sessions_ready_for_reward(&self) -> RepositoryResult<Vec<ListenSession>> {
        let query = "SELECT * FROM listen_sessions WHERE status = 'verified' AND final_reward_tokens IS NOT NULL";
        
        let rows = sqlx::query_as::<_, ListenSessionRow>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(self.row_to_entity(&row)?);
        }
        
        Ok(sessions)
    }

    async fn find_failed_verification_sessions(&self, pagination: &Pagination) -> RepositoryResult<Vec<ListenSession>> {
        let query = "SELECT * FROM listen_sessions WHERE status = 'failed' ORDER BY started_at DESC LIMIT $1 OFFSET $2";
        
        let rows = sqlx::query_as::<_, ListenSessionRow>(query)
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(self.row_to_entity(&row)?);
        }
        
        Ok(sessions)
    }
} 