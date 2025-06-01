use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::User,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<User, AppError>;
    async fn create(&self, user: User) -> Result<User, AppError>;
    async fn update(&self, user: User) -> Result<User, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

// Implementación concreta usando Sea-ORM
pub struct SeaORMUserRepository {
    db: DatabaseConnection,
}

impl SeaORMUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaORMUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        // TODO: Implementar usando Sea-ORM
        unimplemented!()
    }

    async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        unimplemented!()
    }

    async fn create(&self, user: User) -> Result<User, AppError> {
        unimplemented!()
    }

    async fn update(&self, user: User) -> Result<User, AppError> {
        unimplemented!()
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        unimplemented!()
    }
} 