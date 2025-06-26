use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::shared::domain::errors::AppError;

use super::value_objects::{Email, Username};
use super::events::UserRegistered;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub username: Username,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn register(email: Email, username: Username, password_hash: String) -> (Self, UserRegistered) {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let user = Self { id, email: email.clone(), username: username.clone(), password_hash, created_at: now };
        let event = UserRegistered { id, email, username, occurred_on: now };
        (user, event)
    }
} 