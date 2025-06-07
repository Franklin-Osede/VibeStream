use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibesToken {
    pub holder_id: Uuid,
    pub balance: f64,
    pub locked_amount: f64,  // For staking or pending withdrawals
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransaction {
    pub id: Uuid,
    pub from_id: Option<Uuid>,  // None if minting
    pub to_id: Uuid,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Listen,           // Earned from listening
    Creation,         // Earned from song creation/upload
    Royalty,         // Earned from royalties
    Transfer,        // P2P transfer
    Withdrawal,      // Platform withdrawal
    Stake,           // Staking tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token_balance() {
        let token = VibesToken {
            holder_id: Uuid::new_v4(),
            balance: 100.0,
            locked_amount: 0.0,
        };

        assert!(token.balance >= 0.0);
        assert!(token.locked_amount >= 0.0);
    }

    #[test]
    fn test_create_transaction() {
        let tx = TokenTransaction {
            id: Uuid::new_v4(),
            from_id: None,
            to_id: Uuid::new_v4(),
            amount: 10.0,
            transaction_type: TransactionType::Listen,
            timestamp: chrono::Utc::now(),
        };

        assert!(tx.amount > 0.0);
        match tx.transaction_type {
            TransactionType::Listen => assert!(tx.from_id.is_none()),
            _ => {}
        }
    }
} 