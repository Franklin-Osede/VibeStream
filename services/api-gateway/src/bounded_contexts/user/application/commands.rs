// User Application Commands
// This module contains command structures and handlers for user operations

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::shared::application::command::{Command, CommandHandler};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::user::domain::aggregates::UserAggregate;
use crate::bounded_contexts::user::domain::value_objects::{Email, Username, PasswordHash};
use crate::bounded_contexts::user::domain::repository::UserRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::bounded_contexts::user::domain::value_objects::UserId;

// Command definitions
#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub bio: Option<String>,
}

impl Command for RegisterUser {}

#[derive(Debug, Serialize)]
pub struct RegisterUserResult {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
}

impl Command for UpdateUser {}

#[derive(Debug, Deserialize)]
pub struct FollowUser {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
    pub follow: bool, // true = follow, false = unfollow
}

impl Command for FollowUser {}

#[derive(Debug, Deserialize)]
pub struct DeleteUser {
    pub user_id: Uuid,
    pub requesting_user_id: Uuid, // For authorization
}

impl Command for DeleteUser {}

// Command Handlers
pub struct RegisterUserHandler<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> RegisterUserHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> CommandHandler<RegisterUser> for RegisterUserHandler<R> {
    type Output = RegisterUserResult;

    async fn handle(&self, command: RegisterUser) -> Result<Self::Output, AppError> {
        // Check if user already exists
        let email = Email::new(command.email).map_err(|e| AppError::ValidationError(e))?;
        let username = Username::new(command.username).map_err(|e| AppError::ValidationError(e))?;
        
        if let Ok(Some(_)) = self.repository.find_by_email(&email).await {
            return Err(AppError::ValidationError("Email already exists".to_string()));
        }
        
        if let Ok(Some(_)) = self.repository.find_by_username(&username).await {
            return Err(AppError::ValidationError("Username already exists".to_string()));
        }
        
        // Create new user aggregate
        let password_hash = PasswordHash::from_password(command.password).map_err(|e| AppError::ValidationError(e))?;
        let mut user_aggregate = UserAggregate::create(
            email,
            username,
            password_hash,
        ).map_err(|e| AppError::ValidationError(e))?;
        
        // Update profile with additional data
        if !command.display_name.trim().is_empty() {
            user_aggregate.profile.update_display_name(Some(command.display_name.clone()));
        }
        if let Some(bio) = command.bio.clone() {
            user_aggregate.profile.update_bio(Some(bio));
        }
        
        // Save user aggregate
        self.repository.save(&user_aggregate).await?;
        
        Ok(RegisterUserResult {
            user_id: user_aggregate.user.id.to_uuid(),
            username: user_aggregate.user.username.to_string(),
            email: user_aggregate.user.email.to_string(),
            display_name: user_aggregate.profile.display_name.unwrap_or(command.display_name),
            created_at: user_aggregate.user.created_at,
        })
    }
}

pub struct UpdateUserHandler<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UpdateUserHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> CommandHandler<UpdateUser> for UpdateUserHandler<R> {
    type Output = ();

    async fn handle(&self, command: UpdateUser) -> Result<Self::Output, AppError> {
        let mut user_aggregate = self.repository.find_by_id(&command.user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        
        // Update profile fields
        if let Some(display_name) = command.display_name {
            user_aggregate.profile.update_display_name(Some(display_name));
        }
        
        if let Some(bio) = command.bio {
            user_aggregate.profile.update_bio(Some(bio));
        }
        
        if let Some(profile_image_url) = command.profile_image_url {
            use crate::bounded_contexts::user::domain::value_objects::ProfileUrl;
            let profile_url = ProfileUrl::new(profile_image_url).map_err(|e| AppError::ValidationError(e))?;
            user_aggregate.profile.update_avatar(Some(profile_url));
        }
        
        // Save updated user aggregate
        self.repository.update(&user_aggregate).await?;
        
        Ok(())
    }
}

pub struct FollowUserHandler<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> FollowUserHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> CommandHandler<FollowUser> for FollowUserHandler<R> {
    type Output = ();

    async fn handle(&self, command: FollowUser) -> Result<Self::Output, AppError> {
        if command.follower_id == command.followee_id {
            return Err(AppError::ValidationError("Cannot follow yourself".to_string()));
        }
        
        let follower_id = UserId::from_uuid(command.follower_id);
        let followee_id = UserId::from_uuid(command.followee_id);
        
        // Verify both users exist
        self.repository.find_by_id(&follower_id).await?
            .ok_or_else(|| AppError::NotFound("Follower not found".to_string()))?;
            
        self.repository.find_by_id(&followee_id).await?
            .ok_or_else(|| AppError::NotFound("User to follow not found".to_string()))?;
        
        if command.follow {
            self.repository.add_follower(&followee_id, &follower_id).await?;
        } else {
            self.repository.remove_follower(&followee_id, &follower_id).await?;
        }
        
        Ok(())
    }
} 