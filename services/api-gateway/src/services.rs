use redis::{AsyncCommands, Client, Connection};
use vibestream_types::*;
use async_trait::async_trait;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
// use crate::shared::application::bus::InMemoryCommandBus;  // Comentado temporalmente

// Nuevo struct para manejar la base de datos
#[derive(Clone)]
pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .map_err(|e| VibeStreamError::Database {
                message: format!("Failed to connect to PostgreSQL: {}", e),
            })?;

        // Test the connection
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .map_err(|e| VibeStreamError::Database {
                message: format!("Database connection test failed: {}", e),
            })?;

        tracing::info!("✅ Conexión PostgreSQL establecida exitosamente");
        
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| VibeStreamError::Database {
                message: format!("Database health check failed: {}", e),
            })?;
        
        Ok(())
    }
}

pub struct RedisMessageBroker {
    client: Client,
}

impl RedisMessageBroker {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to connect to Redis: {}", e) 
            })?;
        
        Ok(Self { client })
    }
}

#[async_trait]
impl MessageBroker for RedisMessageBroker {
    async fn send_message(&self, queue: &str, message: &str) -> Result<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Redis connection error: {}", e) 
            })?;
        
        let _: () = conn.lpush(queue, message).await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to send message: {}", e) 
            })?;
        
        Ok(())
    }
    
    async fn receive_message(&self, queue: &str) -> Result<Option<String>> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Redis connection error: {}", e) 
            })?;
        
        let result: Option<String> = conn.rpop(queue, None).await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to receive message: {}", e) 
            })?;
        
        Ok(result)
    }
    
    async fn receive_message_blocking(&self, queue: &str, timeout_secs: u64) -> Result<Option<String>> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Redis connection error: {}", e) 
            })?;
        
        let result: Option<Vec<String>> = conn.brpop(queue, timeout_secs as f64).await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to receive message: {}", e) 
            })?;
        
        Ok(result.and_then(|mut v| v.pop()))
    }
}

#[derive(Clone)]
pub struct MessageQueue {
    client: Client,
}

impl MessageQueue {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| VibeStreamError::Network {
                message: format!("Failed to create Redis client: {}", e),
            })?;

        // Test connection
        let mut conn = client
            .get_connection()
            .map_err(|e| VibeStreamError::Network {
                message: format!("Failed to connect to Redis: {}", e),
            })?;

        // Ping to verify connection
        redis::cmd("PING")
            .query::<String>(&mut conn)
            .map_err(|e| VibeStreamError::Network {
                message: format!("Redis ping failed: {}", e),
            })?;

        Ok(Self { client })
    }

    pub async fn ping(&self) -> Result<()> {
        let mut conn = self.client
            .get_connection()
            .map_err(|e| VibeStreamError::Network {
                message: format!("Failed to get Redis connection: {}", e),
            })?;

        redis::cmd("PING")
            .query::<String>(&mut conn)
            .map_err(|e| VibeStreamError::Network {
                message: format!("Redis ping failed: {}", e),
            })?;

        Ok(())
    }

    async fn get_connection(&self) -> Result<Connection> {
        self.client
            .get_connection()
            .map_err(|e| VibeStreamError::Network {
                message: format!("Failed to get Redis connection: {}", e),
            })
    }
}

#[async_trait]
impl MessageBroker for MessageQueue {
    async fn send_message(&self, queue: &str, message: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        redis::cmd("LPUSH")
            .arg(queue)
            .arg(message)
            .query::<i32>(&mut conn)
            .map_err(|e| VibeStreamError::Network {
                message: format!("Failed to send message to queue {}: {}", queue, e),
            })?;

        tracing::debug!("Message sent to queue {}: {}", queue, message);
        Ok(())
    }

    async fn receive_message(&self, queue: &str) -> Result<Option<String>> {
        let mut conn = self.get_connection().await?;
        
        let result: Option<String> = redis::cmd("RPOP")
            .arg(queue)
            .query(&mut conn)
            .map_err(|e| VibeStreamError::Network {
                message: format!("Failed to receive message from queue {}: {}", queue, e),
            })?;

        if let Some(ref msg) = result {
            tracing::debug!("Message received from queue {}: {}", queue, msg);
        }

        Ok(result)
    }

    async fn receive_message_blocking(&self, queue: &str, timeout_secs: u64) -> Result<Option<String>> {
        let mut conn = self.get_connection().await?;
        
        let result: Option<Vec<String>> = redis::cmd("BRPOP")
            .arg(queue)
            .arg(timeout_secs)
            .query(&mut conn)
            .map_err(|e| VibeStreamError::Network {
                message: format!("Failed to receive blocking message from queue {}: {}", queue, e),
            })?;

        match result {
            Some(mut values) if values.len() >= 2 => {
                let message = values.pop().unwrap();
                tracing::debug!("Blocking message received from queue {}: {}", queue, message);
                Ok(Some(message))
            }
            _ => Ok(None),
        }
    }


}

impl MessageQueue {
    pub async fn send_ethereum_message(&self, message: EthereumMessage) -> Result<()> {
        let service_message = ServiceMessage::new(message);
        let serialized = serde_json::to_string(&service_message)
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        self.send_message(QueueNames::ETHEREUM, &serialized).await
    }
    
    pub async fn send_solana_message(&self, message: SolanaMessage) -> Result<()> {
        let service_message = ServiceMessage::new(message);
        let serialized = serde_json::to_string(&service_message)
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        self.send_message(QueueNames::SOLANA, &serialized).await
    }
    
    pub async fn send_zk_message(&self, message: ZkMessage) -> Result<()> {
        let service_message = ServiceMessage::new(message);
        let serialized = serde_json::to_string(&service_message)
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        self.send_message(QueueNames::ZK, &serialized).await
    }
    
    pub async fn receive_response(&self) -> Result<Option<ServiceResponse>> {
        match self.receive_message("response_queue").await? {
            Some(message) => {
                let response: ServiceResponse = serde_json::from_str(&message)
                    .map_err(|e| VibeStreamError::Internal {
                        message: format!("Failed to deserialize response: {}", e),
                    })?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub message_queue: MessageQueue,
    pub database_pool: DatabasePool,
    // pub command_bus: Arc<InMemoryCommandBus>,  // Comentado temporalmente
    // pub blockchain_clients: crate::blockchain::BlockchainClients, // Comentado temporalmente
} 