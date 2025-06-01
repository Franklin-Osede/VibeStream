use sea_orm::{Database, DatabaseConnection};
use anyhow::Result;
use crate::config::{AppConfig, SecretsManager};

pub mod models;
pub mod migrations;

pub async fn create_connection(config: &AppConfig, secrets: &SecretsManager) -> Result<DatabaseConnection> {
    let db_secrets = secrets.get_database_secrets().await?;
    
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_secrets.username,
        db_secrets.password,
        db_secrets.host,
        db_secrets.port,
        db_secrets.database
    );

    let connection_options = sea_orm::ConnectOptions::new(database_url)
        .max_connections(config.database.max_connections)
        .connect_timeout(std::time::Duration::from_secs(config.database.connection_timeout))
        .sqlx_logging(true);

    Database::connect(connection_options)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))
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