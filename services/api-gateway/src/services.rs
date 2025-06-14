use redis::{AsyncCommands, Client};
use vibestream_types::*;
use async_trait::async_trait;

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

pub struct MessageQueue {
    broker: RedisMessageBroker,
}

impl MessageQueue {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let broker = RedisMessageBroker::new(redis_url).await?;
        Ok(Self { broker })
    }
    
    pub async fn send_ethereum_message(&self, message: EthereumMessage) -> Result<()> {
        let service_message = ServiceMessage::new(message);
        let serialized = serde_json::to_string(&service_message)
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        self.broker.send_message(QueueNames::ETHEREUM, &serialized).await
    }
    
    pub async fn send_solana_message(&self, message: SolanaMessage) -> Result<()> {
        let service_message = ServiceMessage::new(message);
        let serialized = serde_json::to_string(&service_message)
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        self.broker.send_message(QueueNames::SOLANA, &serialized).await
    }
    
    pub async fn send_zk_message(&self, message: ZkMessage) -> Result<()> {
        let service_message = ServiceMessage::new(message);
        let serialized = serde_json::to_string(&service_message)
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Serialization error: {}", e) 
            })?;
        
        self.broker.send_message(QueueNames::ZK, &serialized).await
    }
    
    pub async fn receive_response(&self) -> Result<Option<ServiceResponse>> {
        if let Some(message) = self.broker.receive_message(QueueNames::RESPONSES).await? {
            let response: ServiceResponse = serde_json::from_str(&message)
                .map_err(|e| VibeStreamError::Internal { 
                    message: format!("Deserialization error: {}", e) 
                })?;
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
} 