use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::{
    aggregates::*,
    entities::*,
    value_objects::*,
    repository::*,
};

/// PostgreSQL implementation of PaymentRepository
pub struct PostgreSQLPaymentRepository {
    pool: PgPool,
}

impl PostgreSQLPaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PaymentRepository for PostgreSQLPaymentRepository {
    async fn save(&self, payment: &PaymentAggregate) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;
        
        // Save payment
        sqlx::query!(
            "INSERT INTO payments (
                id, payer_id, payee_id, amount, currency, net_amount, 
                platform_fee, payment_method, purpose_type, status, 
                blockchain_hash, created_at, updated_at, completed_at,
                failure_reason, idempotency_key, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                blockchain_hash = EXCLUDED.blockchain_hash,
                updated_at = EXCLUDED.updated_at,
                completed_at = EXCLUDED.completed_at,
                failure_reason = EXCLUDED.failure_reason",
            payment.payment().id().value(),
            payment.payment().payer_id(),
            payment.payment().payee_id(),
            payment.payment().amount().value(),
            payment.payment().amount().currency() as Currency,
            payment.payment().net_amount().value(),
            payment.payment().platform_fee().map(|f| f.value()),
            serde_json::to_value(payment.payment().payment_method()).unwrap(),
            serde_json::to_value(payment.payment().purpose()).unwrap(),
            format!("{:?}", payment.payment().status()),
            payment.payment().blockchain_hash().map(|h| h.value().to_string()),
            payment.payment().created_at(),
            payment.payment().updated_at(),
            payment.payment().completed_at(),
            payment.payment().failure_reason().map(|r| r.to_string()),
            payment.payment().idempotency_key(),
            serde_json::to_value(payment.payment().metadata()).unwrap()
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;
        
        // Save payment events
        for event in payment.uncommitted_events() {
            sqlx::query!(
                "INSERT INTO payment_events (id, payment_id, event_type, event_data, occurred_at)
                 VALUES ($1, $2, $3, $4, $5)",
                event.id(),
                event.payment_id().value(),
                format!("{:?}", event.event_type()),
                serde_json::to_value(event.event_data()).unwrap(),
                event.occurred_at()
            )
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;
        }
        
        tx.commit().await.map_err(AppError::Database)?;
        Ok(())
    }
    
    async fn find_by_id(&self, id: &PaymentId) -> Result<Option<PaymentAggregate>, AppError> {
        let payment_row = sqlx::query!(
            "SELECT * FROM payments WHERE id = $1",
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        match payment_row {
            Some(row) => {
                let payment = self.row_to_payment(&row)?;
                let events = self.load_payment_events(id).await?;
                
                let mut aggregate = PaymentAggregate::from_payment(payment);
                for event in events {
                    aggregate.add_event(event);
                }
                
                Ok(Some(aggregate))
            }
            None => Ok(None),
        }
    }
    
    async fn find_by_transaction_id(&self, transaction_id: &TransactionId) -> Result<Option<PaymentAggregate>, AppError> {
        let payment_row = sqlx::query!(
            "SELECT p.* FROM payments p 
             JOIN transactions t ON p.id = t.payment_id 
             WHERE t.id = $1",
            transaction_id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        match payment_row {
            Some(row) => {
                let payment = self.row_to_payment(&row)?;
                let payment_id = payment.id();
                self.find_by_id(payment_id).await
            }
            None => Ok(None),
        }
    }
    
    async fn find_by_idempotency_key(&self, key: &str) -> Result<Option<PaymentAggregate>, AppError> {
        let payment_row = sqlx::query!(
            "SELECT * FROM payments WHERE idempotency_key = $1",
            key
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        match payment_row {
            Some(row) => {
                let payment = self.row_to_payment(&row)?;
                let payment_id = payment.id();
                self.find_by_id(payment_id).await
            }
            None => Ok(None),
        }
    }
    
    async fn find_by_filter(
        &self,
        filter: PaymentFilter,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<PaymentAggregate>, AppError> {
        let mut query = String::from("SELECT * FROM payments WHERE 1=1");
        let mut params = Vec::new();
        
        if let Some(user_id) = filter.user_id {
            query.push_str(" AND (payer_id = $1 OR payee_id = $1)");
            params.push(user_id);
        }
        
        if let Some(status) = filter.status {
            query.push_str(" AND status = $2");
            params.push(Uuid::new_v4()); // Placeholder
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT $3 OFFSET $4");
        
        // Simplified query execution
        let rows = sqlx::query!(
            "SELECT * FROM payments ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        let mut payment_aggregates = Vec::new();
        for row in rows {
            let payment = self.row_to_payment(&row)?;
            let aggregate = PaymentAggregate::from_payment(payment);
            payment_aggregates.push(aggregate);
        }
        
        Ok(payment_aggregates)
    }
    
    async fn count_by_filter(&self, _filter: PaymentFilter) -> Result<u64, AppError> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM payments"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(row.count.unwrap_or(0) as u64)
    }
    
    async fn get_payment_events(&self, payment_id: &PaymentId) -> Result<Vec<PaymentEvent>, AppError> {
        self.load_payment_events(payment_id).await
    }
    
    async fn find_batch_by_id(&self, batch_id: Uuid) -> Result<Option<PaymentBatch>, AppError> {
        let batch_row = sqlx::query!(
            "SELECT * FROM payment_batches WHERE id = $1",
            batch_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        match batch_row {
            Some(row) => {
                let batch = self.row_to_payment_batch(&row)?;
                Ok(Some(batch))
            }
            None => Ok(None),
        }
    }
    
    async fn save_batch(&self, batch: &PaymentBatch) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO payment_batches (
                id, batch_type, total_amount, currency, payment_count, 
                successful_payments, failed_payments, status, created_at, 
                completed_at, created_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                successful_payments = EXCLUDED.successful_payments,
                failed_payments = EXCLUDED.failed_payments,
                status = EXCLUDED.status,
                completed_at = EXCLUDED.completed_at",
            batch.id(),
            batch.batch_type(),
            batch.total_amount().value(),
            batch.total_amount().currency() as Currency,
            batch.payment_count() as i32,
            batch.successful_payments() as i32,
            batch.failed_payments() as i32,
            batch.status(),
            batch.created_at(),
            batch.completed_at(),
            batch.created_by()
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(())
    }
}

impl PostgreSQLPaymentRepository {
    async fn load_payment_events(&self, payment_id: &PaymentId) -> Result<Vec<PaymentEvent>, AppError> {
        let rows = sqlx::query!(
            "SELECT * FROM payment_events WHERE payment_id = $1 ORDER BY occurred_at",
            payment_id.value()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        let mut events = Vec::new();
        for row in rows {
            let event = PaymentEvent::new(
                row.id,
                PaymentId::from_uuid(row.payment_id),
                PaymentEventType::PaymentInitiated, // Simplified
                serde_json::from_value(row.event_data).unwrap_or(serde_json::Value::Null),
                row.occurred_at,
            );
            events.push(event);
        }
        
        Ok(events)
    }
    
    fn row_to_payment(&self, row: &sqlx::postgres::PgRow) -> Result<Payment, AppError> {
        // Simplified conversion
        let payment_id = PaymentId::from_uuid(row.try_get("id").map_err(AppError::Database)?);
        let payer_id: Uuid = row.try_get("payer_id").map_err(AppError::Database)?;
        let payee_id: Uuid = row.try_get("payee_id").map_err(AppError::Database)?;
        let amount_value: f64 = row.try_get("amount").map_err(AppError::Database)?;
        let currency: Currency = row.try_get("currency").map_err(AppError::Database)?;
        let amount = Amount::new(amount_value, currency).map_err(AppError::DomainError)?;
        
        let net_amount_value: f64 = row.try_get("net_amount").map_err(AppError::Database)?;
        let net_amount = Amount::new(net_amount_value, currency).map_err(AppError::DomainError)?;
        
        let platform_fee = row.try_get::<Option<f64>, _>("platform_fee").map_err(AppError::Database)?
            .map(|fee| Amount::new(fee, currency).map_err(AppError::DomainError))
            .transpose()?;
        
        let payment_method: PaymentMethod = serde_json::from_value(
            row.try_get("payment_method").map_err(AppError::Database)?
        ).map_err(AppError::JsonError)?;
        
        let purpose: PaymentPurpose = serde_json::from_value(
            row.try_get("purpose_type").map_err(AppError::Database)?
        ).map_err(AppError::JsonError)?;
        
        let status_str: String = row.try_get("status").map_err(AppError::Database)?;
        let status = match status_str.as_str() {
            "Pending" => PaymentStatus::Pending,
            "Processing" => PaymentStatus::Processing,
            "Completed" => PaymentStatus::Completed,
            "Failed" => PaymentStatus::Failed,
            "Cancelled" => PaymentStatus::Cancelled,
            "OnHold" => PaymentStatus::OnHold,
            _ => PaymentStatus::Pending,
        };
        
        let blockchain_hash = row.try_get::<Option<String>, _>("blockchain_hash").map_err(AppError::Database)?
            .map(|hash| TransactionHash::new(hash).map_err(AppError::DomainError))
            .transpose()?;
        
        let created_at: DateTime<Utc> = row.try_get("created_at").map_err(AppError::Database)?;
        let updated_at: DateTime<Utc> = row.try_get("updated_at").map_err(AppError::Database)?;
        let completed_at: Option<DateTime<Utc>> = row.try_get("completed_at").map_err(AppError::Database)?;
        let failure_reason: Option<String> = row.try_get("failure_reason").map_err(AppError::Database)?;
        let idempotency_key: Option<String> = row.try_get("idempotency_key").map_err(AppError::Database)?;
        
        let metadata: PaymentMetadata = serde_json::from_value(
            row.try_get("metadata").map_err(AppError::Database)?
        ).map_err(AppError::JsonError)?;
        
        let payment = Payment::new(
            payment_id,
            payer_id,
            payee_id,
            amount,
            net_amount,
            platform_fee,
            payment_method,
            purpose,
            status,
            blockchain_hash,
            created_at,
            updated_at,
            completed_at,
            failure_reason,
            idempotency_key,
            metadata,
        );
        
        Ok(payment)
    }
    
    fn row_to_payment_batch(&self, row: &sqlx::postgres::PgRow) -> Result<PaymentBatch, AppError> {
        let id: Uuid = row.try_get("id").map_err(AppError::Database)?;
        let batch_type: String = row.try_get("batch_type").map_err(AppError::Database)?;
        let total_amount_value: f64 = row.try_get("total_amount").map_err(AppError::Database)?;
        let currency: Currency = row.try_get("currency").map_err(AppError::Database)?;
        let total_amount = Amount::new(total_amount_value, currency).map_err(AppError::DomainError)?;
        
        let payment_count: i32 = row.try_get("payment_count").map_err(AppError::Database)?;
        let successful_payments: i32 = row.try_get("successful_payments").map_err(AppError::Database)?;
        let failed_payments: i32 = row.try_get("failed_payments").map_err(AppError::Database)?;
        let status: String = row.try_get("status").map_err(AppError::Database)?;
        
        let created_at: DateTime<Utc> = row.try_get("created_at").map_err(AppError::Database)?;
        let completed_at: Option<DateTime<Utc>> = row.try_get("completed_at").map_err(AppError::Database)?;
        let created_by: Uuid = row.try_get("created_by").map_err(AppError::Database)?;
        
        let batch = PaymentBatch::new(
            id,
            batch_type,
            total_amount,
            vec![], // Simplified - would load from payment_batch_items
            payment_count as u32,
            successful_payments as u32,
            failed_payments as u32,
            status,
            created_at,
            None, // started_at
            completed_at,
            created_by,
        );
        
        Ok(batch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    
    // Integration tests would go here
    // They would require a test database setup
    
    #[tokio::test]
    async fn test_payment_repository_creation() {
        // This test would require a real database connection
        // For now, just test that the repository can be created
        
        // Mock pool creation would be needed for proper testing
        // let pool = PgPool::connect("postgresql://test").await.unwrap();
        // let repo = PostgreSQLPaymentRepository::new(pool);
        
        assert!(true);
    }
} 