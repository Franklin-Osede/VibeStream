use anyhow::Result;
use ark_bn254::{Bn254, Fr};
use ark_groth16::{prepare_verifying_key, verify_proof};
use ark_serialize::CanonicalDeserialize;

pub struct ProofVerifier {
    vk: ark_groth16::VerifyingKey<Bn254>,
}

impl ProofVerifier {
    pub fn new(vk_bytes: &[u8]) -> Result<Self> {
        let vk = ark_groth16::VerifyingKey::deserialize(vk_bytes)?;
        Ok(Self { vk })
    }

    pub fn verify(
        &self,
        proof: &[u8],
        public_inputs: &[Fr],
    ) -> Result<bool> {
        let proof = ark_groth16::Proof::deserialize(proof)?;
        let pvk = prepare_verifying_key(&self.vk);
        
        Ok(verify_proof(&pvk, &proof, public_inputs)?)
    }
} 