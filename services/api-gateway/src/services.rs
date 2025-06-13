use redis::{AsyncCommands, Client};
use vibestream_types::*;

pub struct MessageQueue {
    client: Client,
}

impl MessageQueue {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to connect to Redis: {}", e) 
            })?;
        
        Ok(Self { client })
    }
    
    pub async fn send_ethereum_message(&self, message: EthereumMessage) -> Result<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Redis connection error: {}", e) 
            })?;
        
        let serialized = serde_json::to_string(&ServiceMessage::new(message))
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        let _: () = conn.lpush("ethereum_queue", serialized).await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to send message: {}", e) 
            })?;
        
        Ok(())
    }
    
    pub async fn send_solana_message(&self, message: SolanaMessage) -> Result<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Redis connection error: {}", e) 
            })?;
        
        let serialized = serde_json::to_string(&ServiceMessage::new(message))
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        let _: () = conn.lpush("solana_queue", serialized).await
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to send message: {}", e) 
            })?;
        
        Ok(())
    }
} 