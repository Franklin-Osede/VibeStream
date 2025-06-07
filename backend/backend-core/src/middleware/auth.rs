use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult};
use chrono::{DateTime, FixedOffset, Utc};

use crate::{
    config::AppConfig,
    error::AppError,
    models::user::{self, Model as User},
    repositories::UserRepository,
    AppState,
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
    pub jwt_config: std::sync::Arc<JwtConfig>,
    pub user_repository: std::sync::Arc<dyn UserRepository>,
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
        // Extract the token Bearer from header
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

        // 5. Obtener usuario de la base de datos usando el repositorio
        let user = state
            .user_repository
            .find_by_id(&DatabaseConnection::default(), claims.sub)
            .await
            .map_err(|_| AppError::Unauthorized)?
            .ok_or(AppError::Unauthorized)?;

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
    State(_state): State<AuthState>,
    auth_user: Result<AuthUser, AppError>,
    request: Request<B>,
) -> Result<Request<B>, AppError> {
    auth_user.map(|_| request)
}

async fn extract_token<B>(request: Request<B>) -> Result<String, StatusCode> {
    let (parts, _) = request.into_parts();
    let auth_header = parts.headers.get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(auth_header[7..].to_string())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{request::Parts, HeaderMap, HeaderValue};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        pub UserRepository {}

        #[async_trait]
        impl UserRepository for UserRepository {
            async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<User>, DbErr>;
            async fn find_by_email(&self, db: &DatabaseConnection, email: &str) -> Result<Option<User>, DbErr>;
            async fn create(&self, db: &DatabaseConnection, user: user::ActiveModel) -> Result<User, DbErr>;
            async fn update(&self, db: &DatabaseConnection, user: user::ActiveModel) -> Result<User, DbErr>;
            async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr>;
        }
    }

    fn create_test_user(id: Uuid) -> User {
        User {
            id,
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            password_hash: "hash".to_string(),
            wallet_address: None,
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        }
    }

    fn create_test_state(secret: String, mock_repo: MockUserRepository) -> AuthState {
        AuthState {
            jwt_config: Arc::new(JwtConfig {
                secret,
                expiration_hours: 1,
            }),
            user_repository: Arc::new(mock_repo),
        }
    }

    fn create_auth_header(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers
    }

    #[tokio::test]
    async fn test_valid_token() {
        let user_id = Uuid::new_v4();
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .with(predicate::always(), predicate::eq(user_id))
            .returning(move |_, _| Ok(Some(create_test_user(user_id))));

        let secret = "test_secret".to_string();
        let claims = AuthClaims {
            sub: user_id,
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
            iat: chrono::Utc::now().timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let state = create_test_state(secret, mock);
        let mut parts = Parts::default();
        parts.headers = create_auth_header(&token);

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_expired_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret".to_string();
        let claims = AuthClaims {
            sub: user_id,
            exp: (chrono::Utc::now() - chrono::Duration::hours(1)).timestamp(),
            iat: chrono::Utc::now().timestamp(),
        };

        let mock_repo = MockUserRepository::new();
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let state = create_test_state(secret, mock_repo);
        let mut parts = Parts::default();
        parts.headers = create_auth_header(&token);

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_invalid_token_format() {
        let mock_repo = MockUserRepository::new();
        let state = create_test_state("secret".to_string(), mock_repo);
        
        let mut parts = Parts::default();
        parts.headers = create_auth_header("invalid_token");

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_missing_auth_header() {
        let mock_repo = MockUserRepository::new();
        let state = create_test_state("secret".to_string(), mock_repo);
        
        let mut parts = Parts::default();
        // No headers added

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret".to_string();
        let claims = AuthClaims {
            sub: user_id,
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
            iat: chrono::Utc::now().timestamp(),
        };

        let mut mock_repo = MockUserRepository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .returning(|_| Ok(None));

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let state = create_test_state(secret, mock_repo);
        let mut parts = Parts::default();
        parts.headers = create_auth_header(&token);

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_require_auth_middleware() {
        let user_id = Uuid::new_v4();
        let auth_user = AuthUser {
            user: create_test_user(user_id),
            claims: AuthClaims {
                sub: user_id,
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
                iat: chrono::Utc::now().timestamp(),
            },
        };

        let state = create_test_state("secret".to_string(), MockUserRepository::new());
        let request = axum::http::Request::new(());

        let result = require_auth(
            State(state),
            Ok(auth_user),
            request,
        ).await;
        
        assert!(result.is_ok());
    }
} 