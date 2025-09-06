pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export main components for easy access
pub use domain::*;
pub use application::*;
pub use infrastructure::*;
// pub use presentation::*; // TODO: Fix naming conflicts 