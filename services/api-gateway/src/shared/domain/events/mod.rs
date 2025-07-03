use chrono::{DateTime, Utc};

/// Trait base para eventos de dominio - dyn compatible
pub trait DomainEvent: Send + Sync + core::fmt::Debug + 'static {
    fn occurred_on(&self) -> DateTime<Utc>;
    fn event_type(&self) -> &'static str;
    /// Nombre del evento (útil para enrutamiento basado en string)
    fn name(&self) -> &'static str {
        self.event_type()
    }
    
    /// Serializar el evento para persistencia/mensajería
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Trait extendido para eventos que necesitan ser clonados
pub trait CloneableDomainEvent: DomainEvent + Clone {
    fn clone_boxed(&self) -> Box<dyn CloneableDomainEvent>;
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