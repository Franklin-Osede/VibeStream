//! JWT Configuration Helper
//! 
//! Provides safe access to JWT configuration from environment variables

use crate::shared::domain::errors::AppError;

/// Get JWT secret from environment variables
/// 
/// # Returns
/// - `Ok(String)` if JWT_SECRET is set
/// - `Err(AppError)` if JWT_SECRET is not set (with clear error message)
/// 
/// # Security
/// This function does NOT provide a fallback value. JWT_SECRET must be explicitly
/// set in environment variables for security reasons.
pub fn get_jwt_secret() -> Result<String, AppError> {
    std::env::var("JWT_SECRET")
        .map_err(|_| AppError::ConfigurationError(
            "JWT_SECRET environment variable is required but not set. \
             Please set JWT_SECRET in your environment variables or .env file. \
             This is a security requirement and cannot use a default value.".to_string()
        ))
}

/// Get JWT access token expiry from environment (optional, defaults to 3600 seconds)
pub fn get_jwt_access_token_expiry() -> u64 {
    std::env::var("JWT_ACCESS_TOKEN_EXPIRY")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3600) // Default: 1 hour
}

/// Get JWT refresh token expiry from environment (optional, defaults to 2592000 seconds = 30 days)
pub fn get_jwt_refresh_token_expiry() -> u64 {
    std::env::var("JWT_REFRESH_TOKEN_EXPIRY")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2592000) // Default: 30 days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_jwt_secret_missing() {
        // Clear JWT_SECRET if it exists
        std::env::remove_var("JWT_SECRET");
        
        let result = get_jwt_secret();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JWT_SECRET"));
    }

    #[test]
    fn test_get_jwt_secret_set() {
        std::env::set_var("JWT_SECRET", "test_secret_123");
        
        let result = get_jwt_secret();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_secret_123");
        
        // Cleanup
        std::env::remove_var("JWT_SECRET");
    }

    #[test]
    fn test_get_jwt_access_token_expiry_default() {
        std::env::remove_var("JWT_ACCESS_TOKEN_EXPIRY");
        assert_eq!(get_jwt_access_token_expiry(), 3600);
    }

    #[test]
    fn test_get_jwt_access_token_expiry_custom() {
        std::env::set_var("JWT_ACCESS_TOKEN_EXPIRY", "7200");
        assert_eq!(get_jwt_access_token_expiry(), 7200);
        std::env::remove_var("JWT_ACCESS_TOKEN_EXPIRY");
    }
}


