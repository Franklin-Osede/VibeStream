pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod services;
pub mod models;
pub mod repositories;
pub mod middleware;

pub use api::create_router;
pub use config::AppConfig;
pub use error::AppError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify that main modules are exported correctly
        let _: Result<AppConfig, _> = AppConfig::new();
    }
} 