//! Test Fixtures and Helpers
//! 
//! Utilities para configurar servicios de test (Postgres, Redis)
//! y ejecutar tests de integración

use std::process::{Command, Child};
use std::time::Duration;
use std::thread;

/// Configuración de servicios para tests
pub struct TestServices {
    pub postgres_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}

impl TestServices {
    /// Crear configuración de test con valores por defecto
    pub fn new() -> Self {
        Self {
            postgres_url: std::env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://vibestream:vibestream@localhost:5433/vibestream_test".to_string()),
            redis_url: std::env::var("TEST_REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379/1".to_string()),
            jwt_secret: std::env::var("TEST_JWT_SECRET")
                .unwrap_or_else(|_| "test_secret_key_for_testing_only".to_string()),
        }
    }

    /// Verificar que los servicios estén disponibles
    pub async fn check_services(&self) -> Result<(), String> {
        // Verificar PostgreSQL
        let pg_result = sqlx::PgPool::connect(&self.postgres_url).await;
        if pg_result.is_err() {
            return Err(format!("PostgreSQL no disponible en {}", self.postgres_url));
        }
        drop(pg_result.unwrap());

        // Verificar Redis
        let redis_result = redis::Client::open(&self.redis_url);
        if redis_result.is_err() {
            return Err(format!("Redis no disponible en {}", self.redis_url));
        }

        let client = redis_result.unwrap();
        let mut conn = match client.get_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return Err("No se pudo conectar a Redis".to_string()),
        };

        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("Redis PING falló: {}", e))?;

        Ok(())
    }
}

impl Default for TestServices {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper para ejecutar migraciones de test
pub async fn setup_test_database(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ejecutar migraciones usando sqlx-cli si está disponible
    // O ejecutar SQL directamente
    let pool = sqlx::PgPool::connect(database_url).await?;
    
    // Crear esquema de test si no existe
    sqlx::query("CREATE SCHEMA IF NOT EXISTS test")
        .execute(&pool)
        .await?;
    
    Ok(())
}

/// Helper para limpiar datos de test
pub async fn cleanup_test_data(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::PgPool::connect(database_url).await?;
    
    // Limpiar tablas de test (ajustar según esquema)
    sqlx::query("TRUNCATE TABLE users CASCADE")
        .execute(&pool)
        .await?;
    
    Ok(())
}

/// Helper para limpiar Redis de test
pub async fn cleanup_test_redis(redis_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_async_connection().await?;
    
    // Limpiar todas las claves de test
    redis::cmd("FLUSHDB")
        .query_async(&mut conn)
        .await?;
    
    Ok(())
}

/// Macro para marcar tests que requieren servicios
#[macro_export]
macro_rules! require_services {
    () => {
        // Este macro puede usarse para verificar servicios antes de ejecutar tests
        // Ejemplo: require_services!();
    };
}

/// Configurar variables de entorno para tests
pub fn setup_test_env() {
    std::env::set_var("TEST_DATABASE_URL", "postgresql://vibestream:vibestream@localhost:5433/vibestream_test");
    std::env::set_var("TEST_REDIS_URL", "redis://localhost:6379/1");
    std::env::set_var("TEST_JWT_SECRET", "test_secret_key_for_testing_only");
    std::env::set_var("JWT_SECRET", "test_secret_key_for_testing_only");
}

