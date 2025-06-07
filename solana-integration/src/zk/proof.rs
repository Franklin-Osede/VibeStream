use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::generator::ProofGenerator;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProofOfListenInputs {
    pub start_time: String,
    pub current_time: String,
    pub end_time: String,
    pub song_hash: String,
    pub user_signature: [String; 3],
    pub user_public_key: [String; 2],
    pub nonce: String,
}

pub struct ProofOfListenService {
    generator: ProofGenerator,
}

impl ProofOfListenService {
    pub fn new() -> Result<Self> {
        let generator = ProofGenerator::new()?;
        Ok(Self { generator })
    }

    pub fn generate(&self, inputs: ProofOfListenInputs) -> Result<(Vec<u8>, Vec<u8>)> {
        // Convertir el struct de entradas a un HashMap
        let inputs_value = serde_json::to_value(inputs)?;
        let inputs_map: HashMap<String, String> = serde_json::from_value(inputs_value)?;

        // Generar la prueba usando el generador
        self.generator.generate_proof(inputs_map)
    }
} 