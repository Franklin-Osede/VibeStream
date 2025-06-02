use sea_orm::DatabaseConnection;
use sea_orm::ActiveValue::Set;
use crate::{
    error::AppError,
    db::models::user::{self, Model as User},
    repositories::user::UserRepository,
};

pub struct AuthService {
    user_repository: Box<dyn UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn login(
        &self,
        db: &DatabaseConnection,
        email: String,
        password: String,
    ) -> Result<User, AppError> {
        let user = self.user_repository
            .find_by_email(db, &email)
            .await
            .map_err(AppError::DatabaseError)?
            .ok_or_else(|| AppError::ValidationError("Invalid credentials".to_string()))?;

        // TODO: Implement password verification
        Ok(user)
    }

    pub async fn register(
        &self,
        db: &DatabaseConnection,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, AppError> {
        // TODO: Implement password hashing
        let user = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password_hash: Set(password),
            ..Default::default()
        };

        self.user_repository
            .create(db, user)
            .await
            .map_err(AppError::DatabaseError)
    }
} 