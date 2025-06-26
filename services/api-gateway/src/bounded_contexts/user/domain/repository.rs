use async_trait::async_trait;
use uuid::Uuid;

use crate::shared::domain::repositories::RepoResult;
use super::entities::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> RepoResult<Option<User>>;
    async fn save(&self, user: &User) -> RepoResult<()>;
} 