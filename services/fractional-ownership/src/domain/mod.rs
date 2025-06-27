// Domain layer - Lógica de negocio pura sin dependencias externas

pub mod aggregates;
pub mod entities;
pub mod errors;
pub mod events;
pub mod repositories;
pub mod services;
pub mod value_objects;

// Re-exports para facilidad de uso
pub use aggregates::*;
pub use entities::*;
pub use errors::*;
pub use events::*;
pub use repositories::*;
pub use services::*;
pub use value_objects::*; 