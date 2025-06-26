use async_trait::async_trait;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;

#[async_trait]
pub trait Repository<T> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<T>, AppError>;
    async fn save(&self, entity: &T) -> Result<(), AppError>;
}

// Alias de resultados comunes de repositorio
pub type RepoResult<T> = Result<T, AppError>; 