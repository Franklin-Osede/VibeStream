// Fractional Ownership Service - Main library
pub mod domain;
pub mod application;
// pub mod infrastructure;  // Commented out to avoid PostgreSQL compilation issues during testing
// pub mod presentation;    // Commented out to focus on domain/application testing

// Re-exports principales
pub use domain::*;
pub use application::*; 
// pub use infrastructure::*; // Commented out
// pub use presentation::*;   // Commented out 