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
// 
// MessageQueue usa redis::aio::ConnectionManager para conexiones completamente async
// Evita bloquear el runtime de Tokio con operaciones síncronas

use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct MessageQueue {
    connection_manager: ConnectionManager,
}

impl MessageQueue {
    /// Crear una nueva instancia de MessageQueue con conexiones async
    /// 
    /// # Arguments
    /// * `redis_url` - URL de conexión a Redis (ej: "redis://localhost:6379")
    /// 
    /// # Returns
    /// * `Result<Self>` - MessageQueue inicializado o error
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = RedisClient::open(redis_url)?;
        
        // Usar ConnectionManager para conexiones async compartidas
        let connection_manager = ConnectionManager::new(client.clone()).await?;
        
        // Test connection usando async
        let mut conn = connection_manager.clone();
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await?;
        
        Ok(Self { connection_manager })
    }
    
    /// Verificar conexión con Redis (async)
    pub async fn ping(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.connection_manager.clone();
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    /// Enviar mensaje a una cola Redis (async)
    /// 
    /// # Arguments
    /// * `queue_name` - Nombre de la cola
    /// * `message` - Mensaje a enviar
    /// 
    /// # Returns
    /// * `Result<()>` - Éxito o error
    pub async fn send_message(&self, queue_name: &str, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.connection_manager.clone();
        let _: i64 = redis::cmd("LPUSH")
            .arg(queue_name)
            .arg(message)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }
    
    /// Recibir mensaje de una cola Redis (async, bloqueante hasta que haya mensaje)
    /// 
    /// # Arguments
    /// * `queue_name` - Nombre de la cola
    /// * `timeout_seconds` - Timeout en segundos (0 = sin timeout)
    /// 
    /// # Returns
    /// * `Result<Option<String>>` - Mensaje recibido o None si timeout
    pub async fn receive_message(&self, queue_name: &str, timeout_seconds: u64) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.connection_manager.clone();
        
        if timeout_seconds > 0 {
            // BRPOP con timeout
            let result: Option<(String, String)> = redis::cmd("BRPOP")
                .arg(queue_name)
                .arg(timeout_seconds)
                .query_async(&mut conn)
                .await?;
            
            Ok(result.map(|(_, message)| message))
        } else {
            // RPOP sin timeout (no bloqueante)
            let result: Option<String> = redis::cmd("RPOP")
                .arg(queue_name)
                .query_async(&mut conn)
                .await?;
            
            Ok(result)
        }
    }
    
    /// Obtener longitud de una cola
    pub async fn queue_length(&self, queue_name: &str) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.connection_manager.clone();
        let length: i64 = redis::cmd("LLEN")
            .arg(queue_name)
            .query_async(&mut conn)
            .await?;
        Ok(length as usize)
    }
} 