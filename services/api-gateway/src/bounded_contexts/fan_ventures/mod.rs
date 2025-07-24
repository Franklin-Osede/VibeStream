// =============================================================================
// FAN VENTURES BOUNDED CONTEXT (Reemplazando Fractional Ownership)
// =============================================================================

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export the fan ventures service
pub use application::simple_service::FanVenturesService;
pub use domain::entities::{
    ArtistVenture, FanInvestment, RevenueDistribution, VentureBenefit,
    VentureStatus, InvestmentStatus, InvestmentType, BenefitType
};
pub use infrastructure::postgres_repository::PostgresFanVenturesRepository; 