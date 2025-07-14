//! Capacidades de dominio compartidas (eventos, errores, repositorios)

pub mod events;
pub mod errors;
pub mod repositories;

pub use events::{DomainEvent, EventMetadata}; 