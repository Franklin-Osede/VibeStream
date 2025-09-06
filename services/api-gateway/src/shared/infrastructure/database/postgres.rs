// PostgreSQL implementation of repositories
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;

use crate::bounded_contexts::user::domain::{
    aggregates::UserAggregate,
    entities::{User, UserProfile, UserPreferences, UserStats},
    value_objects::{UserId, Email, Username, PasswordHash},
    repository::{UserRepository, UserSearchCriteria},
};
use crate::shared::domain::errors::AppError;

// Temporary crate::bounded_contexts::user::domain::aggregates::UserSummary type
#[derive(Debug, Clone)]
pub struct crate::bounded_contexts::user::domain::aggregates::UserSummary {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
}

/// PostgreSQL implementation of UserRepository
#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
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
        
        sqlx::query!(
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
            "#,
            user_id,
            email,
            username,
            password_hash,
            display_name,
            bio,
            created_at,
            updated_at
        )
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
        
        let row = sqlx::query!(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user by ID: {}", e)))?;
        
        if let Some(row) = row {
            // Create User entity with proper error handling
            let email = Email::new(row.email)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.username)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.password_hash);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.id));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.id));
            profile.update_display_name(row.display_name);
            profile.update_bio(row.bio);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.id));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.id));
            
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
        
        let row = sqlx::query!(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email_value
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user by email: {}", e)))?;
        
        if let Some(row) = row {
            // Create User entity with proper error handling
            let email = Email::new(row.email)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.username)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.password_hash);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.id));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.id));
            profile.update_display_name(row.display_name);
            profile.update_bio(row.bio);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.id));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.id));
            
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
        
        let row = sqlx::query!(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username_value
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user by username: {}", e)))?;
        
        if let Some(row) = row {
            // Create User entity with proper error handling
            let email = Email::new(row.email)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.username)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.password_hash);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.id));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.id));
            profile.update_display_name(row.display_name);
            profile.update_bio(row.bio);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.id));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.id));
            
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
        
        sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            user_id
        )
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
        
        let rows = sqlx::query!(
            r#"
            SELECT id, email, username, password_hash, display_name, bio, created_at, updated_at
            FROM users
            WHERE ($1::text IS NULL OR 
                   username ILIKE $1 OR 
                   email ILIKE $1 OR 
                   display_name ILIKE $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to search users: {}", e)))?;
        
        let users = rows.into_iter().map(|row| {
            // Create User entity with proper error handling
            let email = Email::new(row.email)
                .map_err(|e| AppError::ValidationError(format!("Invalid email: {}", e)))?;
            let username = Username::new(row.username)
                .map_err(|e| AppError::ValidationError(format!("Invalid username: {}", e)))?;
            let password_hash = PasswordHash::new(row.password_hash);
            
            let user = User::new(email, username, password_hash);
            
            // Create UserProfile entity
            let profile = UserProfile::new(UserId::from_uuid(row.id));
            // Update profile with actual values
            let mut profile = UserProfile::new(UserId::from_uuid(row.id));
            profile.update_display_name(row.display_name);
            profile.update_bio(row.bio);
            
            // Create default UserPreferences
            let preferences = UserPreferences::new(UserId::from_uuid(row.id));
            
            // Create default UserStats
            let stats = UserStats::new(UserId::from_uuid(row.id));
            
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
        
        sqlx::query!(
            r#"
            INSERT INTO user_followers (follower_id, followee_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (follower_id, followee_id) DO NOTHING
            "#,
            follower_uuid,
            followee_uuid
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to add follower: {}", e)))?;
        
        Ok(())
    }

    async fn remove_follower(&self, follower_id: &UserId, followee_id: &UserId) -> Result<(), AppError> {
        let follower_uuid = follower_id.value();
        let followee_uuid = followee_id.value();
        
        sqlx::query!(
            "DELETE FROM user_followers WHERE follower_id = $1 AND followee_id = $2",
            follower_uuid,
            followee_uuid
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to remove follower: {}", e)))?;
        
        Ok(())
    }

    async fn find_users(&self, criteria: UserSearchCriteria) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement complex search later
        Ok(vec![])
    }

    async fn find_active_users(&self, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }

    async fn find_by_tier(&self, tier: &str, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }

    async fn find_by_role(&self, role: &str, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }

    async fn get_user_stats(&self, user_id: &UserId) -> Result<Option<UserStats>, AppError> {
        // For now, return None - implement later
        Ok(None)
    }

    async fn find_users_registered_between(
        &self,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>
    ) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }

    async fn find_top_users_by_rewards(&self, limit: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }

    async fn find_top_users_by_listening_time(&self, limit: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }

    async fn find_users_with_wallets(&self, page: u32, page_size: u32) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }

    async fn find_users_by_tier_points_range(
        &self,
        min_points: u32,
        max_points: u32,
        page: u32,
        page_size: u32
    ) -> Result<Vec<crate::bounded_contexts::user::domain::aggregates::UserSummary>, AppError> {
        // For now, return empty vector - implement later
        Ok(vec![])
    }
} 