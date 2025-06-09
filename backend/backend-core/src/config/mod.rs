use serde::Deserialize;
use hashicorp_vault::client::{VaultClient, TokenData};
use crate::error::AppError;
use std::env;

pub mod secrets;

use std::sync::Arc;
use config::{Config, ConfigError, Environment, File};
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub solana: SolanaConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SolanaConfig {
    pub service_url: String,
}

#[derive(Debug, Deserialize)]
pub struct VaultConfig {
    pub address: String,
    pub token: String,
    pub mount_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        Ok(Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            },
            solana: SolanaConfig {
                service_url: env::var("SOLANA_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            },
        })
    }
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize()
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