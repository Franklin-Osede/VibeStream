use regex::Regex;
use lazy_static::lazy_static;
use crate::shared::domain::errors::AppError;

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