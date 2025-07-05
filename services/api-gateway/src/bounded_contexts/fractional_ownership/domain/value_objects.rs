use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::shared::domain::errors::AppError;
use super::errors::FractionalOwnershipError;

/// Identificador único para un contrato de ownership
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OwnershipContractId {
    value: Uuid,
}

impl Default for OwnershipContractId {
    fn default() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }
}

impl OwnershipContractId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self { value: uuid }
    }

    pub fn from_string(s: &str) -> Result<Self, AppError> {
        let uuid = Uuid::parse_str(s)
            .map_err(|_| AppError::InvalidInput(format!("Invalid OwnershipContractId: {}", s)))?;
        Ok(Self { value: uuid })
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

/// Porcentaje de ownership (0.0 a 100.0)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnershipPercentage {
    value: f64,
}

impl OwnershipPercentage {
    pub fn new(percentage: f64) -> Result<Self, AppError> {
        if percentage < 0.0 || percentage > 100.0 {
            return Err(AppError::InvalidInput(
                "Ownership percentage must be between 0.0 and 100.0".to_string(),
            ));
        }
        Ok(Self { value: percentage })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn as_decimal(&self) -> f64 {
        self.value / 100.0
    }

    pub fn add(&self, other: &OwnershipPercentage) -> Result<OwnershipPercentage, AppError> {
        OwnershipPercentage::new(self.value + other.value)
    }

    pub fn subtract(&self, other: &OwnershipPercentage) -> Result<OwnershipPercentage, AppError> {
        OwnershipPercentage::new(self.value - other.value)
    }

    pub fn is_valid_for_sale(&self, available: &OwnershipPercentage) -> bool {
        self.value <= available.value && self.value > 0.0
    }
}

/// Precio de una share
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharePrice {
    value: f64, // En USD
}

impl SharePrice {
    pub fn new(price: f64) -> Result<Self, AppError> {
        if price < 0.0 {
            return Err(AppError::InvalidInput(
                "Share price must be non-negative".to_string(),
            ));
        }
        Ok(Self { value: price })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn multiply_by_percentage(&self, percentage: &OwnershipPercentage) -> f64 {
        self.value * percentage.as_decimal()
    }

    pub fn calculate_market_cap(&self, total_shares: u32) -> f64 {
        self.value * total_shares as f64
    }
}

/// Cantidad de revenue en USD
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevenueAmount {
    value: f64,
}

impl RevenueAmount {
    pub fn new(amount: f64) -> Result<Self, AppError> {
        if amount < 0.0 {
            return Err(AppError::InvalidInput(
                "Revenue amount must be non-negative".to_string(),
            ));
        }
        Ok(Self { value: amount })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn calculate_share(&self, percentage: &OwnershipPercentage) -> RevenueAmount {
        RevenueAmount {
            value: self.value * percentage.as_decimal(),
        }
    }

    pub fn add(&self, other: &RevenueAmount) -> RevenueAmount {
        RevenueAmount {
            value: self.value + other.value,
        }
    }

    pub fn subtract(&self, other: &RevenueAmount) -> Result<RevenueAmount, AppError> {
        if self.value < other.value {
            return Err(AppError::DomainRuleViolation(
                "Cannot subtract more revenue than available".to_string(),
            ));
        }
        Ok(RevenueAmount {
            value: self.value - other.value,
        })
    }
}

/// Identificador de una share individual
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShareId {
    value: Uuid,
}

impl ShareId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self { value: uuid }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

/// Rango de fechas para vesting de shares
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VestingPeriod {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

impl VestingPeriod {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, AppError> {
        if start >= end {
            return Err(AppError::InvalidInput(
                "Vesting start date must be before end date".to_string(),
            ));
        }
        if start < Utc::now() {
            return Err(AppError::InvalidInput(
                "Vesting start date cannot be in the past".to_string(),
            ));
        }
        Ok(Self {
            start_date: start,
            end_date: end,
        })
    }

    pub fn start_date(&self) -> DateTime<Utc> {
        self.start_date
    }

    pub fn end_date(&self) -> DateTime<Utc> {
        self.end_date
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.start_date && now <= self.end_date
    }

    pub fn is_fully_vested(&self) -> bool {
        Utc::now() > self.end_date
    }

    pub fn vesting_progress(&self) -> f64 {
        let now = Utc::now();
        if now <= self.start_date {
            return 0.0;
        }
        if now >= self.end_date {
            return 1.0;
        }

        let total_duration = self.end_date.timestamp() - self.start_date.timestamp();
        let elapsed = now.timestamp() - self.start_date.timestamp();
        
        elapsed as f64 / total_duration as f64
    }

    pub fn days_remaining(&self) -> i64 {
        let now = Utc::now();
        if now >= self.end_date {
            return 0;
        }
        (self.end_date.date_naive() - now.date_naive()).num_days()
    }
}

/// Song identifier for ownership contracts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SongId(pub Uuid);

impl SongId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

/// User identifier for shareholders
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_ownership_percentage_validation() {
        // Valid percentages
        assert!(OwnershipPercentage::new(0.0).is_ok());
        assert!(OwnershipPercentage::new(50.0).is_ok());
        assert!(OwnershipPercentage::new(100.0).is_ok());

        // Invalid percentages
        assert!(OwnershipPercentage::new(-1.0).is_err());
        assert!(OwnershipPercentage::new(101.0).is_err());
    }

    #[test]
    fn test_ownership_percentage_operations() {
        let p1 = OwnershipPercentage::new(30.0).unwrap();
        let p2 = OwnershipPercentage::new(20.0).unwrap();

        let sum = p1.add(&p2).unwrap();
        assert_eq!(sum.value(), 50.0);

        let diff = p1.subtract(&p2).unwrap();
        assert_eq!(diff.value(), 10.0);
    }

    #[test]
    fn test_share_price_calculations() {
        let price = SharePrice::new(100.0).unwrap();
        let percentage = OwnershipPercentage::new(25.0).unwrap();

        let cost = price.multiply_by_percentage(&percentage);
        assert_eq!(cost, 25.0);

        let market_cap = price.calculate_market_cap(1000);
        assert_eq!(market_cap, 100000.0);
    }

    #[test]
    fn test_revenue_amount_operations() {
        let revenue = RevenueAmount::new(1000.0).unwrap();
        let percentage = OwnershipPercentage::new(15.0).unwrap();

        let share = revenue.calculate_share(&percentage);
        assert_eq!(share.value(), 150.0);
    }

    #[test]
    fn test_vesting_period_validation() {
        let future_start = Utc::now() + Duration::days(1);
        let future_end = future_start + Duration::days(30);

        // Valid vesting period
        assert!(VestingPeriod::new(future_start, future_end).is_ok());

        // Invalid: start after end
        assert!(VestingPeriod::new(future_end, future_start).is_err());

        // Invalid: start in past
        let past_start = Utc::now() - Duration::days(1);
        assert!(VestingPeriod::new(past_start, future_end).is_err());
    }

    #[test]
    fn test_vesting_progress() {
        let start = Utc::now() - Duration::days(10);
        let end = Utc::now() + Duration::days(10);
        let vesting = VestingPeriod::new(start, end).unwrap();

        let progress = vesting.vesting_progress();
        assert!(progress > 0.0 && progress < 1.0);
        assert!(progress > 0.4 && progress < 0.6); // Should be around 50%
    }
} 