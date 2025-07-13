pub mod controllers;
pub mod routes;

pub use controllers::*;
pub use routes::*;

/// Configuración de presentación P2P
#[derive(Debug, Clone)]
pub struct P2PPresentationConfig {
    /// Configuración de rutas
    pub enable_dashboard: bool,
    pub enable_analytics: bool,
    pub enable_streaming: bool,
    
    /// Configuración de CORS
    pub cors_enabled: bool,
    pub allowed_origins: Vec<String>,
    
    /// Configuración de rate limiting
    pub rate_limit_enabled: bool,
    pub requests_per_minute: u32,
}

impl Default for P2PPresentationConfig {
    fn default() -> Self {
        Self {
            enable_dashboard: true,
            enable_analytics: true,
            enable_streaming: true,
            cors_enabled: true,
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:8080".to_string(),
                "https://vibestream.com".to_string(),
            ],
            rate_limit_enabled: true,
            requests_per_minute: 100,
        }
    }
} 