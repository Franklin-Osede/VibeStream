use anyhow::{Context, Result};
use ark_bn254::Fr;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use light_poseidon::{Poseidon, PoseidonHasher, PoseidonParameters};
use std::io::Cursor;

pub struct ProofVerifier {
    ark: Vec<Fr>,
    mds: Vec<Vec<Fr>>,
}

impl ProofVerifier {
    pub fn new() -> Result<Self> {
        // Crear los mismos parámetros que en el generador
        let ark = vec![Fr::from(1u64); 3];
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

    pub async fn verify(
        &self,
        proof: Vec<u8>,
        public_signals: Vec<u8>,
        _song_id: &str,
        _user_id: &str,
    ) -> Result<bool> {
        // Deserializar la prueba
        let mut proof_cursor = Cursor::new(&proof[..]);
        let proof_hash = Fr::deserialize_with_mode(
            &mut proof_cursor,
            Compress::Yes,
            Validate::Yes,
        ).context("Failed to deserialize proof")?;

        // Deserializar las señales públicas
        let mut public_elements = Vec::new();
        let mut signals_cursor = Cursor::new(&public_signals[..]);
        
        while signals_cursor.position() < public_signals.len() as u64 {
            let element = Fr::deserialize_with_mode(
                &mut signals_cursor,
                Compress::Yes,
                Validate::Yes,
            ).context("Failed to deserialize public signal")?;
            public_elements.push(element);
        }

        // Verificar que tenemos el número correcto de señales públicas
        if public_elements.len() < 2 {
            return Err(anyhow::anyhow!("Invalid number of public signals"));
        }

        // Calcular el hash Poseidon de las señales públicas
        let mut poseidon = self.create_poseidon();
        let computed_hash = PoseidonHasher::hash(&mut poseidon, &public_elements)?;

        // Verificar que el hash coincide con la prueba
        Ok(computed_hash == proof_hash)
    }
} 