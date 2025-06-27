// TODO: Implement FractionalShare, OwnershipContract entities 

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount};
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::collections::HashMap;

/// Entidad que representa una canción con participaciones fraccionadas
#[derive(Debug, Clone, PartialEq)]
pub struct FractionalSong {
    id: Uuid,
    song_id: Uuid, // Referencia al Song Context
    artist_id: Uuid,
    title: String,
    total_shares: u32,
    available_shares: u32,
    share_price: SharePrice,
    total_revenue: RevenueAmount,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl FractionalSong {
    pub fn new(
        song_id: Uuid,
        artist_id: Uuid,
        title: String,
        total_shares: u32,
        share_price: SharePrice,
    ) -> Result<Self, FractionalOwnershipError> {
        if total_shares == 0 {
            return Err(FractionalOwnershipError::ValidationError("Las acciones totales deben ser mayor a 0".to_string()));
        }

        if total_shares > 10000 {
            return Err(FractionalOwnershipError::ValidationError("No se pueden crear más de 10,000 acciones por canción".to_string()));
        }

        Ok(FractionalSong {
            id: Uuid::new_v4(),
            song_id,
            artist_id,
            title,
            total_shares,
            available_shares: total_shares,
            share_price,
            total_revenue: RevenueAmount::new(0.0)?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn song_id(&self) -> Uuid { self.song_id }
    pub fn artist_id(&self) -> Uuid { self.artist_id }
    pub fn title(&self) -> &str { &self.title }
    pub fn total_shares(&self) -> u32 { self.total_shares }
    pub fn available_shares(&self) -> u32 { self.available_shares }
    pub fn share_price(&self) -> &SharePrice { &self.share_price }
    pub fn current_price_per_share(&self) -> &SharePrice { &self.share_price } // Alias para compatibilidad
    pub fn total_revenue(&self) -> &RevenueAmount { &self.total_revenue }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }

    /// Lógica de dominio: Reservar acciones para compra
    pub fn reserve_shares(&mut self, quantity: u32) -> Result<(), FractionalOwnershipError> {
        if quantity == 0 {
            return Err(FractionalOwnershipError::ValidationError("La cantidad debe ser mayor a 0".to_string()));
        }

        if quantity > self.available_shares {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                format!("No hay suficientes acciones disponibles. Disponibles: {}, Solicitadas: {}", 
                        self.available_shares, quantity)
            ));
        }

        self.available_shares -= quantity;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Lógica de dominio: Liberar acciones reservadas (en caso de cancelación)
    pub fn release_shares(&mut self, quantity: u32) -> Result<(), FractionalOwnershipError> {
        if self.available_shares + quantity > self.total_shares {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "No se pueden liberar más acciones de las que existen".to_string()
            ));
        }

        self.available_shares += quantity;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Lógica de dominio: Agregar ingresos y actualizar precio
    pub fn add_revenue(&mut self, amount: RevenueAmount) -> Result<(), FractionalOwnershipError> {
        self.total_revenue = RevenueAmount::new(self.total_revenue.value() + amount.value())?;
        
        // Aumentar el precio de las acciones basado en los ingresos
        let revenue_multiplier = 1.0 + (amount.value() / 1000.0); // Cada $1000 aumenta 1% el precio
        let new_price = self.share_price.value() * revenue_multiplier;
        self.share_price = SharePrice::new(new_price)?;
        
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Verificar si la canción está completamente vendida
    pub fn is_fully_sold(&self) -> bool {
        self.available_shares == 0
    }

    /// Obtener porcentaje de acciones vendidas
    pub fn sold_percentage(&self) -> f64 {
        let sold_shares = self.total_shares - self.available_shares;
        (sold_shares as f64 / self.total_shares as f64) * 100.0
    }
}

/// Entidad que representa la participación de un usuario en una canción
#[derive(Debug, Clone, PartialEq)]
pub struct ShareOwnership {
    id: Uuid,
    user_id: Uuid,
    fractional_song_id: Uuid,
    shares_owned: u32,
    ownership_percentage: OwnershipPercentage,
    purchase_price: SharePrice,
    total_earnings: RevenueAmount,
    purchase_date: DateTime<Utc>,
    last_earning_date: Option<DateTime<Utc>>,
}

impl ShareOwnership {
    pub fn new(
        user_id: Uuid,
        fractional_song_id: Uuid,
        shares_owned: u32,
        total_song_shares: u32,
        purchase_price: SharePrice,
    ) -> Result<Self, FractionalOwnershipError> {
        if shares_owned == 0 {
            return Err(FractionalOwnershipError::ValidationError("Debe poseer al menos 1 acción".to_string()));
        }

        let ownership_percentage_value = (shares_owned as f64 / total_song_shares as f64) * 100.0;
        let ownership_percentage = OwnershipPercentage::new(ownership_percentage_value)?;

        Ok(ShareOwnership {
            id: Uuid::new_v4(),
            user_id,
            fractional_song_id,
            shares_owned,
            ownership_percentage,
            purchase_price,
            total_earnings: RevenueAmount::new(0.0)?,
            purchase_date: Utc::now(),
            last_earning_date: None,
        })
    }

    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn user_id(&self) -> Uuid { self.user_id }
    pub fn fractional_song_id(&self) -> Uuid { self.fractional_song_id }
    pub fn song_id(&self) -> Uuid { self.fractional_song_id }
    pub fn shares_owned(&self) -> u32 { self.shares_owned }
    pub fn ownership_percentage(&self) -> &OwnershipPercentage { &self.ownership_percentage }
    pub fn percentage(&self) -> &OwnershipPercentage { &self.ownership_percentage } // Alias para compatibilidad
    pub fn song_id(&self) -> Uuid { self.fractional_song_id } // Alias para compatibilidad
    pub fn percentage(&self) -> &OwnershipPercentage { &self.ownership_percentage }
    pub fn purchase_price(&self) -> &SharePrice { &self.purchase_price }
    pub fn total_earnings(&self) -> &RevenueAmount { &self.total_earnings }
    pub fn purchase_date(&self) -> DateTime<Utc> { self.purchase_date }
    pub fn last_earning_date(&self) -> Option<DateTime<Utc>> { self.last_earning_date }

    /// Lógica de dominio: Agregar ganancias por royalties
    pub fn add_earnings(&mut self, amount: RevenueAmount) -> Result<(), FractionalOwnershipError> {
        self.total_earnings = RevenueAmount::new(self.total_earnings.value() + amount.value())?;
        self.last_earning_date = Some(Utc::now());
        Ok(())
    }

    /// Lógica de dominio: Calcular ganancias basadas en porcentaje de ownership
    pub fn calculate_revenue_share(&self, total_revenue: &RevenueAmount) -> Result<RevenueAmount, FractionalOwnershipError> {
        self.ownership_percentage.calculate_revenue_share(total_revenue)
    }

    /// Verificar si ha recibido ganancias recientemente
    pub fn has_recent_earnings(&self, days: i64) -> bool {
        if let Some(last_earning) = self.last_earning_date {
            let days_since_earning = Utc::now().signed_duration_since(last_earning).num_days();
            days_since_earning <= days
        } else {
            false
        }
    }

    /// Calcular ROI (Return on Investment)
    pub fn calculate_roi(&self) -> f64 {
        let investment = self.purchase_price.value() * self.shares_owned as f64;
        if investment == 0.0 {
            0.0
        } else {
            (self.total_earnings.value() / investment) * 100.0
        }
    }
}

/// Entidad que representa una transacción de compra/venta de acciones
#[derive(Debug, Clone, PartialEq)]
pub struct ShareTransaction {
    id: Uuid,
    buyer_id: Option<Uuid>, // None para ventas
    seller_id: Option<Uuid>, // None para compras iniciales
    fractional_song_id: Uuid,
    shares_quantity: u32,
    price_per_share: SharePrice,
    total_amount: RevenueAmount,
    transaction_type: TransactionType,
    status: TransactionStatus,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Purchase,  // Compra inicial del artista
    Transfer,  // Transferencia entre usuarios
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

impl ShareTransaction {
    pub fn new_purchase(
        buyer_id: Uuid,
        fractional_song_id: Uuid,
        shares_quantity: u32,
        price_per_share: SharePrice,
    ) -> Result<Self, FractionalOwnershipError> {
        let total_amount = price_per_share.multiply_by_quantity(shares_quantity)?;

        Ok(ShareTransaction {
            id: Uuid::new_v4(),
            buyer_id: Some(buyer_id),
            seller_id: None,
            fractional_song_id,
            shares_quantity,
            price_per_share,
            total_amount,
            transaction_type: TransactionType::Purchase,
            status: TransactionStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
        })
    }

    pub fn new_transfer(
        buyer_id: Uuid,
        seller_id: Uuid,
        fractional_song_id: Uuid,
        shares_quantity: u32,
        price_per_share: SharePrice,
    ) -> Result<Self, FractionalOwnershipError> {
        let total_amount = price_per_share.multiply_by_quantity(shares_quantity)?;

        Ok(ShareTransaction {
            id: Uuid::new_v4(),
            buyer_id: Some(buyer_id),
            seller_id: Some(seller_id),
            fractional_song_id,
            shares_quantity,
            price_per_share,
            total_amount,
            transaction_type: TransactionType::Transfer,
            status: TransactionStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
        })
    }

    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn buyer_id(&self) -> Option<Uuid> { self.buyer_id }
    pub fn seller_id(&self) -> Option<Uuid> { self.seller_id }
    pub fn fractional_song_id(&self) -> Uuid { self.fractional_song_id }
    pub fn shares_quantity(&self) -> u32 { self.shares_quantity }
    pub fn price_per_share(&self) -> &SharePrice { &self.price_per_share }
    pub fn total_amount(&self) -> &RevenueAmount { &self.total_amount }
    pub fn transaction_type(&self) -> &TransactionType { &self.transaction_type }
    pub fn status(&self) -> &TransactionStatus { &self.status }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn completed_at(&self) -> Option<DateTime<Utc>> { self.completed_at }

    /// Lógica de dominio: Completar transacción
    pub fn complete(&mut self) -> Result<(), FractionalOwnershipError> {
        match self.status {
            TransactionStatus::Pending => {
                self.status = TransactionStatus::Completed;
                self.completed_at = Some(Utc::now());
                Ok(())
            }
            _ => Err(FractionalOwnershipError::BusinessRuleViolation(
                "Solo se pueden completar transacciones pendientes".to_string()
            ))
        }
    }

    /// Lógica de dominio: Cancelar transacción
    pub fn cancel(&mut self) -> Result<(), FractionalOwnershipError> {
        match self.status {
            TransactionStatus::Pending => {
                self.status = TransactionStatus::Cancelled;
                Ok(())
            }
            _ => Err(FractionalOwnershipError::BusinessRuleViolation(
                "Solo se pueden cancelar transacciones pendientes".to_string()
            ))
        }
    }

    /// Lógica de dominio: Marcar como fallida
    pub fn fail(&mut self) -> Result<(), FractionalOwnershipError> {
        match self.status {
            TransactionStatus::Pending => {
                self.status = TransactionStatus::Failed;
                Ok(())
            }
            _ => Err(FractionalOwnershipError::BusinessRuleViolation(
                "Solo se pueden fallar transacciones pendientes".to_string()
            ))
        }
    }

    /// Verificar si la transacción está finalizada
    pub fn is_finalized(&self) -> bool {
        matches!(self.status, TransactionStatus::Completed | TransactionStatus::Failed | TransactionStatus::Cancelled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fractional_song_tests {
        use super::*;

        #[test]
        fn should_create_valid_fractional_song() {
            let song_id = Uuid::new_v4();
            let artist_id = Uuid::new_v4();
            let share_price = SharePrice::new(10.0).unwrap();

            let song = FractionalSong::new(
                song_id,
                artist_id,
                "Test Song".to_string(),
                1000,
                share_price
            ).unwrap();

            assert_eq!(song.song_id(), song_id);
            assert_eq!(song.artist_id(), artist_id);
            assert_eq!(song.title(), "Test Song");
            assert_eq!(song.total_shares(), 1000);
            assert_eq!(song.available_shares(), 1000);
        }

        #[test]
        fn should_reject_zero_total_shares() {
            let song_id = Uuid::new_v4();
            let artist_id = Uuid::new_v4();
            let share_price = SharePrice::new(10.0).unwrap();

            let result = FractionalSong::new(
                song_id,
                artist_id,
                "Test Song".to_string(),
                0,
                share_price
            );

            assert!(result.is_err());
        }

        #[test]
        fn should_reserve_shares_correctly() {
            let song_id = Uuid::new_v4();
            let artist_id = Uuid::new_v4();
            let share_price = SharePrice::new(10.0).unwrap();

            let mut song = FractionalSong::new(
                song_id,
                artist_id,
                "Test Song".to_string(),
                1000,
                share_price
            ).unwrap();

            song.reserve_shares(100).unwrap();
            assert_eq!(song.available_shares(), 900);
        }

        #[test]
        fn should_reject_excessive_share_reservation() {
            let song_id = Uuid::new_v4();
            let artist_id = Uuid::new_v4();
            let share_price = SharePrice::new(10.0).unwrap();

            let mut song = FractionalSong::new(
                song_id,
                artist_id,
                "Test Song".to_string(),
                1000,
                share_price
            ).unwrap();

            let result = song.reserve_shares(1001);
            assert!(result.is_err());
        }
    }

    mod share_ownership_tests {
        use super::*;

        #[test]
        fn should_create_valid_share_ownership() {
            let user_id = Uuid::new_v4();
            let song_id = Uuid::new_v4();
            let purchase_price = SharePrice::new(10.0).unwrap();

            let ownership = ShareOwnership::new(
                user_id,
                song_id,
                100,
                1000,
                purchase_price
            ).unwrap();

            assert_eq!(ownership.user_id(), user_id);
            assert_eq!(ownership.shares_owned(), 100);
            assert_eq!(ownership.ownership_percentage().value(), 10.0);
        }

        #[test]
        fn should_calculate_roi_correctly() {
            let user_id = Uuid::new_v4();
            let song_id = Uuid::new_v4();
            let purchase_price = SharePrice::new(10.0).unwrap();

            let mut ownership = ShareOwnership::new(
                user_id,
                song_id,
                100,
                1000,
                purchase_price
            ).unwrap();

            let earnings = RevenueAmount::new(500.0).unwrap();
            ownership.add_earnings(earnings).unwrap();

            // ROI = (500 / (10 * 100)) * 100 = 50%
            assert_eq!(ownership.calculate_roi(), 50.0);
        }
    }

    mod share_transaction_tests {
        use super::*;

        #[test]
        fn should_create_valid_purchase_transaction() {
            let buyer_id = Uuid::new_v4();
            let song_id = Uuid::new_v4();
            let price = SharePrice::new(10.0).unwrap();

            let transaction = ShareTransaction::new_purchase(
                buyer_id,
                song_id,
                100,
                price
            ).unwrap();

            assert_eq!(transaction.buyer_id(), Some(buyer_id));
            assert_eq!(transaction.seller_id(), None);
            assert_eq!(transaction.shares_quantity(), 100);
            assert_eq!(transaction.total_amount().value(), 1000.0);
        }

        #[test]
        fn should_complete_transaction_correctly() {
            let buyer_id = Uuid::new_v4();
            let song_id = Uuid::new_v4();
            let price = SharePrice::new(10.0).unwrap();

            let mut transaction = ShareTransaction::new_purchase(
                buyer_id,
                song_id,
                100,
                price
            ).unwrap();

            transaction.complete().unwrap();
            assert_eq!(*transaction.status(), TransactionStatus::Completed);
            assert!(transaction.completed_at().is_some());
        }
    }
} 