use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::shared::domain::events::DomainEvent;
use super::value_objects::{Email, Username};

#[derive(Clone, Debug)]
pub struct UserRegistered {
    pub id: Uuid,
    pub email: Email,
    pub username: Username,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for UserRegistered {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }
} 