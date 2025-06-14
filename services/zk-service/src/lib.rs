use vibestream_types::*;

pub mod zkp;
pub mod service;

pub use service::{ZkService, ZkProofType};
pub use zkp::{ZkProof, ZkProofGenerator, ZkProofVerifier};

/// Función principal para ejecutar el worker ZK
pub async fn run_zk_worker() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let zk_service = ZkService::new();
    zk_service.run_worker().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zk_service_creation() {
        let service = ZkService::new();
        
        // Test de generación de prueba mock
        let proof_type = ZkProofType::Solvency { 
            balance: 1000, 
            threshold: 500 
        };
        
        let result = service.generate_proof(proof_type).await;
        assert!(result.is_ok());
        
        // Test de verificación
        if let Ok(proof) = result {
            let verification = service.verify_proof(&proof).await;
            assert!(verification.is_ok());
            assert!(verification.unwrap());
        }
    }
} 