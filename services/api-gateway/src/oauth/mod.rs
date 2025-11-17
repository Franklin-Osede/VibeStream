// =============================================================================
// OAUTH MODULE
// =============================================================================
// 
// This module provides OAuth integration for Google and Apple
// Replaces mock implementations with real API calls

pub mod real_providers;
pub mod handlers;

pub use real_providers::{
    GoogleOAuthProvider, AppleOAuthProvider, OAuthConfig, 
    GoogleConfig, AppleConfig, RealOAuthService
};
pub use handlers::create_oauth_routes;
