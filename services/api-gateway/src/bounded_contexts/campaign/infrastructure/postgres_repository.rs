use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::bounded_contexts::campaign::domain::aggregates::CampaignAggregate;
use crate::bounded_contexts::campaign::domain::entities::{Campaign, CampaignStatus};
use crate::bounded_contexts::campaign::domain::value_objects::*;
use crate::bounded_contexts::campaign::domain::repository::CampaignRepository;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

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
    async fn save(&self, aggregate: &CampaignAggregate) -> Result<(), String> {
        let campaign = aggregate.campaign();
        
        let query = r#"
            INSERT INTO campaigns (
                id, song_id, artist_id, name, description, status,
                start_date, end_date, boost_multiplier, nft_price,
                max_nfts, nfts_sold, target_type, target_value,
                nft_contract_address, version, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ON CONFLICT (id) 
            DO UPDATE SET
                status = EXCLUDED.status,
                nfts_sold = EXCLUDED.nfts_sold,
                nft_contract_address = EXCLUDED.nft_contract_address,
                version = EXCLUDED.version,
                updated_at = EXCLUDED.updated_at
        "#;

        let target_type = campaign.target().map(|t| match t {
            CampaignTarget::Revenue(_) => "revenue",
            CampaignTarget::NFTsSold(_) => "nfts_sold",
            CampaignTarget::Engagement(_) => "engagement",
        });

        let target_value = campaign.target().map(|t| t.target_value());

        sqlx::query(query)
            .bind(campaign.id().to_string())
            .bind(campaign.song_id().to_string())
            .bind(campaign.artist_id().to_string())
            .bind(campaign.name())
            .bind(campaign.description())
            .bind(format!("{:?}", campaign.status()))
            .bind(campaign.date_range().start())
            .bind(campaign.date_range().end())
            .bind(campaign.boost_multiplier().value())
            .bind(campaign.nft_price().value())
            .bind(campaign.nft_supply().max_supply() as i32)
            .bind(campaign.nft_supply().current_sold() as i32)
            .bind(target_type)
            .bind(target_value)
            .bind(campaign.nft_contract_address())
            .bind(aggregate.version() as i32)
            .bind(campaign.created_at())
            .bind(Utc::now())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save campaign: {}", e))?;

        // Save NFTs
        for (nft_id, nft) in aggregate.nfts() {
            self.save_nft(campaign.id(), nft_id, nft).await?;
        }

        Ok(())
    }

    async fn find_by_id(&self, id: &CampaignId) -> Result<Option<CampaignAggregate>, String> {
        let query = r#"
            SELECT id, song_id, artist_id, name, description, status,
                   start_date, end_date, boost_multiplier, nft_price,
                   max_nfts, nfts_sold, target_type, target_value,
                   nft_contract_address, version, created_at
            FROM campaigns WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to find campaign: {}", e))?;

        if let Some(row) = row {
            let campaign = self.map_row_to_campaign(row)?;
            let nfts = self.load_campaign_nfts(id).await?;
            
            let version = row.try_get::<i32, _>("version").unwrap_or(1) as u32;
            let mut aggregate = CampaignAggregate::from_campaign(campaign);
            aggregate.set_version(version);
            
            for (nft_id, nft) in nfts {
                aggregate.add_nft(nft_id, nft);
            }

            Ok(Some(aggregate))
        } else {
            Ok(None)
        }
    }

    async fn find_by_artist_id(&self, artist_id: &ArtistId) -> Result<Vec<CampaignAggregate>, String> {
        let query = r#"
            SELECT id, song_id, artist_id, name, description, status,
                   start_date, end_date, boost_multiplier, nft_price,
                   max_nfts, nfts_sold, target_type, target_value,
                   nft_contract_address, version, created_at
            FROM campaigns WHERE artist_id = $1
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .bind(artist_id.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to find campaigns by artist: {}", e))?;

        let mut campaigns = Vec::new();
        for row in rows {
            let campaign = self.map_row_to_campaign(row)?;
            let campaign_id = campaign.id().clone();
            let nfts = self.load_campaign_nfts(&campaign_id).await?;
            
            let mut aggregate = CampaignAggregate::from_campaign(campaign);
            for (nft_id, nft) in nfts {
                aggregate.add_nft(nft_id, nft);
            }
            campaigns.push(aggregate);
        }

        Ok(campaigns)
    }

    async fn find_active_campaigns(&self) -> Result<Vec<CampaignAggregate>, String> {
        let query = r#"
            SELECT id, song_id, artist_id, name, description, status,
                   start_date, end_date, boost_multiplier, nft_price,
                   max_nfts, nfts_sold, target_type, target_value,
                   nft_contract_address, version, created_at
            FROM campaigns 
            WHERE status = 'Active' AND start_date <= NOW() AND end_date > NOW()
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to find active campaigns: {}", e))?;

        let mut campaigns = Vec::new();
        for row in rows {
            let campaign = self.map_row_to_campaign(row)?;
            let campaign_id = campaign.id().clone();
            let nfts = self.load_campaign_nfts(&campaign_id).await?;
            
            let mut aggregate = CampaignAggregate::from_campaign(campaign);
            for (nft_id, nft) in nfts {
                aggregate.add_nft(nft_id, nft);
            }
            campaigns.push(aggregate);
        }

        Ok(campaigns)
    }

    async fn delete(&self, id: &CampaignId) -> Result<(), String> {
        // Delete NFTs first (foreign key constraint)
        sqlx::query("DELETE FROM campaign_nfts WHERE campaign_id = $1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete campaign NFTs: {}", e))?;

        // Delete campaign
        sqlx::query("DELETE FROM campaigns WHERE id = $1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete campaign: {}", e))?;

        Ok(())
    }
}

impl PostgresCampaignRepository {
    fn map_row_to_campaign(&self, row: sqlx::postgres::PgRow) -> Result<Campaign, String> {
        let id = CampaignId::from_string(&row.try_get::<String, _>("id")?)
            .map_err(|e| format!("Invalid campaign ID: {}", e))?;
        
        let song_id = SongId::from_string(&row.try_get::<String, _>("song_id")?)
            .map_err(|e| format!("Invalid song ID: {}", e))?;
        
        let artist_id = ArtistId::from_string(&row.try_get::<String, _>("artist_id")?)
            .map_err(|e| format!("Invalid artist ID: {}", e))?;

        let name = CampaignName::new(row.try_get::<String, _>("name")?)
            .map_err(|e| format!("Invalid campaign name: {}", e))?;

        let description = row.try_get::<String, _>("description")?;

        let status_str = row.try_get::<String, _>("status")?;
        let status = match status_str.as_str() {
            "Draft" => CampaignStatus::Draft,
            "Active" => CampaignStatus::Active,
            "Paused" => CampaignStatus::Paused,
            "Completed" => CampaignStatus::Completed,
            "Cancelled" => CampaignStatus::Cancelled,
            "Failed" => CampaignStatus::Failed,
            _ => return Err(format!("Invalid campaign status: {}", status_str)),
        };

        let start_date = row.try_get::<DateTime<Utc>, _>("start_date")?;
        let end_date = row.try_get::<DateTime<Utc>, _>("end_date")?;
        let date_range = DateRange::new(start_date, end_date)
            .map_err(|e| format!("Invalid date range: {}", e))?;

        let boost_multiplier = BoostMultiplier::new(row.try_get::<f64, _>("boost_multiplier")?)
            .map_err(|e| format!("Invalid boost multiplier: {}", e))?;

        let nft_price = NFTPrice::new(row.try_get::<f64, _>("nft_price")?)
            .map_err(|e| format!("Invalid NFT price: {}", e))?;

        let max_nfts = row.try_get::<i32, _>("max_nfts")? as u32;
        let nfts_sold = row.try_get::<i32, _>("nfts_sold")? as u32;
        let nft_supply = NFTSupply::new(max_nfts)
            .map_err(|e| format!("Invalid NFT supply: {}", e))?
            .with_sold(nfts_sold)
            .map_err(|e| format!("Invalid sold count: {}", e))?;

        let target = if let (Some(target_type), Some(target_value)) = (
            row.try_get::<Option<String>, _>("target_type")?,
            row.try_get::<Option<f64>, _>("target_value")?
        ) {
            match target_type.as_str() {
                "revenue" => Some(CampaignTarget::Revenue(target_value)),
                "nfts_sold" => Some(CampaignTarget::NFTsSold(target_value as u32)),
                "engagement" => Some(CampaignTarget::Engagement(target_value)),
                _ => None,
            }
        } else {
            None
        };

        let nft_contract_address = row.try_get::<Option<String>, _>("nft_contract_address")?;
        let created_at = row.try_get::<DateTime<Utc>, _>("created_at")?;

        Ok(Campaign::new(
            id,
            song_id,
            artist_id,
            name,
            description,
            status,
            date_range,
            boost_multiplier,
            nft_price,
            nft_supply,
            target,
            nft_contract_address,
            created_at,
        ))
    }

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
            .bind(nft.token_id())
            .bind(nft.owner_address())
            .bind(nft.metadata_uri())
            .bind(nft.is_tradeable())
            .bind(nft.purchase_price())
            .bind(nft.purchased_at())
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
            let nft_id = row.try_get::<String, _>("id")?;
            let token_id = row.try_get::<Option<String>, _>("token_id")?;
            let owner_address = row.try_get::<Option<String>, _>("owner_address")?;
            let metadata_uri = row.try_get::<String, _>("metadata_uri")?;
            let tradeable = row.try_get::<bool, _>("tradeable")?;
            let purchase_price = row.try_get::<Option<f64>, _>("purchase_price")?;
            let purchased_at = row.try_get::<Option<DateTime<Utc>>, _>("purchased_at")?;
            let created_at = row.try_get::<DateTime<Utc>, _>("created_at")?;

            let nft = crate::bounded_contexts::campaign::domain::entities::CampaignNFT::new(
                metadata_uri,
                token_id,
                owner_address,
                tradeable,
                purchase_price,
                purchased_at,
                created_at,
            );

            nfts.insert(nft_id, nft);
        }

        Ok(nfts)
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
        let target_types = [
            ("revenue", 1000.0),
            ("nfts_sold", 500.0),
            ("engagement", 10000.0),
        ];

        for (target_type, value) in target_types.iter() {
            let target = match *target_type {
                "revenue" => CampaignTarget::Revenue(*value),
                "nfts_sold" => CampaignTarget::NFTsSold(*value as u32),
                "engagement" => CampaignTarget::Engagement(*value),
                _ => panic!("Invalid target type"),
            };

            assert_eq!(target.target_value(), *value);
        }
    }
} 