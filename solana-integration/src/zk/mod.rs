mod generator;
mod proof;
mod verifier;

pub use proof::{ProofOfListenInputs, ProofOfListenService};
pub use verifier::ProofVerifier;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZKProof {
    pub proof: Vec<u8>,
    pub public_signals: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofRequest {
    pub start_time: u64,
    pub current_time: u64,
    pub end_time: u64,
    pub song_hash: String,
    pub user_signature: [String; 3],
    pub user_public_key: [String; 2],
    pub nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub proof: ZKProof,
    pub song_id: String,
    pub user_id: String,
}

pub struct ZKService {
    proof_service: ProofOfListenService,
    verifier: ProofVerifier,
}

impl ZKService {
    pub fn new() -> Result<Self> {
        let proof_service = ProofOfListenService::new()?;
        let verifier = ProofVerifier::new()?;
        
        Ok(Self {
            proof_service,
            verifier,
        })
    }

    pub async fn generate_proof(&self, request: ProofRequest) -> Result<ZKProof> {
        let inputs = ProofOfListenInputs {
            start_time: request.start_time.to_string(),
            current_time: request.current_time.to_string(),
            end_time: request.end_time.to_string(),
            song_hash: request.song_hash,
            user_signature: request.user_signature,
            user_public_key: request.user_public_key,
            nonce: request.nonce,
        };

        let (proof, public_signals) = self.proof_service.generate(inputs)?;
        
        Ok(ZKProof {
            proof,
            public_signals,
        })
    }

    pub async fn verify_proof(&self, request: VerifyRequest) -> Result<bool> {
        self.verifier.verify(
            request.proof.proof,
            request.proof.public_signals,
            &request.song_id,
            &request.user_id,
        ).await
    }
} 