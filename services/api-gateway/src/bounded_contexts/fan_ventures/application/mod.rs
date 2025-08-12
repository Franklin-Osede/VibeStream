// =============================================================================
// FAN VENTURES - APPLICATION LAYER (Reemplazando Fractional Ownership)
// =============================================================================

pub mod simple_service;
pub mod services;

// Re-export the fan ventures service
pub use simple_service::{
    FanVenturesService,
    FanPortfolio,
    VentureAnalytics,
}; 