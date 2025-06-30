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
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "El precio por acción debe ser mayor a 0".to_string()
            ));
        }
        
        if amount > 1_000_000.0 {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "El precio por acción no puede exceder $1,000,000".to_string()
            ));
        }
        
        Ok(SharePrice { amount })
    }
    
    pub fn value(&self) -> f64 {
        self.amount
    }
    
    pub fn amount(&self) -> f64 {
        self.amount
    }
    
    pub fn as_f64(&self) -> f64 {
        self.amount
    }
    
    pub fn multiply(&self, factor: f64) -> Result<Self, FractionalOwnershipError> {
        Self::new(self.amount * factor)
    }
    
    pub fn multiply_by_quantity(&self, quantity: u32) -> Result<RevenueAmount, FractionalOwnershipError> {
        let total_amount = self.amount * quantity as f64;
        RevenueAmount::new(total_amount)
    }
    
    pub fn add(&self, other: &SharePrice) -> Result<Self, FractionalOwnershipError> {
        Self::new(self.amount + other.amount)
    }
    
    pub fn percentage_change(&self, other: &SharePrice) -> f64 {
        if self.amount == 0.0 {
            return 0.0;
        }
        ((other.amount - self.amount) / self.amount) * 100.0
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
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "El monto de ingresos no puede ser negativo".to_string()
            ));
        }
        
        Ok(RevenueAmount { amount })
    }
    
    pub fn value(&self) -> f64 {
        self.amount
    }
    
    pub fn amount(&self) -> f64 {
        self.amount
    }
    
    pub fn as_f64(&self) -> f64 {
        self.amount
    }
    
    pub fn add(&self, other: &RevenueAmount) -> Result<Self, FractionalOwnershipError> {
        Self::new(self.amount + other.amount)
    }
    
    pub fn multiply(&self, factor: f64) -> Result<Self, FractionalOwnershipError> {
        Self::new(self.amount * factor)
    }
    
    pub fn multiply_by_quantity(&self, quantity: u32) -> Result<RevenueAmount, FractionalOwnershipError> {
        let total_amount = self.amount * quantity as f64;
        RevenueAmount::new(total_amount)
    }
    
    pub fn subtract(&self, other: &RevenueAmount) -> Result<Self, FractionalOwnershipError> {
        if self.amount < other.amount {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "No se puede restar más ingresos de los disponibles".to_string()
            ));
        }
        Self::new(self.amount - other.amount)
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

/// Límites de ownership configurables para el sistema
#[derive(Debug, Clone, PartialEq)]
pub struct OwnershipLimits {
    max_individual_ownership_percentage: f64,
    max_shares_per_transaction: u32,
    artist_can_own_shares: bool,
}

impl Default for OwnershipLimits {
    /// Configuración por defecto: límites conservadores
    fn default() -> Self {
        Self::conservative()
    }
}

impl OwnershipLimits {
    /// Crear límites de ownership con validación
    pub fn new(
        max_individual_ownership_percentage: f64,
        max_shares_per_transaction: u32,
        artist_can_own_shares: bool,
    ) -> Result<Self, FractionalOwnershipError> {
        if max_individual_ownership_percentage <= 0.0 || max_individual_ownership_percentage > 100.0 {
            return Err(FractionalOwnershipError::ValidationError(
                format!("Maximum individual ownership percentage must be between 0 and 100, got {}", max_individual_ownership_percentage)
            ));
        }

        if max_shares_per_transaction == 0 {
            return Err(FractionalOwnershipError::ValidationError(
                "Maximum shares per transaction must be greater than 0".to_string()
            ));
        }

        Ok(Self {
            max_individual_ownership_percentage,
            max_shares_per_transaction,
            artist_can_own_shares,
        })
    }

    /// Crear límites conservadores (30% max ownership, 1000 shares por transacción, artista no puede comprar)
    pub fn conservative() -> Self {
        Self {
            max_individual_ownership_percentage: 30.0,
            max_shares_per_transaction: 1000,
            artist_can_own_shares: false,
        }
    }

    /// Crear límites liberales (50% max ownership, 5000 shares por transacción, artista puede comprar)
    pub fn liberal() -> Self {
        Self {
            max_individual_ownership_percentage: 50.0,
            max_shares_per_transaction: 5000,
            artist_can_own_shares: true,
        }
    }

    /// Crear límites muy restrictivos (10% max ownership, 100 shares por transacción)
    pub fn restrictive() -> Self {
        Self {
            max_individual_ownership_percentage: 10.0,
            max_shares_per_transaction: 100,
            artist_can_own_shares: false,
        }
    }

    /// Máximo porcentaje de ownership individual permitido
    pub fn max_individual_ownership_percentage(&self) -> f64 {
        self.max_individual_ownership_percentage
    }

    /// Máximo número de shares por transacción
    pub fn max_shares_per_transaction(&self) -> u32 {
        self.max_shares_per_transaction
    }

    /// Si el artista puede comprar shares de su propia canción
    pub fn artist_can_own_shares(&self) -> bool {
        self.artist_can_own_shares
    }

    /// Validar si un porcentaje de ownership está dentro de los límites
    pub fn validate_ownership_percentage(&self, percentage: f64) -> Result<(), FractionalOwnershipError> {
        if percentage > self.max_individual_ownership_percentage {
            return Err(FractionalOwnershipError::OwnershipExceedsLimit {
                current: percentage,
                additional: 0.0,
            });
        }
        Ok(())
    }

    /// Validar si una cantidad de shares está dentro de los límites por transacción
    pub fn validate_shares_per_transaction(&self, shares: u32) -> Result<(), FractionalOwnershipError> {
        if shares > self.max_shares_per_transaction {
            return Err(FractionalOwnershipError::ValidationError(
                format!("Cannot purchase more than {} shares in a single transaction", self.max_shares_per_transaction)
            ));
        }
        Ok(())
    }

    /// Validar si un artista puede comprar shares de su propia canción
    pub fn validate_artist_purchase(&self, is_artist_purchase: bool) -> Result<(), FractionalOwnershipError> {
        if is_artist_purchase && !self.artist_can_own_shares {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "Artist cannot buy shares of their own song under current ownership limits".to_string()
            ));
        }
        Ok(())
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

    #[test]
    fn ownership_limits_should_validate_percentage_bounds() {
        // Test: Porcentaje debe estar entre 0 y 100
        let result_zero = OwnershipLimits::new(0.0, 1000, false);
        assert!(result_zero.is_err());

        let result_negative = OwnershipLimits::new(-10.0, 1000, false);
        assert!(result_negative.is_err());

        let result_over_hundred = OwnershipLimits::new(101.0, 1000, false);
        assert!(result_over_hundred.is_err());

        // Casos válidos
        let result_valid = OwnershipLimits::new(50.0, 1000, false);
        assert!(result_valid.is_ok());

        let result_edge_low = OwnershipLimits::new(0.01, 1000, false);
        assert!(result_edge_low.is_ok());

        let result_edge_high = OwnershipLimits::new(100.0, 1000, false);
        assert!(result_edge_high.is_ok());
    }

    #[test]
    fn ownership_limits_should_validate_shares_per_transaction() {
        // Test: Shares por transacción debe ser mayor que 0
        let result_zero = OwnershipLimits::new(50.0, 0, false);
        assert!(result_zero.is_err());

        // Caso válido
        let result_valid = OwnershipLimits::new(50.0, 1, false);
        assert!(result_valid.is_ok());

        let result_large = OwnershipLimits::new(50.0, 10000, false);
        assert!(result_large.is_ok());
    }

    #[test]
    fn ownership_limits_presets_should_work() {
        // Test presets conservadores
        let conservative = OwnershipLimits::conservative();
        assert_eq!(conservative.max_individual_ownership_percentage(), 30.0);
        assert_eq!(conservative.max_shares_per_transaction(), 1000);
        assert!(!conservative.artist_can_own_shares());

        // Test presets liberales
        let liberal = OwnershipLimits::liberal();
        assert_eq!(liberal.max_individual_ownership_percentage(), 50.0);
        assert_eq!(liberal.max_shares_per_transaction(), 5000);
        assert!(liberal.artist_can_own_shares());

        // Test presets restrictivos
        let restrictive = OwnershipLimits::restrictive();
        assert_eq!(restrictive.max_individual_ownership_percentage(), 10.0);
        assert_eq!(restrictive.max_shares_per_transaction(), 100);
        assert!(!restrictive.artist_can_own_shares());
    }

    #[test]
    fn ownership_limits_should_validate_ownership_percentage() {
        let limits = OwnershipLimits::conservative(); // 30% max

        // Test: Porcentaje válido
        let result_valid = limits.validate_ownership_percentage(25.0);
        assert!(result_valid.is_ok());

        let result_edge = limits.validate_ownership_percentage(30.0);
        assert!(result_edge.is_ok());

        // Test: Porcentaje excede límite
        let result_exceed = limits.validate_ownership_percentage(35.0);
        assert!(matches!(result_exceed, Err(FractionalOwnershipError::OwnershipExceedsLimit { .. })));
    }

    #[test]
    fn ownership_limits_should_validate_shares_per_transaction_limits() {
        let limits = OwnershipLimits::conservative(); // 1000 shares max

        // Test: Cantidad válida
        let result_valid = limits.validate_shares_per_transaction(500);
        assert!(result_valid.is_ok());

        let result_edge = limits.validate_shares_per_transaction(1000);
        assert!(result_edge.is_ok());

        // Test: Cantidad excede límite
        let result_exceed = limits.validate_shares_per_transaction(1500);
        assert!(result_exceed.is_err());
        if let Err(FractionalOwnershipError::ValidationError(msg)) = result_exceed {
            assert!(msg.contains("1000"));
        }
    }

    #[test]
    fn ownership_limits_should_validate_artist_purchase_rules() {
        // Test con límites que NO permiten al artista comprar
        let conservative = OwnershipLimits::conservative();
        
        let result_non_artist = conservative.validate_artist_purchase(false);
        assert!(result_non_artist.is_ok());

        let result_artist = conservative.validate_artist_purchase(true);
        assert!(result_artist.is_err());
        if let Err(FractionalOwnershipError::BusinessRuleViolation(msg)) = result_artist {
            assert!(msg.contains("Artist cannot buy shares"));
        }

        // Test con límites que SÍ permiten al artista comprar
        let liberal = OwnershipLimits::liberal();
        
        let result_artist_allowed = liberal.validate_artist_purchase(true);
        assert!(result_artist_allowed.is_ok());

        let result_non_artist_allowed = liberal.validate_artist_purchase(false);
        assert!(result_non_artist_allowed.is_ok());
    }

    #[test]
    fn ownership_limits_should_have_different_configurations() {
        let conservative = OwnershipLimits::conservative();
        let liberal = OwnershipLimits::liberal();
        let restrictive = OwnershipLimits::restrictive();

        // Test que sean diferentes
        assert_ne!(conservative, liberal);
        assert_ne!(conservative, restrictive);
        assert_ne!(liberal, restrictive);

        // Test ordenación por restricción
        assert!(restrictive.max_individual_ownership_percentage() < conservative.max_individual_ownership_percentage());
        assert!(conservative.max_individual_ownership_percentage() < liberal.max_individual_ownership_percentage());
    }

    #[test]
    fn ownership_limits_custom_configuration_should_work() {
        // Test configuración personalizada
        let custom = OwnershipLimits::new(15.5, 250, true).unwrap();
        
        assert_eq!(custom.max_individual_ownership_percentage(), 15.5);
        assert_eq!(custom.max_shares_per_transaction(), 250);
        assert!(custom.artist_can_own_shares());

        // Test validación con configuración personalizada
        let valid_percentage = custom.validate_ownership_percentage(10.0);
        assert!(valid_percentage.is_ok());

        let invalid_percentage = custom.validate_ownership_percentage(20.0);
        assert!(invalid_percentage.is_err());

        let valid_shares = custom.validate_shares_per_transaction(200);
        assert!(valid_shares.is_ok());

        let invalid_shares = custom.validate_shares_per_transaction(300);
        assert!(invalid_shares.is_err());
    }
} 