use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;

use crate::bounded_contexts::user::domain::{User, UserRepository, Email, Username};
use crate::shared::domain::repositories::RepoResult;

pub struct UserPostgresRepository {
    pool: PgPool,
}

impl UserPostgresRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl UserRepository for UserPostgresRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn find_by_email(&self, email_str: &str) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE email = $1"#, email_str)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn save(&self, user: &User) -> RepoResult<()> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, username, password_hash, created_at)
               VALUES ($1,$2,$3,$4,$5)
               ON CONFLICT (id) DO NOTHING"#,
            user.id,
            user.email.as_str(),
            user.username.as_str(),
            user.password_hash,
            user.created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(())
    }
} 
use uuid::Uuid;
use sqlx::PgPool;

use crate::bounded_contexts::user::domain::{User, UserRepository, Email, Username};
use crate::shared::domain::repositories::RepoResult;

pub struct UserPostgresRepository {
    pool: PgPool,
}

impl UserPostgresRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl UserRepository for UserPostgresRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn find_by_email(&self, email_str: &str) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE email = $1"#, email_str)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn save(&self, user: &User) -> RepoResult<()> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, username, password_hash, created_at)
               VALUES ($1,$2,$3,$4,$5)
               ON CONFLICT (id) DO NOTHING"#,
            user.id,
            user.email.as_str(),
            user.username.as_str(),
            user.password_hash,
            user.created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(())
    }
} 
use uuid::Uuid;
use sqlx::PgPool;

use crate::bounded_contexts::user::domain::{User, UserRepository, Email, Username};
use crate::shared::domain::repositories::RepoResult;

pub struct UserPostgresRepository {
    pool: PgPool,
}

impl UserPostgresRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl UserRepository for UserPostgresRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn find_by_email(&self, email_str: &str) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE email = $1"#, email_str)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn save(&self, user: &User) -> RepoResult<()> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, username, password_hash, created_at)
               VALUES ($1,$2,$3,$4,$5)
               ON CONFLICT (id) DO NOTHING"#,
            user.id,
            user.email.as_str(),
            user.username.as_str(),
            user.password_hash,
            user.created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(())
    }
} 
use uuid::Uuid;
use sqlx::PgPool;

use crate::bounded_contexts::user::domain::{User, UserRepository, Email, Username};
use crate::shared::domain::repositories::RepoResult;

pub struct UserPostgresRepository {
    pool: PgPool,
}

impl UserPostgresRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl UserRepository for UserPostgresRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn find_by_email(&self, email_str: &str) -> RepoResult<Option<User>> {
        let rec = sqlx::query!(
            r#"SELECT id, email, username, password_hash, created_at
               FROM users WHERE email = $1"#, email_str)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(rec.map(|r| User {
            id: r.id,
            email: Email::parse(&r.email).unwrap(),
            username: Username::parse(&r.username).unwrap(),
            password_hash: r.password_hash,
            created_at: r.created_at,
        }))
    }

    async fn save(&self, user: &User) -> RepoResult<()> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, username, password_hash, created_at)
               VALUES ($1,$2,$3,$4,$5)
               ON CONFLICT (id) DO NOTHING"#,
            user.id,
            user.email.as_str(),
            user.username.as_str(),
            user.password_hash,
            user.created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(())
    }
} 