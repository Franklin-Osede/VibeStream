use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::shared::domain::errors::AppError;

// =============================================================================
// JWT SERVICE - Implementación real de autenticación
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub username: String,   // Username
    pub email: String,      // Email
    pub role: String,       // User role
    pub tier: String,       // User tier
    pub exp: u64,           // Expiration time
    pub iat: u64,           // Issued at time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_expiry: Duration,
    refresh_token_expiry: Duration,
}

impl JwtService {
    pub fn new(secret: &str) -> Result<Self, AppError> {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        
        Ok(Self {
            encoding_key,
            decoding_key,
            access_token_expiry: Duration::from_secs(3600), // 1 hour
            refresh_token_expiry: Duration::from_secs(2592000), // 30 days
        })
    }
    
    /// Generate access token for user
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        role: &str,
        tier: &str,
    ) -> Result<String, AppError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InternalError("Failed to get current time".to_string()))?
            .as_secs();
        
        let expires_at = now + self.access_token_expiry.as_secs();
        
        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            tier: tier.to_string(),
            exp: expires_at,
            iat: now,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Failed to encode JWT: {}", e)))
    }
    
    /// Generate refresh token for user
    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        role: &str,
        tier: &str,
    ) -> Result<String, AppError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InternalError("Failed to get current time".to_string()))?
            .as_secs();
        
        let expires_at = now + self.refresh_token_expiry.as_secs();
        
        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            tier: tier.to_string(),
            exp: expires_at,
            iat: now,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Failed to encode refresh JWT: {}", e)))
    }
    
    /// Generate both access and refresh tokens
    pub fn generate_token_pair(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        role: &str,
        tier: &str,
    ) -> Result<TokenPair, AppError> {
        let access_token = self.generate_access_token(user_id, username, email, role, tier)?;
        let refresh_token = self.generate_refresh_token(user_id, username, email, role, tier)?;
        
        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: self.access_token_expiry.as_secs(),
        })
    }
    
    /// Validate and decode access token
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))?;
        
        Ok(token_data.claims)
    }
    
    /// Validate and decode refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid refresh token: {}", e)))?;
        
        Ok(token_data.claims)
    }
    
    /// Refresh access token using refresh token
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String, AppError> {
        let claims = self.validate_refresh_token(refresh_token)?;
        
        // Parse user ID from claims
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::AuthenticationError("Invalid user ID in token".to_string()))?;
        
        // Generate new access token
        self.generate_access_token(
            user_id,
            &claims.username,
            &claims.email,
            &claims.role,
            &claims.tier,
        )
    }
    
    /// Check if token is expired
    pub fn is_token_expired(&self, token: &str) -> Result<bool, AppError> {
        let claims = self.validate_access_token(token)?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InternalError("Failed to get current time".to_string()))?
            .as_secs();
        
        Ok(now > claims.exp)
    }
}

// =============================================================================
// PASSWORD HASHING SERVICE
// =============================================================================

use bcrypt::{hash, verify, DEFAULT_COST};

pub struct PasswordService;

impl PasswordService {
    /// Hash a password using bcrypt
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))
    }
    
    /// Verify a password against its hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        verify(password, hash)
            .map_err(|e| AppError::InternalError(format!("Failed to verify password: {}", e)))
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jwt_token_generation_and_validation() {
        let jwt_service = JwtService::new("test_secret").unwrap();
        let user_id = Uuid::new_v4();
        
        // Generate token
        let token = jwt_service
            .generate_access_token(&user_id, "testuser", "test@example.com", "user", "bronze")
            .unwrap();
        
        // Validate token
        let claims = jwt_service.validate_access_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.email, "test@example.com");
    }
    
    #[test]
    fn test_password_hashing_and_verification() {
        let password = "testpassword123";
        
        // Hash password
        let hash = PasswordService::hash_password(password).unwrap();
        
        // Verify password
        let is_valid = PasswordService::verify_password(password, &hash).unwrap();
        assert!(is_valid);
        
        // Verify wrong password
        let is_valid = PasswordService::verify_password("wrongpassword", &hash).unwrap();
        assert!(!is_valid);
    }
}
