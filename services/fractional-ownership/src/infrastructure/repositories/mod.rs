// Implementaciones concretas de repositorios
// Estas implementan los traits definidos en el domain

pub mod postgres_repositories;
pub mod in_memory_repositories;

// Re-exports de implementaciones
pub use postgres_repositories::*;
pub use in_memory_repositories::*; 