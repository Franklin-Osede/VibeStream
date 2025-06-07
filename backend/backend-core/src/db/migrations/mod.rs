use sea_orm::{DatabaseConnection, ConnectionTrait};
use sqlx::migrate::{Migrator, MigrateDatabase};
use anyhow::Result;
use std::path::Path;

pub async fn run_migrations(db: &DatabaseConnection) -> Result<()> {
    let db_backend = db.get_database_backend();
    let db_url = match db_backend {
        sea_orm::DatabaseBackend::Postgres => "postgres",
        sea_orm::DatabaseBackend::MySql => "mysql",
        sea_orm::DatabaseBackend::Sqlite => "sqlite",
    };
    
    // Las siguientes líneas se comentan porque la feature "migrate" de sqlx
    // causa conflictos de dependencias con las librerías de Solana.
    // La creación de la base de datos debe gestionarse de forma externa.
    // if !sqlx::Postgres::database_exists(db_url).await? {
    //     sqlx::Postgres::create_database(db_url).await?;
    // }

    // Cargar y ejecutar migraciones
    let migrations_path = Path::new("./migrations");
    let migrator = Migrator::new(migrations_path).await?;
    
    let pool = sqlx::PgPool::connect(db_url).await?;
    migrator.run(&pool).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, SecretsManager};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_migrations() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(Arc::new(vault_client), config.vault.mount_path.clone());
        
        let db = crate::db::create_connection(&config, &secrets)
            .await
            .expect("Failed to connect to database");

        let result = run_migrations(&db).await;
        assert!(result.is_ok(), "Migrations should run successfully");
    }
} 