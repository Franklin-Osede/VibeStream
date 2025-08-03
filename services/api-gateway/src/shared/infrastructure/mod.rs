//! Shared infrastructure components (database, messaging, security, websocket, cdn, discovery).

pub mod event_bus;
pub mod database;
pub mod websocket;
pub mod cdn;
pub mod discovery;
pub mod app_state;

// Re-export common database types
pub use database::postgres::PostgresUserRepository;
pub use app_state::AppState; 