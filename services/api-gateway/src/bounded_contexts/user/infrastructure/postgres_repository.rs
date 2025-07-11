use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{PgPool, Row};

use crate::bounded_contexts::user::domain::{
    aggregates::UserAggregate,
    entities::{User, UserProfile, UserPreferences, UserStats},
    repository::UserRepository,
    value_objects::{UserId, Email, Username, PasswordHash, UserTier, UserRole, WalletAddress, ProfileUrl},
};
use crate::shared::domain::errors::AppError;

pub struct UserPostgresRepository {
    pool: PgPool,
}

impl UserPostgresRepository {
    pub fn new(pool: PgPool) -> Self { 
        Self { pool } 
    }

    fn row_to_aggregate(&self, row: sqlx::postgres::PgRow) -> Result<UserAggregate, AppError> {
        use sqlx::Row;
        
        let id: Uuid = row.try_get("id").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let email_str: String = row.try_get("email").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let username_str: String = row.try_get("username").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let password_hash: String = row.try_get("password_hash").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let updated_at: chrono::DateTime<chrono::Utc> = row.try_get("updated_at").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let last_login_at: Option<chrono::DateTime<chrono::Utc>> = row.try_get("last_login_at").ok();

        // Create value objects using proper constructors
        let user_id = UserId::from_uuid(id);
        let email = Email::new(email_str).map_err(|e| AppError::ValidationError(e))?;
        let username = Username::new(username_str).map_err(|e| AppError::ValidationError(e))?;
        let password_hash = PasswordHash::new(password_hash);

        // Get optional fields
        let display_name: Option<String> = row.try_get("display_name").ok();
        let bio: Option<String> = row.try_get("bio").ok();
        let avatar_url: Option<String> = row.try_get("avatar_url").ok();
        let wallet_address: Option<String> = row.try_get("wallet_address").ok();
        let tier: String = row.try_get("tier").unwrap_or_else(|_| "free".to_string());
        let role: String = row.try_get("role").unwrap_or_else(|_| "user".to_string());
        let tier_points: i32 = row.try_get("tier_points").unwrap_or(0);
        let total_rewards: f64 = row.try_get("total_rewards").unwrap_or(0.0);
        let listen_time: i64 = row.try_get("listen_time").unwrap_or(0);
        let is_verified: bool = row.try_get("is_verified").unwrap_or(false);
        let is_active: bool = row.try_get("is_active").unwrap_or(true);

        // Create User entity
        let user = User {
            id: user_id,
            email,
            username,
            password_hash,
            wallet_address: wallet_address.and_then(|addr| WalletAddress::new(addr).ok()),
            tier: UserTier::from_str(&tier).unwrap_or(UserTier::Free),
            role: UserRole::from_str(&role).unwrap_or(UserRole::User),
            is_verified,
            is_active,
            created_at,
            updated_at,
            last_login_at,
        };

        // Create UserProfile
        let profile = UserProfile {
            display_name,
            bio,
            avatar_url: avatar_url.and_then(|url| ProfileUrl::new(url).ok()),
        };

        // Create UserPreferences (default for now)
        let preferences = UserPreferences::new(user_id.clone());

        // Create UserStats with correct field names
        let now = chrono::Utc::now();
        let stats = UserStats {
            user_id: user_id.clone(),
            total_listening_time_minutes: 0, // Changed from total_listen_time
            total_songs_listened: 0, // Changed from songs_listened  
            total_rewards_earned: 0.0, // Changed from total_rewards
            current_listening_streak: 0,
            longest_listening_streak: 0,
            total_investments: 0.0,
            investment_count: 0, // Changed from playlists_created
            nfts_owned: 0,
            campaigns_participated: 0, // Changed from followers_count  
            tier_points: 0, // Changed from following_count
            achievements_unlocked: Vec::new(),
            created_at: now,
            updated_at: now,
        };

        Ok(UserAggregate {
            user,
            profile,
            preferences,
            stats,
            pending_events: std::collections::VecDeque::new(),
            version: 1,
        })
    }
}

#[async_trait]
impl UserRepository for UserPostgresRepository {
    async fn save(&self, aggregate: &UserAggregate) -> Result<(), AppError> {
        sqlx::query(
            r#"INSERT INTO users (
                   id, email, username, password_hash, display_name, bio, avatar_url,
                   wallet_address, tier, role, tier_points, total_rewards_earned, total_listening_time_minutes,
                   is_verified, is_active, created_at, updated_at
               ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
               ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                username = EXCLUDED.username,
                password_hash = EXCLUDED.password_hash,
                display_name = EXCLUDED.display_name,
                bio = EXCLUDED.bio,
                avatar_url = EXCLUDED.avatar_url,
                wallet_address = EXCLUDED.wallet_address,
                tier = EXCLUDED.tier,
                role = EXCLUDED.role,
                tier_points = EXCLUDED.tier_points,
                total_rewards_earned = EXCLUDED.total_rewards_earned,
                total_listening_time_minutes = EXCLUDED.total_listening_time_minutes,
                is_verified = EXCLUDED.is_verified,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(aggregate.user.id.to_uuid())
        .bind(aggregate.user.email.as_str())
        .bind(aggregate.user.username.to_string())
        .bind(aggregate.user.password_hash.value())
        .bind(aggregate.profile.display_name.as_deref())
        .bind(aggregate.profile.bio.as_deref())
        .bind(aggregate.profile.avatar_url.as_ref().map(|u| u.value()))
        .bind(aggregate.user.wallet_address.as_ref().map(|w| w.value()))
        .bind(aggregate.user.tier.to_string())
        .bind(aggregate.user.role.to_string())
        .bind(aggregate.stats.tier_points as i32)
        .bind(aggregate.stats.total_rewards_earned) // Changed from total_rewards
        .bind(aggregate.stats.total_listening_time_minutes as i64) // Changed from total_listen_time
        .bind(aggregate.user.is_verified)
        .bind(aggregate.user.is_active)
        .bind(aggregate.user.created_at)
        .bind(aggregate.user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update(&self, aggregate: &UserAggregate) -> Result<(), AppError> {
        // For now, just use save which does upsert
        self.save(aggregate).await
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<UserAggregate>, AppError> {
        let rec = sqlx::query(
            r#"SELECT id, email, username, password_hash, display_name, bio, avatar_url,
                      wallet_address, tier, role, tier_points, total_rewards_earned, total_listening_time_minutes,
                      is_verified, is_active, last_login_at, created_at, updated_at
               FROM users WHERE id = $1"#
        )
        .bind(id.to_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match rec {
            Some(row) => Ok(Some(self.row_to_aggregate(row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<UserAggregate>, AppError> {
        let rec = sqlx::query(
            r#"SELECT id, email, username, password_hash, display_name, bio, avatar_url,
                      wallet_address, tier, role, tier_points, total_rewards_earned, total_listening_time_minutes,
                      is_verified, is_active, last_login_at, created_at, updated_at
               FROM users WHERE email = $1"#
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match rec {
            Some(row) => Ok(Some(self.row_to_aggregate(row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<UserAggregate>, AppError> {
        let rec = sqlx::query(
            r#"SELECT id, email, username, password_hash, display_name, bio, avatar_url,
                      wallet_address, tier, role, tier_points, total_rewards_earned, total_listening_time_minutes,
                      is_verified, is_active, last_login_at, created_at, updated_at
               FROM users WHERE username = $1"#
        )
        .bind(username.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match rec {
            Some(row) => Ok(Some(self.row_to_aggregate(row)?)),
            None => Ok(None),
        }
    }

    async fn email_exists(&self, email: &Email) -> Result<bool, AppError> {
        Ok(self.find_by_email(email).await?.is_some())
    }

    async fn username_exists(&self, username: &Username) -> Result<bool, AppError> {
        Ok(self.find_by_username(username).await?.is_some())
    }

    async fn search_users(&self, search_text: Option<&str>, limit: u32, offset: u32) -> Result<Vec<UserAggregate>, AppError> {
        // Construimos un criterio básico y reutilizamos find_users para obtener resúmenes,
        // luego devolvemos los agregados completos.
        use crate::bounded_contexts::user::domain::repository::UserSearchCriteria;

        let mut criteria = UserSearchCriteria::new()
            .page(offset)
            .page_size(limit);

        if let Some(text) = search_text {
            let text_owned = text.to_string();
            criteria = criteria.username_contains(text_owned.clone())
                .email_contains(text_owned.clone())
                .display_name_contains(text_owned);
        }

        let summaries = self.find_users(criteria).await?;

        // Cargar los agregados completos basados en los IDs obtenidos
        let mut aggregates = Vec::new();
        for summary in summaries {
            if let Some(agg) = self.find_by_id(&summary.id).await? {
                aggregates.push(agg);
            }
        }

        Ok(aggregates)
    }

    async fn delete(&self, id: &UserId) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET is_active = false WHERE id = $1")
            .bind(id.to_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    // Basic implementations for other required methods
    async fn find_users(&self, criteria: crate::bounded_contexts::user::domain::repository::UserSearchCriteria) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // Simple implementation for now - just handle basic pagination
        let offset = criteria.page * criteria.page_size;
        
        let rows = sqlx::query(
            r#"SELECT id, username, email, display_name, avatar_url, tier, role, tier_points, 
                      total_rewards_earned, total_listening_time_minutes, is_verified, is_active, created_at 
               FROM users WHERE is_active = true
               ORDER BY created_at DESC
               LIMIT $1 OFFSET $2"#
        )
        .bind(criteria.page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut summaries = Vec::new();
        for row in rows {
            let summary = crate::bounded_contexts::user::domain::aggregates::UserSummary {
                id: UserId::from_uuid(row.try_get("id").map_err(|e| AppError::DatabaseError(e.to_string()))?),
                username: Username::new(row.try_get("username").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                email: Email::new(row.try_get("email").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                display_name: row.try_get("display_name").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                avatar_url: row.try_get("avatar_url").ok(),
                tier: row.try_get("tier").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                role: row.try_get("role").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                tier_points: row.try_get("tier_points").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                total_rewards: row.try_get("total_rewards_earned").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                total_listening_time: row.try_get("total_listening_time_minutes").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_verified: row.try_get("is_verified").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_active: row.try_get("is_active").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.to_string()))?,
            };
            summaries.push(summary);
        }

        Ok(summaries)
    }

    async fn find_active_users(&self, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let criteria = crate::bounded_contexts::user::domain::repository::UserSearchCriteria {
            page,
            page_size,
            ..Default::default()
        };
        self.find_users(criteria).await
    }

    async fn find_by_tier(&self, tier: &str, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let criteria = crate::bounded_contexts::user::domain::repository::UserSearchCriteria {
            tier: Some(tier.to_string()),
            page,
            page_size,
            ..Default::default()
        };
        self.find_users(criteria).await
    }

    async fn find_by_role(&self, role: &str, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let criteria = crate::bounded_contexts::user::domain::repository::UserSearchCriteria {
            role: Some(role.to_string()),
            page,
            page_size,
            ..Default::default()
        };
        self.find_users(criteria).await
    }

    async fn count_users(&self) -> Result<u64, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE is_active = true")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let count: i64 = row.try_get("count").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(count as u64)
    }

    async fn get_user_stats(&self, user_id: &UserId) -> Result<Option<crate::bounded_contexts::user::domain::entities::UserStats>, AppError> {
        let row = sqlx::query(
            r#"SELECT id, tier_points, total_rewards, listen_time, total_investments, investment_count,
                      nfts_owned, campaigns_participated, current_streak, longest_streak, achievements
               FROM users WHERE id = $1 AND is_active = true"#
        )
        .bind(user_id.to_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let achievements: Vec<String> = row.try_get::<Option<serde_json::Value>, _>("achievements")
                    .unwrap_or(None)
                    .and_then(|v| serde_json::from_value(v).ok())
                    .unwrap_or_default();

                let stats = crate::bounded_contexts::user::domain::entities::UserStats {
                    user_id: user_id.clone(),
                    tier_points: row.try_get("tier_points").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    total_listening_time_minutes: row.try_get("listen_time").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    total_songs_listened: 0, // TODO: Calculate from listen_sessions table
                    total_rewards_earned: row.try_get("total_rewards").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    current_listening_streak: row.try_get("current_streak").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    longest_listening_streak: row.try_get("longest_streak").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    total_investments: row.try_get("total_investments").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    investment_count: row.try_get("investment_count").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    nfts_owned: row.try_get("nfts_owned").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    campaigns_participated: row.try_get("campaigns_participated").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                    achievements_unlocked: achievements,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                Ok(Some(stats))
            }
            None => Ok(None),
        }
    }

    async fn find_users_registered_between(&self, start_date: chrono::DateTime<chrono::Utc>, end_date: chrono::DateTime<chrono::Utc>) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let rows = sqlx::query(
            r#"SELECT id, username, email, display_name, avatar_url, tier, role, tier_points, 
                      total_rewards, is_verified, is_active, created_at 
               FROM users 
               WHERE created_at BETWEEN $1 AND $2 AND is_active = true
               ORDER BY created_at DESC"#
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut summaries = Vec::new();
        for row in rows {
            let summary = crate::bounded_contexts::user::domain::aggregates::UserSummary {
                id: UserId::from_uuid(row.try_get("id").map_err(|e| AppError::DatabaseError(e.to_string()))?),
                username: Username::new(row.try_get("username").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                email: Email::new(row.try_get("email").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                display_name: row.try_get("display_name").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                avatar_url: row.try_get("avatar_url").ok(),
                tier: row.try_get("tier").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                role: row.try_get("role").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                tier_points: row.try_get("tier_points").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                total_rewards: row.try_get("total_rewards").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_verified: row.try_get("is_verified").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_active: row.try_get("is_active").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.to_string()))?,
            };
            summaries.push(summary);
        }

        Ok(summaries)
    }

    async fn find_top_users_by_rewards(&self, limit: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let rows = sqlx::query(
            r#"SELECT id, username, email, display_name, avatar_url, tier, role, tier_points, 
                      total_rewards, is_verified, is_active, created_at 
               FROM users 
               WHERE is_active = true
               ORDER BY total_rewards DESC
               LIMIT $1"#
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut summaries = Vec::new();
        for row in rows {
            let summary = crate::bounded_contexts::user::domain::aggregates::UserSummary {
                id: UserId::from_uuid(row.try_get("id").map_err(|e| AppError::DatabaseError(e.to_string()))?),
                username: Username::new(row.try_get("username").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                email: Email::new(row.try_get("email").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                display_name: row.try_get("display_name").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                avatar_url: row.try_get("avatar_url").ok(),
                tier: row.try_get("tier").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                role: row.try_get("role").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                tier_points: row.try_get("tier_points").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                total_rewards: row.try_get("total_rewards").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_verified: row.try_get("is_verified").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_active: row.try_get("is_active").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.to_string()))?,
            };
            summaries.push(summary);
        }

        Ok(summaries)
    }

    async fn find_top_users_by_listening_time(&self, limit: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let rows = sqlx::query(
            r#"SELECT id, username, email, display_name, avatar_url, tier, role, tier_points, 
                      total_rewards, is_verified, is_active, created_at 
               FROM users 
               WHERE is_active = true
               ORDER BY listen_time DESC
               LIMIT $1"#
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut summaries = Vec::new();
        for row in rows {
            let summary = crate::bounded_contexts::user::domain::aggregates::UserSummary {
                id: UserId::from_uuid(row.try_get("id").map_err(|e| AppError::DatabaseError(e.to_string()))?),
                username: Username::new(row.try_get("username").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                email: Email::new(row.try_get("email").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                display_name: row.try_get("display_name").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                avatar_url: row.try_get("avatar_url").ok(),
                tier: row.try_get("tier").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                role: row.try_get("role").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                tier_points: row.try_get("tier_points").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                total_rewards: row.try_get("total_rewards").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_verified: row.try_get("is_verified").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_active: row.try_get("is_active").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.to_string()))?,
            };
            summaries.push(summary);
        }

        Ok(summaries)
    }

    async fn find_users_with_wallets(&self, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let offset = page * page_size;
        let rows = sqlx::query(
            r#"SELECT id, username, email, display_name, avatar_url, tier, role, tier_points, 
                      total_rewards, is_verified, is_active, created_at 
               FROM users 
               WHERE wallet_address IS NOT NULL AND is_active = true
               ORDER BY created_at DESC
               LIMIT $1 OFFSET $2"#
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut summaries = Vec::new();
        for row in rows {
            let summary = crate::bounded_contexts::user::domain::aggregates::UserSummary {
                id: UserId::from_uuid(row.try_get("id").map_err(|e| AppError::DatabaseError(e.to_string()))?),
                username: Username::new(row.try_get("username").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                email: Email::new(row.try_get("email").map_err(|e| AppError::DatabaseError(e.to_string()))?)
                    .map_err(|e| AppError::ValidationError(e))?,
                display_name: row.try_get("display_name").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                avatar_url: row.try_get("avatar_url").ok(),
                tier: row.try_get("tier").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                role: row.try_get("role").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                tier_points: row.try_get("tier_points").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                total_rewards: row.try_get("total_rewards").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_verified: row.try_get("is_verified").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                is_active: row.try_get("is_active").map_err(|e| AppError::DatabaseError(e.to_string()))?,
                created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.to_string()))?,
            };
            summaries.push(summary);
        }

        Ok(summaries)
    }

    async fn find_users_by_tier_points_range(&self, min_points: u32, max_points: u32, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        let criteria = crate::bounded_contexts::user::domain::repository::UserSearchCriteria {
            min_tier_points: Some(min_points),
            max_tier_points: Some(max_points),
            page,
            page_size,
            ..Default::default()
        };
        self.find_users(criteria).await
    }

    async fn add_follower(&self, follower_id: &UserId, followee_id: &UserId) -> Result<(), AppError> {
        sqlx::query(
            r#"INSERT INTO user_followers (follower_id, followee_id, created_at)
               VALUES ($1, $2, $3)
               ON CONFLICT (follower_id, followee_id) DO NOTHING"#
        )
        .bind(follower_id.to_uuid())
        .bind(followee_id.to_uuid())
        .bind(chrono::Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn remove_follower(&self, follower_id: &UserId, followee_id: &UserId) -> Result<(), AppError> {
        sqlx::query(
            r#"DELETE FROM user_followers 
               WHERE follower_id = $1 AND followee_id = $2"#
        )
        .bind(follower_id.to_uuid())
        .bind(followee_id.to_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
} 