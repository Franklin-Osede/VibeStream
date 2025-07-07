use vibestream_types::*;
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_ff::PrimeField;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;
use tokio::fs;
use tracing::{info, warn, error, debug};
use anyhow::{Result as AnyResult, Context};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Estructura para representar una prueba ZK
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    /// Proof data serialized as base64
    pub proof: String,
    /// Public inputs as JSON
    pub public_inputs: serde_json::Value,
    /// Verification key as base64
    pub verification_key: String,
    /// Circuit identifier
    pub circuit_id: String,
    /// Proof generation timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Circuit manager para compilar y ejecutar circuitos circom
pub struct CircuitManager {
    circuits_dir: std::path::PathBuf,
    cache_dir: std::path::PathBuf,
    redis_client: Option<redis::Client>,
    compiled_circuits: HashMap<String, CompiledCircuit>,
}

#[derive(Debug, Clone)]
struct CompiledCircuit {
    proving_key: Vec<u8>,
    verification_key: Vec<u8>,
    wasm_path: std::path::PathBuf,
    r1cs_path: std::path::PathBuf,
}

impl CircuitManager {
    pub async fn new(circuits_dir: &Path, cache_dir: &Path, redis_url: Option<&str>) -> AnyResult<Self> {
        // Create cache directory if it doesn't exist
        fs::create_dir_all(cache_dir).await?;

        let redis_client = if let Some(url) = redis_url {
            match redis::Client::open(url) {
                Ok(client) => Some(client),
                Err(e) => {
                    warn!("Failed to connect to Redis for circuit caching: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let mut manager = Self {
            circuits_dir: circuits_dir.to_path_buf(),
            cache_dir: cache_dir.to_path_buf(),
            redis_client,
            compiled_circuits: HashMap::new(),
        };

        // Pre-compile essential circuits
        manager.compile_circuit("proof_of_listen").await?;
        
        info!("✅ CircuitManager initialized with {} circuits", manager.compiled_circuits.len());
        Ok(manager)
    }

    async fn compile_circuit(&mut self, circuit_name: &str) -> AnyResult<()> {
        let circuit_path = self.circuits_dir.join(format!("{}.circom", circuit_name));
        
        if !circuit_path.exists() {
            return Err(anyhow::anyhow!("Circuit file not found: {}", circuit_path.display()));
        }

        info!("🔨 Compiling circuit: {}", circuit_name);

        // Create temporary directory for compilation
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Step 1: Compile circom to r1cs and wasm
        let compile_output = Command::new("circom")
            .arg(&circuit_path)
            .arg("--r1cs")
            .arg("--wasm")
            .arg("--output")
            .arg(temp_path)
            .arg("-l")  // Add include library path
            .arg(&self.circuits_dir)  // Add circuits directory as include path
            .output()?;

        if !compile_output.status.success() {
            let error_msg = String::from_utf8_lossy(&compile_output.stderr);
            return Err(anyhow::anyhow!("Circom compilation failed: {}", error_msg));
        }

        // Step 2: Generate proving and verification keys using powers of tau
        let ptau_path = self.cache_dir.join("powersoftau28_hez_final_14.ptau");
        
        // Download powers of tau if not exists
        if !ptau_path.exists() {
            info!("📥 Downloading powers of tau ceremony file...");
            let ptau_url = "https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_14.ptau";
            let response = reqwest::get(ptau_url).await?;
            let ptau_bytes = response.bytes().await?;
            fs::write(&ptau_path, ptau_bytes).await?;
        }

        let r1cs_path = temp_path.join(format!("{}.r1cs", circuit_name));
        let zkey_path = temp_path.join(format!("{}.zkey", circuit_name));

        // Generate initial zkey
        let zkey_output = Command::new("snarkjs")
            .arg("groth16")
            .arg("setup")
            .arg(&r1cs_path)
            .arg(&ptau_path)
            .arg(&zkey_path)
            .output()?;

        if !zkey_output.status.success() {
            let error_msg = String::from_utf8_lossy(&zkey_output.stderr);
            return Err(anyhow::anyhow!("zkey generation failed: {}", error_msg));
        }

        // Export verification key
        let vkey_path = temp_path.join(format!("{}_vkey.json", circuit_name));
        let vkey_output = Command::new("snarkjs")
            .arg("zkey")
            .arg("export")
            .arg("verificationkey")
            .arg(&zkey_path)
            .arg(&vkey_path)
            .output()?;

        if !vkey_output.status.success() {
            let error_msg = String::from_utf8_lossy(&vkey_output.stderr);
            return Err(anyhow::anyhow!("Verification key export failed: {}", error_msg));
        }

        // Copy compiled files to cache
        let cache_circuit_dir = self.cache_dir.join(circuit_name);
        fs::create_dir_all(&cache_circuit_dir).await?;

        let wasm_src = temp_path.join(format!("{}_js", circuit_name)).join(format!("{}.wasm", circuit_name));
        let wasm_dst = cache_circuit_dir.join(format!("{}.wasm", circuit_name));
        fs::copy(&wasm_src, &wasm_dst).await?;

        let r1cs_dst = cache_circuit_dir.join(format!("{}.r1cs", circuit_name));
        fs::copy(&r1cs_path, &r1cs_dst).await?;

        let zkey_dst = cache_circuit_dir.join(format!("{}.zkey", circuit_name));
        fs::copy(&zkey_path, &zkey_dst).await?;

        let vkey_dst = cache_circuit_dir.join(format!("{}_vkey.json", circuit_name));
        fs::copy(&vkey_path, &vkey_dst).await?;

        // Read keys into memory
        let proving_key = fs::read(&zkey_dst).await?;
        let verification_key = fs::read(&vkey_dst).await?;

        let compiled_circuit = CompiledCircuit {
            proving_key,
            verification_key,
            wasm_path: wasm_dst,
            r1cs_path: r1cs_dst,
        };

        self.compiled_circuits.insert(circuit_name.to_string(), compiled_circuit);

        info!("✅ Circuit compiled successfully: {}", circuit_name);
        Ok(())
    }

    async fn generate_witness(&self, circuit_name: &str, input: &serde_json::Value) -> AnyResult<Vec<u8>> {
        let compiled = self.compiled_circuits.get(circuit_name)
            .ok_or_else(|| anyhow::anyhow!("Circuit not compiled: {}", circuit_name))?;

        // Create temporary directory for witness generation
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Write input to file
        let input_path = temp_path.join("input.json");
        fs::write(&input_path, serde_json::to_string_pretty(input)?).await?;

        // Generate witness
        let witness_path = temp_path.join("witness.wtns");
        let witness_output = Command::new("node")
            .arg("generate_witness.js")
            .arg(&compiled.wasm_path)
            .arg(&input_path)
            .arg(&witness_path)
            .current_dir(&temp_path)
            .output()?;

        if !witness_output.status.success() {
            let error_msg = String::from_utf8_lossy(&witness_output.stderr);
            return Err(anyhow::anyhow!("Witness generation failed: {}", error_msg));
        }

        // Read witness
        let witness = fs::read(&witness_path).await?;
        Ok(witness)
    }

    async fn generate_proof(&self, circuit_name: &str, input: &serde_json::Value) -> AnyResult<ZkProof> {
        let compiled = self.compiled_circuits.get(circuit_name)
            .ok_or_else(|| anyhow::anyhow!("Circuit not compiled: {}", circuit_name))?;

        // Generate witness
        let witness = self.generate_witness(circuit_name, input).await?;

        // Create temporary directory for proof generation
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Write witness to file
        let witness_path = temp_path.join("witness.wtns");
        fs::write(&witness_path, &witness).await?;

        // Write zkey to temp
        let zkey_path = temp_path.join("circuit.zkey");
        fs::write(&zkey_path, &compiled.proving_key).await?;

        // Generate proof
        let proof_path = temp_path.join("proof.json");
        let public_path = temp_path.join("public.json");

        let proof_output = Command::new("snarkjs")
            .arg("groth16")
            .arg("prove")
            .arg(&zkey_path)
            .arg(&witness_path)
            .arg(&proof_path)
            .arg(&public_path)
            .output()?;

        if !proof_output.status.success() {
            let error_msg = String::from_utf8_lossy(&proof_output.stderr);
            return Err(anyhow::anyhow!("Proof generation failed: {}", error_msg));
        }

        // Read proof and public inputs
        let proof_json = fs::read_to_string(&proof_path).await?;
        let public_json = fs::read_to_string(&public_path).await?;

        let proof = ZkProof {
            proof: BASE64.encode(&proof_json),
            public_inputs: serde_json::from_str(&public_json)?,
            verification_key: BASE64.encode(&compiled.verification_key),
            circuit_id: circuit_name.to_string(),
            generated_at: chrono::Utc::now(),
        };

        Ok(proof)
    }

    async fn verify_proof(&self, proof: &ZkProof) -> AnyResult<bool> {
        let compiled = self.compiled_circuits.get(&proof.circuit_id)
            .ok_or_else(|| anyhow::anyhow!("Circuit not compiled: {}", proof.circuit_id))?;

        // Create temporary directory for verification
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Write proof, public inputs, and verification key to files
        let proof_json = String::from_utf8(BASE64.decode(&proof.proof)?)?;
        let proof_path = temp_path.join("proof.json");
        fs::write(&proof_path, &proof_json).await?;

        let public_path = temp_path.join("public.json");
        fs::write(&public_path, serde_json::to_string_pretty(&proof.public_inputs)?).await?;

        let vkey_json = String::from_utf8(BASE64.decode(&proof.verification_key)?)?;
        let vkey_path = temp_path.join("vkey.json");
        fs::write(&vkey_path, &vkey_json).await?;

        // Verify proof
        let verify_output = Command::new("snarkjs")
            .arg("groth16")
            .arg("verify")
            .arg(&vkey_path)
            .arg(&public_path)
            .arg(&proof_path)
            .output()?;

        if !verify_output.status.success() {
            let error_msg = String::from_utf8_lossy(&verify_output.stderr);
            warn!("Proof verification failed: {}", error_msg);
            return Ok(false);
        }

        // Parse verification output
        let output_str = String::from_utf8_lossy(&verify_output.stdout);
        let is_valid = output_str.contains("OK");

        Ok(is_valid)
    }
}

/// Generador de pruebas ZK
pub struct ZkProofGenerator {
    circuit_manager: CircuitManager,
}

impl ZkProofGenerator {
    pub async fn new(circuits_dir: &Path, cache_dir: &Path, redis_url: Option<&str>) -> AnyResult<Self> {
        let circuit_manager = CircuitManager::new(circuits_dir, cache_dir, redis_url).await?;
        Ok(Self { circuit_manager })
    }
    
    /// Genera una prueba de solvencia sin revelar el balance exacto
    pub async fn generate_solvency_proof(&self, balance: u64, min_threshold: u64) -> Result<ZkProof> {
        if balance < min_threshold {
            return Err(VibeStreamError::Validation {
                message: "Insufficient balance for proof generation".to_string(),
            });
        }

        // TODO: Implement solvency circuit
        // For now, return a mock proof that's properly structured
        let proof = ZkProof {
            proof: BASE64.encode(b"solvency_proof_data"),
            public_inputs: json!({
                "min_threshold": min_threshold.to_string(),
                "proof_type": "solvency"
            }),
            verification_key: BASE64.encode(b"solvency_vkey"),
            circuit_id: "solvency".to_string(),
            generated_at: chrono::Utc::now(),
        };

        Ok(proof)
    }
    
    /// Genera una prueba de transacción privada
    pub async fn generate_transaction_proof(&self, amount: u64, sender_balance: u64) -> Result<ZkProof> {
        if sender_balance < amount {
            return Err(VibeStreamError::InsufficientBalance {
                required: amount,
                available: sender_balance,
            });
        }

        // TODO: Implement transaction circuit
        // For now, return a mock proof that's properly structured
        let proof = ZkProof {
            proof: BASE64.encode(b"transaction_proof_data"),
            public_inputs: json!({
                "amount": amount.to_string(),
                "proof_type": "transaction"
            }),
            verification_key: BASE64.encode(b"transaction_vkey"),
            circuit_id: "transaction".to_string(),
            generated_at: chrono::Utc::now(),
        };

        Ok(proof)
    }

    /// Genera una prueba de proof of listen usando el circuito circom
    pub async fn generate_listen_proof(
        &self,
        start_time: u64,
        current_time: u64,
        end_time: u64,
        song_hash: &str,
        user_signature: &[String; 3],
        user_public_key: &[String; 2],
        nonce: &str,
    ) -> Result<ZkProof> {
        let input = json!({
            "startTime": start_time.to_string(),
            "currentTime": current_time.to_string(),
            "endTime": end_time.to_string(),
            "songHash": song_hash,
            "userSignature": user_signature,
            "userPublicKey": user_public_key,
            "nonce": nonce
        });

        let proof = self.circuit_manager.generate_proof("proof_of_listen", &input).await
            .map_err(|e| VibeStreamError::Internal { 
                message: format!("Failed to generate listen proof: {}", e) 
            })?;

        Ok(proof)
    }
}

/// Verificador de pruebas ZK
pub struct ZkProofVerifier {
    circuit_manager: CircuitManager,
}

impl ZkProofVerifier {
    pub async fn new(circuits_dir: &Path, cache_dir: &Path, redis_url: Option<&str>) -> AnyResult<Self> {
        let circuit_manager = CircuitManager::new(circuits_dir, cache_dir, redis_url).await?;
        Ok(Self { circuit_manager })
    }
    
    /// Verifica una prueba ZK
    pub async fn verify_proof(&self, proof: &ZkProof) -> Result<bool> {
        if proof.proof.is_empty() || proof.verification_key.is_empty() {
            return Ok(false);
        }

        // Use real verification for supported circuits
        match proof.circuit_id.as_str() {
            "proof_of_listen" => {
                let is_valid = self.circuit_manager.verify_proof(proof).await
                    .map_err(|e| VibeStreamError::Internal { 
                        message: format!("Verification failed: {}", e) 
                    })?;
                Ok(is_valid)
            }
            "solvency" | "transaction" => {
                // For now, mock verification for these circuits
                info!("Mock verification for circuit: {}", proof.circuit_id);
                Ok(true)
            }
            _ => {
                warn!("Unknown circuit type: {}", proof.circuit_id);
                Ok(false)
            }
        }
    }
}

// Add reqwest dependency for downloading powers of tau
#[tokio::main]
async fn download_powers_of_tau() -> AnyResult<()> {
    // This function is used internally by CircuitManager
    Ok(())
} 