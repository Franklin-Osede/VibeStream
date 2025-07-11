// User Application Handlers
// This module contains command and query handlers for user operations

use crate::bounded_contexts::user::domain::entities::User;
use crate::shared::domain::errors::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Command Handlers
#[derive(Debug, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FollowUserCommand {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
    pub follow: bool,
}

// Query Handlers
#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SearchUsersQuery {
    pub search_text: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Handler traits
#[async_trait]
pub trait UserCommandHandler {
    async fn handle_create_user(&self, command: CreateUserCommand) -> Result<UserResponse, AppError>;
    async fn handle_update_user(&self, command: UpdateUserCommand) -> Result<UserResponse, AppError>;
    async fn handle_follow_user(&self, command: FollowUserCommand) -> Result<(), AppError>;
}

#[async_trait]
pub trait UserQueryHandler {
    async fn handle_get_user(&self, query: GetUserQuery) -> Result<UserResponse, AppError>;
    async fn handle_search_users(&self, query: SearchUsersQuery) -> Result<Vec<UserResponse>, AppError>;
}

// Implementation will be in services.rs
pub struct UserHandlers;

impl UserHandlers {
    pub fn new() -> Self {
        Self
    }
} 