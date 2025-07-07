pub mod value_objects;
pub mod entities;
pub mod aggregates;
pub mod events;
pub mod services;
pub mod repository;
pub mod specifications;

// Re-export key types
pub use value_objects::{
    UserId, Email, Username, PasswordHash, WalletAddress, 
    UserTier, UserRole, ProfileUrl
};
pub use entities::{
    User, UserProfile, UserPreferences, UserStats
};
pub use aggregates::UserAggregate;
pub use events::{
    UserRegistered, UserAuthenticated, UserProfileUpdated,
    UserTierUpgraded, UserDeactivated, UserReactivated
};
pub use services::{
    AuthenticationDomainService, UserDomainService, 
    PasswordDomainService, UserValidationService
};
pub use repository::UserRepository;
pub use specifications::{
    EmailSpecification, UsernameSpecification, 
    PasswordSpecification, UserActiveSpecification
}; 