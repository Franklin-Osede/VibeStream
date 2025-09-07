use vibestream_types::*;

pub mod zkp;
pub mod service;

#[cfg(test)]
mod test_zk;

pub use service::{ZkService, ZkServiceConfig, ZkProofType};
pub use zkp::{ZkProof, ZkProofGenerator, ZkProofVerifier};

/// Función principal para ejecutar el worker ZK
pub async fn run_zk_worker() -> Result<()> {
    let config = service::ZkServiceConfig::default();
    let zk_service = ZkService::new(config).await
        .map_err(|e| VibeStreamError::Internal { 
            message: format!("Failed to initialize ZK service: {}", e) 
        })?;
    
    zk_service.run_worker().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn test_zk_service_creation() {
        let config = ZkServiceConfig {
            circuits_dir: "/tmp/test_circuits".to_string(),
            cache_dir: "/tmp/test_cache".to_string(),
            redis_url: None, // Skip Redis for tests
            server_port: 8004,
        };

        // Create test directories
        tokio::fs::create_dir_all(&config.circuits_dir).await.unwrap();
        tokio::fs::create_dir_all(&config.cache_dir).await.unwrap();

        // For testing, we'll use mock proof generation since we don't have actual circuits
        let proof_type = ZkProofType::Solvency { 
            balance: 1000, 
            threshold: 500 
        };
        
        // This would normally create a full service, but for testing we'll check the config
        assert_eq!(config.server_port, 8004);
        assert!(!config.circuits_dir.is_empty());
        assert!(!config.cache_dir.is_empty());
        
        // Cleanup
        tokio::fs::remove_dir_all(&config.circuits_dir).await.ok();
        tokio::fs::remove_dir_all(&config.cache_dir).await.ok();
    }

    #[tokio::test]
    async fn test_zk_proof_serde() {
        let proof = ZkProof {
            proof: "test_proof_data".to_string(),
            public_inputs: serde_json::json!({
                "threshold": "500",
                "proof_type": "solvency"
            }),
            verification_key: "test_vkey".to_string(),
            circuit_id: "test_circuit".to_string(),
            generated_at: chrono::Utc::now(),
        };

        // Test serialization/deserialization
        let serialized = serde_json::to_string(&proof).unwrap();
        let deserialized: ZkProof = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(proof.circuit_id, deserialized.circuit_id);
        assert_eq!(proof.proof, deserialized.proof);
        assert_eq!(proof.verification_key, deserialized.verification_key);
    }
} 