use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::errors::FractionalOwnershipError;

/// Percentage of ownership (0.01% to 100%)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnershipPercentage {
    value: f64,
}

impl OwnershipPercentage {
    pub fn new(percentage: f64) -> Result<Self, FractionalOwnershipError> {
        if percentage <= 0.0 || percentage > 100.0 {
            return Err(FractionalOwnershipError::InvalidPercentage(
                format!("Porcentaje inválido: {}. Debe estar entre 0.01 y 100.0", percentage)
            ));
        }
        Ok(Self { value: percentage })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }

    pub fn add(&self, other: &OwnershipPercentage) -> Result<OwnershipPercentage, FractionalOwnershipError> {
        let new_value = self.value + other.value;
        if new_value > 100.0 {
            return Err(FractionalOwnershipError::InvalidPercentage(
                format!("La suma de porcentajes excede 100%: {}", new_value)
            ));
        }
        Ok(OwnershipPercentage { value: new_value })
    }

    /// Calcular la participación en ingresos basada en este porcentaje
    pub fn calculate_revenue_share(&self, total_revenue: &RevenueAmount) -> Result<RevenueAmount, FractionalOwnershipError> {
        let share_amount = total_revenue.value() * (self.value / 100.0);
        RevenueAmount::new(share_amount)
    }
}

/// Price per share in platform tokens
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharePrice {
    amount: f64,
}

impl SharePrice {
    pub fn new(amount: f64) -> Result<Self, FractionalOwnershipError> {
        if amount <= 0.0 {
            return Err(FractionalOwnershipError::InvalidPrice(
                format!("Precio de acción inválido: {}. Debe ser mayor a 0", amount)
            ));
        }
        Ok(Self { amount })
    }

    pub fn from_amount(amount: f64) -> Result<Self, FractionalOwnershipError> {
        Self::new(amount)
    }

    pub fn value(&self) -> f64 {
        self.amount
    }

    pub fn as_f64(&self) -> f64 {
        self.amount
    }

    pub fn multiply_by_quantity(&self, quantity: u32) -> Result<RevenueAmount, FractionalOwnershipError> {
        let total_amount = self.amount * quantity as f64;
        RevenueAmount::new(total_amount)
    }
}

/// Revenue amount for distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevenueAmount {
    amount: f64,
}

impl RevenueAmount {
    pub fn new(amount: f64) -> Result<Self, FractionalOwnershipError> {
        if amount < 0.0 {
            return Err(FractionalOwnershipError::InvalidAmount(
                format!("Cantidad de ingresos no puede ser negativa: {}", amount)
            ));
        }
        Ok(Self { amount })
    }

    pub fn from_amount(amount: f64) -> Result<Self, FractionalOwnershipError> {
        Self::new(amount)
    }

    pub fn value(&self) -> f64 {
        self.amount
    }

    pub fn as_f64(&self) -> f64 {
        self.amount
    }

    pub fn add(&self, other: &RevenueAmount) -> Result<RevenueAmount, FractionalOwnershipError> {
        RevenueAmount::new(self.amount + other.amount)
    }

    pub fn subtract(&self, other: &RevenueAmount) -> Result<RevenueAmount, FractionalOwnershipError> {
        if self.amount < other.amount {
            return Err(FractionalOwnershipError::InvalidOperation(
                "No se puede sustraer más de lo disponible".to_string()
            ));
        }
        RevenueAmount::new(self.amount - other.amount)
    }

    pub fn multiply(&self, multiplier: f64) -> Result<RevenueAmount, FractionalOwnershipError> {
        RevenueAmount::new(self.amount * multiplier)
    }

    pub fn is_zero(&self) -> bool {
        self.amount == 0.0
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

/// Fecha de vencimiento para contratos o campañas
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpirationDate {
    date: chrono::DateTime<chrono::Utc>,
}

impl ExpirationDate {
    pub fn new(date: chrono::DateTime<chrono::Utc>) -> Result<Self, FractionalOwnershipError> {
        if date <= chrono::Utc::now() {
            return Err(FractionalOwnershipError::InvalidExpirationDate(
                "La fecha de expiración debe ser en el futuro".to_string()
            ));
        }
        Ok(Self { date })
    }

    pub fn value(&self) -> chrono::DateTime<chrono::Utc> {
        self.date
    }

    pub fn is_expired(&self) -> bool {
        self.date <= chrono::Utc::now()
    }

    pub fn days_until_expiration(&self) -> i64 {
        let now = chrono::Utc::now();
        if self.date <= now {
            0
        } else {
            self.date.signed_duration_since(now).num_days()
        }
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
            assert!(result.is_err());
        }

        #[test]
        fn should_reject_negative_percentage() {
            let result = OwnershipPercentage::new(-5.0);
            assert!(result.is_err());
        }

        #[test]
        fn should_reject_percentage_over_100() {
            let result = OwnershipPercentage::new(101.0);
            assert!(result.is_err());
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
            assert!(result.is_err());
        }
    }

    mod share_price_tests {
        use super::*;

        #[test]
        fn should_create_valid_share_price() {
            let price = SharePrice::new(10.5).unwrap();
            assert_eq!(price.value(), 10.5);
        }

        #[test]
        fn should_reject_zero_price() {
            let result = SharePrice::new(0.0);
            assert!(result.is_err());
        }

        #[test]
        fn should_reject_negative_price() {
            let result = SharePrice::new(-5.0);
            assert!(result.is_err());
        }

        #[test]
        fn should_multiply_price_by_quantity() {
            let price = SharePrice::new(10.0).unwrap();
            let total = price.multiply_by_quantity(5).unwrap();
            assert_eq!(total.value(), 50.0);
        }
    }

    mod revenue_amount_tests {
        use super::*;

        #[test]
        fn should_calculate_revenue_share_correctly() {
            let revenue = RevenueAmount::new(1000.0).unwrap();
            let percentage = OwnershipPercentage::new(10.0).unwrap();
            let share = percentage.calculate_revenue_share(&revenue).unwrap();
            assert_eq!(share.value(), 100.0);
        }

        #[test]
        fn should_calculate_small_percentage_correctly() {
            let revenue = RevenueAmount::new(1000.0).unwrap();
            let percentage = OwnershipPercentage::new(0.5).unwrap();
            let share = percentage.calculate_revenue_share(&revenue).unwrap();
            assert_eq!(share.value(), 5.0);
        }
    }
} 