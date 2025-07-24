// =============================================================================
// FAN VENTURES - DOMAIN LAYER (Reemplazando Fractional Ownership)
// =============================================================================

pub mod entities;

// Re-export the fan ventures entities
pub use entities::{
    ArtistVenture, FanInvestment, RevenueDistribution, VentureBenefit,
    VentureStatus, InvestmentStatus, InvestmentType, BenefitType
}; 