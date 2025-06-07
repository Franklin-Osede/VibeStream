use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::generator::ProofGenerator;

// Estructura que representa las entradas para el circuito ProofOfListen.
// Los nombres de los campos deben coincidir exactamente con las señales de entrada del circuito.
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

// Servicio para manejar la lógica específica de Proof of Listen.
pub struct ProofOfListenService {
    generator: ProofGenerator,
}

impl ProofOfListenService {
    // Inicializa el servicio, cargando los archivos del circuito.
    pub fn new(wasm_path: PathBuf, pk_path: PathBuf) -> Result<Self> {
        let generator = ProofGenerator::new(wasm_path, pk_path)?;
        Ok(Self { generator })
    }

    // Genera una prueba para una sesión de escucha.
    pub fn generate(&self, inputs: ProofOfListenInputs) -> Result<(Vec<u8>, Vec<u8>)> {
        // Convierte el struct de entradas a un HashMap<String, String> que el generador puede usar.
        let inputs_value = serde_json::to_value(inputs)?;
        let inputs_map: std::collections::HashMap<String, String> = serde_json::from_value(inputs_value)
            .expect("Failed to convert inputs struct to map");

        // Llama al generador de pruebas genérico.
        let (proof, public_inputs) = self.generator.generate_proof(inputs_map)?;

        Ok((proof, public_inputs))
    }
} 