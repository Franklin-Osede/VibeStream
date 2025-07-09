//! Shared infrastructure components (database, messaging, security).

pub mod event_bus;
pub mod database; // Newly added database module

// Re-export common database types
pub use database::postgres::PostgresUserRepository; 