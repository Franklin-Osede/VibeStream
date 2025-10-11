pub mod domain;
pub mod application;
pub mod infrastructure;
// pub mod integration; // TODO: Implement integration module

#[cfg(test)]
pub mod tests;

// Re-export commonly used types
pub use domain::*;
pub use application::commands::*;
pub use application::queries::*;
// pub use integration::*;