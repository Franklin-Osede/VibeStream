use sqlx::{PgPool, Postgres, Row};
use std::env;
use crate::domain::errors::FractionalOwnershipError;

/// Database connection pool manager
#[derive(Clone)]
pub struct DatabaseConnection {
    pool: PgPool,
}

impl DatabaseConnection {
    /// Crear nueva conexión desde variables de entorno
    pub async fn new() -> Result<Self, FractionalOwnershipError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| FractionalOwnershipError::InfrastructureError("DATABASE_URL no configurada".to_string()))?;

        let pool = PgPool::connect(&database_url)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error conectando a DB: {}", e)))?;

        Ok(Self { pool })
    }

    /// Crear conexión con URL personalizada
    pub async fn new_with_url(database_url: &str) -> Result<Self, FractionalOwnershipError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error conectando a DB: {}", e)))?;

        Ok(Self { pool })
    }

    /// Obtener referencia al pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Verificar que la conexión esté activa
    pub async fn health_check(&self) -> Result<(), FractionalOwnershipError> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Health check falló: {}", e)))?;

        Ok(())
    }

    /// Ejecutar migraciones
    pub async fn run_migrations(&self) -> Result<(), FractionalOwnershipError> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error ejecutando migraciones: {}", e)))?;

        Ok(())
    }

    /// Cerrar todas las conexiones
    pub async fn close(&self) {
        self.pool.close().await;
    }
}

/// Configuración de base de datos
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

impl DatabaseConfig {
    /// Cargar configuración desde variables de entorno
    pub fn from_env() -> Result<Self, FractionalOwnershipError> {
        Ok(Self {
            host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .map_err(|_| FractionalOwnershipError::InfrastructureError("DB_PORT inválido".to_string()))?,
            database: env::var("DB_NAME")
                .map_err(|_| FractionalOwnershipError::InfrastructureError("DB_NAME no configurado".to_string()))?,
            username: env::var("DB_USER")
                .map_err(|_| FractionalOwnershipError::InfrastructureError("DB_USER no configurado".to_string()))?,
            password: env::var("DB_PASSWORD")
                .map_err(|_| FractionalOwnershipError::InfrastructureError("DB_PASSWORD no configurado".to_string()))?,
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            acquire_timeout_seconds: env::var("DB_ACQUIRE_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            idle_timeout_seconds: env::var("DB_IDLE_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .unwrap_or(600),
            max_lifetime_seconds: env::var("DB_MAX_LIFETIME")
                .unwrap_or_else(|_| "1800".to_string())
                .parse()
                .unwrap_or(1800),
        })
    }

    /// Construir URL de conexión
    pub fn connection_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database
        )
    }

    /// Crear conexión con esta configuración
    pub async fn create_connection(&self) -> Result<DatabaseConnection, FractionalOwnershipError> {
        use sqlx::postgres::PgPoolOptions;
        use std::time::Duration;

        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(Duration::from_secs(self.acquire_timeout_seconds))
            .idle_timeout(Duration::from_secs(self.idle_timeout_seconds))
            .max_lifetime(Duration::from_secs(self.max_lifetime_seconds))
            .connect(&self.connection_url())
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error creando pool: {}", e)))?;

        Ok(DatabaseConnection { pool })
    }
}

/// Helper para transacciones
pub struct DatabaseTransaction<'a> {
    tx: sqlx::Transaction<'a, Postgres>,
}

impl<'a> DatabaseTransaction<'a> {
    /// Comenzar nueva transacción
    pub async fn begin(pool: &PgPool) -> Result<DatabaseTransaction<'_>, FractionalOwnershipError> {
        let tx = pool
            .begin()
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error iniciando transacción: {}", e)))?;

        Ok(DatabaseTransaction { tx })
    }

    /// Confirmar transacción
    pub async fn commit(self) -> Result<(), FractionalOwnershipError> {
        self.tx
            .commit()
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error confirmando transacción: {}", e)))?;

        Ok(())
    }

    /// Cancelar transacción
    pub async fn rollback(self) -> Result<(), FractionalOwnershipError> {
        self.tx
            .rollback()
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error cancelando transacción: {}", e)))?;

        Ok(())
    }

    /// Obtener referencia al executor
    pub fn executor(&mut self) -> &mut sqlx::Transaction<'a, Postgres> {
        &mut self.tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_connection_url_correctly() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            database: "test_db".to_string(),
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
            max_connections: 10,
            min_connections: 1,
            acquire_timeout_seconds: 30,
            idle_timeout_seconds: 600,
            max_lifetime_seconds: 1800,
        };

        let url = config.connection_url();
        assert_eq!(url, "postgresql://test_user:test_pass@localhost:5432/test_db");
    }

    #[tokio::test]
    async fn should_handle_invalid_connection_url() {
        let result = DatabaseConnection::new_with_url("invalid_url").await;
        assert!(result.is_err());
    }
} 