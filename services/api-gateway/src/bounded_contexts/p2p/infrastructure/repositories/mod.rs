pub mod analytics_repository;
pub mod in_memory_analytics_repository;
pub mod postgresql_analytics_repository;

pub use analytics_repository::*;
pub use in_memory_analytics_repository::*;
pub use postgresql_analytics_repository::*;

/// Factory para crear repositorios de analíticas P2P
pub struct P2PAnalyticsRepositoryFactory;

impl P2PAnalyticsRepositoryFactory {
    /// Crear repositorio en memoria (para desarrollo/testing)
    pub fn create_in_memory() -> Box<dyn P2PAnalyticsRepository> {
        Box::new(InMemoryP2PAnalyticsRepository::new())
    }

    /// Crear repositorio PostgreSQL (para producción)
    pub async fn create_postgresql(pool: sqlx::PgPool) -> Result<Box<dyn P2PAnalyticsRepository>, Box<dyn std::error::Error>> {
        let repository = PostgreSQLP2PAnalyticsRepository::new(pool);
        
        // Crear tablas si no existen
        repository.create_tables().await?;
        
        Ok(Box::new(repository))
    }
} 