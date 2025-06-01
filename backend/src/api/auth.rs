use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::User;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    wallet_address: Option<String>,
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Implementar lógica de autenticación
    // 1. Verificar credenciales
    // 2. Generar JWT
    // 3. Devolver token y datos del usuario

    Err(AppError::NotImplemented)
}

pub async fn register(
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Implementar lógica de registro
    // 1. Validar datos
    // 2. Verificar que el usuario no existe
    // 3. Hashear contraseña
    // 4. Crear usuario
    // 5. Devolver datos del usuario creado

    Err(AppError::NotImplemented)
} 