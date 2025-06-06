use anchor_lang::prelude::*;

declare_id!("Vi6eSTREAMzkkProof111111111111111111111111");

#[program]
pub mod vibestream_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn verify_proof(
        ctx: Context<VerifyProof>,
        proof_data: Vec<u8>,
        timestamp: i64,
    ) -> Result<()> {
        // Implementación de verificación de prueba
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyProof<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ProofError {
    #[msg("La prueba ZK no es válida")]
    InvalidProof,
    #[msg("Timestamp inválido")]
    InvalidTimestamp,
} 