use crate::zkp::{ZkProofGenerator, ZkProofVerifier, ZkProof};
use vibestream_types::*;

pub struct ZkService {
    generator: ZkProofGenerator,
    verifier: ZkProofVerifier,
}

impl ZkService {
    pub fn new() -> Self {
        Self {
            generator: ZkProofGenerator::new(),
            verifier: ZkProofVerifier::new(),
        }
    }
    
    /// Procesa solicitudes de generación de pruebas ZK
    pub async fn generate_proof(&self, proof_type: ZkProofType) -> Result<ZkProof> {
        match proof_type {
            ZkProofType::Solvency { balance, threshold } => {
                self.generator.generate_solvency_proof(balance, threshold).await
            }
            ZkProofType::Transaction { amount, sender_balance } => {
                self.generator.generate_transaction_proof(amount, sender_balance).await
            }
        }
    }
    
    /// Verifica una prueba ZK
    pub async fn verify_proof(&self, proof: &ZkProof) -> Result<bool> {
        self.verifier.verify_proof(proof).await
    }
    
    /// Función principal del worker ZK
    pub async fn run_worker(&self) -> Result<()> {
        tracing::info!("Starting ZK service worker...");
        
        // TODO: Conectar a Redis y procesar mensajes ZK
        // Por ahora solo mantenemos el servicio corriendo
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
            tracing::debug!("ZK worker is running...");
        }
    }
}

/// Tipos de pruebas ZK que el servicio puede generar
#[derive(Debug, Clone)]
pub enum ZkProofType {
    /// Prueba de solvencia: demuestra que el balance >= threshold sin revelar el balance exacto
    Solvency { balance: u64, threshold: u64 },
    /// Prueba de transacción: demuestra que se puede realizar una transacción sin revelar el balance
    Transaction { amount: u64, sender_balance: u64 },
} 