use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;

use crate::bounded_contexts::user::domain::{User, UserRepository, Email, Username};
use crate::shared::domain::repositories::RepoResult;

pub struct UserPostgresRepository {
    pool: PgPool,
}

impl UserPostgresRepository {
    pub fn new(pool: PgPool) -> Self { 
        Self { pool } 
    }
}

impl User {
    fn from_row(row: sqlx::postgres::PgRow) -> Result<Self, String> {
        use sqlx::Row;
        
        let id: Uuid = row.try_get("id").map_err(|e| e.to_string())?;
        let email_str: String = row.try_get("email").map_err(|e| e.to_string())?;
        let username_str: String = row.try_get("username").map_err(|e| e.to_string())?;
        let password_hash: String = row.try_get("password_hash").map_err(|e| e.to_string())?;
        let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").map_err(|e| e.to_string())?;

        let email = Email::parse(&email_str).map_err(|e| format!("Invalid email: {}", e))?;
        let username = Username::parse(&username_str).map_err(|e| format!("Invalid username: {}", e))?;

        Ok(User {
            id,
            email,
            username,
            password_hash,
            created_at,
        })
    }
}

#[async_trait]
impl UserRepository for UserPostgresRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>> {
        let rec = sqlx::query(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        match rec {
            Some(row) => {
                let user = User::from_row(row)
                    .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> RepoResult<Option<User>> {
        let email_str = email.to_string();
        let rec = sqlx::query(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE email = $1"#
        )
        .bind(&email_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        match rec {
            Some(row) => {
                let user = User::from_row(row)
                    .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, user: &User) -> RepoResult<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, username, password_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                username = EXCLUDED.username,
                password_hash = EXCLUDED.password_hash
            "#
        )
        .bind(user.id)
        .bind(user.email.as_str())
        .bind(user.username.as_str())
        .bind(&user.password_hash)
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        Ok(())
    }
} 