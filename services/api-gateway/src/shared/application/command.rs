use async_trait::async_trait;

use crate::shared::domain::errors::AppError;

/// Representa un comando que muta el estado del sistema.
pub trait Command: Send + Sync {}

/// Un manejador de comando.
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    type Output: Send + 'static;

    async fn handle(&self, command: C) -> Result<Self::Output, AppError>;
} 