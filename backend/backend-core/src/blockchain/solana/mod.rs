use anyhow::Result;
use async_trait::async_trait;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use spl_token::instruction as token_instruction;
use spl_associated_token_account::get_associated_token_address;
use std::str::FromStr;

use crate::blockchain::common::{
    BlockchainClient, TokenClient, TokenInfo, TransactionInfo, TransactionStatus,
    BlockchainResult,
};

pub struct SolanaClient {
    pub keypair: Keypair,
    pub rpc_client: RpcClient,
}

impl SolanaClient {
    pub fn new(keypair: Keypair, rpc_url: String) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url,
            CommitmentConfig::confirmed(),
        );
        Self { keypair, rpc_client }
    }

    pub fn get_public_key(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    async fn create_and_send_transaction(
        &self,
        instructions: Vec<solana_sdk::instruction::Instruction>,
    ) -> BlockchainResult<String> {
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_transaction(&transaction)?;
        Ok(signature.to_string())
    }
}

#[async_trait]
impl BlockchainClient for SolanaClient {
    async fn get_balance(&self, address: &str) -> Result<u64> {
        let pubkey = Pubkey::from_str(address)?;
        let balance = self.rpc_client.get_balance(&pubkey)?;
        Ok(balance)
    }

    async fn transfer(&self, to: &str, amount: u64) -> Result<TransactionInfo> {
        let to_pubkey = Pubkey::from_str(to)?;
        let instruction = system_instruction::transfer(
            &self.keypair.pubkey(),
            &to_pubkey,
            amount,
        );

        let tx_hash = self.create_and_send_transaction(vec![instruction]).await?;

        Ok(TransactionInfo {
            hash: tx_hash,
            from: self.get_public_key().to_string(),
            to: to_pubkey.to_string(),
            value: amount.to_string(),
            status: TransactionStatus::Pending,
        })
    }

    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus> {
        let signature = solana_sdk::signature::Signature::from_str(tx_hash)?;
        let status = self.rpc_client.get_signature_status(&signature)?;

        match status {
            Some(Ok(_)) => Ok(TransactionStatus::Confirmed),
            Some(Err(_)) => Ok(TransactionStatus::Failed),
            None => Ok(TransactionStatus::Pending),
        }
    }
}

#[async_trait]
impl TokenClient for SolanaClient {
    async fn get_token_info(&self, token_address: &str) -> Result<TokenInfo> {
        let token_pubkey = Pubkey::from_str(token_address)?;
        let token_supply = self.rpc_client.get_token_supply(&token_pubkey)?;
        let token_account = self.rpc_client.get_token_account(&token_pubkey)?;

        Ok(TokenInfo {
            address: token_pubkey.to_string(),
            symbol: token_account.map(|a| a.symbol).unwrap_or_else(|| "UNKNOWN".to_string()),
            decimals: token_account.map(|a| a.decimals).unwrap_or(9),
            total_supply: token_supply.amount,
        })
    }

    async fn transfer_token(&self, token_address: &str, to: &str, amount: u64) -> Result<TransactionInfo> {
        let token_pubkey = Pubkey::from_str(token_address)?;
        let to_pubkey = Pubkey::from_str(to)?;
        
        let to_token_account = get_associated_token_address(
            &to_pubkey,
            &token_pubkey,
        );

        let from_token_account = get_associated_token_address(
            &self.keypair.pubkey(),
            &token_pubkey,
        );

        let instruction = token_instruction::transfer(
            &spl_token::id(),
            &from_token_account,
            &to_token_account,
            &self.keypair.pubkey(),
            &[&self.keypair.pubkey()],
            amount,
        )?;

        let tx_hash = self.create_and_send_transaction(vec![instruction]).await?;

        Ok(TransactionInfo {
            hash: tx_hash,
            from: from_token_account.to_string(),
            to: to_token_account.to_string(),
            value: amount.to_string(),
            status: TransactionStatus::Pending,
        })
    }

    async fn get_token_balance(&self, token_address: &str, address: &str) -> Result<u64> {
        let token_pubkey = Pubkey::from_str(token_address)?;
        let owner_pubkey = Pubkey::from_str(address)?;
        
        let token_account = get_associated_token_address(
            &owner_pubkey,
            &token_pubkey,
        );

        let balance = self.rpc_client.get_token_account_balance(&token_account)?;
        Ok(balance.amount)
    }
}

#[cfg(test)]
mod tests; 