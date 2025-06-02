use std::sync::Arc;
use serde::Deserialize;
use hashicorp_vault::client::{VaultClient, TokenData};
use crate::error::AppError;
use anyhow::Result;
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize)]
pub struct DatabaseSecrets {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtSecrets {
    pub secret: String,
    pub expiration: String,
}

#[derive(Debug, Deserialize)]
pub struct Web3Secrets {
    pub private_key: String,
    pub rpc_url: String,
    pub chain_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisSecrets {
    pub host: String,
    pub port: String,
}

pub struct SecretsManager {
    client: Arc<VaultClient<TokenData>>,
    mount_path: String,
}

impl SecretsManager {
    pub fn new(client: Arc<VaultClient<TokenData>>, mount_path: String) -> Self {
        Self {
            client,
            mount_path,
        }
    }

    pub async fn get_database_secrets(&self) -> Result<DatabaseSecrets, AppError> {
        let secret = self.client
            .get_secret(&format!("{}/database", self.mount_path))
            .map_err(|e| AppError::ConfigError(e.to_string()))?;

        let secret: JsonValue = serde_json::from_str(&secret)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse secret: {}", e)))?;

        Ok(DatabaseSecrets {
            username: secret["username"].as_str().unwrap_or_default().to_string(),
            password: secret["password"].as_str().unwrap_or_default().to_string(),
            host: secret["host"].as_str().unwrap_or_default().to_string(),
            port: secret["port"].as_str().unwrap_or_default().parse().unwrap_or(5432),
            database: secret["database"].as_str().unwrap_or_default().to_string(),
        })
    }

    pub async fn get_jwt_secrets(&self) -> Result<JwtSecrets> {
        self.get_secret("jwt").await
    }

    pub async fn get_web3_secrets(&self) -> Result<Web3Secrets> {
        self.get_secret("web3").await
    }

    pub async fn get_redis_secrets(&self) -> Result<RedisSecrets> {
        self.get_secret("redis").await
    }

    async fn get_secret<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let secret = self.client
            .get_secret(&format!("{}/{}", self.mount_path, path))
            .map_err(|e| anyhow::anyhow!("Failed to get secret from Vault: {}", e))?;

        let secret: JsonValue = serde_json::from_str(&secret)
            .map_err(|e| anyhow::anyhow!("Failed to parse secret: {}", e))?;

        serde_json::from_value(secret)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize secret: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_get_database_secrets() {
        let config = AppConfig::new().unwrap();
        let client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(client, config.vault.mount_path);
        
        let db_secrets = secrets.get_database_secrets().await;
        assert!(db_secrets.is_ok(), "Should get database secrets successfully");
    }

    #[tokio::test]
    async fn test_get_jwt_secrets() {
        let config = AppConfig::new().unwrap();
        let client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(client, config.vault.mount_path);
        
        let jwt_secrets = secrets.get_jwt_secrets().await;
        assert!(jwt_secrets.is_ok(), "Should get JWT secrets successfully");
    }
} 