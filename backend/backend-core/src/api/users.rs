use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{
    AppState,
    error::AppError,
    models::user::Model as User,
    services::user::UserService,
};

pub fn create_user_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

async fn create_user(
    state: axum::extract::State<AppState>,
    Json(_request): Json<CreateUserRequest>,
) -> impl axum::response::IntoResponse {
    let user_service = UserService::new(state.db.clone());
    // TODO: Implement user creation
    todo!()
}

async fn get_user(
    state: axum::extract::State<AppState>,
    Path(_id): Path<Uuid>,
) -> impl axum::response::IntoResponse {
    let user_service = UserService::new(state.db.clone());
    // TODO: Implement get user
    todo!()
} 