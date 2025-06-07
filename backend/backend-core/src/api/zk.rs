use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde::Serialize;
use base64::{engine::general_purpose, Engine as _};
use tokio::task;

use crate::{
    error::AppError,
    zk::proof_of_listen::ProofOfListenInputs,
    AppState,
};

pub fn create_zk_router(state: AppState) -> Router {
    Router::new()
        .route("/generate-proof", post(generate_proof_handler))
        .with_state(state)
}

#[derive(Serialize)]
struct GenerateProofResponse {
    proof: String,
    public_inputs: String,
}

async fn generate_proof_handler(
    State(state): State<AppState>,
    Json(inputs): Json<ProofOfListenInputs>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Received request to generate proof with inputs: {:?}", inputs);

    let service = state.proof_of_listen_service.clone();

    // spawn_blocking se usa para ejecutar código bloqueante (como llamar a un proceso externo)
    // sin bloquear el event loop de tokio.
    let result = task::spawn_blocking(move || {
        service.generate(inputs)
    }).await.map_err(|e| {
        tracing::error!("Task join error: {}", e);
        AppError::Internal("Failed to execute proof generation task".to_string())
    })??; // El primer '?' es por el Result de spawn_blocking, el segundo por el Result de generate()

    let (proof_bytes, public_inputs_bytes) = result;

    // Codificar los resultados binarios a Base64 para una transmisión segura en JSON
    let proof_base64 = general_purpose::STANDARD.encode(&proof_bytes);
    let public_inputs_base64 = general_purpose::STANDARD.encode(&public_inputs_bytes);

    let response = GenerateProofResponse {
        proof: proof_base64,
        public_inputs: public_inputs_base64,
    };

    Ok((StatusCode::OK, Json(response)))
} 