use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc, DateTime};
use uuid::Uuid;

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";
const TOKEN_EXPIRY_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub username: String,
    pub email: String,
    pub role: String,
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, email: String, role: String) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::hours(TOKEN_EXPIRY_HOURS)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        Self {
            sub: user_id.to_string(),
            username,
            email,
            role,
            exp,
            iat,
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
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
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

// Modelo de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Fan,
    Artist,
    Admin,
}

// Servicio de autenticación
pub struct AuthService {
    // En una implementación real, esto sería una conexión a DB
    users: Vec<User>,
}

impl AuthService {
    pub fn new() -> Self {
        // Usuarios demo
        let users = vec![
            User {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
                username: "demo".to_string(),
                email: "demo@example.com".to_string(),
                role: UserRole::Fan,
                created_at: Utc::now(),
            },
            User {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
                username: "artist".to_string(),
                email: "artist@example.com".to_string(),
                role: UserRole::Artist,
                created_at: Utc::now(),
            },
        ];
        
        Self { users }
    }
    
    pub fn authenticate(&self, username: &str, password: &str) -> Option<&User> {
        // En una implementación real, verificaríamos el hash de la contraseña
        if password != "password" {
            return None;
        }
        
        self.users.iter().find(|u| u.username == username)
    }
    
    pub fn get_user_by_id(&self, id: &Uuid) -> Option<&User> {
        self.users.iter().find(|u| u.id == *id)
    }
} 