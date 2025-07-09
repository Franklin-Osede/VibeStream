// User Application Events
// This module contains domain events for user operations

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserEvent {
    UserCreated(UserCreatedEvent),
    UserUpdated(UserUpdatedEvent),
    UserFollowed(UserFollowedEvent),
    UserUnfollowed(UserUnfollowedEvent),
    UserDeleted(UserDeletedEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: Uuid,
    pub updated_fields: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFollowedEvent {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
    pub followed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnfollowedEvent {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
    pub unfollowed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeletedEvent {
    pub user_id: Uuid,
    pub deleted_at: DateTime<Utc>,
}

impl UserEvent {
    pub fn user_created(
        user_id: Uuid,
        email: String,
        username: String,
        display_name: String,
    ) -> Self {
        Self::UserCreated(UserCreatedEvent {
            user_id,
            email,
            username,
            display_name,
            created_at: Utc::now(),
        })
    }
    
    pub fn user_updated(user_id: Uuid, updated_fields: Vec<String>) -> Self {
        Self::UserUpdated(UserUpdatedEvent {
            user_id,
            updated_fields,
            updated_at: Utc::now(),
        })
    }
    
    pub fn user_followed(follower_id: Uuid, followee_id: Uuid) -> Self {
        Self::UserFollowed(UserFollowedEvent {
            follower_id,
            followee_id,
            followed_at: Utc::now(),
        })
    }
    
    pub fn user_unfollowed(follower_id: Uuid, followee_id: Uuid) -> Self {
        Self::UserUnfollowed(UserUnfollowedEvent {
            follower_id,
            followee_id,
            unfollowed_at: Utc::now(),
        })
    }
    
    pub fn user_deleted(user_id: Uuid) -> Self {
        Self::UserDeleted(UserDeletedEvent {
            user_id,
            deleted_at: Utc::now(),
        })
    }
    
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::UserCreated(_) => "UserCreated",
            Self::UserUpdated(_) => "UserUpdated", 
            Self::UserFollowed(_) => "UserFollowed",
            Self::UserUnfollowed(_) => "UserUnfollowed",
            Self::UserDeleted(_) => "UserDeleted",
        }
    }
    
    pub fn user_id(&self) -> Uuid {
        match self {
            Self::UserCreated(event) => event.user_id,
            Self::UserUpdated(event) => event.user_id,
            Self::UserFollowed(event) => event.follower_id, // Could be either, but follower makes sense
            Self::UserUnfollowed(event) => event.follower_id,
            Self::UserDeleted(event) => event.user_id,
        }
    }
} 