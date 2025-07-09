use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;

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
        };

        // Create UserProfile
        let profile = UserProfile {
            display_name,
            bio,
            avatar_url: avatar_url.and_then(|url| ProfileUrl::new(url).ok()),
        };

        // Create UserPreferences (default for now)
        let preferences = UserPreferences::default();

        // Create UserStats
        let stats = UserStats {
            tier_points: tier_points as u32,
            total_rewards,
            total_listen_time: listen_time as u64,
            songs_listened: 0, // Would need to be calculated
            playlists_created: 0, // Would need to be calculated
            followers_count: 0, // Would need to be calculated
            following_count: 0, // Would need to be calculated
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
            r#"
            INSERT INTO users (id, email, username, password_hash, display_name, bio, avatar_url,
                              wallet_address, tier, role, tier_points, total_rewards, listen_time,
                              is_verified, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
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
                total_rewards = EXCLUDED.total_rewards,
                listen_time = EXCLUDED.listen_time,
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
        .bind(aggregate.stats.total_rewards)
        .bind(aggregate.stats.total_listen_time as i64)
        .bind(aggregate.user.is_verified)
        .bind(aggregate.user.is_active)
        .bind(aggregate.user.created_at)
        .bind(aggregate.user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<UserAggregate>, AppError> {
        let rec = sqlx::query(
            r#"SELECT id, email, username, password_hash, display_name, bio, avatar_url,
                      wallet_address, tier, role, tier_points, total_rewards, listen_time,
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
                      wallet_address, tier, role, tier_points, total_rewards, listen_time,
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
                      wallet_address, tier, role, tier_points, total_rewards, listen_time,
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

    async fn delete(&self, id: &UserId) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET is_active = false WHERE id = $1")
            .bind(id.to_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    // Basic implementations for other required methods
    async fn find_users(&self, _criteria: crate::bounded_contexts::user::domain::repository::UserSearchCriteria) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn find_active_users(&self, _page: u32, _page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn find_by_tier(&self, _tier: &str, _page: u32, _page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn find_by_role(&self, _role: &str, _page: u32, _page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn count_users(&self) -> Result<u64, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE is_active = true")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let count: i64 = row.try_get("count").map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(count as u64)
    }

    async fn get_user_stats(&self, _user_id: &UserId) -> Result<Option<crate::bounded_contexts::user::domain::entities::UserStats>, AppError> {
        Ok(None) // TODO: Implement properly
    }

    async fn find_users_registered_between(&self, _start_date: chrono::DateTime<chrono::Utc>, _end_date: chrono::DateTime<chrono::Utc>) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn find_top_users_by_rewards(&self, _limit: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn find_top_users_by_listening_time(&self, _limit: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn find_users_with_wallets(&self, _page: u32, _page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }

    async fn find_users_by_tier_points_range(&self, _min_points: u32, _max_points: u32, _page: u32, _page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        Ok(vec![]) // TODO: Implement properly
    }
} 