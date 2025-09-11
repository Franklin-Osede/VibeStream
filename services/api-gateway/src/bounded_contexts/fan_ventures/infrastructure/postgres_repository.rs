use crate::bounded_contexts::fan_ventures::domain::entities::{
    ArtistVenture, FanInvestment, VentureTier, VentureBenefit, VentureCategory, RiskLevel, VentureStatus, InvestmentType
};
use crate::shared::domain::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub struct PostgresFanVenturesRepository {
    pool: PgPool,
}

impl PostgresFanVenturesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =============================================================================
    // ARTIST VENTURES
    // =============================================================================

    pub async fn create_venture(&self, venture: &ArtistVenture) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO artist_ventures (
                id, artist_id, title, description, category, tags, risk_level,
                expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                funding_goal, current_funding, min_investment, max_investment, status,
                start_date, end_date, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                category = EXCLUDED.category,
                tags = EXCLUDED.tags,
                risk_level = EXCLUDED.risk_level,
                expected_return = EXCLUDED.expected_return,
                artist_rating = EXCLUDED.artist_rating,
                artist_previous_ventures = EXCLUDED.artist_previous_ventures,
                artist_success_rate = EXCLUDED.artist_success_rate,
                funding_goal = EXCLUDED.funding_goal,
                current_funding = EXCLUDED.current_funding,
                min_investment = EXCLUDED.min_investment,
                max_investment = EXCLUDED.max_investment,
                status = EXCLUDED.status,
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date,
                updated_at = EXCLUDED.updated_at"#,
            venture.id,
            venture.artist_id,
            venture.title,
            venture.description,
            venture.category.to_string(),
            serde_json::to_value(&venture.tags).map_err(|e| AppError::SerializationError(e.to_string()))?,
            venture.risk_level.to_string(),
            venture.expected_return,
            venture.artist_rating,
            venture.artist_previous_ventures,
            venture.artist_success_rate,
            venture.funding_goal,
            venture.current_funding,
            venture.min_investment,
            venture.max_investment,
            venture.status.to_string(),
            venture.start_date,
            venture.end_date,
            venture.created_at,
            venture.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_venture(&self, venture_id: Uuid) -> Result<Option<ArtistVenture>, AppError> {
        let row = sqlx::query!(
            r#"SELECT id, artist_id, title, description, category, tags, risk_level,
                      expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                      funding_goal, current_funding, min_investment, max_investment, status,
                      start_date, end_date, created_at, updated_at
               FROM artist_ventures WHERE id = $1"#,
            venture_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let tags: Vec<String> = serde_json::from_value(row.tags)
                    .map_err(|e| AppError::SerializationError(e.to_string()))?;
                
                let venture = ArtistVenture {
                    id: row.id,
                    artist_id: row.artist_id,
                    title: row.title,
                    description: row.description,
                    category: row.category.parse().unwrap_or_default(),
                    tags,
                    risk_level: row.risk_level.parse().unwrap_or_default(),
                    expected_return: row.expected_return,
                    artist_rating: row.artist_rating,
                    artist_previous_ventures: row.artist_previous_ventures,
                    artist_success_rate: row.artist_success_rate,
                    funding_goal: row.funding_goal,
                    current_funding: row.current_funding,
                    min_investment: row.min_investment,
                    max_investment: row.max_investment,
                    status: row.status.parse().unwrap_or_default(),
                    start_date: row.start_date,
                    end_date: row.end_date,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    benefits: vec![], // TODO: Load benefits separately
                };
                Ok(Some(venture))
            }
            None => Ok(None),
        }
    }

    pub async fn list_open_ventures(&self, limit: Option<i32>) -> Result<Vec<ArtistVenture>, AppError> {
        let limit = limit.unwrap_or(50) as i64;
        
        let rows = sqlx::query!(
            r#"SELECT id, artist_id, title, description, category, tags, risk_level,
                      expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                      funding_goal, current_funding, min_investment, max_investment, status,
                      start_date, end_date, created_at, updated_at
               FROM artist_ventures 
               WHERE status = 'Open'
               ORDER BY created_at DESC
               LIMIT $1"#,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut ventures = Vec::new();
        for row in rows {
            let tags: Vec<String> = serde_json::from_value(row.tags)
                .map_err(|e| AppError::SerializationError(e.to_string()))?;
            
            let venture = ArtistVenture {
                id: row.id,
                artist_id: row.artist_id,
                title: row.title,
                description: row.description,
                category: row.category.parse().unwrap_or_default(),
                tags,
                risk_level: row.risk_level.parse().unwrap_or_default(),
                expected_return: row.expected_return,
                artist_rating: row.artist_rating,
                artist_previous_ventures: row.artist_previous_ventures,
                artist_success_rate: row.artist_success_rate,
                funding_goal: row.funding_goal,
                current_funding: row.current_funding,
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                status: row.status.parse().unwrap_or_default(),
                start_date: row.start_date,
                end_date: row.end_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
                benefits: vec![], // TODO: Load benefits separately
            };
            ventures.push(venture);
        }

        Ok(ventures)
    }

    pub async fn update_venture(&self, _venture: &ArtistVenture) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn delete_venture(&self, _venture_id: Uuid) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn get_ventures_by_artist(&self, _artist_id: Uuid) -> Result<Vec<ArtistVenture>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn get_ventures_by_category(&self, _category: &str) -> Result<Vec<ArtistVenture>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn get_ventures_by_status(&self, _status: &str) -> Result<Vec<ArtistVenture>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn search_ventures(&self, _query: &str, _limit: Option<i32>) -> Result<Vec<ArtistVenture>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    /// Search ventures with filters and sorting (overloaded method)
    pub async fn search_ventures_with_filters(
        &self, 
        _filters: &serde_json::Value, 
        _sorting: &serde_json::Value, 
        _page: u32, 
        _page_size: u32
    ) -> Result<Vec<ArtistVenture>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn get_venture_count(&self) -> Result<u64, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(0)
    }

    pub async fn get_venture_revenue(&self, _venture_id: Uuid) -> Result<f64, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(0.0)
    }

    // =============================================================================
    // FAN INVESTMENTS
    // =============================================================================

    pub async fn create_fan_investment(&self, investment: &FanInvestment) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO fan_investments (
                id, fan_id, venture_id, investment_amount, investment_type, status,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                investment_amount = EXCLUDED.investment_amount,
                investment_type = EXCLUDED.investment_type,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at"#,
            investment.id,
            investment.fan_id,
            investment.venture_id,
            investment.investment_amount,
            serde_json::to_value(&investment.investment_type).map_err(|e| AppError::SerializationError(e.to_string()))?,
            investment.status.to_string(),
            investment.created_at,
            investment.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_fan_investments(&self, _fan_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn update_fan_investment(&self, _investment: &FanInvestment) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn delete_fan_investment(&self, _investment_id: Uuid) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn get_investment_by_id(&self, investment_id: Uuid) -> Result<Option<FanInvestment>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT id, fan_id, venture_id, amount, investment_date, status, created_at, updated_at
            FROM fan_investments
            WHERE id = $1
            "#,
            investment_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get investment by ID: {}", e)))?;
        
        if let Some(row) = row {
            let investment = FanInvestment::new(
                row.id,
                row.fan_id,
                row.venture_id,
                row.amount,
                InvestmentType::RevenueShare, // Default type
                row.status.into(),
            );
            Ok(Some(investment))
        } else {
        Ok(None)
        }
    }

    pub async fn get_investments_by_venture(&self, venture_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, fan_id, venture_id, amount, investment_date, status, created_at, updated_at
            FROM fan_investments
            WHERE venture_id = $1
            ORDER BY investment_date DESC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get investments by venture: {}", e)))?;
        
        let investments = rows.into_iter().map(|row| {
            FanInvestment::new(
                row.id,
                row.fan_id,
                row.venture_id,
                row.amount,
                InvestmentType::RevenueShare, // Default type
                row.status.into(),
            )
        }).collect();
        
        Ok(investments)
    }

    pub async fn get_investment_count(&self) -> Result<u64, AppError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM fan_investments"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get investment count: {}", e)))?;
        
        Ok(count.unwrap_or(0) as u64)
    }

    pub async fn get_total_invested_amount(&self) -> Result<f64, AppError> {
        let total = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(amount), 0) FROM fan_investments WHERE status = 'active'"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get total invested amount: {}", e)))?;
        
        Ok(total.unwrap_or(0.0))
    }

    // =============================================================================
    // VENTURE TIERS
    // =============================================================================

    pub async fn create_venture_tier(&self, tier: &VentureTier) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO venture_tiers (id, venture_id, tier_name, min_investment, max_investment, 
                                     description, benefits, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            tier.id,
            tier.venture_id,
            tier.tier_name,
            tier.min_investment,
            tier.max_investment,
            tier.description,
            serde_json::to_value(&tier.benefits).unwrap_or(serde_json::Value::Null),
            tier.created_at,
            tier.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create venture tier: {}", e)))?;
        
        Ok(())
    }

    pub async fn get_venture_tiers(&self, venture_id: Uuid) -> Result<Vec<VentureTier>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, venture_id, tier_name, min_investment, max_investment, 
                   description, benefits, created_at, updated_at
            FROM venture_tiers
            WHERE venture_id = $1
            ORDER BY min_investment ASC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get venture tiers: {}", e)))?;
        
        let tiers = rows.into_iter().map(|row| {
            VentureTier {
                id: row.id,
                venture_id: row.venture_id,
                tier_name: row.tier_name,
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                description: row.description,
                benefits: serde_json::from_value(row.benefits.unwrap_or(serde_json::Value::Null))
                    .unwrap_or_default(),
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();
        
        Ok(tiers)
    }

    pub async fn update_venture_tier(&self, _tier: &VentureTier) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn delete_venture_tier(&self, _tier_id: Uuid) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn get_tier_by_id(&self, tier_id: Uuid) -> Result<Option<VentureTier>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT id, venture_id, tier_name, min_investment, max_investment, 
                   description, benefits, created_at, updated_at
            FROM venture_tiers
            WHERE id = $1
            "#,
            tier_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get tier by ID: {}", e)))?;
        
        if let Some(row) = row {
            let tier = VentureTier {
                id: row.id,
                venture_id: row.venture_id,
                tier_name: row.tier_name,
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                description: row.description,
                benefits: serde_json::from_value(row.benefits.unwrap_or(serde_json::Value::Null))
                    .unwrap_or_default(),
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            Ok(Some(tier))
        } else {
        Ok(None)
        }
    }

    // =============================================================================
    // VENTURE BENEFITS
    // =============================================================================

    pub async fn create_venture_benefit(&self, _benefit: &VentureBenefit) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn get_venture_benefits(&self, _venture_id: Uuid) -> Result<Vec<VentureBenefit>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn update_venture_benefit(&self, _benefit: &VentureBenefit) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn delete_venture_benefit(&self, _benefit_id: Uuid) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn get_benefit_by_id(&self, _benefit_id: Uuid) -> Result<Option<VentureBenefit>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(None)
    }

    // =============================================================================
    // ANALYTICS & REPORTING
    // =============================================================================

    pub async fn get_venture_analytics(&self, _venture_id: Uuid) -> Result<serde_json::Value, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(serde_json::json!({}))
    }

    pub async fn get_fan_investment_history(&self, _fan_id: Uuid, _limit: Option<u32>) -> Result<Vec<FanInvestment>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn get_top_performing_ventures(&self, _limit: Option<u32>) -> Result<Vec<ArtistVenture>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    pub async fn get_venture_performance_metrics(&self, _venture_id: Uuid) -> Result<serde_json::Value, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(serde_json::json!({}))
    }

    // =============================================================================
    // MÉTODOS FALTANTES PARA COMPATIBILIDAD
    // =============================================================================

    /// Create revenue distribution
    pub async fn create_revenue_distribution(&self, _distribution: &crate::bounded_contexts::fan_ventures::domain::entities::RevenueDistribution) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    /// Get venture distributions
    pub async fn get_venture_distributions(&self, _venture_id: Uuid) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::RevenueDistribution>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    /// Get venture tier (singular) - alias for get_tier_by_id
    pub async fn get_venture_tier(&self, tier_id: Uuid) -> Result<Option<VentureTier>, AppError> {
        self.get_tier_by_id(tier_id).await
    }

    /// List ventures by artist (alias for get_ventures_by_artist)
    pub async fn list_ventures_by_artist(&self, artist_id: Uuid) -> Result<Vec<ArtistVenture>, AppError> {
        self.get_ventures_by_artist(artist_id).await
    }

    /// Create benefit delivery
    pub async fn create_benefit_delivery(&self, _delivery: &crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    /// Get benefit delivery
    pub async fn get_benefit_delivery(&self, _delivery_id: Uuid) -> Result<Option<crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(None)
    }

    /// Update benefit delivery
    pub async fn update_benefit_delivery(&self, _delivery_id: Uuid, _delivery: &crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    /// Get fan deliveries
    pub async fn get_fan_deliveries(&self, _fan_id: Uuid) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    /// Get venture deliveries
    pub async fn get_venture_deliveries(&self, _venture_id: Uuid) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    /// Get venture recommendations
    pub async fn get_venture_recommendations(&self, _fan_id: Uuid, _limit: u32) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::VentureRecommendation>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    /// Save fan preferences
    pub async fn save_fan_preferences(&self, _preferences: &crate::bounded_contexts::fan_ventures::domain::entities::FanPreferences) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    /// Get fan preferences
    pub async fn get_fan_preferences(&self, _fan_id: Uuid) -> Result<Option<crate::bounded_contexts::fan_ventures::domain::entities::FanPreferences>, AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(None)
    }
} 