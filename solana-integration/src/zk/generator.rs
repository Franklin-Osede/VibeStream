use anyhow::{Context, Result};
use ark_bn254::Fr;
use ark_serialize::{CanonicalSerialize, Compress};
use ark_std::{UniformRand, str::FromStr};
use light_poseidon::{Poseidon, PoseidonHasher, PoseidonParameters};
use std::collections::HashMap;

pub struct ProofGenerator {
    ark: Vec<Fr>,
    mds: Vec<Vec<Fr>>,
}

impl ProofGenerator {
    pub fn new() -> Result<Self> {
        // Crear parámetros para Poseidon
        let ark = vec![Fr::from(1u64); 3];  // Ejemplo simple
        let mds = vec![
            vec![Fr::from(1u64), Fr::from(1u64)],
            vec![Fr::from(1u64), Fr::from(1u64)],
        ];
        
        Ok(Self { ark, mds })
    }

    fn create_poseidon(&self) -> Poseidon<Fr> {
        let params = PoseidonParameters::new(
            self.ark.clone(),
            self.mds.clone(),
            8,  // full_rounds
            57, // partial_rounds
            5,  // alpha
            1,  // rate
        );
        Poseidon::new(params)
    }

    pub fn generate_proof(
        &self,
        inputs: HashMap<String, String>,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        // Convertir las entradas a elementos del campo
        let mut field_elements = Vec::new();
        for (key, value) in inputs.iter() {
            let fr = Fr::from_str(value)
                .map_err(|_| anyhow::anyhow!("Failed to convert {} to field element", key))?;
            field_elements.push(fr);
        }

        // Generar un nonce aleatorio
        let mut rng = ark_std::rand::thread_rng();
        let nonce = Fr::rand(&mut rng);
        field_elements.push(nonce);

        // Calcular el hash Poseidon de las entradas
        let mut poseidon = self.create_poseidon();
        let hash = PoseidonHasher::hash(&mut poseidon, &field_elements)?;

        // Serializar el hash como prueba
        let mut proof = Vec::new();
        hash.serialize_with_mode(&mut proof, Compress::Yes)
            .context("Failed to serialize hash")?;
        
        // Serializar las entradas públicas
        let mut public_signals = Vec::new();
        for element in field_elements.iter() {
            element.serialize_with_mode(&mut public_signals, Compress::Yes)
                .context("Failed to serialize public signals")?;
        }

        Ok((proof, public_signals))
    }
} 