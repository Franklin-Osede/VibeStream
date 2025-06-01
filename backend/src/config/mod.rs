use std::sync::Arc;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use hashicorp_vault::client::{VaultClient, VaultClientSettingsBuilder};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub vault: VaultConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub connection_timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct VaultConfig {
    pub addr: String,
    pub token: String,
    pub mount_path: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub ttl: u64,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Base config
            .add_source(File::with_name("config/base/app"))
            // Override with environment specific config
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()?;

        config.try_deserialize()
    }

    pub async fn init_vault_client(&self) -> Result<Arc<VaultClient>, anyhow::Error> {
        let client = VaultClientSettingsBuilder::default()
            .address(&self.vault.addr)
            .token(&self.vault.token)
            .build()
            .map(VaultClient::new)?;

        // Verificar conexión
        client.get_secret(&format!("{}/database", self.vault.mount_path))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to Vault: {}", e))?;

        Ok(Arc::new(client))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config = AppConfig::new();
        assert!(config.is_ok(), "Should load config successfully");
    }

    #[tokio::test]
    async fn test_vault_client_init() {
        let config = AppConfig::new().unwrap();
        let client = config.init_vault_client().await;
        assert!(client.is_ok(), "Should initialize Vault client successfully");
    }
} 