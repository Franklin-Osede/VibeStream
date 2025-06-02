use sea_orm::{Database, DatabaseConnection};
use crate::{
    config::{AppConfig, SecretsManager},
    error::AppError,
};

pub mod migrations;

pub async fn create_connection(
    config: &AppConfig,
    _secrets: &SecretsManager,
) -> Result<DatabaseConnection, AppError> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name
    );

    Database::connect(&database_url)
        .await
        .map_err(AppError::DatabaseError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_database_connection() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(Arc::new(vault_client), config.vault.mount_path.clone());
        
        let connection = create_connection(&config, &secrets).await;
        assert!(connection.is_ok(), "Should connect to database successfully");
    }
} 