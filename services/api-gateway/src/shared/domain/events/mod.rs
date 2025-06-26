use chrono::{DateTime, Utc};

/// Trait que todo evento de dominio debe implementar.
/// Inspirado en las definiciones de DDD, los eventos deben ser:
///  - Inmutables
///  - Serieables (para mensajería)
///  - Enviables entre threads (`Send + Sync`)
///  - Clonables para poder propagar en distintas capas
pub trait DomainEvent: Clone + Send + Sync + core::fmt::Debug + 'static {
    fn occurred_on(&self) -> DateTime<Utc>;
    /// Nombre del evento (útil para enrutamiento basado en string)
    fn name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
}

/// Envoltura estándar para publicar eventos en buses o persistir en outbox.
#[derive(Clone, Debug)]
pub struct EventEnvelope<E: DomainEvent> {
    pub aggregate_id: uuid::Uuid,
    pub payload: E,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug)]
pub struct EventMetadata {
    pub occurred_on: DateTime<Utc>,
}

impl<E: DomainEvent> EventEnvelope<E> {
    pub fn new(aggregate_id: uuid::Uuid, payload: E) -> Self {
        let occurred_on = payload.occurred_on();
        Self {
            aggregate_id,
            payload,
            metadata: EventMetadata { occurred_on },
        }
    }
} 