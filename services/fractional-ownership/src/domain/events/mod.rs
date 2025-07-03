use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt::Debug;

use crate::domain::errors::FractionalOwnershipError;
use crate::domain::value_objects::RevenueAmount;

/// Trait base para todos los eventos de dominio
pub trait DomainEvent: Debug + Send + Sync {
    fn event_id(&self) -> Uuid;
    fn aggregate_id(&self) -> Uuid;
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn event_version(&self) -> u32;
}

/// Evento: Se compraron acciones de una canción fraccionada
#[derive(Debug, Clone)]
pub struct SharePurchased {
    event_id: Uuid,
    fractional_song_id: Uuid,
    buyer_id: Uuid,
    shares_quantity: u32,
    total_amount: RevenueAmount,
    occurred_at: DateTime<Utc>,
}

impl SharePurchased {
    pub fn new(
        fractional_song_id: Uuid,
        buyer_id: Uuid,
        shares_quantity: u32,
        total_amount: RevenueAmount,
    ) -> Self {
        SharePurchased {
            event_id: Uuid::new_v4(),
            fractional_song_id,
            buyer_id,
            shares_quantity,
            total_amount,
            occurred_at: Utc::now(),
        }
    }

    // Getters específicos del evento
    pub fn buyer_id(&self) -> Uuid { self.buyer_id }
    pub fn shares_quantity(&self) -> u32 { self.shares_quantity }
    pub fn total_amount(&self) -> &RevenueAmount { &self.total_amount }
}

impl DomainEvent for SharePurchased {
    fn event_id(&self) -> Uuid { self.event_id }
    fn aggregate_id(&self) -> Uuid { self.fractional_song_id }
    fn event_type(&self) -> &'static str { "SharePurchased" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_version(&self) -> u32 { 1 }
}

/// Evento: Se transfirieron acciones entre usuarios
#[derive(Debug, Clone)]
pub struct ShareTransferred {
    event_id: Uuid,
    fractional_song_id: Uuid,
    from_user_id: Uuid,
    to_user_id: Uuid,
    shares_quantity: u32,
    total_amount: RevenueAmount,
    occurred_at: DateTime<Utc>,
}

impl ShareTransferred {
    pub fn new(
        fractional_song_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
        shares_quantity: u32,
        total_amount: RevenueAmount,
    ) -> Self {
        ShareTransferred {
            event_id: Uuid::new_v4(),
            fractional_song_id,
            from_user_id,
            to_user_id,
            shares_quantity,
            total_amount,
            occurred_at: Utc::now(),
        }
    }

    // Getters específicos del evento
    pub fn from_user_id(&self) -> Uuid { self.from_user_id }
    pub fn to_user_id(&self) -> Uuid { self.to_user_id }
    pub fn shares_quantity(&self) -> u32 { self.shares_quantity }
    pub fn total_amount(&self) -> &RevenueAmount { &self.total_amount }
}

impl DomainEvent for ShareTransferred {
    fn event_id(&self) -> Uuid { self.event_id }
    fn aggregate_id(&self) -> Uuid { self.fractional_song_id }
    fn event_type(&self) -> &'static str { "ShareTransferred" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_version(&self) -> u32 { 1 }
}

/// Evento: Se distribuyeron ingresos entre los poseedores de acciones
#[derive(Debug, Clone)]
pub struct RevenueDistributed {
    event_id: Uuid,
    fractional_song_id: Uuid,
    total_revenue: RevenueAmount,
    shareholders_count: u32,
    occurred_at: DateTime<Utc>,
}

impl RevenueDistributed {
    pub fn new(
        fractional_song_id: Uuid,
        total_revenue: RevenueAmount,
        shareholders_count: u32,
    ) -> Self {
        RevenueDistributed {
            event_id: Uuid::new_v4(),
            fractional_song_id,
            total_revenue,
            shareholders_count,
            occurred_at: Utc::now(),
        }
    }

    // Getters específicos del evento
    pub fn total_revenue(&self) -> &RevenueAmount { &self.total_revenue }
    pub fn shareholders_count(&self) -> u32 { self.shareholders_count }
}

impl DomainEvent for RevenueDistributed {
    fn event_id(&self) -> Uuid { self.event_id }
    fn aggregate_id(&self) -> Uuid { self.fractional_song_id }
    fn event_type(&self) -> &'static str { "RevenueDistributed" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_version(&self) -> u32 { 1 }
}

/// Evento: Se creó una nueva canción fraccionada
#[derive(Debug, Clone)]
pub struct FractionalSongCreated {
    event_id: Uuid,
    fractional_song_id: Uuid,
    song_id: Uuid, // Referencia al Song Context
    artist_id: Uuid,
    title: String,
    total_shares: u32,
    initial_share_price: RevenueAmount,
    occurred_at: DateTime<Utc>,
}

impl FractionalSongCreated {
    pub fn new(
        fractional_song_id: Uuid,
        song_id: Uuid,
        artist_id: Uuid,
        title: String,
        total_shares: u32,
        initial_share_price: RevenueAmount,
    ) -> Self {
        FractionalSongCreated {
            event_id: Uuid::new_v4(),
            fractional_song_id,
            song_id,
            artist_id,
            title,
            total_shares,
            initial_share_price,
            occurred_at: Utc::now(),
        }
    }

    // Getters específicos del evento
    pub fn song_id(&self) -> Uuid { self.song_id }
    pub fn artist_id(&self) -> Uuid { self.artist_id }
    pub fn title(&self) -> &str { &self.title }
    pub fn total_shares(&self) -> u32 { self.total_shares }
    pub fn initial_share_price(&self) -> &RevenueAmount { &self.initial_share_price }
}

impl DomainEvent for FractionalSongCreated {
    fn event_id(&self) -> Uuid { self.event_id }
    fn aggregate_id(&self) -> Uuid { self.fractional_song_id }
    fn event_type(&self) -> &'static str { "FractionalSongCreated" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_version(&self) -> u32 { 1 }
}

/// Evento: El precio de las acciones se actualizó
#[derive(Debug, Clone)]
pub struct SharePriceUpdated {
    event_id: Uuid,
    fractional_song_id: Uuid,
    old_price: RevenueAmount,
    new_price: RevenueAmount,
    price_change_percentage: f64,
    occurred_at: DateTime<Utc>,
}

impl SharePriceUpdated {
    pub fn new(
        fractional_song_id: Uuid,
        old_price: RevenueAmount,
        new_price: RevenueAmount,
    ) -> Self {
        let price_change_percentage = if old_price.amount() > 0.0 {
            ((new_price.amount() - old_price.amount()) / old_price.amount()) * 100.0
        } else {
            0.0
        };

        SharePriceUpdated {
            event_id: Uuid::new_v4(),
            fractional_song_id,
            old_price,
            new_price,
            price_change_percentage,
            occurred_at: Utc::now(),
        }
    }

    // Getters específicos del evento
    pub fn old_price(&self) -> &RevenueAmount { &self.old_price }
    pub fn new_price(&self) -> &RevenueAmount { &self.new_price }
    pub fn price_change_percentage(&self) -> f64 { self.price_change_percentage }
}

impl DomainEvent for SharePriceUpdated {
    fn event_id(&self) -> Uuid { self.event_id }
    fn aggregate_id(&self) -> Uuid { self.fractional_song_id }
    fn event_type(&self) -> &'static str { "SharePriceUpdated" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_version(&self) -> u32 { 1 }
}

/// Enum para handlers específicos que permite dyn compatibility
pub enum DomainEventHandlerWrapper {
    SharePurchase(SharePurchaseEventHandler),
    RevenueDistribution(RevenueDistributionEventHandler),
}

impl DomainEventHandlerWrapper {
    pub async fn handle(&self, event: &dyn DomainEvent) -> Result<(), FractionalOwnershipError> {
        match self {
            DomainEventHandlerWrapper::SharePurchase(handler) => handler.handle(event).await,
            DomainEventHandlerWrapper::RevenueDistribution(handler) => handler.handle(event).await,
        }
    }
    
    pub fn interested_in(&self) -> Vec<String> {
        match self {
            DomainEventHandlerWrapper::SharePurchase(handler) => handler.interested_in(),
            DomainEventHandlerWrapper::RevenueDistribution(handler) => handler.interested_in(),
        }
    }
}

/// Trait para manejar eventos de dominio
pub trait DomainEventHandler: Send + Sync {
    fn handle(&self, event: &dyn DomainEvent) -> impl std::future::Future<Output = Result<(), FractionalOwnershipError>> + Send;
    fn interested_in(&self) -> Vec<String>;
}

/// Handler específico para eventos de compra de acciones
pub struct SharePurchaseEventHandler {
    // Aquí irían dependencias como repositories, services, etc.
}

impl SharePurchaseEventHandler {
    pub fn new() -> Self {
        SharePurchaseEventHandler {}
    }
}

impl DomainEventHandler for SharePurchaseEventHandler {
    fn handle(&self, event: &dyn DomainEvent) -> impl std::future::Future<Output = Result<(), FractionalOwnershipError>> + Send {
        async move {
            match event.event_type() {
                "SharePurchased" => {
                    // Lógica para manejar compra de acciones
                    // Ejemplo: Notificar al usuario, actualizar métricas, etc.
                    println!("Manejando evento de compra de acciones: {:?}", event.aggregate_id());
                    Ok(())
                }
                _ => Ok(())
            }
        }
    }

    fn interested_in(&self) -> Vec<String> {
        vec!["SharePurchased".to_string()]
    }
}

/// Handler para eventos de distribución de ingresos
pub struct RevenueDistributionEventHandler {
    // Dependencias para comunicarse con payment context, user context, etc.
}

impl RevenueDistributionEventHandler {
    pub fn new() -> Self {
        RevenueDistributionEventHandler {}
    }
}

impl DomainEventHandler for RevenueDistributionEventHandler {
    fn handle(&self, event: &dyn DomainEvent) -> impl std::future::Future<Output = Result<(), FractionalOwnershipError>> + Send {
        async move {
            match event.event_type() {
                "RevenueDistributed" => {
                    // Lógica para manejar distribución de ingresos
                    // Ejemplo: Procesar pagos, actualizar balances de usuarios, etc.
                    println!("Manejando evento de distribución de ingresos: {:?}", event.aggregate_id());
                    Ok(())
                }
                _ => Ok(())
            }
        }
    }

    fn interested_in(&self) -> Vec<String> {
        vec!["RevenueDistributed".to_string()]
    }
}

// Re-exports
pub mod event_store;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_share_purchased_event() {
        let song_id = Uuid::new_v4();
        let buyer_id = Uuid::new_v4();
        let amount = RevenueAmount::new(100.0).unwrap();

        let event = SharePurchased::new(song_id, buyer_id, 10, amount);

        assert_eq!(event.aggregate_id(), song_id);
        assert_eq!(event.buyer_id(), buyer_id);
        assert_eq!(event.shares_quantity(), 10);
        assert_eq!(event.event_type(), "SharePurchased");
    }

    #[test]
    fn should_create_revenue_distributed_event() {
        let song_id = Uuid::new_v4();
        let revenue = RevenueAmount::new(1000.0).unwrap();

        let event = RevenueDistributed::new(song_id, revenue, 5);

        assert_eq!(event.aggregate_id(), song_id);
        assert_eq!(event.shareholders_count(), 5);
        assert_eq!(event.event_type(), "RevenueDistributed");
    }

    #[test]
    fn should_calculate_price_change_percentage() {
        let song_id = Uuid::new_v4();
        let old_price = RevenueAmount::new(10.0).unwrap();
        let new_price = RevenueAmount::new(12.0).unwrap();

        let event = SharePriceUpdated::new(song_id, old_price, new_price);

        assert_eq!(event.price_change_percentage(), 20.0);
    }

    #[tokio::test]
    async fn should_dispatch_events_to_handlers() {
        // Simplified test without dispatcher
        let handler = SharePurchaseEventHandler::new();

        let song_id = Uuid::new_v4();
        let buyer_id = Uuid::new_v4();
        let amount = RevenueAmount::new(100.0).unwrap();
        let event = SharePurchased::new(song_id, buyer_id, 10, amount);

        // Test que el handler puede manejar el evento directamente
        let result = handler.handle(&event).await;
        assert!(result.is_ok());
    }
} 