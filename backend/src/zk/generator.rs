use anyhow::Result;
use circom_proving::{CircomProver, WitnessCalculator};
use std::path::Path;

pub struct ProofGenerator {
    prover: CircomProver,
    witness_calculator: WitnessCalculator,
}

impl ProofGenerator {
    pub fn new(
        wasm_path: &Path,
        r1cs_path: &Path,
        pk_path: &Path,
    ) -> Result<Self> {
        let prover = CircomProver::new(r1cs_path, pk_path)?;
        let witness_calculator = WitnessCalculator::new(wasm_path)?;

        Ok(Self {
            prover,
            witness_calculator,
        })
    }

    pub async fn generate_proof(
        &self,
        inputs: serde_json::Value,
    ) -> Result<Vec<u8>> {
        let witness = self.witness_calculator.calculate_witness(inputs)?;
        let proof = self.prover.prove(&witness)?;
        
        Ok(proof)
    }
} 