use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;

use super::{
    aggregates::{UserAggregate, UserSummary},
    entities::{User, UserProfile, UserPreferences, UserStats},
    value_objects::{UserId, Email, Username},
};
use crate::shared::domain::errors::AppError;

/// Repository trait for User aggregate
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Save user aggregate (create or update)
    async fn save(&self, aggregate: &UserAggregate) -> Result<(), AppError>;

    /// Update existing user aggregate
    async fn update(&self, aggregate: &UserAggregate) -> Result<(), AppError>;

    /// Find user aggregate by ID
    async fn find_by_id(&self, id: &UserId) -> Result<Option<UserAggregate>, AppError>;

    /// Find user by email
    async fn find_by_email(&self, email: &Email) -> Result<Option<UserAggregate>, AppError>;

    /// Find user by username
    async fn find_by_username(&self, username: &Username) -> Result<Option<UserAggregate>, AppError>;

    /// Check if email exists
    async fn email_exists(&self, email: &Email) -> Result<bool, AppError>;

    /// Check if username exists
    async fn username_exists(&self, username: &Username) -> Result<bool, AppError>;

    /// Search users by text with pagination (simple helper)
    async fn search_users(&self, search_text: Option<&str>, limit: u32, offset: u32) -> Result<Vec<UserAggregate>, AppError>;

    /// Add follower relationship
    async fn add_follower(&self, follower_id: &UserId, followee_id: &UserId) -> Result<(), AppError>;

    /// Remove follower relationship
    async fn remove_follower(&self, follower_id: &UserId, followee_id: &UserId) -> Result<(), AppError>;

    /// Find users by criteria
    async fn find_users(&self, criteria: UserSearchCriteria) -> Result<Vec<UserSummary>, AppError>;

    /// Find active users
    async fn find_active_users(&self, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError>;

    /// Find users by tier
    async fn find_by_tier(&self, tier: &str, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError>;

    /// Find users by role
    async fn find_by_role(&self, role: &str, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError>;

    /// Get user count
    async fn count_users(&self) -> Result<u64, AppError>;

    /// Get user statistics
    async fn get_user_stats(&self, user_id: &UserId) -> Result<Option<UserStats>, AppError>;

    /// Delete user (soft delete - deactivate)
    async fn delete(&self, id: &UserId) -> Result<(), AppError>;

    /// Get users registered in date range
    async fn find_users_registered_between(
        &self,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>
    ) -> Result<Vec<UserSummary>, AppError>;

    /// Get top users by rewards
    async fn find_top_users_by_rewards(&self, limit: u32) -> Result<Vec<UserSummary>, AppError>;

    /// Get top users by listening time
    async fn find_top_users_by_listening_time(&self, limit: u32) -> Result<Vec<UserSummary>, AppError>;

    /// Find users with linked wallets
    async fn find_users_with_wallets(&self, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError>;

    /// Find users by tier points range
    async fn find_users_by_tier_points_range(
        &self,
        min_points: u32,
        max_points: u32,
        page: u32,
        page_size: u32
    ) -> Result<Vec<UserSummary>, AppError>;
}

/// Search criteria for finding users
#[derive(Debug, Clone, Default)]
pub struct UserSearchCriteria {
    pub username_contains: Option<String>,
    pub email_contains: Option<String>,
    pub display_name_contains: Option<String>,
    pub tier: Option<String>,
    pub role: Option<String>,
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub has_wallet: Option<bool>,
    pub min_tier_points: Option<u32>,
    pub max_tier_points: Option<u32>,
    pub min_rewards: Option<f64>,
    pub max_rewards: Option<f64>,
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<UserSortField>,
    pub sort_order: Option<SortOrder>,
}

impl UserSearchCriteria {
    pub fn new() -> Self {
        Self {
            page: 0,
            page_size: 20,
            ..Default::default()
        }
    }

    pub fn username_contains(mut self, username: String) -> Self {
        self.username_contains = Some(username);
        self
    }

    pub fn email_contains(mut self, email: String) -> Self {
        self.email_contains = Some(email);
        self
    }

    pub fn display_name_contains(mut self, display_name: String) -> Self {
        self.display_name_contains = Some(display_name);
        self
    }

    pub fn tier(mut self, tier: String) -> Self {
        self.tier = Some(tier);
        self
    }

    pub fn role(mut self, role: String) -> Self {
        self.role = Some(role);
        self
    }

    pub fn is_verified(mut self, verified: bool) -> Self {
        self.is_verified = Some(verified);
        self
    }

    pub fn is_active(mut self, active: bool) -> Self {
        self.is_active = Some(active);
        self
    }

    pub fn has_wallet(mut self, has_wallet: bool) -> Self {
        self.has_wallet = Some(has_wallet);
        self
    }

    pub fn tier_points_range(mut self, min: u32, max: u32) -> Self {
        self.min_tier_points = Some(min);
        self.max_tier_points = Some(max);
        self
    }

    pub fn rewards_range(mut self, min: f64, max: f64) -> Self {
        self.min_rewards = Some(min);
        self.max_rewards = Some(max);
        self
    }

    pub fn created_after(mut self, date: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_after = Some(date);
        self
    }

    pub fn created_before(mut self, date: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_before = Some(date);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn sort_by(mut self, field: UserSortField, order: SortOrder) -> Self {
        self.sort_by = Some(field);
        self.sort_order = Some(order);
        self
    }
}

/// Fields available for sorting users
#[derive(Debug, Clone)]
pub enum UserSortField {
    Username,
    Email,
    CreatedAt,
    UpdatedAt,
    LastLoginAt,
    TierPoints,
    TotalRewards,
    ListeningTime,
}

/// Sort order
#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Query result with pagination info
#[derive(Debug, Clone)]
pub struct UserQueryResult {
    pub users: Vec<UserSummary>,
    pub total_count: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl UserQueryResult {
    pub fn new(
        users: Vec<UserSummary>,
        total_count: u64,
        page: u32,
        page_size: u32,
    ) -> Self {
        let total_pages = if page_size > 0 {
            (total_count as f64 / page_size as f64).ceil() as u32
        } else {
            0
        };

        Self {
            users,
            total_count,
            page,
            page_size,
            total_pages,
        }
    }

    pub fn empty(page: u32, page_size: u32) -> Self {
        Self {
            users: Vec::new(),
            total_count: 0,
            page,
            page_size,
            total_pages: 0,
        }
    }

    pub fn has_next_page(&self) -> bool {
        self.page < self.total_pages.saturating_sub(1)
    }

    pub fn has_previous_page(&self) -> bool {
        self.page > 0
    }
}

/// Extended repository trait for advanced queries
#[async_trait]
pub trait UserAnalyticsRepository: Send + Sync {
    /// Get user registration statistics by period
    async fn get_registration_stats(
        &self,
        period: AnalyticsPeriod,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<RegistrationStat>, AppError>;

    /// Get user tier distribution
    async fn get_tier_distribution(&self) -> Result<HashMap<String, u64>, AppError>;

    /// Get user role distribution
    async fn get_role_distribution(&self) -> Result<HashMap<String, u64>, AppError>;

    /// Get user activity statistics
    async fn get_activity_stats(
        &self,
        period: AnalyticsPeriod,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<ActivityStat>, AppError>;

    /// Get user retention metrics
    async fn get_retention_metrics(
        &self,
        cohort_period: AnalyticsPeriod,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<RetentionMetric>, AppError>;
}

/// Analytics period for grouping data
#[derive(Debug, Clone)]
pub enum AnalyticsPeriod {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

/// Registration statistics
#[derive(Debug, Clone)]
pub struct RegistrationStat {
    pub period: chrono::DateTime<chrono::Utc>,
    pub count: u64,
    pub verified_count: u64,
}

/// Activity statistics
#[derive(Debug, Clone)]
pub struct ActivityStat {
    pub period: chrono::DateTime<chrono::Utc>,
    pub active_users: u64,
    pub new_users: u64,
    pub total_listening_time: u64,
    pub total_rewards_earned: f64,
}

/// Retention metrics
#[derive(Debug, Clone)]
pub struct RetentionMetric {
    pub cohort_period: chrono::DateTime<chrono::Utc>,
    pub users_in_cohort: u64,
    pub retained_users: Vec<u64>, // Retained users for each subsequent period
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_search_criteria_builder() {
        let criteria = UserSearchCriteria::new()
            .username_contains("test".to_string())
            .tier("premium".to_string())
            .is_active(true)
            .page(1)
            .page_size(10);

        assert_eq!(criteria.username_contains, Some("test".to_string()));
        assert_eq!(criteria.tier, Some("premium".to_string()));
        assert_eq!(criteria.is_active, Some(true));
        assert_eq!(criteria.page, 1);
        assert_eq!(criteria.page_size, 10);
    }

    #[test]
    fn test_user_query_result() {
        let users = vec![];
        let result = UserQueryResult::new(users, 100, 0, 20);

        assert_eq!(result.total_count, 100);
        assert_eq!(result.total_pages, 5);
        assert!(!result.has_previous_page());
        assert!(result.has_next_page());

        let result_last_page = UserQueryResult::new(vec![], 100, 4, 20);
        assert!(!result_last_page.has_next_page());
        assert!(result_last_page.has_previous_page());
    }
} 