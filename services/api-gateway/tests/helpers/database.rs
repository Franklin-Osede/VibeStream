use sqlx::{PgPool, PgPoolOptions};
use std::sync::Once;
use uuid::Uuid;

static INIT: Once = Once::new();

pub struct TestDatabase {
    pub pool: PgPool,
    pub database_name: String,
}

impl TestDatabase {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        INIT.call_once(|| {
            dotenv::dotenv().ok();
        });

        let database_name = format!("test_vibestream_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        // Connect to default database to create test database
        let default_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://postgres:password@localhost:5432/vibestream".to_string()
            }))
            .await?;

        // Create test database
        sqlx::query(&format!("CREATE DATABASE {}", database_name))
            .execute(&default_pool)
            .await?;

        // Connect to test database
        let test_url = format!(
            "postgresql://postgres:password@localhost:5432/{}",
            database_name
        );
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&test_url)
            .await?;

        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await?;

        Ok(TestDatabase {
            pool,
            database_name,
        })
    }

    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Drop test database
        let default_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://postgres:password@localhost:5432/vibestream".to_string()
            }))
            .await?;

        sqlx::query(&format!("DROP DATABASE IF EXISTS {}", self.database_name))
            .execute(&default_pool)
            .await?;

        Ok(())
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Note: We can't do async cleanup in Drop, so we'll rely on manual cleanup
    }
} 