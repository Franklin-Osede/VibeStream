use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait};
use uuid::Uuid;
use anyhow::anyhow;
use crate::{
    models::user::{self, Model as User},
    error::AppError,
    utils::{hash_password, verify_password},
};

#[derive(Clone)]
pub struct AuthService {
    db: DatabaseConnection,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<User, AppError> {
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e: sea_orm::DbErr| AppError::InternalError(anyhow!(e)))?
            .ok_or(AppError::Unauthorized)?;

        if !verify_password(password, &user.password_hash)
            .map_err(|e| AppError::InternalError(anyhow!(e)))?
        {
            return Err(AppError::Unauthorized);
        }

        Ok(user)
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
        wallet_address: Option<String>,
    ) -> Result<User, AppError> {
        let password_hash = hash_password(&password)
            .map_err(|e| AppError::InternalError(anyhow!(e)))?;

        let user = user::ActiveModel {
            id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
            username: sea_orm::ActiveValue::Set(username),
            email: sea_orm::ActiveValue::Set(email),
            password_hash: sea_orm::ActiveValue::Set(password_hash),
            wallet_address: sea_orm::ActiveValue::Set(wallet_address),
            created_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
        };

        user.insert(&self.db)
            .await
            .map_err(|e: sea_orm::DbErr| AppError::InternalError(anyhow!(e)))?
    }
} 