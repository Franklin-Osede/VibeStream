use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::listen_reward::{
    domain::entities::listen_session::ListenSession,
    domain::value_objects::ListenSessionId,
    infrastructure::repositories::repository_traits::ListenSessionRepository,
    infrastructure::repositories::{RepositoryResult, Pagination, ListenSessionFilter},
};

/// Mock repository for Listen Reward context testing
#[derive(Debug, Clone)]
pub struct MockListenSessionRepository;

#[async_trait]
impl ListenSessionRepository for MockListenSessionRepository {
    async fn save(&self, _session: &ListenSession) -> RepositoryResult<()> {
        Ok(())
    }
    
    async fn update(&self, _session: &ListenSession, _expected_version: i32) -> RepositoryResult<()> {
        Ok(())
    }
    
    async fn find_by_id(&self, _id: &ListenSessionId) -> RepositoryResult<Option<ListenSession>> {
        Ok(None)
    }
    
    async fn delete(&self, _id: &ListenSessionId) -> RepositoryResult<()> {
        Ok(())
    }
    
    async fn exists(&self, _id: &ListenSessionId) -> RepositoryResult<bool> {
        Ok(false)
    }
    
    async fn find_active_sessions_for_user(&self, _user_id: Uuid) -> RepositoryResult<Vec<ListenSession>> {
        Ok(vec![])
    }
    
    async fn count_user_sessions_in_period(
        &self,
        _user_id: Uuid,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> RepositoryResult<i64> {
        Ok(0)
    }
} 