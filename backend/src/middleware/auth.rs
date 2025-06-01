use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::User,
    repositories::UserRepository,
};

// Claims JWT tipados
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: Uuid,        // ID del usuario
    pub exp: i64,         // Timestamp de expiración
    pub iat: i64,         // Timestamp de emisión
}

// Configuración JWT compartida
#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

// Estado de autenticación compartido
#[derive(Clone)]
pub struct AuthState {
    pub jwt_config: Arc<JwtConfig>,
    pub user_repository: Arc<dyn UserRepository>,
}

// Usuario autenticado - será extraído por los handlers
#[derive(Debug)]
pub struct AuthUser {
    pub user: User,
    pub claims: AuthClaims,
}

// Implementación del extractor personalizado
#[async_trait]
impl FromRequestParts<AuthState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AuthState) -> Result<Self, Self::Rejection> {
        // 1. Extraer el token Bearer del header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;

        // 2. Validar y decodificar el JWT
        let token_data = decode::<AuthClaims>(
            bearer.token(),
            &DecodingKey::from_secret(state.jwt_config.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        // 3. Extraer y validar los claims
        let claims = token_data.claims;

        // 4. Verificar expiración
        let now = chrono::Utc::now().timestamp();
        if claims.exp < now {
            return Err(AppError::Unauthorized);
        }

        // 5. Obtener usuario de la base de datos
        let user = state
            .user_repository
            .find_by_id(claims.sub)
            .await?;

        Ok(AuthUser { user, claims })
    }
}

// Constructor del middleware
pub fn auth_middleware(
    jwt_config: JwtConfig,
    user_repository: Arc<dyn UserRepository>,
) -> AuthState {
    AuthState {
        jwt_config: Arc::new(jwt_config),
        user_repository,
    }
}

// Helper para proteger rutas
pub async fn require_auth<B>(
    State(state): State<AuthState>,
    auth_user: Result<AuthUser, AppError>,
    request: axum::http::Request<B>,
) -> Result<axum::http::Request<B>, AppError> {
    auth_user.map(|_| request)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use mockall::predicate::*;
    use mockall::mock;

    // Mock del UserRepository
    mock! {
        UserRepository {}
        #[async_trait]
        impl UserRepository for UserRepository {
            async fn find_by_id(&self, id: Uuid) -> Result<User, AppError>;
            async fn find_by_email(&self, email: &str) -> Result<User, AppError>;
            async fn create(&self, user: User) -> Result<User, AppError>;
            async fn update(&self, user: User) -> Result<User, AppError>;
            async fn delete(&self, id: Uuid) -> Result<(), AppError>;
        }
    }

    #[tokio::test]
    async fn test_valid_token() {
        // Setup
        let user_id = Uuid::new_v4();
        let secret = "test_secret".to_string();
        let claims = AuthClaims {
            sub: user_id,
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
            iat: chrono::Utc::now().timestamp(),
        };

        // Create mock user
        let mut mock_repo = MockUserRepository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .returning(move |_| {
                Ok(User {
                    id: user_id,
                    username: "test".to_string(),
                    email: "test@test.com".to_string(),
                    password_hash: "hash".to_string(),
                    wallet_address: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });

        // Create token
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        // Create auth state
        let state = AuthState {
            jwt_config: Arc::new(JwtConfig {
                secret,
                expiration_hours: 1,
            }),
            user_repository: Arc::new(mock_repo),
        };

        // Create request parts with token
        let mut parts = Parts::default();
        parts.headers.insert(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );

        // Test
        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_expired_token() {
        // Similar to above but with expired token
        let claims = AuthClaims {
            sub: Uuid::new_v4(),
            exp: (chrono::Utc::now() - chrono::Duration::hours(1)).timestamp(),
            iat: chrono::Utc::now().timestamp(),
        };
        // ... implement test
    }
} 