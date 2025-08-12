// =============================================================================
// FAN VENTURES - INFRASTRUCTURE LAYER (Reemplazando Fractional Ownership)
// =============================================================================

pub mod postgres_repository;
pub mod mock_repository;

// Re-export the fan ventures repository
pub use postgres_repository::PostgresFanVenturesRepository; 
pub use mock_repository::MockArtistVentureRepository;