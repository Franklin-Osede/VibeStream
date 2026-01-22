use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::value_objects::Amount;

/// Refund entity (simplified for repository usage)
pub struct Refund {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub amount: Amount,
    pub reason: String,
    pub status: String,
    pub gateway_refund_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait RefundRepository {
    async fn save(&self, refund: &Refund) -> Result<(), AppError>;
    async fn find_by_payment_id(&self, payment_id: Uuid) -> Result<Vec<Refund>, AppError>;
}

pub struct PostgresRefundRepository {
    pool: PgPool,
}

impl PostgresRefundRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefundRepository for PostgresRefundRepository {
    async fn save(&self, refund: &Refund) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO refunds (
                id, payment_id, amount, currency, reason, status, 
                gateway_refund_id, metadata, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                gateway_refund_id = EXCLUDED.gateway_refund_id,
                updated_at = EXCLUDED.updated_at",
            refund.id,
            refund.payment_id,
            refund.amount.value(),
            refund.amount.currency().to_string(),
            refund.reason,
            refund.status,
            refund.gateway_refund_id,
            refund.metadata,
            refund.created_at,
            refund.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_payment_id(&self, payment_id: Uuid) -> Result<Vec<Refund>, AppError> {
        let rows = sqlx::query!(
            "SELECT * FROM refunds WHERE payment_id = $1 ORDER BY created_at DESC",
            payment_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut refunds = Vec::new();
        for row in rows {
            let currency_str = row.currency;
            let currency = match currency_str.as_str() {
                "USD" => crate::bounded_contexts::payment::domain::value_objects::Currency::USD,
                "ETH" => crate::bounded_contexts::payment::domain::value_objects::Currency::ETH,
                "VIBES" => crate::bounded_contexts::payment::domain::value_objects::Currency::VIBES,
                _ => crate::bounded_contexts::payment::domain::value_objects::Currency::USD,
            };
            
            refunds.push(Refund {
                id: row.id,
                payment_id: row.payment_id,
                amount: Amount::new(row.amount, currency).map_err(|e| AppError::DomainRuleViolation(e))?,
                reason: row.reason,
                status: row.status,
                gateway_refund_id: row.gateway_refund_id,
                metadata: row.metadata,
                created_at: row.created_at.expect("created_at should be not null"),
                updated_at: row.updated_at.expect("updated_at should be not null"),
            });
        }
        Ok(refunds)
    }
}
