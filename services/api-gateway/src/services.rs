use std::sync::Arc;
use sqlx::PgPool;
use redis::Client as RedisClient;
use crate::shared::domain::errors::AppError;

// =============================================================================
// DATABASE POOL SERVICE
// =============================================================================

#[derive(Clone)]
pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // Test connection
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await?;
        
        Ok(Self { pool })
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    
    pub async fn health_check(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// =============================================================================
// MESSAGE QUEUE SERVICE
// =============================================================================

#[derive(Clone)]
pub struct MessageQueue {
    client: RedisClient,
}

impl MessageQueue {
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = RedisClient::open(redis_url)?;
        
        // Test connection using sync connection
        let mut conn = client.get_connection()?;
        let _: String = redis::cmd("PING").query(&mut conn)?;
        
        Ok(Self { client })
    }
    
    pub async fn ping(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.client.get_connection()?;
        let _: String = redis::cmd("PING").query(&mut conn)?;
        Ok(())
    }

    pub async fn send_message(&self, queue_name: &str, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.client.get_connection()?;
        let _: i64 = redis::cmd("LPUSH")
            .arg(queue_name)
            .arg(message)
            .query(&mut conn)?;
        Ok(())
    }
} 