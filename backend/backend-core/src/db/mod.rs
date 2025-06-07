use sea_orm::DatabaseConnection;
use sqlx::postgres::PgPoolOptions;
use anyhow::Result;

pub async fn create_connection(database_url: &str) -> Result<DatabaseConnection> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(DatabaseConnection::from(pool))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let connection = create_connection("postgres://postgres:postgres@localhost:5432/vibestream_test").await;
        assert!(connection.is_ok(), "Should connect to database successfully");
    }
} 