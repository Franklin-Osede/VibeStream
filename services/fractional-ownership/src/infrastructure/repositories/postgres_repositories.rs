// Implementaciones de repositorios con PostgreSQL
use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::entities::{FractionalSong, ShareOwnership, ShareTransaction};
use crate::domain::repositories::{
    FractionalOwnershipRepository, ShareOwnershipRepository, ShareTransactionRepository,
    FractionalOwnershipAnalyticsRepository, UserPortfolioSummary, TrendingSong, 
    GenreMarketStats, TopShareholder, SongInvestmentSummary
};
use crate::domain::errors::FractionalOwnershipError;
use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount};
use crate::infrastructure::database::connection::DatabaseConnection;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::collections::HashMap;

/// Implementación PostgreSQL del FractionalOwnershipRepository
pub struct PostgresFractionalOwnershipRepository {
    db_connection: DatabaseConnection,
}

impl PostgresFractionalOwnershipRepository {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }

    /// Helper para construir aggregate desde rows de la DB
    async fn build_aggregate_from_song_id(&self, song_id: Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();

        // Cargar FractionalSong
        let song_row = sqlx::query!(
            "SELECT id, song_id, artist_id, title, total_shares, available_shares, current_price_per_share, created_at, updated_at
             FROM fractional_songs WHERE id = $1",
            song_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error cargando song: {}", e)))?;

        let song_row = match song_row {
            Some(row) => row,
            None => return Ok(None),
        };

        // Crear FractionalSong
        let fractional_song = FractionalSong::new(
            song_row.id,
            song_row.song_id,
            song_row.artist_id,
            song_row.title,
            song_row.total_shares as u32,
            song_row.available_shares as u32,
            SharePrice::new(song_row.current_price_per_share as f64)
                .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid price: {}", e)))?,
        );

        // Cargar ShareOwnerships
        let ownership_rows = sqlx::query!(
            "SELECT id, fractional_song_id, user_id, shares_owned, purchase_price, total_earnings, purchase_date
             FROM share_ownerships WHERE fractional_song_id = $1",
            song_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error cargando ownerships: {}", e)))?;

        let mut ownerships = HashMap::new();
        for row in ownership_rows {
            let ownership = ShareOwnership::new(
                row.id,
                row.fractional_song_id,
                row.user_id,
                row.shares_owned as u32,
                SharePrice::new(row.purchase_price as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid purchase price: {}", e)))?,
                RevenueAmount::new(row.total_earnings as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid earnings: {}", e)))?,
            );
            ownerships.insert(row.user_id, ownership);
        }

        // Construir aggregate
        let aggregate = FractionalOwnershipAggregate::new(fractional_song, ownerships);
        Ok(Some(aggregate))
    }
}

#[async_trait]
impl FractionalOwnershipRepository for PostgresFractionalOwnershipRepository {
    async fn get_by_id(&self, song_id: Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        self.build_aggregate_from_song_id(song_id).await
    }

    async fn load_aggregate(&self, song_id: &Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        self.build_aggregate_from_song_id(*song_id).await
    }

    async fn save_aggregate(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        let mut tx = pool.begin().await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error comenzando transacción: {}", e)))?;

        // Guardar FractionalSong
        let song = aggregate.fractional_song();
        sqlx::query!(
            "INSERT INTO fractional_songs (id, song_id, artist_id, title, total_shares, available_shares, current_price_per_share, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
             ON CONFLICT (id) DO UPDATE SET
                available_shares = $6,
                current_price_per_share = $7,
                updated_at = NOW()",
            song.id(),
            song.song_id(),
            song.artist_id(),
            song.title(),
            song.total_shares() as i32,
            song.available_shares() as i32,
            song.current_price_per_share().amount() as f64
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error guardando song: {}", e)))?;

        // Guardar ShareOwnerships
        for ownership in aggregate.ownerships().values() {
            sqlx::query!(
                "INSERT INTO share_ownerships (id, fractional_song_id, user_id, shares_owned, purchase_price, total_earnings, purchase_date)
                 VALUES ($1, $2, $3, $4, $5, $6, NOW())
                 ON CONFLICT (id) DO UPDATE SET
                    shares_owned = $4,
                    total_earnings = $6",
                ownership.id(),
                ownership.fractional_song_id(),
                ownership.user_id(),
                ownership.shares_owned() as i32,
                ownership.purchase_price().amount() as f64,
                ownership.total_earnings().amount() as f64
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error guardando ownership: {}", e)))?;
        }

        tx.commit().await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error confirmando transacción: {}", e)))?;

        Ok(())
    }

    async fn get_user_ownerships(&self, user_id: &Uuid) -> Result<Vec<ShareOwnership>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let rows = sqlx::query!(
            "SELECT id, fractional_song_id, user_id, shares_owned, purchase_price, total_earnings, purchase_date
             FROM share_ownerships WHERE user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo ownerships: {}", e)))?;

        let mut ownerships = Vec::new();
        for row in rows {
            let ownership = ShareOwnership::new(
                row.id,
                row.fractional_song_id,
                row.user_id,
                row.shares_owned as u32,
                SharePrice::new(row.purchase_price as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid price: {}", e)))?,
                RevenueAmount::new(row.total_earnings as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid earnings: {}", e)))?,
            );
            ownerships.push(ownership);
        }

        Ok(ownerships)
    }

    async fn get_user_revenue_for_song(&self, user_id: &Uuid, song_id: &Uuid) -> Result<Option<RevenueAmount>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let row = sqlx::query!(
            "SELECT total_earnings FROM share_ownerships WHERE user_id = $1 AND fractional_song_id = $2",
            user_id, song_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo revenue: {}", e)))?;

        match row {
            Some(row) => {
                let revenue = RevenueAmount::new(row.total_earnings as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid revenue: {}", e)))?;
                Ok(Some(revenue))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError> {
        self.save_aggregate(aggregate).await
    }

    async fn delete(&self, song_id: Uuid) -> Result<(), FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        let mut tx = pool.begin().await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error comenzando transacción: {}", e)))?;

        // Eliminar ownerships primero (FK constraint)
        sqlx::query!("DELETE FROM share_ownerships WHERE fractional_song_id = $1", song_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error eliminando ownerships: {}", e)))?;

        // Eliminar song
        sqlx::query!("DELETE FROM fractional_songs WHERE id = $1", song_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error eliminando song: {}", e)))?;

        tx.commit().await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error confirmando transacción: {}", e)))?;

        Ok(())
    }

    async fn find_by_artist_id(&self, artist_id: Uuid) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let song_rows = sqlx::query!(
            "SELECT id FROM fractional_songs WHERE artist_id = $1",
            artist_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error buscando por artist: {}", e)))?;

        let mut aggregates = Vec::new();
        for row in song_rows {
            if let Some(aggregate) = self.build_aggregate_from_song_id(row.id).await? {
                aggregates.push(aggregate);
            }
        }

        Ok(aggregates)
    }

    async fn get_all_paginated(&self, page: u32, size: u32) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        let offset = (page.saturating_sub(1)) * size;
        
        let song_rows = sqlx::query!(
            "SELECT id FROM fractional_songs ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            size as i64, offset as i64
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo paginado: {}", e)))?;

        let mut aggregates = Vec::new();
        for row in song_rows {
            if let Some(aggregate) = self.build_aggregate_from_song_id(row.id).await? {
                aggregates.push(aggregate);
            }
        }

        Ok(aggregates)
    }
}

/// Implementación PostgreSQL para ShareOwnershipRepository
pub struct PostgresShareOwnershipRepository {
    db_connection: DatabaseConnection,
}

impl PostgresShareOwnershipRepository {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl ShareOwnershipRepository for PostgresShareOwnershipRepository {
    async fn get_by_user_and_song(&self, user_id: Uuid, song_id: Uuid) -> Result<Option<ShareOwnership>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let row = sqlx::query!(
            "SELECT id, fractional_song_id, user_id, shares_owned, purchase_price, total_earnings, purchase_date
             FROM share_ownerships WHERE user_id = $1 AND fractional_song_id = $2",
            user_id, song_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo ownership: {}", e)))?;

        match row {
            Some(row) => {
                let ownership = ShareOwnership::new(
                    row.id,
                    row.fractional_song_id,
                    row.user_id,
                    row.shares_owned as u32,
                    SharePrice::new(row.purchase_price as f64)
                        .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid price: {}", e)))?,
                    RevenueAmount::new(row.total_earnings as f64)
                        .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid earnings: {}", e)))?,
                );
                Ok(Some(ownership))
            }
            None => Ok(None),
        }
    }

    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<ShareOwnership>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let rows = sqlx::query!(
            "SELECT id, fractional_song_id, user_id, shares_owned, purchase_price, total_earnings, purchase_date
             FROM share_ownerships WHERE user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo ownerships por user: {}", e)))?;

        let mut ownerships = Vec::new();
        for row in rows {
            let ownership = ShareOwnership::new(
                row.id,
                row.fractional_song_id,
                row.user_id,
                row.shares_owned as u32,
                SharePrice::new(row.purchase_price as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid price: {}", e)))?,
                RevenueAmount::new(row.total_earnings as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid earnings: {}", e)))?,
            );
            ownerships.push(ownership);
        }

        Ok(ownerships)
    }

    async fn get_by_song(&self, song_id: Uuid) -> Result<Vec<ShareOwnership>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let rows = sqlx::query!(
            "SELECT id, fractional_song_id, user_id, shares_owned, purchase_price, total_earnings, purchase_date
             FROM share_ownerships WHERE fractional_song_id = $1",
            song_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo ownerships por song: {}", e)))?;

        let mut ownerships = Vec::new();
        for row in rows {
            let ownership = ShareOwnership::new(
                row.id,
                row.fractional_song_id,
                row.user_id,
                row.shares_owned as u32,
                SharePrice::new(row.purchase_price as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid price: {}", e)))?,
                RevenueAmount::new(row.total_earnings as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid earnings: {}", e)))?,
            );
            ownerships.push(ownership);
        }

        Ok(ownerships)
    }

    async fn save(&self, ownership: &ShareOwnership) -> Result<(), FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        sqlx::query!(
            "INSERT INTO share_ownerships (id, fractional_song_id, user_id, shares_owned, purchase_price, total_earnings, purchase_date)
             VALUES ($1, $2, $3, $4, $5, $6, NOW())
             ON CONFLICT (id) DO UPDATE SET
                shares_owned = $4,
                total_earnings = $6",
            ownership.id(),
            ownership.fractional_song_id(),
            ownership.user_id(),
            ownership.shares_owned() as i32,
            ownership.purchase_price().amount() as f64,
            ownership.total_earnings().amount() as f64
        )
        .execute(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error guardando ownership: {}", e)))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        sqlx::query!("DELETE FROM share_ownerships WHERE id = $1", id)
            .execute(pool)
            .await
            .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error eliminando ownership: {}", e)))?;

        Ok(())
    }
}

/// Implementación PostgreSQL para ShareTransactionRepository
pub struct PostgresShareTransactionRepository {
    db_connection: DatabaseConnection,
}

impl PostgresShareTransactionRepository {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl ShareTransactionRepository for PostgresShareTransactionRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<ShareTransaction>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let row = sqlx::query!(
            "SELECT id, fractional_song_id, from_user_id, to_user_id, shares_quantity, total_amount, status, created_at
             FROM share_transactions WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo transaction: {}", e)))?;

        match row {
            Some(row) => {
                let transaction = ShareTransaction::new(
                    row.id,
                    row.fractional_song_id,
                    row.from_user_id,
                    row.to_user_id,
                    row.shares_quantity as u32,
                    RevenueAmount::new(row.total_amount as f64)
                        .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid amount: {}", e)))?,
                );
                Ok(Some(transaction))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, transaction: &ShareTransaction) -> Result<(), FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        sqlx::query!(
            "INSERT INTO share_transactions (id, fractional_song_id, from_user_id, to_user_id, shares_quantity, total_amount, status, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, 'pending', NOW())
             ON CONFLICT (id) DO UPDATE SET
                status = 'pending'",
            transaction.id(),
            transaction.fractional_song_id(),
            transaction.from_user_id(),
            transaction.to_user_id(),
            transaction.shares_quantity() as i32,
            transaction.total_amount().amount() as f64
        )
        .execute(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error guardando transaction: {}", e)))?;

        Ok(())
    }

    async fn get_pending_by_user(&self, user_id: Uuid) -> Result<Vec<ShareTransaction>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let rows = sqlx::query!(
            "SELECT id, fractional_song_id, from_user_id, to_user_id, shares_quantity, total_amount, status, created_at
             FROM share_transactions 
             WHERE (from_user_id = $1 OR to_user_id = $1) AND status = 'pending'",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo pending transactions: {}", e)))?;

        let mut transactions = Vec::new();
        for row in rows {
            let transaction = ShareTransaction::new(
                row.id,
                row.fractional_song_id,
                row.from_user_id,
                row.to_user_id,
                row.shares_quantity as u32,
                RevenueAmount::new(row.total_amount as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid amount: {}", e)))?,
            );
            transactions.push(transaction);
        }

        Ok(transactions)
    }

    async fn get_by_song(&self, song_id: Uuid) -> Result<Vec<ShareTransaction>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        
        let rows = sqlx::query!(
            "SELECT id, fractional_song_id, from_user_id, to_user_id, shares_quantity, total_amount, status, created_at
             FROM share_transactions WHERE fractional_song_id = $1",
            song_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo transactions por song: {}", e)))?;

        let mut transactions = Vec::new();
        for row in rows {
            let transaction = ShareTransaction::new(
                row.id,
                row.fractional_song_id,
                row.from_user_id,
                row.to_user_id,
                row.shares_quantity as u32,
                RevenueAmount::new(row.total_amount as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid amount: {}", e)))?,
            );
            transactions.push(transaction);
        }

        Ok(transactions)
    }

    async fn get_transactions_history(&self, user_id: Uuid, page: u32, size: u32) -> Result<Vec<ShareTransaction>, FractionalOwnershipError> {
        let pool = self.db_connection.pool();
        let offset = (page.saturating_sub(1)) * size;
        
        let rows = sqlx::query!(
            "SELECT id, fractional_song_id, from_user_id, to_user_id, shares_quantity, total_amount, status, created_at
             FROM share_transactions 
             WHERE from_user_id = $1 OR to_user_id = $1
             ORDER BY created_at DESC
             LIMIT $2 OFFSET $3",
            user_id, size as i64, offset as i64
        )
        .fetch_all(pool)
        .await
        .map_err(|e| FractionalOwnershipError::InfrastructureError(format!("Error obteniendo transaction history: {}", e)))?;

        let mut transactions = Vec::new();
        for row in rows {
            let transaction = ShareTransaction::new(
                row.id,
                row.fractional_song_id,
                row.from_user_id,
                row.to_user_id,
                row.shares_quantity as u32,
                RevenueAmount::new(row.total_amount as f64)
                    .map_err(|e| FractionalOwnershipError::BusinessRuleViolation(format!("Invalid amount: {}", e)))?,
            );
            transactions.push(transaction);
        }

        Ok(transactions)
    }
} 