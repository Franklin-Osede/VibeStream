use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use std::collections::HashMap;

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";
const TOKEN_EXPIRY_HOURS: i64 = 24;
const REFRESH_TOKEN_EXPIRY_DAYS: i64 = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub username: String,
    pub email: String,
    pub role: String,
    pub exp: i64,   // Expiration time
    pub iat: i64,   // Issued at
    pub token_type: String, // "access" or "refresh"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserInfo,
    pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthRequest {
    pub provider: String, // "google", "apple", "github"
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, email: String, role: String, token_type: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            sub: user_id.to_string(),
            username,
            email,
            role,
            exp: (now + chrono::Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
            token_type,
        }
    }

    pub fn to_jwt(&self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(JWT_SECRET),
        )
    }

    pub fn from_jwt(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

// Extractor para obtener claims del JWT desde las requests
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extraer token del header Authorization
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .ok_or(AuthError::MissingToken)?;

        // Decodificar y validar JWT
        Claims::from_jwt(auth_header).map_err(|_| AuthError::InvalidToken)
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    TokenExpired,
    InsufficientPermissions,
    UserNotFound,
    InvalidCredentials,
    OAuthError(String),
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token".to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
            AuthError::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions".to_string()),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AuthError::OAuthError(msg) => (StatusCode::BAD_REQUEST, msg),
            AuthError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(serde_json::json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

// Modelo de usuario mejorado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub is_verified: bool,
    pub oauth_providers: HashMap<String, String>, // provider -> provider_user_id
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Fan,
    Artist,
    Admin,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Fan => write!(f, "fan"),
            UserRole::Artist => write!(f, "artist"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

// Servicio de autenticación mejorado
pub struct AuthService {
    users: HashMap<Uuid, User>,
    oauth_providers: HashMap<String, Box<dyn OAuthProvider>>,
}

#[async_trait]
pub trait OAuthProvider: Send + Sync {
    async fn verify_token(&self, token: &str) -> Result<OAuthUserInfo, AuthError>;
    fn provider_name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct OAuthUserInfo {
    pub provider_user_id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

impl AuthService {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(Uuid::new_v4(), User {
            id: Uuid::new_v4(),
            username: "demo_user".to_string(),
            email: "demo@vibestream.com".to_string(),
            role: UserRole::Fan,
            is_verified: true,
            oauth_providers: HashMap::new(),
            created_at: chrono::Utc::now(),
            last_login_at: None,
        });

        let mut oauth_providers: HashMap<String, Box<dyn OAuthProvider>> = HashMap::new();
        oauth_providers.insert("google".to_string(), Box::new(GoogleOAuthProvider));
        oauth_providers.insert("apple".to_string(), Box::new(AppleOAuthProvider));
        
        Self { users, oauth_providers }
    }
    
    pub fn authenticate(&self, username: &str, password: &str) -> Option<&User> {
        // En una implementación real, verificaríamos el hash de la contraseña
        if password != "password" {
            return None;
        }
        
        self.users.values().find(|u| u.username == username)
    }
    
    pub fn get_user_by_id(&self, id: &Uuid) -> Option<&User> {
        self.users.get(id)
    }

    pub async fn authenticate_oauth(&self, provider: &str, token: &str) -> Result<&User, AuthError> {
        let oauth_provider = self.oauth_providers.get(provider)
            .ok_or_else(|| AuthError::OAuthError("Unsupported OAuth provider".to_string()))?;
        
        let oauth_user_info = oauth_provider.verify_token(token).await?;
        
        // Buscar usuario por email o crear uno nuevo
        self.users.values().find(|u| u.email == oauth_user_info.email)
            .ok_or(AuthError::UserNotFound)
    }

    pub fn generate_tokens(&self, user: &User) -> Result<(String, String), AuthError> {
        let access_claims = Claims::new(
            user.id,
            user.username.clone(),
            user.email.clone(),
            user.role.to_string(),
            "access".to_string(),
        );
        
        let refresh_claims = Claims::new(
            user.id,
            user.username.clone(),
            user.email.clone(),
            user.role.to_string(),
            "refresh".to_string(),
        );

        let access_token = access_claims.to_jwt()
            .map_err(|_| AuthError::InternalError)?;
        let refresh_token = refresh_claims.to_jwt()
            .map_err(|_| AuthError::InternalError)?;

        Ok((access_token, refresh_token))
    }

    pub fn verify_permission(&self, user: &User, required_role: UserRole) -> bool {
        match (&user.role, required_role) {
            (UserRole::Admin, _) => true,
            (UserRole::Artist, UserRole::Fan) => true,
            (UserRole::Fan, UserRole::Fan) => true,
            _ => false,
        }
    }
}

// Implementaciones de OAuth providers
pub struct GoogleOAuthProvider;

#[async_trait]
impl OAuthProvider for GoogleOAuthProvider {
    async fn verify_token(&self, token: &str) -> Result<OAuthUserInfo, AuthError> {
        // En una implementación real, verificaríamos con Google API
        // Por ahora, simulamos la verificación
        if token.is_empty() {
            return Err(AuthError::OAuthError("Invalid Google token".to_string()));
        }
        
        Ok(OAuthUserInfo {
            provider_user_id: "google_123".to_string(),
            email: "user@gmail.com".to_string(),
            name: "Google User".to_string(),
            picture: Some("https://example.com/avatar.jpg".to_string()),
        })
    }

    fn provider_name(&self) -> &str {
        "google"
    }
}

pub struct AppleOAuthProvider;

#[async_trait]
impl OAuthProvider for AppleOAuthProvider {
    async fn verify_token(&self, token: &str) -> Result<OAuthUserInfo, AuthError> {
        // En una implementación real, verificaríamos con Apple API
        if token.is_empty() {
            return Err(AuthError::OAuthError("Invalid Apple token".to_string()));
        }
        
        Ok(OAuthUserInfo {
            provider_user_id: "apple_123".to_string(),
            email: "user@icloud.com".to_string(),
            name: "Apple User".to_string(),
            picture: None,
        })
    }

    fn provider_name(&self) -> &str {
        "apple"
    }
} 