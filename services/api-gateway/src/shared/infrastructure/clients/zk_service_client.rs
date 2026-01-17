use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use vibestream_types::*; // Assuming types are available here

#[derive(Clone)]
pub struct ZkServiceClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct GenerateProofRequest {
    proof_type: ZkProofType,
}

#[derive(Debug, Serialize)]
struct VerifyProofRequest {
    proof: ZkProof,
}

#[derive(Debug, Deserialize)]
pub struct VerifyProofResponse {
    valid: bool,
    #[allow(dead_code)]
    circuit_id: String,
    #[allow(dead_code)]
    verified_at: chrono::DateTime<chrono::Utc>,
}

// ZK Types need to be mirrored or imported if not in shared types.
// Assuming vibestream_types exports them. If not, we will need to define them.
// Based on `services/zk-service/src/service.rs`, they are simple Enums/Structs.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkProofType {
    Solvency { balance: u64, threshold: u64 },
    Transaction { amount: u64, sender_balance: u64 },
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    pub proof_data: Vec<u8>,
    pub public_inputs: Vec<String>,
    pub circuit_id: String,
}

impl ZkServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30)) // Proof generation takes time
                .build()
                .unwrap_or_default(),
            base_url,
        }
    }

    pub async fn generate_proof(&self, proof_type: ZkProofType) -> Result<ZkProof> {
        let url = format!("{}/generate", self.base_url);
        let request = GenerateProofRequest { proof_type };

        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to request proof generation")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Proof generation failed: {}", error_text);
        }

        let proof: ZkProof = response.json().await
            .context("Failed to parse proof response")?;

        Ok(proof)
    }

    pub async fn verify_proof(&self, proof: ZkProof) -> Result<bool> {
        let url = format!("{}/verify", self.base_url);
        let request = VerifyProofRequest { proof };

        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to request proof verification")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Proof verification failed: {}", error_text);
        }

        let body: VerifyProofResponse = response.json().await
            .context("Failed to parse verification response")?;

        Ok(body.valid)
    }
}
