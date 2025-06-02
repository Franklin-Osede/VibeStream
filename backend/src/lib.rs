use sea_orm::DatabaseConnection;

pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify that main modules are exported correctly
        let _: Result<AppConfig, _> = AppConfig::new();
    }
} 