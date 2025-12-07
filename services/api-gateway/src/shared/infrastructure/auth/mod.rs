pub mod jwt_service;
pub mod middleware;
pub mod config;

pub use jwt_service::{JwtService, PasswordService, Claims, TokenPair};
pub use middleware::{
    jwt_auth_middleware, 
    optional_jwt_auth_middleware, 
    extract_claims,
    AuthenticatedUser,
};
pub use config::{get_jwt_secret, get_jwt_access_token_expiry, get_jwt_refresh_token_expiry};
