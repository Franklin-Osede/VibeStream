// =============================================================================
// FAN VENTURES - INFRASTRUCTURE LAYER (Reemplazando Fractional Ownership)
// =============================================================================

pub mod postgres_repository;

// Re-export the fan ventures repository
pub use postgres_repository::PostgresFanVenturesRepository; 