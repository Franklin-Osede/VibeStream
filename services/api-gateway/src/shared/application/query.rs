use async_trait::async_trait;

use crate::shared::domain::errors::AppError;

/// Representa una consulta que NO muta estado (CQRS).
pub trait Query: Send + Sync {}

/// Manejador de consulta.
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    type Output: Send + 'static;

    async fn handle(&self, query: Q) -> Result<Self::Output, AppError>;
} 