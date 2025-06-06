use ark_bn254::{Bn254, Fr};
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use ark_ff::Field;
use std::sync::Arc;

pub struct MockProver {
    min_duration: u32,
}

impl MockProver {
    pub fn new() -> Self {
        Self {
            min_duration: 60 // 1 minuto mínimo
        }
    }

    pub fn generate_mock_proving_key() -> ProvingKey<Bn254> {
        // En un entorno real, esto vendría de un archivo generado por el trusted setup
        // Para testing, creamos una clave mock
        unimplemented!("Implementar en el siguiente ciclo de TDD")
    }

    pub fn generate_mock_verification_key() -> VerifyingKey<Bn254> {
        // En un entorno real, esto vendría de un archivo generado por el trusted setup
        unimplemented!("Implementar en el siguiente ciclo de TDD")
    }

    pub fn verify_duration(&self, duration: u32) -> bool {
        duration >= self.min_duration
    }
}

pub struct MockSolanaClient {
    pub is_connected: bool,
    pub last_proof: Option<Vec<u8>>,
}

impl MockSolanaClient {
    pub fn new() -> Self {
        Self {
            is_connected: true,
            last_proof: None,
        }
    }

    pub async fn submit_proof(&mut self, proof_data: Vec<u8>) -> Result<String, String> {
        if !self.is_connected {
            return Err("Cliente no conectado".to_string());
        }
        
        self.last_proof = Some(proof_data);
        Ok("tx_hash_mock_123".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_prover_duration_validation() {
        let prover = MockProver::new();
        
        // Given: Una duración válida
        assert!(prover.verify_duration(120)); // 2 minutos
        
        // Given: Una duración inválida
        assert!(!prover.verify_duration(30)); // 30 segundos
    }

    #[tokio::test]
    async fn test_mock_solana_client() {
        let mut client = MockSolanaClient::new();
        
        // Given: Un cliente conectado
        let proof_data = vec![1, 2, 3, 4];
        let result = client.submit_proof(proof_data.clone()).await;
        assert!(result.is_ok());
        assert_eq!(client.last_proof, Some(proof_data));
        
        // Given: Un cliente desconectado
        client.is_connected = false;
        let result = client.submit_proof(vec![5, 6, 7, 8]).await;
        assert!(result.is_err());
    }
} 