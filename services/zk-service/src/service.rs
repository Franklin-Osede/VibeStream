use crate::zkp::{ZkProofGenerator, ZkProofVerifier, ZkProof};
use vibestream_types::*;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use anyhow::Result as AnyResult;
use serde::{Deserialize, Serialize};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkServiceConfig {
    pub circuits_dir: String,
    pub cache_dir: String,
    pub redis_url: Option<String>,
    pub server_port: u16,
}

impl Default for ZkServiceConfig {
    fn default() -> Self {
        Self {
            circuits_dir: "../../backend/circuits".to_string(),
            cache_dir: "/tmp/zk_cache".to_string(),
            redis_url: Some("redis://localhost:6379".to_string()),
            server_port: 8003,
        }
    }
}

pub struct ZkService {
    generator: Arc<ZkProofGenerator>,
    verifier: Arc<ZkProofVerifier>,
    config: ZkServiceConfig,
    stats: Arc<RwLock<ZkServiceStats>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ZkServiceStats {
    proofs_generated: u64,
    proofs_verified: u64,
    proofs_failed: u64,
    average_generation_time_ms: f64,
    average_verification_time_ms: f64,
}

impl ZkService {
    pub async fn new(config: ZkServiceConfig) -> AnyResult<Self> {
        info!("🔧 Initializing ZK Service...");
        
        let circuits_dir = Path::new(&config.circuits_dir);
        let cache_dir = Path::new(&config.cache_dir);
        let redis_url = config.redis_url.as_deref();

        // Create cache directory if it doesn't exist
        tokio::fs::create_dir_all(cache_dir).await?;

        let generator = Arc::new(
            ZkProofGenerator::new(circuits_dir, cache_dir, redis_url).await
                .map_err(|e| anyhow::anyhow!("Failed to initialize ZK generator: {}", e))?
        );

        let verifier = Arc::new(
            ZkProofVerifier::new(circuits_dir, cache_dir, redis_url).await
                .map_err(|e| anyhow::anyhow!("Failed to initialize ZK verifier: {}", e))?
        );

        info!("✅ ZK Service initialized successfully");

        Ok(Self {
            generator,
            verifier,
            config,
            stats: Arc::new(RwLock::new(ZkServiceStats::default())),
        })
    }
    
    /// Procesa solicitudes de generación de pruebas ZK
    pub async fn generate_proof(&self, proof_type: ZkProofType) -> Result<ZkProof> {
        let start_time = std::time::Instant::now();
        
        let result = match proof_type {
            ZkProofType::Solvency { balance, threshold } => {
                self.generator.generate_solvency_proof(balance, threshold).await
            }
            ZkProofType::Transaction { amount, sender_balance } => {
                self.generator.generate_transaction_proof(amount, sender_balance).await
            }
            ZkProofType::Listen { 
                start_time, 
                current_time, 
                end_time, 
                song_hash, 
                user_signature, 
                user_public_key, 
                nonce 
            } => {
                self.generator.generate_listen_proof(
                    start_time, 
                    current_time, 
                    end_time, 
                    &song_hash, 
                    &user_signature, 
                    &user_public_key, 
                    &nonce
                ).await
            }
        };

        let duration = start_time.elapsed();
        
        // Update stats
        let mut stats = self.stats.write().await;
        if result.is_ok() {
            stats.proofs_generated += 1;
            stats.average_generation_time_ms = 
                (stats.average_generation_time_ms * (stats.proofs_generated - 1) as f64 + duration.as_millis() as f64) 
                / stats.proofs_generated as f64;
        } else {
            stats.proofs_failed += 1;
        }

        result
    }
    
    /// Verifica una prueba ZK
    pub async fn verify_proof(&self, proof: &ZkProof) -> Result<bool> {
        let start_time = std::time::Instant::now();
        
        let result = self.verifier.verify_proof(proof).await;
        
        let duration = start_time.elapsed();
        
        // Update stats
        let mut stats = self.stats.write().await;
        if result.is_ok() {
            stats.proofs_verified += 1;
            stats.average_verification_time_ms = 
                (stats.average_verification_time_ms * (stats.proofs_verified - 1) as f64 + duration.as_millis() as f64) 
                / stats.proofs_verified as f64;
        } else {
            stats.proofs_failed += 1;
        }

        result
    }

    /// Obtiene estadísticas del servicio
    pub async fn get_stats(&self) -> ZkServiceStats {
        self.stats.read().await.clone()
    }
    
    /// Función principal del worker ZK
    pub async fn run_worker(&self) -> Result<()> {
        info!("🚀 Starting ZK service worker...");
        
        // Start HTTP server for ZK service endpoints
        let app = self.create_router().await;
        
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.config.server_port)).await
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Failed to bind to port: {}", e) 
            })?;
        
        info!("🌐 ZK Service HTTP server listening on port {}", self.config.server_port);
        
        axum::serve(listener, app.into_make_service()).await
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Server error: {}", e) 
            })?;
        
        Ok(())
    }

    async fn create_router(&self) -> Router {
        Router::new()
            .route("/health", get(health_check))
            .route("/stats", get(get_stats_handler))
            .route("/generate", post(generate_proof_handler))
            .route("/verify", post(verify_proof_handler))
            .layer(CorsLayer::permissive())
            .with_state(Arc::new(self.clone()))
    }
}

impl Clone for ZkService {
    fn clone(&self) -> Self {
        Self {
            generator: self.generator.clone(),
            verifier: self.verifier.clone(),
            config: self.config.clone(),
            stats: self.stats.clone(),
        }
    }
}

/// Tipos de pruebas ZK que el servicio puede generar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkProofType {
    /// Prueba de solvencia: demuestra que el balance >= threshold sin revelar el balance exacto
    Solvency { balance: u64, threshold: u64 },
    /// Prueba de transacción: demuestra que se puede realizar una transacción sin revelar el balance
    Transaction { amount: u64, sender_balance: u64 },
    /// Prueba de escucha: demuestra que el usuario escuchó una canción por el tiempo mínimo requerido
    Listen {
        start_time: u64,
        current_time: u64,
        end_time: u64,
        song_hash: String,
        user_signature: [String; 3],
        user_public_key: [String; 2],
        nonce: String,
    },
}

// HTTP handlers - Todos usan State para consistencia
async fn health_check(
    State(_service): State<Arc<ZkService>>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "zk-service",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}

async fn get_stats_handler(
    State(service): State<Arc<ZkService>>,
) -> std::result::Result<Json<ZkServiceStats>, StatusCode> {
    let stats = service.get_stats().await;
    Ok(Json(stats))
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateProofRequest {
    proof_type: ZkProofType,
}

async fn generate_proof_handler(
    State(service): State<Arc<ZkService>>,
    Json(request): Json<GenerateProofRequest>,
) -> std::result::Result<Json<ZkProof>, StatusCode> {
    match service.generate_proof(request.proof_type).await {
        Ok(proof) => Ok(Json(proof)),
        Err(e) => {
            error!("Failed to generate proof: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VerifyProofRequest {
    proof: ZkProof,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerifyProofResponse {
    valid: bool,
    circuit_id: String,
    verified_at: chrono::DateTime<chrono::Utc>,
}

async fn verify_proof_handler(
    State(service): State<Arc<ZkService>>,
    Json(request): Json<VerifyProofRequest>,
) -> std::result::Result<Json<VerifyProofResponse>, StatusCode> {
    match service.verify_proof(&request.proof).await {
        Ok(valid) => Ok(Json(VerifyProofResponse {
            valid,
            circuit_id: request.proof.circuit_id.clone(),
            verified_at: chrono::Utc::now(),
        })),
        Err(e) => {
            error!("Failed to verify proof: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
} 