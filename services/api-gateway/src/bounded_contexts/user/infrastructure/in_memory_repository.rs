use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::bounded_contexts::user::domain::{
    aggregates::{UserAggregate, UserSummary},
    entities::{UserStats},
    repository::{UserRepository, UserSearchCriteria},
    value_objects::{UserId, Email, Username, UserTier, UserRole},
};
use crate::shared::domain::errors::AppError;

pub struct InMemoryUserRepository {
    users: RwLock<HashMap<UserId, UserAggregate>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, aggregate: &UserAggregate) -> Result<(), AppError> {
        let mut users = self.users.write().unwrap();
        users.insert(aggregate.user.id.clone(), aggregate.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<UserAggregate>, AppError> {
        let users = self.users.read().unwrap();
        Ok(users.get(id).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<UserAggregate>, AppError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|user| &user.user.email == email).cloned())
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<UserAggregate>, AppError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|user| &user.user.username == username).cloned())
    }

    async fn email_exists(&self, email: &Email) -> Result<bool, AppError> {
        Ok(self.find_by_email(email).await?.is_some())
    }

    async fn username_exists(&self, username: &Username) -> Result<bool, AppError> {
        Ok(self.find_by_username(username).await?.is_some())
    }

    async fn find_users(&self, criteria: UserSearchCriteria) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .filter(|user| {
                // Apply filters based on criteria
                if let Some(ref username) = criteria.username_contains {
                    if !user.user.username.value().contains(username) {
                        return false;
                    }
                }
                if let Some(ref email) = criteria.email_contains {
                    if !user.user.email.value().contains(email) {
                        return false;
                    }
                }
                if let Some(ref tier) = criteria.tier {
                    if format!("{}", user.user.tier) != *tier {
                        return false;
                    }
                }
                if let Some(ref role) = criteria.role {
                    if format!("{}", user.user.role) != *role {
                        return false;
                    }
                }
                if let Some(is_active) = criteria.is_active {
                    if user.user.is_active != is_active {
                        return false;
                    }
                }
                if let Some(is_verified) = criteria.is_verified {
                    if user.user.is_verified != is_verified {
                        return false;
                    }
                }
                true
            })
            .map(|user| user.get_summary())
            .collect();

        // Apply pagination
        let start = (criteria.page * criteria.page_size) as usize;
        let end = start + criteria.page_size as usize;
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results = Vec::new();
        }

        Ok(results)
    }

    async fn find_active_users(&self, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .filter(|user| user.user.is_active)
            .map(|user| user.get_summary())
            .collect();

        // Apply pagination
        let start = (page * page_size) as usize;
        let end = start + page_size as usize;
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results = Vec::new();
        }

        Ok(results)
    }

    async fn find_by_tier(&self, tier: &str, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .filter(|user| format!("{}", user.user.tier) == tier)
            .map(|user| user.get_summary())
            .collect();

        // Apply pagination
        let start = (page * page_size) as usize;
        let end = start + page_size as usize;
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results = Vec::new();
        }

        Ok(results)
    }

    async fn find_by_role(&self, role: &str, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .filter(|user| format!("{}", user.user.role) == role)
            .map(|user| user.get_summary())
            .collect();

        // Apply pagination
        let start = (page * page_size) as usize;
        let end = start + page_size as usize;
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results = Vec::new();
        }

        Ok(results)
    }

    async fn count_users(&self) -> Result<u64, AppError> {
        let users = self.users.read().unwrap();
        Ok(users.len() as u64)
    }

    async fn get_user_stats(&self, user_id: &UserId) -> Result<Option<UserStats>, AppError> {
        let users = self.users.read().unwrap();
        Ok(users.get(user_id).map(|user| user.stats.clone()))
    }

    async fn delete(&self, id: &UserId) -> Result<(), AppError> {
        let mut users = self.users.write().unwrap();
        users.remove(id);
        Ok(())
    }

    async fn find_users_registered_between(
        &self,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>
    ) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let results: Vec<UserSummary> = users.values()
            .filter(|user| {
                user.user.created_at >= start_date && user.user.created_at <= end_date
            })
            .map(|user| user.get_summary())
            .collect();

        Ok(results)
    }

    async fn find_top_users_by_rewards(&self, limit: u32) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .map(|user| user.get_summary())
            .collect();

        // Sort by total rewards descending
        results.sort_by(|a, b| b.total_rewards.partial_cmp(&a.total_rewards).unwrap());
        results.truncate(limit as usize);

        Ok(results)
    }

    async fn find_top_users_by_listening_time(&self, limit: u32) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .map(|user| user.get_summary())
            .collect();

        // Sort by total listening time descending
        results.sort_by(|a, b| b.total_listening_time.cmp(&a.total_listening_time));
        results.truncate(limit as usize);

        Ok(results)
    }

    async fn find_users_with_wallets(&self, page: u32, page_size: u32) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .filter(|user| user.user.wallet_address.is_some())
            .map(|user| user.get_summary())
            .collect();

        // Apply pagination
        let start = (page * page_size) as usize;
        let end = start + page_size as usize;
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results = Vec::new();
        }

        Ok(results)
    }

    async fn find_users_by_tier_points_range(
        &self,
        min_points: u32,
        max_points: u32,
        page: u32,
        page_size: u32
    ) -> Result<Vec<UserSummary>, AppError> {
        let users = self.users.read().unwrap();
        let mut results: Vec<UserSummary> = users.values()
            .filter(|user| {
                user.stats.tier_points >= min_points && user.stats.tier_points <= max_points
            })
            .map(|user| user.get_summary())
            .collect();

        // Apply pagination
        let start = (page * page_size) as usize;
        let end = start + page_size as usize;
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results = Vec::new();
        }

        Ok(results)
    }
} 