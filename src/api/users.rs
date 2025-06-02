use axum::{
    routing::{get, post},
    Router,
    extract::{Json, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{
    error::AppError,
    db::models::user::Model as User,
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users", post(create_user))
}

pub async fn get_current_user() -> Result<Json<User>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn create_user(
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    Err(AppError::NotImplemented)
} 