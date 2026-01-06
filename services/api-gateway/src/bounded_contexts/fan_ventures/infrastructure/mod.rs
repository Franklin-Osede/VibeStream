// =============================================================================
// FAN VENTURES - INFRASTRUCTURE LAYER (Reemplazando Fractional Ownership)
// =============================================================================

pub mod postgres_repository;
pub mod mock_repository;
pub mod payment_integration;
pub mod payment_helper;
pub mod payment_event_listener;

// Re-export the fan ventures repository
pub use postgres_repository::PostgresFanVenturesRepository; 
pub use mock_repository::MockArtistVentureRepository;
pub use payment_integration::FanVenturesPaymentIntegration;
pub use payment_helper::create_payment_command_handler;
pub use payment_event_listener::FanVenturesPaymentEventListener;