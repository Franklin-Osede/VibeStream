//! Implementaciones de infraestructura para Campaign (repositorios, mensajería, etc.)

pub mod in_memory_repository;
pub mod postgres_repository;
pub mod event_publisher;

pub use postgres_repository::*;
pub use event_publisher::*; 