// User Application Queries
// This module contains query structures and handlers for user operations

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::shared::application::query::{Query, QueryHandler};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::user::domain::entities::User;
use crate::bounded_contexts::user::domain::repository::UserRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Get user by ID query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserQuery {
    pub user_id: Uuid,
    pub include_profile: bool,
    pub include_stats: bool,
    pub include_preferences: bool,
}

impl Query for GetUserQuery {}

/// Get user by email query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByEmailQuery {
    pub email: String,
    pub include_profile: bool,
    pub include_stats: bool,
}

impl Query for GetUserByEmailQuery {}

/// Get user by username query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByUsernameQuery {
    pub username: String,
    pub include_profile: bool,
    pub include_stats: bool,
}

impl Query for GetUserByUsernameQuery {}

/// Get user statistics query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserStatsQuery {
    pub user_id: Uuid,
    pub detailed: bool,
}

impl Query for GetUserStatsQuery {}

/// Get user preferences query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPreferencesQuery {
    pub user_id: Uuid,
}

impl Query for GetUserPreferencesQuery {}

/// Search users query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersQuery {
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
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for SearchUsersQuery {}

impl Default for SearchUsersQuery {
    fn default() -> Self {
        Self {
            username_contains: None,
            email_contains: None,
            display_name_contains: None,
            tier: None,
            role: None,
            is_verified: None,
            is_active: None,
            has_wallet: None,
            min_tier_points: None,
            max_tier_points: None,
            min_rewards: None,
            max_rewards: None,
            created_after: None,
            created_before: None,
            page: 0,
            page_size: 20,
            sort_by: None,
            sort_order: None,
        }
    }
}

/// Get user list query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserListQuery {
    pub page: u32,
    pub page_size: u32,
    pub filter_active: Option<bool>,
    pub filter_verified: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for GetUserListQuery {}

impl Default for GetUserListQuery {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 20,
            filter_active: None,
            filter_verified: None,
            sort_by: Some("created_at".to_string()),
            sort_order: Some("desc".to_string()),
        }
    }
}

/// Get top users query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTopUsersQuery {
    pub metric: String, // "rewards", "listening_time", "tier_points", "investments"
    pub limit: u32,
    pub time_period: Option<String>, // "day", "week", "month", "year", "all_time"
    pub tier_filter: Option<String>,
}

impl Query for GetTopUsersQuery {}

/// Get users by tier query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersByTierQuery {
    pub tier: String,
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for GetUsersByTierQuery {}

/// Get users by role query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersByRoleQuery {
    pub role: String,
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for GetUsersByRoleQuery {}

/// Get users with wallets query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersWithWalletsQuery {
    pub wallet_type: Option<String>, // "ethereum", "solana", "all"
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Query for GetUsersWithWalletsQuery {}

/// Get user analytics query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserAnalyticsQuery {
    pub period: String, // "day", "week", "month", "year"
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub include_tier_distribution: bool,
    pub include_role_distribution: bool,
    pub include_registration_stats: bool,
    pub include_activity_stats: bool,
}

impl Query for GetUserAnalyticsQuery {}

/// Get user activity summary query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserActivitySummaryQuery {
    pub user_id: Uuid,
    pub include_achievements: bool,
    pub include_tier_progress: bool,
}

impl Query for GetUserActivitySummaryQuery {}

/// Get user achievements query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserAchievementsQuery {
    pub user_id: Uuid,
    pub unlocked_only: bool,
    pub category: Option<String>,
}

impl Query for GetUserAchievementsQuery {}

/// Get tier upgrade requirements query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTierUpgradeRequirementsQuery {
    pub user_id: Uuid,
    pub target_tier: String,
}

impl Query for GetTierUpgradeRequirementsQuery {}

/// Get recently registered users query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentUsersQuery {
    pub days: u32,
    pub limit: u32,
    pub include_unverified: bool,
}

impl Query for GetRecentUsersQuery {}

/// Get inactive users query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInactiveUsersQuery {
    pub days_inactive: u32,
    pub page: u32,
    pub page_size: u32,
}

impl Query for GetInactiveUsersQuery {}

/// Check username availability query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckUsernameAvailabilityQuery {
    pub username: String,
}

impl Query for CheckUsernameAvailabilityQuery {}

/// Check email availability query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckEmailAvailabilityQuery {
    pub email: String,
}

impl Query for CheckEmailAvailabilityQuery {}

/// Get user count query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserCountQuery {
    pub filter_active: Option<bool>,
    pub filter_verified: Option<bool>,
    pub tier: Option<String>,
    pub role: Option<String>,
}

impl Query for GetUserCountQuery {}

/// Get user listening history query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserListeningHistoryQuery {
    pub user_id: Uuid,
    pub page: u32,
    pub page_size: u32,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl Query for GetUserListeningHistoryQuery {}

/// Get user investment history query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserInvestmentHistoryQuery {
    pub user_id: Uuid,
    pub page: u32,
    pub page_size: u32,
    pub investment_type: Option<String>,
}

impl Query for GetUserInvestmentHistoryQuery {}

/// Get user rewards history query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRewardsHistoryQuery {
    pub user_id: Uuid,
    pub page: u32,
    pub page_size: u32,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl Query for GetUserRewardsHistoryQuery {}

/// Get user recommendations query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRecommendationsQuery {
    pub user_id: Uuid,
    pub recommendation_type: String, // "music", "artists", "campaigns", "investments"
    pub limit: u32,
}

impl Query for GetUserRecommendationsQuery {}

/// Get user social connections query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSocialConnectionsQuery {
    pub user_id: Uuid,
    pub connection_type: Option<String>, // "followers", "following", "friends"
    pub page: u32,
    pub page_size: u32,
}

impl Query for GetUserSocialConnectionsQuery {}

/// Get user notifications query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserNotificationsQuery {
    pub user_id: Uuid,
    pub unread_only: bool,
    pub notification_type: Option<String>,
    pub page: u32,
    pub page_size: u32,
}

impl Query for GetUserNotificationsQuery {}

/// Get user session history query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSessionHistoryQuery {
    pub user_id: Uuid,
    pub page: u32,
    pub page_size: u32,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl Query for GetUserSessionHistoryQuery {}

/// Advanced user search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedUserSearchQuery {
    pub criteria: UserSearchCriteria,
    pub facets: Vec<String>, // For faceted search results
    pub aggregations: Vec<String>, // For analytics
}

impl Query for AdvancedUserSearchQuery {}

/// User search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchCriteria {
    pub text_search: Option<String>, // Full-text search across multiple fields
    pub filters: HashMap<String, serde_json::Value>,
    pub date_ranges: HashMap<String, DateRange>,
    pub numeric_ranges: HashMap<String, NumericRange>,
    pub sort_criteria: Vec<SortCriterion>,
    pub page: u32,
    pub page_size: u32,
}

/// Date range for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

/// Numeric range for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Sort criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortCriterion {
    pub field: String,
    pub direction: String, // "asc" or "desc"
    pub priority: u32,
}

/// Query result wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub data: T,
    pub success: bool,
    pub message: String,
    pub execution_time_ms: u64,
}

impl<T> QueryResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            success: true,
            message: "Query executed successfully".to_string(),
            execution_time_ms: 0,
        }
    }

    pub fn with_execution_time(mut self, execution_time_ms: u64) -> Self {
        self.execution_time_ms = execution_time_ms;
        self
    }
}

/// Paginated query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedQueryResult<T> {
    pub items: Vec<T>,
    pub pagination: PaginationInfo,
    pub success: bool,
    pub message: String,
    pub execution_time_ms: u64,
}

/// Pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl PaginationInfo {
    pub fn new(page: u32, page_size: u32, total_count: u64) -> Self {
        let total_pages = if page_size > 0 {
            ((total_count as f64) / (page_size as f64)).ceil() as u32
        } else {
            0
        };

        Self {
            page,
            page_size,
            total_count,
            total_pages,
            has_next_page: page + 1 < total_pages,
            has_previous_page: page > 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_query() {
        let user_id = Uuid::new_v4();
        let query = GetUserQuery {
            user_id,
            include_profile: true,
            include_stats: true,
            include_preferences: false,
        };

        assert_eq!(query.query_type(), "GetUser");
        assert_eq!(query.user_id, user_id);
    }

    #[test]
    fn test_search_users_query_default() {
        let query = SearchUsersQuery::default();
        
        assert_eq!(query.page, 0);
        assert_eq!(query.page_size, 20);
        assert!(query.username_contains.is_none());
        assert!(query.tier.is_none());
    }

    #[test]
    fn test_get_top_users_query() {
        let query = GetTopUsersQuery {
            metric: "rewards".to_string(),
            limit: 10,
            time_period: Some("month".to_string()),
            tier_filter: Some("premium".to_string()),
        };

        assert_eq!(query.query_type(), "GetTopUsers");
        assert_eq!(query.metric, "rewards");
        assert_eq!(query.limit, 10);
    }

    #[test]
    fn test_pagination_info() {
        let pagination = PaginationInfo::new(0, 20, 100);
        
        assert_eq!(pagination.total_pages, 5);
        assert!(!pagination.has_previous_page);
        assert!(pagination.has_next_page);

        let last_page = PaginationInfo::new(4, 20, 100);
        assert!(!last_page.has_next_page);
        assert!(last_page.has_previous_page);
    }

    #[test]
    fn test_query_result() {
        let result = QueryResult::success("test data".to_string())
            .with_execution_time(150);
        
        assert!(result.success);
        assert_eq!(result.data, "test data");
        assert_eq!(result.execution_time_ms, 150);
    }
} 