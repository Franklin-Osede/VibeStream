use sea_orm::DatabaseConnection;

pub mod api;
pub mod config;
pub mod core;
pub mod db;
pub mod error;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

// Nuevos módulos para ZK y blockchain
pub mod zk {
    pub mod circuits;
    pub mod proof;
    pub mod verifier;
}

pub mod blockchain {
    pub mod solana;
    pub mod ethereum;
    pub mod layerzero;
    pub mod common;
}

pub use api::create_router;
pub use config::AppConfig;
pub use error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// Re-exports
pub use error::Error;
pub use error::Result;

// Constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify that main modules are exported correctly
        let _: Result<AppConfig, _> = AppConfig::new();
    }
} 