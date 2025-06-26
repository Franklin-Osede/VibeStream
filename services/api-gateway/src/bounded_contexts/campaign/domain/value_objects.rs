use chrono::{DateTime, Utc};
use crate::shared::domain::errors::AppError;

#[derive(Clone, Debug)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, AppError> {
        if end <= start {
            return Err(AppError::DomainRuleViolation("End date must be later than start date".into()));
        }
        Ok(Self { start, end })
    }

    pub fn total_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }

    pub fn contains(&self, date: DateTime<Utc>) -> bool {
        date >= self.start && date <= self.end
    }
} 