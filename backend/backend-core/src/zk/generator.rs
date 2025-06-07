use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize};
use serde_json;

// Estructura para deserializar el resultado de snarkjs
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FullProof {
    proof: serde_json::Value,
    public_signals: serde_json::Value,
}

#[derive(Debug)]
pub struct ProofGenerator {
    wasm_path: PathBuf,
    pk_path: PathBuf,
}

impl ProofGenerator {
    pub fn new(wasm_path: PathBuf, pk_path: PathBuf) -> Result<Self> {
        if !wasm_path.exists() {
            return Err(anyhow::anyhow!("WASM file not found at: {:?}", wasm_path));
        }
        if !pk_path.exists() {
            return Err(anyhow::anyhow!("Proving key file not found at: {:?}", pk_path));
        }
        Ok(Self { wasm_path, pk_path })
    }

    pub fn generate_proof(
        &self,
        inputs: HashMap<String, String>,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let inputs_json = serde_json::to_string(&inputs)
            .context("Failed to serialize proof inputs")?;

        // Apuntar a la nueva ubicación del script en la carpeta /scripts
        let script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("scripts")
            .join("prove.js");

        if !script_path.exists() {
             return Err(anyhow::anyhow!("prove.js script not found at: {:?}", script_path));
        }

        let output = Command::new("node")
            .arg(&script_path)
            .arg(&inputs_json)
            .arg(&self.wasm_path)
            .arg(&self.pk_path)
            .output()
            .context("Failed to execute node prove.js script")?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "Proof generation script failed: {}",
                error_message
            ));
        }

        let full_proof: FullProof = serde_json::from_slice(&output.stdout)
            .context("Failed to deserialize proof from script output")?;
            
        let proof_bytes = serde_json::to_vec(&full_proof.proof)
            .context("Failed to serialize proof to bytes")?;
        let public_signals_bytes = serde_json::to_vec(&full_proof.public_signals)
            .context("Failed to serialize public signals to bytes")?;

        Ok((proof_bytes, public_signals_bytes))
    }
} 