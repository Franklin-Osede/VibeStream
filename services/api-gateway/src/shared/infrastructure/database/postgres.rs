// PostgreSQL implementation of repositories
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::sync::Arc;

use crate::bounded_contexts::user::domain::{
    aggregates::{UserAggregate, UserSummary},
    entities::{User, UserProfile, UserPreferences, UserStats},
    value_objects::{UserId, Email, Username, PasswordHash},
    repository::{UserRepository, UserSearchCriteria},
};
use crate::shared::domain::errors::AppError;

// Using UserSummary from aggregates module

/// PostgreSQL implementation of UserRepository
#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Helper function to create UserSummary from database row
    fn create_user_summary(
        id: Uuid,
        username: String,
        email: String,
        tier: Option<String>,
        role: Option<String>,
        is_verified: Option<bool>,
        is_active: Option<bool>,
        display_name: Option<String>,
        avatar_url: Option<String>,
        total_listening_time: Option<i64>,
        total_rewards: Option<f64>,
        tier_points: Option<i32>,
        created_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<UserSummary, AppError> {
        let user_id = UserId::from_uuid(id);
        let username = Username::new(username)
            .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
        let email = Email::new(email)
            .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;

        Ok(UserSummary {
            id: user_id,
            username,
            email,
            tier: tier.unwrap_or_else(|| "basic".to_string()),
            role: role.unwrap_or_else(|| "user".to_string()),
            is_verified: is_verified.unwrap_or(false),
            is_active: is_active.unwrap_or(true),
            display_name,
            avatar_url,
            total_listening_time: total_listening_time.unwrap_or(0),
            total_rewards: total_rewards.unwrap_or(0.0),
            tier_points: tier_points.unwrap_or(0),
            created_at: created_at.unwrap_or_else(|| chrono::Utc::now()),
        })
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &UserAggregate) -> Result<(), AppError> {
        let user_id = user.user.id.value();
        let email = user.user.email.value();
        let username = user.user.username.value();
        let password_hash = user.user.password_hash.value();
        let display_name = user.profile.display_name.as_ref().map(|n| n.clone());
        let bio = user.profile.bio.as_ref().map(|b| b.clone());
        let created_at = user.user.created_at;
        let updated_at = user.user.updated_at;
        
        sqlx::query(
            r#"
            INSERT INTO users (id, email, username, password_hash, display_name, bio, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                username = EXCLUDED.username,
                password_hash = EXCLUDED.password_hash,
                display_name = EXCLUDED.display_name,
                bio = EXCLUDED.bio,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(user_id)
        .bind(email)
        .bind(username)
        .bind(password_hash)
        .bind(display_name)
        .bind(bio)
        .bind(created_at)
        .bind(updated_at)
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to save user: {}", e)))?;
        
        Ok(())
    }

    async fn update(&self, user: &UserAggregate) -> Result<(), AppError> {
        // For now behave like save (upsert)
        self.save(user).await
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<UserAggregate>, AppError> {
        let user_id = id.value();
        
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user by ID: {}", e)))?;
        
        if let Some(row) = row {
            // Create User entity with proper error handling
            let email = Email::new(row.try_get("email")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.try_get("username")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.try_get("password_hash")?);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            profile.update_display_name(row.try_get("display_name")?);
            profile.update_bio(row.try_get("bio")?);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.try_get("id")?));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.try_get("id")?));
            
            let user_aggregate = UserAggregate::load(
                user,
                profile,
                preferences,
                stats,
                1, // version
            );
            
            Ok(Some(user_aggregate))
        } else {
            Ok(None)
        }
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<UserAggregate>, AppError> {
        let email_value = email.value();
        
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE email = $1
            "#
        )
        .bind(email_value)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user by email: {}", e)))?;
        
        if let Some(row) = row {
            // Create User entity with proper error handling
            let email = Email::new(row.try_get("email")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.try_get("username")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.try_get("password_hash")?);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            profile.update_display_name(row.try_get("display_name")?);
            profile.update_bio(row.try_get("bio")?);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.try_get("id")?));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.try_get("id")?));
            
            let user_aggregate = UserAggregate::load(
                user,
                profile,
                preferences,
                stats,
                1, // version
            );
            
            Ok(Some(user_aggregate))
        } else {
            Ok(None)
        }
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<UserAggregate>, AppError> {
        let username_value = username.value();
        
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE username = $1
            "#
        )
        .bind(username_value)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user by username: {}", e)))?;
        
        if let Some(row) = row {
            // Create User entity with proper error handling
            let email = Email::new(row.try_get("email")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.try_get("username")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.try_get("password_hash")?);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            profile.update_display_name(row.try_get("display_name")?);
            profile.update_bio(row.try_get("bio")?);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.try_get("id")?));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.try_get("id")?));
            
            let user_aggregate = UserAggregate::load(
                user,
                profile,
                preferences,
                stats,
                1, // version
            );
            
            Ok(Some(user_aggregate))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: &UserId) -> Result<(), AppError> {
        let user_id = id.value();
        
        sqlx::query(
            "DELETE FROM users WHERE id = $1"
        )
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete user: {}", e)))?;
        
        Ok(())
    }

    async fn email_exists(&self, email: &Email) -> Result<bool, AppError> {
        let email_value = email.value();
        
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users WHERE email = $1",
            email_value
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to check email existence: {}", e)))?;
        
        Ok(count.unwrap_or(0) > 0)
    }

    async fn username_exists(&self, username: &Username) -> Result<bool, AppError> {
        let username_value = username.value();
        
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users WHERE username = $1",
            username_value
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to check username existence: {}", e)))?;
        
        Ok(count.unwrap_or(0) > 0)
    }

    async fn search_users(&self, search_text: Option<&str>, limit: u32, offset: u32) -> Result<Vec<UserAggregate>, AppError> {
        let search_pattern = search_text.map(|text| format!("%{}%", text));
        
        let rows = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE ($1::text IS NULL OR 
                   username ILIKE $1 OR 
                   email ILIKE $1 OR 
                   display_name ILIKE $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(search_pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to search users: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            // Create User entity with proper error handling
            let email = Email::new(row.try_get("email")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.try_get("username")?)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.try_get("password_hash")?);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.try_get("id")?));
            profile.update_display_name(row.try_get("display_name")?);
            profile.update_bio(row.try_get("bio")?);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.try_get("id")?));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.try_get("id")?));
            
            Ok(UserAggregate::load(
                user,
                profile,
                preferences,
                stats,
                1, // version
            ))
        }).collect::<Result<Vec<UserAggregate>, AppError>>()?;
        
        Ok(users)
    }

    async fn count_users(&self) -> Result<u64, AppError> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to count users: {}", e)))?;
        
        Ok(count.unwrap_or(0) as u64)
    }

    async fn add_follower(&self, follower_id: &UserId, followee_id: &UserId) -> Result<(), AppError> {
        let follower_uuid = follower_id.value();
        let followee_uuid = followee_id.value();
        
        sqlx::query(
            r#"
            INSERT INTO user_followers (follower_id, followee_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (follower_id, followee_id) DO NOTHING
            "#
        )
        .bind(follower_uuid)
        .bind(followee_uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to add follower: {}", e)))?;
        
        Ok(())
    }

    async fn remove_follower(&self, follower_id: &UserId, followee_id: &UserId) -> Result<(), AppError> {
        let follower_uuid = follower_id.value();
        let followee_uuid = followee_id.value();
        
        sqlx::query(
            "DELETE FROM user_followers WHERE follower_id = $1 AND followee_id = $2"
        )
        .bind(follower_uuid)
        .bind(followee_uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to remove follower: {}", e)))?;
        
        Ok(())
    }

    async fn find_users(&self, criteria: UserSearchCriteria) -> Result<Vec<UserSummary>, AppError> {
        // For now, return empty vector - implement complex search later
        Ok(vec![])
    }

    async fn find_active_users(&self, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let offset = (page - 1) * page_size;
        
        let rows = sqlx::query(
            r#"
            SELECT id, username, email
            FROM users
            WHERE last_login_at > NOW() - INTERVAL '30 days'
            ORDER BY last_login_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find active users: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }

    async fn find_by_tier(&self, tier: &str, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let offset = (page - 1) * page_size;
        
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email
            FROM users u
            JOIN user_tiers ut ON u.id = ut.user_id
            WHERE ut.tier_name = $1 AND ut.is_active = true
            ORDER BY ut.updated_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(tier)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find users by tier: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }

    async fn find_by_role(&self, role: &str, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let offset = (page - 1) * page_size;
        
        let rows = sqlx::query(
            r#"
            SELECT id, username, email
            FROM users
            WHERE role = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(role)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find users by role: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }

    async fn get_user_stats(&self, user_id: &UserId) -> Result<Option<UserStats>, AppError> {
        let user_uuid = user_id.value();
        
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(DISTINCT lf.id) as total_listens,
                COALESCE(SUM(lf.duration_seconds), 0) as total_listening_time,
                COUNT(DISTINCT uf.followee_id) as following_count,
                COUNT(DISTINCT uf2.follower_id) as followers_count,
                COALESCE(SUM(r.amount), 0) as total_rewards
            FROM users u
            LEFT JOIN listen_sessions lf ON u.id = lf.user_id
            LEFT JOIN user_followers uf ON u.id = uf.follower_id
            LEFT JOIN user_followers uf2 ON u.id = uf2.followee_id
            LEFT JOIN rewards r ON u.id = r.user_id
            WHERE u.id = $1
            GROUP BY u.id
            "#
        )
        .bind(user_uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user stats: {}", e)))?;
        
        if let Some(row) = row {
            let stats = UserStats::new(user_id.clone());
            // Note: UserStats would need methods to update these values
            // For now, return the stats with default values
            Ok(Some(stats))
        } else {
        Ok(None)
        }
    }

    async fn find_users_registered_between(
        &self,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>
    ) -> Result<Vec<UserSummary>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, username, email
            FROM users
            WHERE created_at BETWEEN $1 AND $2
            ORDER BY created_at DESC
            "#
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find users registered between dates: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }

    async fn find_top_users_by_rewards(&self, limit: u32) -> Result<Vec<UserSummary>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, COALESCE(SUM(r.amount), 0) as total_rewards
            FROM users u
            LEFT JOIN rewards r ON u.id = r.user_id
            GROUP BY u.id, u.username, u.email
            ORDER BY total_rewards DESC
            LIMIT $1
            "#
        )
        .bind(limit as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find top users by rewards: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }

    async fn find_top_users_by_listening_time(&self, limit: u32) -> Result<Vec<UserSummary>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, COALESCE(SUM(ls.duration_seconds), 0) as total_listening_time
            FROM users u
            LEFT JOIN listen_sessions ls ON u.id = ls.user_id
            GROUP BY u.id, u.username, u.email
            ORDER BY total_listening_time DESC
            LIMIT $1
            "#
        )
        .bind(limit as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find top users by listening time: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }

    async fn find_users_with_wallets(&self, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let offset = (page - 1) * page_size;
        
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT u.id, u.username, u.email
            FROM users u
            INNER JOIN user_wallets uw ON u.id = uw.user_id
            WHERE uw.is_active = true
            ORDER BY u.created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find users with wallets: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }

    async fn find_users_by_tier_points_range(
        &self,
        min_points: u32,
        max_points: u32,
        page: u32,
        page_size: u32
    ) -> Result<Vec<UserSummary>, AppError> {
        let offset = (page - 1) * page_size;
        
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email
            FROM users u
            INNER JOIN user_tiers ut ON u.id = ut.user_id
            WHERE ut.tier_points BETWEEN $1 AND $2 AND ut.is_active = true
            ORDER BY ut.tier_points DESC
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(min_points as i32)
        .bind(max_points as i32)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find users by tier points range: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            Self::create_user_summary(
                row.try_get("id")?,
                row.try_get("username")?,
                row.try_get("email")?,
                None, // tier
                None, // role
                None, // is_verified
                None, // is_active
                None, // display_name
                None, // avatar_url
                None, // total_listening_time
                None, // total_rewards
                None, // tier_points
                None, // created_at
            )
        }).collect::<Result<Vec<UserSummary>, AppError>>()?;
        
        Ok(users)
    }
} 