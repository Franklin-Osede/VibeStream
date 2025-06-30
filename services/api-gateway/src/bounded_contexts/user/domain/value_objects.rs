use regex::Regex;
use lazy_static::lazy_static;
use crate::shared::domain::errors::AppError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, AppError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
        }
        if RE.is_match(s) {
            Ok(Self(s.to_lowercase()))
        } else {
            Err(AppError::DomainRuleViolation("Invalid email format".into()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn parse(s: &str) -> Result<Self, AppError> {
        if s.len() >= 3 && s.len() <= 30 && s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Ok(Self(s.to_string()))
        } else {
            Err(AppError::DomainRuleViolation("Invalid username".into()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId {
    value: Uuid,
}

impl UserId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self { value: uuid }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let uuid = Uuid::parse_str(s)
            .map_err(|e| format!("Invalid UUID format: {}", e))?;
        Ok(Self::from_uuid(uuid))
    }

    pub fn value(&self) -> Uuid {
        self.value
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
} 