// =============================================================================
// FAN VENTURES - DOMAIN LAYER (Reemplazando Fractional Ownership)
// =============================================================================

pub mod entities;
pub mod repositories;

// Re-export the fan ventures entities
pub use entities::{
    ArtistVenture, FanInvestment, RevenueDistribution, VentureBenefit,
    VentureStatus, InvestmentStatus, InvestmentType, BenefitType
}; 