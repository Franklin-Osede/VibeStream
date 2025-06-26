use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;

use crate::bounded_contexts::campaign::domain::{Campaign, CampaignRepository};
use crate::shared::domain::repositories::RepoResult;

pub struct CampaignPostgresRepository {
    pool: PgPool,
}

impl CampaignPostgresRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl CampaignRepository for CampaignPostgresRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Campaign>> {
        let rec = sqlx::query!(
            r#"SELECT id, artist_id, nft_contract, start_date, end_date, multiplier, is_active
               FROM campaigns WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;

        Ok(rec.map(|r| Campaign {
            id: r.id,
            artist_id: r.artist_id,
            nft_contract: r.nft_contract,
            period: crate::bounded_contexts::campaign::domain::value_objects::DateRange::new(r.start_date, r.end_date).unwrap(),
            multiplier: r.multiplier as f64,
            is_active: r.is_active,
        }))
    }

    async fn save(&self, campaign: &Campaign) -> RepoResult<()> {
        sqlx::query!(
            r#"INSERT INTO campaigns (id, artist_id, nft_contract, start_date, end_date, multiplier, is_active)
               VALUES ($1,$2,$3,$4,$5,$6,$7) ON CONFLICT (id) DO NOTHING"#,
            campaign.id,
            campaign.artist_id,
            campaign.nft_contract,
            campaign.period.start,
            campaign.period.end,
            campaign.multiplier as f32,
            campaign.is_active,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| crate::shared::domain::errors::AppError::Infrastructure(e.to_string()))?;
        Ok(())
    }
} 