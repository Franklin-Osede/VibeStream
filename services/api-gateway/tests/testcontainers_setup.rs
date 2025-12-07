//! Testcontainers Setup para Tests de Integración
//! 
//! Este módulo proporciona helpers para configurar PostgreSQL y Redis
//! usando testcontainers, permitiendo tests aislados y reproducibles.

use testcontainers::{clients, Container, RunnableImage};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::redis::Redis;
use std::time::Duration;
use tokio::time::sleep;

/// Configuración de servicios de test usando testcontainers
pub struct TestContainersSetup {
    pub postgres_container: Container<'static, Postgres>,
    pub redis_container: Container<'static, Redis>,
    pub docker: clients::Cli,
}

impl TestContainersSetup {
    /// Crear nueva configuración de testcontainers
    pub fn new() -> Self {
        let docker = clients::Cli::default();
        
        // Iniciar PostgreSQL
        let postgres_image = RunnableImage::from(Postgres::default())
            .with_tag("15-alpine");
        let postgres_container = docker.run(postgres_image);
        
        // Iniciar Redis
        let redis_image = RunnableImage::from(Redis::default())
            .with_tag("7-alpine");
        let redis_container = docker.run(redis_image);
        
        Self {
            postgres_container,
            redis_container,
            docker,
        }
    }
    
    /// Obtener URL de conexión a PostgreSQL
    pub fn get_postgres_url(&self) -> String {
        let host = self.postgres_container.get_host();
        let port = self.postgres_container.get_host_port_ipv4(5432);
        format!("postgresql://postgres:postgres@{}:{}/postgres", host, port)
    }
    
    /// Obtener URL de conexión a Redis
    pub fn get_redis_url(&self) -> String {
        let host = self.redis_container.get_host();
        let port = self.redis_container.get_host_port_ipv4(6379);
        format!("redis://{}:{}", host, port)
    }
    
    /// Esperar a que PostgreSQL esté listo
    pub async fn wait_for_postgres(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.get_postgres_url();
        let max_retries = 30;
        let mut retries = 0;
        
        while retries < max_retries {
            match sqlx::PgPool::connect(&url).await {
                Ok(pool) => {
                    // Verificar conexión
                    sqlx::query("SELECT 1").fetch_one(&pool).await?;
                    drop(pool);
                    return Ok(());
                }
                Err(_) => {
                    retries += 1;
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
        
        Err("PostgreSQL no está disponible después de múltiples intentos".into())
    }
    
    /// Esperar a que Redis esté listo
    pub async fn wait_for_redis(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.get_redis_url();
        let max_retries = 30;
        let mut retries = 0;
        
        while retries < max_retries {
            match redis::Client::open(&url) {
                Ok(client) => {
                    match client.get_async_connection().await {
                        Ok(mut conn) => {
                            // Verificar conexión con PING
                            let _: String = redis::cmd("PING")
                                .query_async(&mut conn)
                                .await?;
                            return Ok(());
                        }
                        Err(_) => {
                            retries += 1;
                            sleep(Duration::from_millis(500)).await;
                        }
                    }
                }
                Err(_) => {
                    retries += 1;
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
        
        Err("Redis no está disponible después de múltiples intentos".into())
    }
    
    /// Ejecutar migraciones en la base de datos de test
    pub async fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.get_postgres_url();
        let pool = sqlx::PgPool::connect(&url).await?;
        
        // Buscar directorio de migraciones
        let migrations_paths = vec![
            "../../migrations",
            "../migrations",
            "migrations",
        ];
        
        for path in migrations_paths {
            if std::path::Path::new(path).exists() {
                let migrator = sqlx::migrate::Migrator::new(std::path::Path::new(path)).await?;
                migrator.run(&pool).await?;
                return Ok(());
            }
        }
        
        Err("No se encontró directorio de migraciones".into())
    }
    
    /// Configurar variables de entorno para tests
    pub fn setup_env(&self) {
        std::env::set_var("DATABASE_URL", self.get_postgres_url());
        std::env::set_var("REDIS_URL", self.get_redis_url());
        std::env::set_var("JWT_SECRET", "test_secret_key_for_testing_only");
        std::env::set_var("TEST_DATABASE_URL", self.get_postgres_url());
        std::env::set_var("TEST_REDIS_URL", self.get_redis_url());
        std::env::set_var("TEST_JWT_SECRET", "test_secret_key_for_testing_only");
    }
}

impl Default for TestContainersSetup {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper macro para tests con testcontainers
#[macro_export]
macro_rules! test_with_containers {
    ($test_name:ident, $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            // Setup testcontainers
            let setup = crate::testcontainers_setup::TestContainersSetup::new();
            setup.setup_env();
            
            // Esperar a que los servicios estén listos
            setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
            setup.wait_for_redis().await.expect("Redis debe estar listo");
            
            // Ejecutar migraciones
            setup.run_migrations().await.expect("Migraciones deben ejecutarse");
            
            // Ejecutar test
            $test_body
            
            // Cleanup automático (los containers se destruyen al salir del scope)
        }
    };
}

/// Helper para crear AppState con testcontainers
pub async fn create_test_app_state() -> Result<crate::shared::infrastructure::app_state::AppState, Box<dyn std::error::Error>> {
    let setup = TestContainersSetup::new();
    setup.setup_env();
    
    // Esperar a que los servicios estén listos
    setup.wait_for_postgres().await?;
    setup.wait_for_redis().await?;
    
    // Ejecutar migraciones
    setup.run_migrations().await?;
    
    // Crear AppState
    let app_state = crate::shared::infrastructure::app_state::AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await?;
    
    Ok(app_state)
}



