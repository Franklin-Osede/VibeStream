pub mod payment_repository;
pub mod royalty_repository;
pub mod revenue_sharing_repository;
// Módulos comentados porque los archivos no existen aún
// pub mod fraud_repository;
// pub mod payment_analytics_repository;

pub use payment_repository::*;
pub use royalty_repository::*;
pub use revenue_sharing_repository::*;
// pub use fraud_repository::*;
// pub use payment_analytics_repository::*;

// Re-export type aliases for controller compatibility
pub use royalty_repository::PostgresRoyaltyRepository;
pub use revenue_sharing_repository::PostgresWalletRepository;

pub mod mock_repositories;
pub use mock_repositories::*; 