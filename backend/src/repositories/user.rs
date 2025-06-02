use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;
use crate::models::user::{self, Model as User};
use crate::error::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<User>, DbErr>;
    async fn find_by_email(&self, db: &DatabaseConnection, email: &str) -> Result<Option<User>, DbErr>;
    async fn create(&self, db: &DatabaseConnection, user: user::ActiveModel) -> Result<User, DbErr>;
    async fn update(&self, db: &DatabaseConnection, user: user::ActiveModel) -> Result<User, DbErr>;
    async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr>;
}

pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<User>, DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    async fn find_by_email(&self, db: &DatabaseConnection, email: &str) -> Result<Option<User>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    async fn create(&self, db: &DatabaseConnection, user: user::ActiveModel) -> Result<User, DbErr> {
        user.insert(db).await
    }

    async fn update(&self, db: &DatabaseConnection, user: user::ActiveModel) -> Result<User, DbErr> {
        user.update(db).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        user::Entity::delete_by_id(id).exec(db).await
    }
} 