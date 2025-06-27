// Infrastructure layer - Implementaciones concretas de persistencia y servicios externos

pub mod repositories;
pub mod database;
pub mod messaging;
pub mod external_services;

// Re-exports
pub use repositories::*;
pub use database::*;
pub use messaging::*; 