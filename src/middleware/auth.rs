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
use sea_orm::{DatabaseConnection, DbErr};
use chrono::Utc;

use crate::{
    error::AppError,
    db::models::user::{Model as User},
    repositories::UserRepository,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

pub struct AuthState {
    pub jwt_config: Arc<JwtConfig>,
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

pub struct AuthUser {
    pub user: User,
    pub claims: AuthClaims,
}

#[async_trait]
impl FromRequestParts<AuthState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AuthState) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;

        let token_data = decode::<AuthClaims>(
            bearer.token(),
            &DecodingKey::from_secret(state.jwt_config.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        let claims = token_data.claims;

        if claims.exp < Utc::now().timestamp() {
            return Err(AppError::Unauthorized);
        }

        let user = state
            .user_repository
            .find_by_id(&DatabaseConnection::default(), claims.sub)
            .await
            .map_err(|_| AppError::Unauthorized)?
            .ok_or(AppError::Unauthorized)?;

        Ok(AuthUser { user, claims })
    }
}

pub async fn require_auth<B>(
    State(state): State<AuthState>,
    auth_user: Result<AuthUser, AppError>,
    request: Request<B>,
) -> Result<Request<B>, AppError> {
    auth_user.map(|_| request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{request::Parts, HeaderMap, HeaderValue, Version, Method, Uri};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use mockall::predicate::*;
    use mockall::mock;
    use http::header;
    use chrono::Utc;

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
            created_at: Utc::now(),
            updated_at: Utc::now(),
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
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers
    }

    fn create_test_parts() -> Parts {
        Parts {
            method: Method::GET,
            uri: Uri::from_static("/"),
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            extensions: axum::http::Extensions::new(),
        }
    }

    #[tokio::test]
    async fn test_valid_token() {
        let user_id = Uuid::new_v4();
        let mut mock = MockUserRepository::new();
        
        mock.expect_find_by_id()
            .with(predicate::always(), predicate::eq(user_id))
            .times(1)
            .returning(move |_, _| Ok(Some(create_test_user(user_id))));

        let secret = "test_secret".to_string();
        let claims = AuthClaims {
            sub: user_id,
            exp: (Utc::now() + chrono::Duration::hours(1)).timestamp(),
            iat: Utc::now().timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let state = create_test_state(secret, mock);
        let mut parts = create_test_parts();
        parts.headers = create_auth_header(&token);

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_expired_token() {
        let user_id = Uuid::new_v4();
        let mut mock = MockUserRepository::new();

        let secret = "test_secret".to_string();
        let claims = AuthClaims {
            sub: user_id,
            exp: (Utc::now() - chrono::Duration::hours(1)).timestamp(),
            iat: Utc::now().timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let state = create_test_state(secret, mock);
        let mut parts = create_test_parts();
        parts.headers = create_auth_header(&token);

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_invalid_token() {
        let mock = MockUserRepository::new();
        let secret = "test_secret".to_string();
        let state = create_test_state(secret, mock);
        let mut parts = create_test_parts();
        parts.headers = create_auth_header("invalid.token.here");

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_missing_token() {
        let mock = MockUserRepository::new();
        let secret = "test_secret".to_string();
        let state = create_test_state(secret, mock);
        let mut parts = create_test_parts();

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let user_id = Uuid::new_v4();
        let mut mock = MockUserRepository::new();
        
        mock.expect_find_by_id()
            .with(predicate::always(), predicate::eq(user_id))
            .times(1)
            .returning(|_, _| Ok(None));

        let secret = "test_secret".to_string();
        let claims = AuthClaims {
            sub: user_id,
            exp: (Utc::now() + chrono::Duration::hours(1)).timestamp(),
            iat: Utc::now().timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let state = create_test_state(secret, mock);
        let mut parts = create_test_parts();
        parts.headers = create_auth_header(&token);

        let result = AuthUser::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }
} 