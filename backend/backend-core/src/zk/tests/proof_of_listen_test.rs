use super::*;
use crate::zk::tests::mocks::{MockProver, MockSolanaClient};
use tokio::test;

#[cfg(test)]
mod proof_of_listen_tests {
    use super::*;

    // Feature: Proof of Listen
    // Como un usuario del sistema de streaming
    // Quiero generar pruebas de que escuché una canción
    // Para recibir recompensas de manera justa

    #[tokio::test]
    async fn given_valid_listen_when_generating_proof_then_succeeds() {
        // Arrange
        let prover = MockProver::new();
        let user_id = "user123".to_string();
        let song_id = "song456".to_string();
        let timestamp = 1234567890;
        let duration = 180; // 3 minutos
        
        // Act
        let proof = ProofOfListen::generate(
            user_id.clone(),
            song_id.clone(),
            timestamp,
            duration,
            &MockProver::generate_mock_proving_key(),
        ).await;

        // Assert
        assert!(proof.is_ok());
        let proof = proof.unwrap();
        assert!(proof.verify(&MockProver::generate_mock_verification_key()).await.unwrap());
    }

    #[tokio::test]
    async fn given_short_duration_when_generating_proof_then_fails() {
        // Arrange
        let prover = MockProver::new();
        let user_id = "user123".to_string();
        let song_id = "song456".to_string();
        let timestamp = 1234567890;
        let duration = 30; // Solo 30 segundos
        
        // Act & Assert
        assert!(!prover.verify_duration(duration));
    }

    #[tokio::test]
    async fn given_valid_proof_when_verifying_on_chain_then_succeeds() {
        // Arrange
        let mut solana_client = MockSolanaClient::new();
        let proof_data = vec![1, 2, 3, 4]; // Mock proof data

        // Act
        let result = solana_client.submit_proof(proof_data.clone()).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(solana_client.last_proof, Some(proof_data));
    }
} 