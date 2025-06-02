use serde::Deserialize;
use hashicorp_vault::client::{VaultClient, TokenData};
use crate::error::AppError;

mod secrets;
pub use secrets::SecretsManager;

use std::sync::Arc;
use config::{Config, ConfigError, Environment, File};
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub vault: VaultConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct VaultConfig {
    pub address: String,
    pub token: String,
    pub mount_path: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, AppError> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(|e| AppError::ConfigError(e.to_string()))?;

        config.try_deserialize()
            .map_err(|e| AppError::ConfigError(e.to_string()))
    }

    pub async fn init_vault_client(&self) -> Result<VaultClient<TokenData>, AppError> {
        let vault_client = VaultClient::new(
            &self.vault.address,
            &self.vault.token
        ).map_err(|e| AppError::ConfigError(e.to_string()))?;

        Ok(vault_client)
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