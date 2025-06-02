use std::sync::Arc;
use vibestream_backend::{
    config::{AppConfig, SecretsManager},
    db::migrations::run_migrations,
    db::create_connection,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Cargar configuración
    let config = AppConfig::new()?;
    let vault_client = config.init_vault_client().await?;
    let secrets = SecretsManager::new(Arc::new(vault_client), config.vault.mount_path.clone());

    // Conectar a la base de datos
    let db = create_connection(&config, &secrets).await?;

    // Ejecutar migraciones
    println!("Ejecutando migraciones...");
    run_migrations(&db).await?;
    println!("¡Migraciones completadas!");

    Ok(())
} 