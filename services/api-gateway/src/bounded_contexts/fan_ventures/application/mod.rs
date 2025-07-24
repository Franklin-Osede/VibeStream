// =============================================================================
// FAN VENTURES - APPLICATION LAYER (Reemplazando Fractional Ownership)
// =============================================================================

pub mod simple_service;

// Re-export the fan ventures service
pub use simple_service::{
    FanVenturesService,
    FanPortfolio,
    VentureAnalytics,
}; 