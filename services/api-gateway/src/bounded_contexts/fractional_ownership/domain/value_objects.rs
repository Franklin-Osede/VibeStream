use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::shared::domain::errors::AppError;
use super::errors::FractionalOwnershipError;

/// Percentage of ownership (0.01% to 100%)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnershipPercentage {
    value: f64,
}

impl OwnershipPercentage {
    pub fn new(percentage: f64) -> Result<Self, FractionalOwnershipError> {
        if percentage <= 0.0 || percentage > 100.0 {
            return Err(FractionalOwnershipError::InvalidOwnershipPercentage { percentage });
        }
        Ok(Self { value: percentage })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn add(&self, other: &OwnershipPercentage) -> Result<OwnershipPercentage, FractionalOwnershipError> {
        let new_value = self.value + other.value;
        if new_value > 100.0 {
            return Err(FractionalOwnershipError::OwnershipExceedsLimit {
                current: self.value,
                additional: other.value,
            });
        }
        Ok(OwnershipPercentage { value: new_value })
    }
}

/// Price per share in platform tokens
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharePrice {
    amount: f64,
    currency: String, // "VIBE" token
}

impl SharePrice {
    pub fn new(amount: f64, currency: String) -> Result<Self, FractionalOwnershipError> {
        if amount <= 0.0 {
            return Err(FractionalOwnershipError::InvalidSharePrice { price: amount });
        }
        Ok(Self { amount, currency })
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn multiply(&self, quantity: u32) -> SharePrice {
        SharePrice {
            amount: self.amount * quantity as f64,
            currency: self.currency.clone(),
        }
    }
}

/// Revenue amount for distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevenueAmount {
    amount: f64,
    currency: String,
}

impl RevenueAmount {
    pub fn new(amount: f64, currency: String) -> Self {
        Self { amount, currency }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn calculate_share(&self, ownership_percentage: &OwnershipPercentage) -> RevenueAmount {
        let share_amount = self.amount * (ownership_percentage.value() / 100.0);
        RevenueAmount {
            amount: share_amount,
            currency: self.currency.clone(),
        }
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

    mod ownership_percentage_tests {
        use super::*;

        #[test]
        fn should_create_valid_ownership_percentage() {
            let percentage = OwnershipPercentage::new(25.5).unwrap();
            assert_eq!(percentage.value(), 25.5);
        }

        #[test]
        fn should_reject_zero_percentage() {
            let result = OwnershipPercentage::new(0.0);
            assert!(matches!(result, Err(FractionalOwnershipError::InvalidOwnershipPercentage { .. })));
        }

        #[test]
        fn should_reject_negative_percentage() {
            let result = OwnershipPercentage::new(-5.0);
            assert!(matches!(result, Err(FractionalOwnershipError::InvalidOwnershipPercentage { .. })));
        }

        #[test]
        fn should_reject_percentage_over_100() {
            let result = OwnershipPercentage::new(101.0);
            assert!(matches!(result, Err(FractionalOwnershipError::InvalidOwnershipPercentage { .. })));
        }

        #[test]
        fn should_add_percentages_correctly() {
            let p1 = OwnershipPercentage::new(30.0).unwrap();
            let p2 = OwnershipPercentage::new(20.0).unwrap();
            let result = p1.add(&p2).unwrap();
            assert_eq!(result.value(), 50.0);
        }

        #[test]
        fn should_reject_addition_exceeding_100_percent() {
            let p1 = OwnershipPercentage::new(60.0).unwrap();
            let p2 = OwnershipPercentage::new(50.0).unwrap();
            let result = p1.add(&p2);
            assert!(matches!(result, Err(FractionalOwnershipError::OwnershipExceedsLimit { .. })));
        }
    }

    mod share_price_tests {
        use super::*;

        #[test]
        fn should_create_valid_share_price() {
            let price = SharePrice::new(10.5, "VIBE".to_string()).unwrap();
            assert_eq!(price.amount(), 10.5);
            assert_eq!(price.currency(), "VIBE");
        }

        #[test]
        fn should_reject_zero_price() {
            let result = SharePrice::new(0.0, "VIBE".to_string());
            assert!(matches!(result, Err(FractionalOwnershipError::InvalidSharePrice { .. })));
        }

        #[test]
        fn should_reject_negative_price() {
            let result = SharePrice::new(-5.0, "VIBE".to_string());
            assert!(matches!(result, Err(FractionalOwnershipError::InvalidSharePrice { .. })));
        }

        #[test]
        fn should_multiply_price_by_quantity() {
            let price = SharePrice::new(10.0, "VIBE".to_string()).unwrap();
            let total = price.multiply(5);
            assert_eq!(total.amount(), 50.0);
            assert_eq!(total.currency(), "VIBE");
        }
    }

    mod revenue_amount_tests {
        use super::*;

        #[test]
        fn should_calculate_revenue_share_correctly() {
            let revenue = RevenueAmount::new(1000.0, "USD".to_string());
            let ownership = OwnershipPercentage::new(25.0).unwrap();
            let share = revenue.calculate_share(&ownership);
            assert_eq!(share.amount(), 250.0);
        }

        #[test]
        fn should_calculate_small_percentage_correctly() {
            let revenue = RevenueAmount::new(10000.0, "USD".to_string());
            let ownership = OwnershipPercentage::new(0.5).unwrap(); // 0.5%
            let share = revenue.calculate_share(&ownership);
            assert_eq!(share.amount(), 50.0);
        }
    }
} 