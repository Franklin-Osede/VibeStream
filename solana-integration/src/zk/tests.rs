#[cfg(test)]
mod tests {
    use super::*;
    use crate::zk::{ProofRequest, VerifyRequest, ZKService};

    #[tokio::test]
    async fn test_proof_generation_and_verification() -> Result<(), anyhow::Error> {
        // Crear el servicio ZK
        let zk_service = ZKService::new()?;

        // Crear una solicitud de prueba
        let proof_request = ProofRequest {
            start_time: 1234567890,
            current_time: 1234568890,
            end_time: 1234569890,
            song_hash: "0x1234567890abcdef".to_string(),
            user_signature: [
                "sig1".to_string(),
                "sig2".to_string(),
                "sig3".to_string(),
            ],
            user_public_key: ["pk1".to_string(), "pk2".to_string()],
            nonce: "0x5678".to_string(),
        };

        // Generar la prueba
        let proof = zk_service.generate_proof(proof_request).await?;

        // Crear una solicitud de verificación
        let verify_request = VerifyRequest {
            proof,
            song_id: "song123".to_string(),
            user_id: "user456".to_string(),
        };

        // Verificar la prueba
        let is_valid = zk_service.verify_proof(verify_request).await?;

        // La prueba debería ser válida
        assert!(is_valid, "La prueba debería ser válida");

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_proof() -> Result<(), anyhow::Error> {
        let zk_service = ZKService::new()?;

        // Crear una prueba inválida (datos vacíos)
        let invalid_proof = ZKProof {
            proof: vec![0; 32],
            public_signals: vec![0; 64],
        };

        let verify_request = VerifyRequest {
            proof: invalid_proof,
            song_id: "song123".to_string(),
            user_id: "user456".to_string(),
        };

        // La verificación debería fallar o retornar false
        let is_valid = zk_service.verify_proof(verify_request).await?;
        assert!(!is_valid, "La prueba inválida debería ser rechazada");

        Ok(())
    }
} 