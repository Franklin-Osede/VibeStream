use axum::{Router, routing::post, extract::State, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::AppState;

use crate::bounded_contexts::user::application::commands::{RegisterUser, RegisterUserResult};

#[derive(Deserialize)]
struct RegisterUserRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterUserResponse {
    user_id: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/users", post(register_user))
}

async fn register_user(State(state): State<AppState>, Json(req): Json<RegisterUserRequest>) -> Result<(StatusCode, Json<RegisterUserResponse>), StatusCode> {
    let cmd = RegisterUser {
        email: req.email,
        username: req.username,
        password: req.password,
    };

    let boxed = state
        .command_bus
        .dispatch(cmd)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res = *boxed.downcast::<RegisterUserResult>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(RegisterUserResponse { user_id: res.user_id.to_string() })))
} 