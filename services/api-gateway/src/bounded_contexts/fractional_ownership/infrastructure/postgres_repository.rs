use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::fractional_ownership::domain::{
    aggregates::{OwnershipContractAggregate, OwnershipAnalytics},
    repository::{
        OwnershipContractRepository, OwnershipContractSpecification, 
        MarketStatistics, ShareRepository, OwnershipContractQueryRepository
    },
    value_objects::{
        OwnershipContractId, OwnershipPercentage, SharePrice, RevenueAmount, 
        ShareId, VestingPeriod
    },
    entities::{FractionalShare, RevenueDistribution, ContractStatus},
    events::DomainEvent,
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::bounded_contexts::user::domain::value_objects::UserId;

/// PostgreSQL implementation of OwnershipContractRepository
/// 
/// This repository handles the persistence of ownership contract aggregates
/// to PostgreSQL database, including proper mapping between domain objects
/// and database records.
pub struct PostgresOwnershipContractRepository {
    pool: PgPool,
}

impl PostgresOwnershipContractRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Maps database row to domain aggregate
    async fn map_row_to_aggregate(&self, row: &sqlx::postgres::PgRow) -> Result<OwnershipContractAggregate, AppError> {
        let contract_id: Uuid = row.get("id");
        let song_id: Uuid = row.get("song_id");
        let artist_id: Uuid = row.get("artist_id");
        let total_shares: i32 = row.get("total_shares");
        let price_per_share: f64 = row.get("price_per_share");
        let artist_retained_percentage: f64 = row.get("artist_retained_percentage");
        let minimum_investment: Option<f64> = row.get("minimum_investment");
        let maximum_ownership_per_user: Option<f64> = row.get("maximum_ownership_per_user");
        let contract_status: String = row.get("contract_status");
        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: DateTime<Utc> = row.get("updated_at");
        let version: i64 = row.get("version");

        // Load shares for this contract
        let shares = self.load_shares_for_contract(&OwnershipContractId::from_uuid(contract_id)).await?;

        // Reconstruct aggregate
        let mut aggregate = OwnershipContractAggregate::reconstruct(
            OwnershipContractId::from_uuid(contract_id),
            SongId::from_uuid(song_id),
            ArtistId::from_uuid(artist_id),
            total_shares as u32,
            SharePrice::new(price_per_share)?,
            OwnershipPercentage::new(artist_retained_percentage)?,
            minimum_investment.map(|mi| RevenueAmount::new(mi)).transpose()?,
            maximum_ownership_per_user.map(|mo| OwnershipPercentage::new(mo)).transpose()?,
            Self::parse_contract_status(&contract_status)?,
            created_at,
            updated_at,
            version,
            shares,
        )?;

        Ok(aggregate)
    }

    /// Load all shares belonging to a contract
    async fn load_shares_for_contract(&self, contract_id: &OwnershipContractId) -> Result<HashMap<ShareId, FractionalShare>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, contract_id, song_id, owner_id, ownership_percentage,
                purchase_price, current_market_value, total_revenue_received,
                is_locked, lock_reason, vesting_start_date, vesting_end_date,
                purchased_at, created_at, updated_at
            FROM fractional_shares 
            WHERE contract_id = $1
            "#,
            contract_id.value()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut shares = HashMap::new();
        for row in rows {
            let share_id = ShareId::from_uuid(row.id);
            let owner_id = UserId::from_uuid(row.owner_id);
            let song_id = SongId::from_uuid(row.song_id);
            let ownership_percentage = OwnershipPercentage::new(row.ownership_percentage)?;
            let purchase_price = SharePrice::new(row.purchase_price)?;
            let current_market_value = SharePrice::new(row.current_market_value)?;
            let total_revenue_received = RevenueAmount::new(row.total_revenue_received)?;

            let vesting_period = if let (Some(start), Some(end)) = (row.vesting_start_date, row.vesting_end_date) {
                Some(VestingPeriod::new(start, end)?)
            } else {
                None
            };

            let share = FractionalShare::reconstruct(
                share_id.clone(),
                OwnershipContractId::from_uuid(row.contract_id),
                song_id,
                owner_id,
                ownership_percentage,
                purchase_price,
                current_market_value,
                total_revenue_received,
                row.is_locked,
                row.lock_reason,
                vesting_period,
                row.purchased_at,
                row.created_at,
                row.updated_at,
            )?;

            shares.insert(share_id, share);
        }

        Ok(shares)
    }

    /// Parse string to ContractStatus enum
    fn parse_contract_status(status: &str) -> Result<ContractStatus, AppError> {
        match status {
            "Draft" => Ok(ContractStatus::Draft),
            "Active" => Ok(ContractStatus::Active),
            "Paused" => Ok(ContractStatus::Paused),
            "SoldOut" => Ok(ContractStatus::SoldOut),
            "Terminated" => Ok(ContractStatus::Terminated),
            _ => Err(AppError::InvalidInput(format!("Invalid contract status: {}", status))),
        }
    }

    /// Save shares to database
    async fn save_shares(&self, contract_id: &OwnershipContractId, shares: &HashMap<ShareId, FractionalShare>) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Delete existing shares for this contract
        sqlx::query!(
            "DELETE FROM fractional_shares WHERE contract_id = $1",
            contract_id.value()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Insert all shares
        for share in shares.values() {
            let vesting_start = share.vesting_period().as_ref().map(|vp| vp.start_date());
            let vesting_end = share.vesting_period().as_ref().map(|vp| vp.end_date());

            sqlx::query!(
                r#"
                INSERT INTO fractional_shares (
                    id, contract_id, song_id, owner_id, ownership_percentage,
                    purchase_price, current_market_value, total_revenue_received,
                    is_locked, lock_reason, vesting_start_date, vesting_end_date,
                    purchased_at, created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                "#,
                share.id().value(),
                share.contract_id().value(),
                share.song_id().value(),
                share.owner_id().value(),
                share.ownership_percentage().value(),
                share.purchase_price().value(),
                share.current_market_value().value(),
                share.total_revenue_received().value(),
                share.is_locked(),
                share.lock_reason(),
                vesting_start,
                vesting_end,
                share.purchased_at(),
                share.created_at(),
                share.updated_at()
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    /// Save domain events to event store
    async fn save_events(&self, aggregate: &OwnershipContractAggregate) -> Result<(), AppError> {
        if aggregate.pending_events().is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        for event in aggregate.pending_events() {
            let event_data = serde_json::to_value(event)
                .map_err(|e| AppError::SerializationError(e.to_string()))?;

            sqlx::query!(
                r#"
                INSERT INTO domain_events (
                    id, aggregate_id, aggregate_type, event_type, 
                    event_data, event_version, occurred_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                Uuid::new_v4(),
                aggregate.id().value(),
                "OwnershipContract",
                event.event_type(),
                event_data,
                event.version(),
                event.occurred_at()
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl OwnershipContractRepository for PostgresOwnershipContractRepository {
    async fn save(&self, aggregate: &OwnershipContractAggregate) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let contract = aggregate.contract();

        // Insert ownership contract
        sqlx::query!(
            r#"
            INSERT INTO ownership_contracts (
                id, song_id, artist_id, total_shares, price_per_share,
                artist_retained_percentage, shares_available_for_sale, shares_sold,
                minimum_investment, maximum_ownership_per_user, contract_status,
                created_at, updated_at, version
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
            aggregate.id().value(),
            contract.song_id.value(),
            contract.artist_id.value(),
            contract.total_shares as i32,
            contract.price_per_share.value(),
            contract.artist_retained_percentage.value(),
            contract.shares_available_for_sale as i32,
            contract.shares_sold as i32,
            contract.minimum_investment.as_ref().map(|mi| mi.value()),
            contract.maximum_ownership_per_user.as_ref().map(|mo| mo.value()),
            format!("{:?}", contract.contract_status),
            contract.created_at,
            contract.updated_at,
            aggregate.version()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Save shares separately
        self.save_shares(aggregate.id(), aggregate.shares()).await?;

        // Save domain events
        self.save_events(aggregate).await?;

        Ok(())
    }

    async fn update(&self, aggregate: &OwnershipContractAggregate) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let contract = aggregate.contract();

        // Update ownership contract with optimistic locking
        let result = sqlx::query!(
            r#"
            UPDATE ownership_contracts 
            SET 
                shares_available_for_sale = $1,
                shares_sold = $2,
                contract_status = $3,
                updated_at = $4,
                version = $5
            WHERE id = $6 AND version = $7
            "#,
            contract.shares_available_for_sale as i32,
            contract.shares_sold as i32,
            format!("{:?}", contract.contract_status),
            Utc::now(),
            aggregate.version() + 1,
            aggregate.id().value(),
            aggregate.version()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::ConcurrencyConflict("Contract was modified by another transaction".to_string()));
        }

        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update shares
        self.save_shares(aggregate.id(), aggregate.shares()).await?;

        // Save domain events
        self.save_events(aggregate).await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &OwnershipContractId) -> Result<Option<OwnershipContractAggregate>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                id, song_id, artist_id, total_shares, price_per_share,
                artist_retained_percentage, shares_available_for_sale, shares_sold,
                minimum_investment, maximum_ownership_per_user, contract_status,
                created_at, updated_at, version
            FROM ownership_contracts 
            WHERE id = $1
            "#,
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let aggregate = self.map_row_to_aggregate(&row).await?;
                Ok(Some(aggregate))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, id: &OwnershipContractId) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Delete shares first (foreign key constraint)
        sqlx::query!(
            "DELETE FROM fractional_shares WHERE contract_id = $1",
            id.value()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Delete contract
        sqlx::query!(
            "DELETE FROM ownership_contracts WHERE id = $1",
            id.value()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_song_id(&self, song_id: &SongId) -> Result<Vec<OwnershipContractAggregate>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, song_id, artist_id, total_shares, price_per_share,
                artist_retained_percentage, shares_available_for_sale, shares_sold,
                minimum_investment, maximum_ownership_per_user, contract_status,
                created_at, updated_at, version
            FROM ownership_contracts 
            WHERE song_id = $1
            ORDER BY created_at DESC
            "#,
            song_id.value()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut contracts = Vec::new();
        for row in rows {
            let aggregate = self.map_row_to_aggregate(&row).await?;
            contracts.push(aggregate);
        }

        Ok(contracts)
    }

    async fn find_by_artist_id(&self, artist_id: &ArtistId) -> Result<Vec<OwnershipContractAggregate>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, song_id, artist_id, total_shares, price_per_share,
                artist_retained_percentage, shares_available_for_sale, shares_sold,
                minimum_investment, maximum_ownership_per_user, contract_status,
                created_at, updated_at, version
            FROM ownership_contracts 
            WHERE artist_id = $1
            ORDER BY created_at DESC
            "#,
            artist_id.value()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut contracts = Vec::new();
        for row in rows {
            let aggregate = self.map_row_to_aggregate(&row).await?;
            contracts.push(aggregate);
        }

        Ok(contracts)
    }

    async fn find_active_contracts(&self) -> Result<Vec<OwnershipContractAggregate>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, song_id, artist_id, total_shares, price_per_share,
                artist_retained_percentage, shares_available_for_sale, shares_sold,
                minimum_investment, maximum_ownership_per_user, contract_status,
                created_at, updated_at, version
            FROM ownership_contracts 
            WHERE contract_status = 'Active'
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut contracts = Vec::new();
        for row in rows {
            let aggregate = self.map_row_to_aggregate(&row).await?;
            contracts.push(aggregate);
        }

        Ok(contracts)
    }

    async fn find_paginated(&self, offset: u32, limit: u32) -> Result<Vec<OwnershipContractAggregate>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, song_id, artist_id, total_shares, price_per_share,
                artist_retained_percentage, shares_available_for_sale, shares_sold,
                minimum_investment, maximum_ownership_per_user, contract_status,
                created_at, updated_at, version
            FROM ownership_contracts 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut contracts = Vec::new();
        for row in rows {
            let aggregate = self.map_row_to_aggregate(&row).await?;
            contracts.push(aggregate);
        }

        Ok(contracts)
    }

    async fn exists_for_song(&self, song_id: &SongId) -> Result<bool, AppError> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM ownership_contracts WHERE song_id = $1",
            song_id.value()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.count.unwrap_or(0) > 0)
    }

    async fn get_contract_analytics(&self, id: &OwnershipContractId) -> Result<Option<OwnershipAnalytics>, AppError> {
        // This would be implemented with complex SQL queries for analytics
        // For now, return None to indicate analytics should be calculated from the aggregate
        Ok(None)
    }

    async fn get_total_market_value(&self) -> Result<f64, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(total_shares * price_per_share), 0) as total_value
            FROM ownership_contracts 
            WHERE contract_status IN ('Active', 'SoldOut')
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.total_value.unwrap_or(0.0))
    }

    async fn find_by_completion_range(&self, min_completion: f64, max_completion: f64) -> Result<Vec<OwnershipContractAggregate>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, song_id, artist_id, total_shares, price_per_share,
                artist_retained_percentage, shares_available_for_sale, shares_sold,
                minimum_investment, maximum_ownership_per_user, contract_status,
                created_at, updated_at, version,
                (shares_sold::float / total_shares::float * 100) as completion_percentage
            FROM ownership_contracts 
            WHERE (shares_sold::float / total_shares::float * 100) BETWEEN $1 AND $2
            ORDER BY completion_percentage DESC
            "#,
            min_completion,
            max_completion
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut contracts = Vec::new();
        for row in rows {
            let aggregate = self.map_row_to_aggregate(&row).await?;
            contracts.push(aggregate);
        }

        Ok(contracts)
    }

    async fn find_contracts_with_user_shares(&self, user_id: &UserId) -> Result<Vec<OwnershipContractAggregate>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT DISTINCT
                oc.id, oc.song_id, oc.artist_id, oc.total_shares, oc.price_per_share,
                oc.artist_retained_percentage, oc.shares_available_for_sale, oc.shares_sold,
                oc.minimum_investment, oc.maximum_ownership_per_user, oc.contract_status,
                oc.created_at, oc.updated_at, oc.version
            FROM ownership_contracts oc
            INNER JOIN fractional_shares fs ON oc.id = fs.contract_id
            WHERE fs.owner_id = $1
            ORDER BY oc.created_at DESC
            "#,
            user_id.value()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut contracts = Vec::new();
        for row in rows {
            let aggregate = self.map_row_to_aggregate(&row).await?;
            contracts.push(aggregate);
        }

        Ok(contracts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        // This would set up a test database
        // For now, we'll skip the actual database tests
        todo!("Setup test database for integration tests")
    }

    #[tokio::test]
    #[ignore] // Ignore until test database is set up
    async fn test_postgres_repository_save_and_find() {
        let pool = setup_test_db().await;
        let repo = PostgresOwnershipContractRepository::new(pool);

        // Create test aggregate
        let aggregate = OwnershipContractAggregate::create_contract(
            SongId::new(),
            ArtistId::new(),
            1000,
            SharePrice::new(10.0).unwrap(),
            OwnershipPercentage::new(51.0).unwrap(),
            Some(RevenueAmount::new(100.0).unwrap()),
            Some(OwnershipPercentage::new(20.0).unwrap()),
        ).unwrap();

        // Save
        repo.save(&aggregate).await.unwrap();

        // Find
        let found = repo.find_by_id(aggregate.id()).await.unwrap();
        assert!(found.is_some());

        let found_aggregate = found.unwrap();
        assert_eq!(found_aggregate.id(), aggregate.id());
    }
} 