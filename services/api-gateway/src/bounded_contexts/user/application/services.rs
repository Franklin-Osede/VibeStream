// User Application Services
// This module contains the main application services for user operations

use crate::bounded_contexts::user::domain::{
    aggregates::UserAggregate,
    value_objects::{Email, Username, PasswordHash, ProfileUrl, UserId},
    repository::UserRepository,
    services::{UserDomainService, DefaultUserDomainService},
};
use crate::bounded_contexts::user::application::handlers::{
    CreateUserCommand, UpdateUserCommand, FollowUserCommand,
    GetUserQuery, SearchUsersQuery, UserResponse,
    UserCommandHandler, UserQueryHandler,
};
use crate::shared::domain::errors::AppError;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserApplicationService<R: UserRepository> {
    repository: Arc<R>,
    domain_service: Arc<dyn UserDomainService + Send + Sync>,
}

impl<R: UserRepository + 'static> UserApplicationService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        let domain_service = Arc::new(DefaultUserDomainService::new(repository.clone()));
        Self {
            repository,
            domain_service,
        }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UserCommandHandler for UserApplicationService<R> {
    async fn handle_create_user(&self, command: CreateUserCommand) -> Result<UserResponse, AppError> {
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
        if let Some(display_name) = command.display_name {
            user_aggregate.profile.update_display_name(Some(display_name));
        }
        if let Some(bio) = command.bio {
            user_aggregate.profile.update_bio(Some(bio));
        }
        
        // Save user
        self.repository.save(&user_aggregate).await?;
        
        Ok(UserResponse {
            id: user_aggregate.user.id.to_uuid(),
            username: user_aggregate.user.username.to_string(),
            email: user_aggregate.user.email.to_string(),
            display_name: user_aggregate.profile.display_name.clone(),
            bio: user_aggregate.profile.bio.clone(),
            profile_image_url: user_aggregate.profile.avatar_url.as_ref().map(|u| u.to_string()),
            created_at: user_aggregate.user.created_at,
        })
    }
    
    async fn handle_update_user(&self, command: UpdateUserCommand) -> Result<UserResponse, AppError> {
        let user_id = UserId::from_uuid(command.user_id);
        let mut user_aggregate = self.repository.find_by_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        
        // Update profile fields
        if let Some(display_name) = command.display_name {
            user_aggregate.profile.update_display_name(Some(display_name));
        }
        
        if let Some(bio) = command.bio {
            user_aggregate.profile.update_bio(Some(bio));
        }
        
        if let Some(profile_image_url) = command.profile_image_url {
            let profile_url = ProfileUrl::new(profile_image_url).map_err(|e| AppError::ValidationError(e))?;
            user_aggregate.profile.update_avatar(Some(profile_url));
        }
        
        // Save updated user
        self.repository.update(&user_aggregate).await?;
        
        Ok(UserResponse {
            id: user_aggregate.user.id.to_uuid(),
            username: user_aggregate.user.username.to_string(),
            email: user_aggregate.user.email.to_string(),
            display_name: user_aggregate.profile.display_name.clone(),
            bio: user_aggregate.profile.bio.clone(),
            profile_image_url: user_aggregate.profile.avatar_url.as_ref().map(|u| u.to_string()),
            created_at: user_aggregate.user.created_at,
        })
    }
    
    async fn handle_follow_user(&self, command: FollowUserCommand) -> Result<(), AppError> {
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

#[async_trait]
impl<R: UserRepository + Send + Sync> UserQueryHandler for UserApplicationService<R> {
    async fn handle_get_user(&self, query: GetUserQuery) -> Result<UserResponse, AppError> {
        let user_id = UserId::from_uuid(query.user_id);
        let user_aggregate = self.repository.find_by_id(&user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
            
        Ok(UserResponse {
            id: user_aggregate.user.id.to_uuid(),
            username: user_aggregate.user.username.to_string(),
            email: user_aggregate.user.email.to_string(),
            display_name: user_aggregate.profile.display_name.clone(),
            bio: user_aggregate.profile.bio.clone(),
            profile_image_url: user_aggregate.profile.avatar_url.as_ref().map(|u| u.to_string()),
            created_at: user_aggregate.user.created_at,
        })
    }
    
    async fn handle_search_users(&self, query: SearchUsersQuery) -> Result<Vec<UserResponse>, AppError> {
        let user_aggregates = self.repository.search_users(
            query.search_text.as_deref(),
            query.limit.unwrap_or(10),
            query.offset.unwrap_or(0),
        ).await?;
        
        let responses = user_aggregates.into_iter().map(|user_aggregate| UserResponse {
            id: user_aggregate.user.id.to_uuid(),
            username: user_aggregate.user.username.to_string(),
            email: user_aggregate.user.email.to_string(),
            display_name: user_aggregate.profile.display_name.clone(),
            bio: user_aggregate.profile.bio.clone(),
            profile_image_url: user_aggregate.profile.avatar_url.as_ref().map(|u| u.to_string()),
            created_at: user_aggregate.user.created_at,
        }).collect();
        
        Ok(responses)
    }
} 