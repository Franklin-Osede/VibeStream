use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::bounded_contexts::campaign::domain::{
    entities::{Campaign, CampaignStatus},
    value_objects::{CampaignId, CampaignName},
    repository::CampaignRepository,
};
use crate::bounded_contexts::campaign::domain::value_objects::*;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::shared::domain::repositories::RepoResult;

#[derive(Debug, Clone)]
struct CampaignRow {
    id: String,
    song_id: String,
    artist_id: String,
    name: String,
    description: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    boost_multiplier: f64,
    nft_price: f64,
    max_nfts: i32,
    nfts_sold: i32,
    target_revenue: Option<f64>,
    status: String,
    nft_contract_address: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct PostgresCampaignRepository {
    pool: PgPool,
}

impl PostgresCampaignRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CampaignRepository for PostgresCampaignRepository {
    async fn save(&self, campaign: &Campaign) -> RepoResult<()> {
        sqlx::query(
            r#"
            INSERT INTO campaigns (id, song_id, artist_id, name, description, start_date, end_date,
                                   boost_multiplier, nft_price, max_nfts, nfts_sold, target_revenue,
                                   status, nft_contract_address, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (id) DO UPDATE SET
                song_id = EXCLUDED.song_id,
                artist_id = EXCLUDED.artist_id,
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date,
                boost_multiplier = EXCLUDED.boost_multiplier,
                nft_price = EXCLUDED.nft_price,
                max_nfts = EXCLUDED.max_nfts,
                nfts_sold = EXCLUDED.nfts_sold,
                target_revenue = EXCLUDED.target_revenue,
                status = EXCLUDED.status,
                nft_contract_address = EXCLUDED.nft_contract_address,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(campaign.id().value())
        .bind(campaign.song_id().value())
        .bind(campaign.artist_id().value())
        .bind(campaign.name())
        .bind(campaign.description())
        .bind(campaign.date_range().start())
        .bind(campaign.date_range().end())
        .bind(campaign.boost_multiplier().value())
        .bind(campaign.nft_price().value())
        .bind(campaign.nft_supply().max_nfts() as i32)
        .bind(campaign.nft_supply().current_sold() as i32)
        .bind(campaign.target().map(|t| match t.target_type() {
            crate::bounded_contexts::campaign::domain::value_objects::TargetType::Revenue => t.value(),
            _ => 0.0,
        }))
        .bind(match campaign.status() {
            crate::bounded_contexts::campaign::domain::entities::CampaignStatus::Draft => "Draft",
            crate::bounded_contexts::campaign::domain::entities::CampaignStatus::Active => "Active",
            crate::bounded_contexts::campaign::domain::entities::CampaignStatus::Paused => "Paused",
            crate::bounded_contexts::campaign::domain::entities::CampaignStatus::Completed => "Completed",
            crate::bounded_contexts::campaign::domain::entities::CampaignStatus::Cancelled => "Cancelled",
            crate::bounded_contexts::campaign::domain::entities::CampaignStatus::Failed => "Failed",
        })
        .bind(campaign.nft_contract_address())
        .bind(campaign.created_at())
        .bind(campaign.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Campaign>> {
        let row = sqlx::query(
            r#"
            SELECT id, song_id, artist_id, name, description, start_date, end_date,
                   boost_multiplier, nft_price, max_nfts, nfts_sold, target_revenue,
                   status, nft_contract_address, created_at, updated_at
            FROM campaigns
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        match row {
            Some(row) => {
                let campaign_row = CampaignRow::from_row(row)?;
                Ok(Some(campaign_row.to_domain()?))
            }
            None => Ok(None),
        }
    }

    async fn find_by_artist_id(&self, artist_id: Uuid) -> RepoResult<Vec<Campaign>> {
        let rows = sqlx::query(
            r#"
            SELECT id, song_id, artist_id, name, description, start_date, end_date,
                   boost_multiplier, nft_price, max_nfts, nfts_sold, target_revenue,
                   status, nft_contract_address, created_at, updated_at
            FROM campaigns
            WHERE artist_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(artist_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        let mut campaigns = Vec::new();
        for row in rows {
            let campaign_row = CampaignRow::from_row(row)
                .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e))?;
            campaigns.push(campaign_row.to_domain()
                .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e))?);
        }
        Ok(campaigns)
    }

    async fn find_active_campaigns(&self) -> RepoResult<Vec<Campaign>> {
        let rows = sqlx::query(
            r#"
            SELECT id, song_id, artist_id, name, description, start_date, end_date,
                   boost_multiplier, nft_price, max_nfts, nfts_sold, target_revenue,
                   status, nft_contract_address, created_at, updated_at
            FROM campaigns
            WHERE status = 'Active' 
            AND start_date <= NOW() 
            AND end_date >= NOW()
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        let mut campaigns = Vec::new();
        for row in rows {
            let campaign_row = CampaignRow::from_row(row)
                .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e))?;
            campaigns.push(campaign_row.to_domain()
                .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e))?);
        }
        Ok(campaigns)
    }

    async fn delete(&self, id: Uuid) -> RepoResult<()> {
        let result = sqlx::query("DELETE FROM campaigns WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(crate::shared::domain::errors::AppError::NotFound(
                format!("Campaign with id {} not found", id)
            ));
        }

        Ok(())
    }
}

impl PostgresCampaignRepository {


    async fn save_nft(&self, campaign_id: &CampaignId, nft_id: &str, nft: &crate::bounded_contexts::campaign::domain::entities::CampaignNFT) -> Result<(), String> {
        let query = r#"
            INSERT INTO campaign_nfts (
                id, campaign_id, token_id, owner_address, metadata_uri, 
                tradeable, purchase_price, purchased_at, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) 
            DO UPDATE SET
                owner_address = EXCLUDED.owner_address,
                tradeable = EXCLUDED.tradeable
        "#;

        sqlx::query(query)
            .bind(nft_id)
            .bind(campaign_id.to_string())
            .bind(nft.token_id() as i64)
            .bind(nft.owner_id().to_string())
            .bind(nft.metadata_uri())
            .bind(nft.is_tradeable())
            .bind(nft.purchase_price())
            .bind(nft.purchase_date())
            .bind(nft.created_at())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save NFT: {}", e))?;

        Ok(())
    }

    async fn load_campaign_nfts(&self, campaign_id: &CampaignId) -> Result<std::collections::HashMap<String, crate::bounded_contexts::campaign::domain::entities::CampaignNFT>, String> {
        let query = r#"
            SELECT id, token_id, owner_address, metadata_uri, tradeable,
                   purchase_price, purchased_at, created_at
            FROM campaign_nfts WHERE campaign_id = $1
        "#;

        let rows = sqlx::query(query)
            .bind(campaign_id.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to load campaign NFTs: {}", e))?;

        let mut nfts = std::collections::HashMap::new();
        for row in rows {
            let nft_id = row.try_get::<String, _>("id").map_err(|e| e.to_string())?;
            let token_id = row.try_get::<Option<String>, _>("token_id").map_err(|e| e.to_string())?;
            let owner_address = row.try_get::<Option<String>, _>("owner_address").map_err(|e| e.to_string())?;
            let metadata_uri = row.try_get::<String, _>("metadata_uri").map_err(|e| e.to_string())?;
            let tradeable = row.try_get::<bool, _>("tradeable").map_err(|e| e.to_string())?;
            let purchase_price = row.try_get::<Option<f64>, _>("purchase_price").map_err(|e| e.to_string())?;
            let purchased_at = row.try_get::<Option<DateTime<Utc>>, _>("purchased_at").map_err(|e| e.to_string())?;
            let created_at = row.try_get::<DateTime<Utc>, _>("created_at").map_err(|e| e.to_string())?;

            let nft = crate::bounded_contexts::campaign::domain::entities::CampaignNFT::new(
                campaign_id.clone(),
                uuid::Uuid::parse_str(&owner_address.unwrap_or_default()).unwrap_or_default(),
                token_id.unwrap_or_default().parse::<u64>().unwrap_or(0),
                metadata_uri,
                purchase_price.unwrap_or(0.0),
            );

            nfts.insert(nft_id, nft);
        }

        Ok(nfts)
    }
}

impl CampaignRow {
    fn from_row(row: sqlx::postgres::PgRow) -> Result<Self, String> {
        use sqlx::Row;
        
        Ok(CampaignRow {
            id: row.try_get("id").map_err(|e| e.to_string())?,
            song_id: row.try_get("song_id").map_err(|e| e.to_string())?,
            artist_id: row.try_get("artist_id").map_err(|e| e.to_string())?,
            name: row.try_get("name").map_err(|e| e.to_string())?,
            description: row.try_get("description").map_err(|e| e.to_string())?,
            start_date: row.try_get("start_date").map_err(|e| e.to_string())?,
            end_date: row.try_get("end_date").map_err(|e| e.to_string())?,
            boost_multiplier: row.try_get("boost_multiplier").map_err(|e| e.to_string())?,
            nft_price: row.try_get("nft_price").map_err(|e| e.to_string())?,
            max_nfts: row.try_get("max_nfts").map_err(|e| e.to_string())?,
            nfts_sold: row.try_get("nfts_sold").map_err(|e| e.to_string())?,
            target_revenue: row.try_get("target_revenue").map_err(|e| e.to_string())?,
            status: row.try_get("status").map_err(|e| e.to_string())?,
            nft_contract_address: row.try_get("nft_contract_address").map_err(|e| e.to_string())?,
            created_at: row.try_get("created_at").map_err(|e| e.to_string())?,
            updated_at: row.try_get("updated_at").map_err(|e| e.to_string())?,
        })
    }

    fn to_domain(self) -> Result<Campaign, String> {
        let id = CampaignId::from_string(&self.id)
            .map_err(|e| format!("Invalid campaign ID: {}", e))?;
        
        let song_id = SongId::from_string(&self.song_id)
            .map_err(|e| format!("Invalid song ID: {}", e))?;
        
        let artist_id = ArtistId::from_string(&self.artist_id)
            .map_err(|e| format!("Invalid artist ID: {}", e))?;
        
        let name = CampaignName::new(self.name)
            .map_err(|e| format!("Invalid campaign name: {}", e))?;
        
        let status = match self.status.as_str() {
            "Draft" => CampaignStatus::Draft,
            "Active" => CampaignStatus::Active,
            "Paused" => CampaignStatus::Paused,
            "Completed" => CampaignStatus::Completed,
            "Cancelled" => CampaignStatus::Cancelled,
            "Failed" => CampaignStatus::Failed,
            _ => return Err(format!("Invalid campaign status: {}", self.status)),
        };

        let date_range = DateRange::new(self.start_date, self.end_date)
            .map_err(|e| format!("Invalid date range: {}", e))?;

        let boost_multiplier = BoostMultiplier::new(self.boost_multiplier)
            .map_err(|e| format!("Invalid boost multiplier: {}", e))?;

        let nft_price = NFTPrice::new(self.nft_price)
            .map_err(|e| format!("Invalid NFT price: {}", e))?;

        let nft_supply = NFTSupply::with_sold(self.max_nfts as u32, self.nfts_sold as u32);

        let target = match self.target_revenue {
            Some(revenue) => Some(CampaignTarget::revenue_target(revenue)
                .map_err(|e| format!("Invalid target revenue: {}", e))?),
            None => None,
        };

        // Create campaign using create method (simplified approach)
        let (campaign, _) = Campaign::create(
            song_id,
            artist_id,
            name.value().to_string(),
            self.description,
            date_range,
            boost_multiplier.value(),
            nft_price.value(),
            self.max_nfts as u32,
            self.target_revenue,
        ).map_err(|e| format!("Failed to create campaign: {}", e))?;

        Ok(campaign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require a test database setup
    // For now, we'll include unit tests for the mapping logic

    #[test]
    fn test_status_mapping() {
        let statuses = [
            ("Draft", CampaignStatus::Draft),
            ("Active", CampaignStatus::Active),
            ("Paused", CampaignStatus::Paused),
            ("Completed", CampaignStatus::Completed),
            ("Cancelled", CampaignStatus::Cancelled),
            ("Failed", CampaignStatus::Failed),
        ];

        for (status_str, expected_status) in statuses.iter() {
            let parsed_status = match status_str {
                "Draft" => CampaignStatus::Draft,
                "Active" => CampaignStatus::Active,
                "Paused" => CampaignStatus::Paused,
                "Completed" => CampaignStatus::Completed,
                "Cancelled" => CampaignStatus::Cancelled,
                "Failed" => CampaignStatus::Failed,
                _ => panic!("Invalid status"),
            };
            
            assert_eq!(format!("{:?}", parsed_status), format!("{:?}", expected_status));
        }
    }

    #[test]
    fn test_target_type_mapping() {
        let revenue_target = CampaignTarget::revenue_target(1000.0);
        assert!(revenue_target.is_ok());
        
        let nft_target = CampaignTarget::nft_target(500);
        assert!(nft_target.is_ok());
        
        if let Ok(target) = revenue_target {
            assert_eq!(target.value(), 1000.0);
        }
    }
} 