use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use num_traits::FromPrimitive;
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::fan_ventures::domain::entities::{
    FanInvestment, ArtistVenture, RevenueDistribution, VentureBenefit,
    InvestmentStatus, VentureStatus, InvestmentType, BenefitType, DeliveryMethod,
    VentureTier, BenefitDelivery, DeliveryStatus, TrackingInfo,
    VentureDiscovery, ExplorationFilters, ExplorationSorting, VentureRecommendation,
    FanPreferences, VentureCategory, RiskLevel
};

// =============================================================================
// FAN VENTURES - POSTGRES REPOSITORY
// =============================================================================

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
            r#"
            INSERT INTO artist_ventures (
                id, artist_id, title, description, category, tags, risk_level,
                expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                funding_goal, current_funding, min_investment, max_investment, status,
                start_date, end_date, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            "#,
            venture.id,
            venture.artist_id,
            venture.title,
            venture.description,
            venture.category.to_string(),
            &venture.tags,
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

        // Insert benefits
        for benefit in &venture.benefits {
            self.create_venture_benefit(benefit).await?;
        }

        Ok(())
    }

    pub async fn get_venture(&self, venture_id: Uuid) -> Result<Option<ArtistVenture>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM artist_ventures WHERE id = $1
            "#,
            venture_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let benefits = self.get_venture_benefits(venture_id).await?;
                
                Ok(Some(ArtistVenture {
                    id: row.id,
                    artist_id: row.artist_id,
                    title: row.title,
                    description: row.description,
                    category: VentureCategory::from_string(&row.category.unwrap_or_else(|| "other".to_string()))?,
                    tags: row.tags.unwrap_or_default(),
                    risk_level: RiskLevel::from_string(&row.risk_level.unwrap_or_else(|| "medium".to_string()))?,
                    expected_return: row.expected_return.unwrap_or(0.0),
                    artist_rating: row.artist_rating.unwrap_or(0.0),
                    artist_previous_ventures: row.artist_previous_ventures.unwrap_or(0),
                    artist_success_rate: row.artist_success_rate.unwrap_or(0.0),
                    funding_goal: row.funding_goal,
                    current_funding: row.current_funding.unwrap_or(0.0),
                    min_investment: row.min_investment,
                    max_investment: row.max_investment,
                    status: VentureStatus::from_string(&row.status.unwrap_or_else(|| "draft".to_string()))?,
                    start_date: row.start_date,
                    end_date: row.end_date,
                    created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                    updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
                    benefits,
                }))
            },
            None => Ok(None),
        }
    }

    pub async fn list_open_ventures(&self, limit: Option<i32>) -> Result<Vec<ArtistVenture>, AppError> {
        let limit = limit.unwrap_or(50);
        
        let rows = sqlx::query!(
            r#"
            SELECT * FROM artist_ventures 
            WHERE status = 'active' AND (end_date IS NULL OR end_date > NOW())
            ORDER BY created_at DESC 
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut ventures = Vec::new();
        for row in rows {
            let benefits = self.get_venture_benefits(row.id).await?;
            
            ventures.push(ArtistVenture {
                id: row.id,
                artist_id: row.artist_id,
                title: row.title,
                description: row.description,
                category: VentureCategory::from_string(&row.category.unwrap_or_else(|| "other".to_string())).unwrap_or(VentureCategory::Other),
                tags: row.tags.unwrap_or_default(),
                risk_level: RiskLevel::from_string(&row.risk_level.unwrap_or_else(|| "medium".to_string())).unwrap_or(RiskLevel::Medium),
                expected_return: row.expected_return.unwrap_or(0.0),
                artist_rating: row.artist_rating.unwrap_or(0.0),
                artist_previous_ventures: row.artist_previous_ventures.unwrap_or(0),
                artist_success_rate: row.artist_success_rate.unwrap_or(0.0),
                funding_goal: row.funding_goal,
                current_funding: row.current_funding.unwrap_or(0.0),
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                status: VentureStatus::from_string(&row.status.unwrap_or_else(|| "draft".to_string())).unwrap_or(VentureStatus::Draft),
                start_date: row.start_date,
                end_date: row.end_date,
                created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
                benefits,
            });
        }

        Ok(ventures)
    }

    // =============================================================================
    // FAN INVESTMENTS
    // =============================================================================

    pub async fn create_fan_investment(&self, investment: &FanInvestment) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO fan_investments (
                id, fan_id, venture_id, investment_amount, investment_type, status, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            investment.id,
            investment.fan_id,
            investment.venture_id,
            investment.investment_amount,
            investment.investment_type.to_string(),
            investment.status.to_string(),
            investment.created_at,
            investment.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_fan_investments(&self, fan_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM fan_investments WHERE fan_id = $1
            "#,
            fan_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut investments = Vec::new();
        for row in rows {
            investments.push(FanInvestment {
                id: row.id,
                fan_id: row.fan_id,
                venture_id: row.venture_id,
                investment_amount: row.investment_amount,
                investment_type: InvestmentType::from_string(&row.investment_type.unwrap_or_else(|| "equity".to_string())).unwrap_or(InvestmentType::Custom("Unknown".to_string())),
                status: InvestmentStatus::from_string(&row.status.unwrap_or_else(|| "pending".to_string())).unwrap_or(InvestmentStatus::Pending),
                created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
            });
        }

        Ok(investments)
    }

    // =============================================================================
    // VENTURE TIERS
    // =============================================================================

    pub async fn create_venture_tier(&self, tier: &VentureTier) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO venture_tiers (
                id, venture_id, name, min_investment, max_investment, description, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            tier.id,
            tier.venture_id,
            tier.name,
            tier.min_investment,
            tier.max_investment,
            tier.description,
            tier.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Insert benefits for this tier
        for benefit in &tier.benefits {
            self.create_venture_benefit(benefit).await?;
        }

        Ok(())
    }

    pub async fn get_venture_tiers(&self, venture_id: Uuid) -> Result<Vec<VentureTier>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM venture_tiers WHERE venture_id = $1
            ORDER BY min_investment ASC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut tiers = Vec::new();
        for row in rows {
            let benefits = self.get_venture_benefits_by_tier(row.id).await?;
            
            tiers.push(VentureTier {
                id: row.id,
                venture_id: row.venture_id,
                name: row.name,
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                description: row.description,
                created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                benefits,
            });
        }

        Ok(tiers)
    }

    pub async fn get_venture_tier(&self, tier_id: Uuid) -> Result<Option<VentureTier>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM venture_tiers WHERE id = $1
            "#,
            tier_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let benefits = self.get_venture_benefits_by_tier(row.id).await?;
                
                Ok(Some(VentureTier {
                    id: row.id,
                    venture_id: row.venture_id,
                    name: row.name,
                    min_investment: row.min_investment,
                    max_investment: row.max_investment,
                    description: row.description,
                    created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                    benefits,
                }))
            },
            None => Ok(None),
        }
    }

    pub async fn update_venture_tier(&self, tier: &VentureTier) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE venture_tiers SET
                name = $2, min_investment = $3, max_investment = $4, 
                description = $5
            WHERE id = $1
            "#,
            tier.id,
            tier.name,
            tier.min_investment,
            tier.max_investment,
            tier.description
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_venture_tier(&self, tier_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM venture_tiers WHERE id = $1
            "#,
            tier_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // =============================================================================
    // VENTURE BENEFITS
    // =============================================================================

    pub async fn create_venture_benefit(&self, benefit: &VentureBenefit) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO venture_benefits (
                id, venture_id, tier_id, title, description, benefit_type, 
                delivery_method, estimated_delivery_date, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            benefit.id,
            benefit.venture_id,
            benefit.tier_id,
            benefit.title,
            benefit.description,
            benefit.benefit_type.to_string(),
            benefit.delivery_method.to_string(),
            benefit.estimated_delivery_date,
            benefit.created_at.unwrap_or_else(|| Utc::now()),
            benefit.updated_at.unwrap_or_else(|| Utc::now())
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_venture_benefits(&self, venture_id: Uuid) -> Result<Vec<VentureBenefit>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM venture_benefits WHERE venture_id = $1
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut benefits = Vec::new();
        for row in rows {
            benefits.push(VentureBenefit {
                id: row.id,
                venture_id: row.venture_id,
                tier_id: row.tier_id,
                title: row.title,
                description: row.description,
                benefit_type: BenefitType::from_string(&row.benefit_type).unwrap_or(BenefitType::Custom("Unknown".to_string())),
                delivery_method: DeliveryMethod::from_string(&row.delivery_method.unwrap_or_else(|| "manual".to_string())).unwrap_or(DeliveryMethod::Manual),
                estimated_delivery_date: row.estimated_delivery_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }

        Ok(benefits)
    }

    pub async fn get_venture_benefits_by_tier(&self, tier_id: Uuid) -> Result<Vec<VentureBenefit>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM venture_benefits WHERE tier_id = $1
            ORDER BY created_at ASC
            "#,
            tier_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let benefits = rows.into_iter().map(|row| VentureBenefit {
            id: row.id,
            venture_id: row.venture_id,
            tier_id: row.tier_id,
            title: row.title,
            description: row.description,
            benefit_type: BenefitType::from_string(&row.benefit_type).unwrap_or(BenefitType::Custom("Unknown".to_string())),
            delivery_method: DeliveryMethod::from_string(&row.delivery_method.unwrap_or_else(|| "manual".to_string())).unwrap_or(DeliveryMethod::Manual),
            estimated_delivery_date: row.estimated_delivery_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(benefits)
    }

    // =============================================================================
    // REVENUE DISTRIBUTIONS
    // =============================================================================

    pub async fn create_revenue_distribution(&self, _distribution: &RevenueDistribution) -> Result<(), AppError> {
        // TODO: Uncomment when revenue_distributions table is created
        // sqlx::query!(
        //     r#"
        //     INSERT INTO revenue_distributions (
        //         id, venture_id, total_revenue, artist_share, 
        //         fan_share, platform_fee, distributed_at, period_start, period_end
        //     ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        //     "#,
        //     distribution.id,
        //     distribution.venture_id,
        //     distribution.total_revenue,
        //     distribution.artist_share,
        //     distribution.fan_share,
        //     distribution.platform_fee,
        //     distribution.distributed_at,
        //     distribution.period_start,
        //     distribution.period_end
        // )
        // .execute(&self.pool)
        // .await
        // .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_venture_distributions(&self, _venture_id: Uuid) -> Result<Vec<RevenueDistribution>, AppError> {
        // TODO: Uncomment when revenue_distributions table is created
        // let rows = sqlx::query!(
        //     r#"
        //     SELECT * FROM revenue_distributions 
        //     WHERE venture_id = $1
        //     ORDER BY distributed_at DESC
        //     "#,
        //     venture_id
        // )
        // .fetch_all(&self.pool)
        // .await
        // .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // let distributions = rows.into_iter().map(|row| RevenueDistribution {
        //     id: row.id,
        //     venture_id: row.venture_id,
        //     total_revenue: row.total_revenue,
        //     artist_share: row.artist_share,
        //     fan_share: row.fan_share,
        //     platform_fee: row.platform_fee,
        //     distributed_at: row.distributed_at,
        //     period_start: row.period_start,
        //     period_end: row.period_end,
        // }).collect();

        Ok(Vec::new())
    }

    /// Get all ventures for a specific artist
    pub async fn list_ventures_by_artist(&self, artist_id: Uuid) -> Result<Vec<ArtistVenture>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM artist_ventures WHERE artist_id = $1
            "#,
            artist_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut ventures = Vec::new();
        for row in rows {
            let benefits = self.get_venture_benefits(row.id).await?;
            
            ventures.push(ArtistVenture {
                id: row.id,
                artist_id: row.artist_id,
                title: row.title,
                description: row.description,
                category: VentureCategory::from_string(&row.category.unwrap_or_else(|| "other".to_string())).unwrap_or(VentureCategory::Other),
                tags: row.tags.unwrap_or_default(),
                risk_level: RiskLevel::from_string(&row.risk_level.unwrap_or_else(|| "medium".to_string())).unwrap_or(RiskLevel::Medium),
                expected_return: row.expected_return.unwrap_or(0.0),
                artist_rating: row.artist_rating.unwrap_or(0.0),
                artist_previous_ventures: row.artist_previous_ventures.unwrap_or(0),
                artist_success_rate: row.artist_success_rate.unwrap_or(0.0),
                funding_goal: row.funding_goal,
                current_funding: row.current_funding.unwrap_or(0.0),
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                status: VentureStatus::from_string(&row.status.unwrap_or_else(|| "draft".to_string())).unwrap_or(VentureStatus::Draft),
                start_date: row.start_date,
                end_date: row.end_date,
                created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
                benefits,
            });
        }

        Ok(ventures)
    }

    // =============================================================================
    // BENEFIT DELIVERY METHODS
    // =============================================================================

    /// Create a new benefit delivery
    pub async fn create_benefit_delivery(&self, delivery: &BenefitDelivery) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO benefit_deliveries (
                id, benefit_id, venture_id, fan_id, tier_id, delivery_status, delivery_method,
                delivery_date, tracking_number, carrier, estimated_delivery, actual_delivery,
                delivery_notes, notes, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
            delivery.id,
            delivery.benefit_id,
            delivery.venture_id,
            delivery.fan_id,
            delivery.tier_id,
            delivery.delivery_status.to_string(),
            delivery.delivery_method.to_string(),
            delivery.delivery_date,
            delivery.tracking_info.as_ref().and_then(|t| t.tracking_number.clone()),
            delivery.tracking_info.as_ref().and_then(|t| t.carrier.clone()),
            delivery.tracking_info.as_ref().and_then(|t| t.estimated_delivery),
            delivery.tracking_info.as_ref().and_then(|t| t.actual_delivery),
            delivery.tracking_info.as_ref().and_then(|t| t.delivery_notes.clone()),
            delivery.notes,
            delivery.created_at,
            delivery.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Get benefit delivery by ID
    pub async fn get_benefit_delivery(&self, delivery_id: Uuid) -> Result<Option<BenefitDelivery>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM benefit_deliveries WHERE id = $1
            "#,
            delivery_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let tracking_info = if row.tracking_number.is_some() || row.carrier.is_some() {
                    Some(TrackingInfo {
                        tracking_number: row.tracking_number,
                        carrier: row.carrier,
                        estimated_delivery: row.estimated_delivery,
                        actual_delivery: row.actual_delivery,
                        delivery_notes: row.delivery_notes,
                    })
                } else {
                    None
                };

                Ok(Some(BenefitDelivery {
                    id: row.id,
                    benefit_id: row.benefit_id,
                    venture_id: row.venture_id,
                    fan_id: row.fan_id,
                    tier_id: row.tier_id,
                    delivery_status: DeliveryStatus::from_string(&row.delivery_status).unwrap_or(DeliveryStatus::Pending),
                    delivery_method: DeliveryMethod::from_string(&row.delivery_method).unwrap_or(DeliveryMethod::Manual),
                    delivery_date: row.delivery_date,
                    tracking_info,
                    notes: row.notes,
                    created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                    updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
                }))
            }
            None => Ok(None)
        }
    }

    /// Update benefit delivery
    pub async fn update_benefit_delivery(&self, delivery_id: Uuid, delivery: &BenefitDelivery) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE benefit_deliveries SET
                delivery_status = $1,
                delivery_method = $2,
                delivery_date = $3,
                tracking_number = $4,
                carrier = $5,
                estimated_delivery = $6,
                actual_delivery = $7,
                delivery_notes = $8,
                notes = $9,
                updated_at = $10
            WHERE id = $11
            "#,
            delivery.delivery_status.to_string(),
            delivery.delivery_method.to_string(),
            delivery.delivery_date,
            delivery.tracking_info.as_ref().and_then(|t| t.tracking_number.clone()),
            delivery.tracking_info.as_ref().and_then(|t| t.carrier.clone()),
            delivery.tracking_info.as_ref().and_then(|t| t.estimated_delivery),
            delivery.tracking_info.as_ref().and_then(|t| t.actual_delivery),
            delivery.tracking_info.as_ref().and_then(|t| t.delivery_notes.clone()),
            delivery.notes,
            Utc::now(),
            delivery_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Get deliveries for a specific fan
    pub async fn get_fan_deliveries(&self, fan_id: Uuid) -> Result<Vec<BenefitDelivery>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM benefit_deliveries WHERE fan_id = $1
            ORDER BY created_at DESC
            "#,
            fan_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let deliveries = rows.into_iter().map(|row| {
            let tracking_info = if row.tracking_number.is_some() || row.carrier.is_some() {
                Some(TrackingInfo {
                    tracking_number: row.tracking_number,
                    carrier: row.carrier,
                    estimated_delivery: row.estimated_delivery,
                    actual_delivery: row.actual_delivery,
                    delivery_notes: row.delivery_notes,
                })
            } else {
                None
            };

            BenefitDelivery {
                id: row.id,
                benefit_id: row.benefit_id,
                venture_id: row.venture_id,
                fan_id: row.fan_id,
                tier_id: row.tier_id,
                delivery_status: DeliveryStatus::from_string(&row.delivery_status).unwrap_or(DeliveryStatus::Pending),
                delivery_method: DeliveryMethod::from_string(&row.delivery_method).unwrap_or(DeliveryMethod::Manual),
                delivery_date: row.delivery_date,
                tracking_info,
                notes: row.notes,
                created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
            }
        }).collect();

        Ok(deliveries)
    }

    /// Get deliveries for a specific venture
    pub async fn get_venture_deliveries(&self, venture_id: Uuid) -> Result<Vec<BenefitDelivery>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM benefit_deliveries WHERE venture_id = $1
            ORDER BY created_at DESC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let deliveries = rows.into_iter().map(|row| {
            let tracking_info = if row.tracking_number.is_some() || row.carrier.is_some() {
                Some(TrackingInfo {
                    tracking_number: row.tracking_number,
                    carrier: row.carrier,
                    estimated_delivery: row.estimated_delivery,
                    actual_delivery: row.actual_delivery,
                    delivery_notes: row.delivery_notes,
                })
            } else {
                None
            };

            BenefitDelivery {
                id: row.id,
                benefit_id: row.benefit_id,
                venture_id: row.venture_id,
                fan_id: row.fan_id,
                tier_id: row.tier_id,
                delivery_status: DeliveryStatus::from_string(&row.delivery_status).unwrap_or(DeliveryStatus::Pending),
                delivery_method: DeliveryMethod::from_string(&row.delivery_method).unwrap_or(DeliveryMethod::Manual),
                delivery_date: row.delivery_date,
                tracking_info,
                notes: row.notes,
                created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
            }
        }).collect();

        Ok(deliveries)
    }

    // =============================================================================
    // VENTURE EXPLORATION METHODS
    // =============================================================================

    /// Search ventures with filters and sorting
    pub async fn search_ventures(
        &self,
        filters: &ExplorationFilters,
        sorting: &ExplorationSorting,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<VentureDiscovery>, AppError> {
        let offset = (page - 1) * page_size;
        
        let mut query = String::from(
            r#"
            SELECT 
                av.id, av.artist_id, av.title, av.description, av.investment_type,
                av.min_investment, av.max_investment, av.total_goal, av.current_amount,
                av.max_investors, av.current_investors, av.status, av.expires_at,
                av.created_at, av.category, av.tags, av.risk_level, av.expected_return,
                av.artist_rating, av.artist_previous_ventures, av.artist_success_rate
            FROM artist_ventures av
            WHERE av.status = 'open'
            "#
        );

        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
        let mut param_count = 0;

        // Apply filters
        if let Some(categories) = &filters.categories {
            param_count += 1;
            conditions.push(format!("av.category = ANY(${}::text[])", param_count));
            let category_strings: Vec<String> = categories.iter().map(|c| c.to_string()).collect();
            params.push(Box::new(category_strings));
        }

        if let Some(investment_types) = &filters.investment_types {
            param_count += 1;
            conditions.push(format!("av.investment_type = ANY(${}::text[])", param_count));
            let type_strings: Vec<String> = investment_types.iter().map(|t| t.to_string()).collect();
            params.push(Box::new(type_strings));
        }

        if let Some(risk_levels) = &filters.risk_levels {
            param_count += 1;
            conditions.push(format!("av.risk_level = ANY(${}::text[])", param_count));
            let risk_strings: Vec<String> = risk_levels.iter().map(|r| r.to_string()).collect();
            params.push(Box::new(risk_strings));
        }

        if let Some(min_investment) = filters.min_investment {
            param_count += 1;
            conditions.push(format!("av.min_investment >= ${}", param_count));
            params.push(Box::new(min_investment));
        }

        if let Some(max_investment) = filters.max_investment {
            param_count += 1;
            conditions.push(format!("av.min_investment <= ${}", param_count));
            params.push(Box::new(max_investment));
        }

        if let Some(min_funding_progress) = filters.min_funding_progress {
            param_count += 1;
            conditions.push(format!("(av.current_amount / av.total_goal) * 100 >= ${}", param_count));
            params.push(Box::new(min_funding_progress));
        }

        if let Some(max_funding_progress) = filters.max_funding_progress {
            param_count += 1;
            conditions.push(format!("(av.current_amount / av.total_goal) * 100 <= ${}", param_count));
            params.push(Box::new(max_funding_progress));
        }

        if let Some(min_artist_rating) = filters.min_artist_rating {
            param_count += 1;
            conditions.push(format!("av.artist_rating >= ${}", param_count));
            params.push(Box::new(min_artist_rating));
        }

        if let Some(search_query) = &filters.search_query {
            param_count += 1;
            conditions.push(format!("(av.title ILIKE ${} OR av.description ILIKE ${})", param_count, param_count));
            let search_pattern = format!("%{}%", search_query);
            params.push(Box::new(search_pattern.clone()));
        }

        if let Some(expires_within_days) = filters.expires_within_days {
            param_count += 1;
            conditions.push(format!("av.expires_at <= NOW() + INTERVAL '{} days'", expires_within_days));
        }

        // Add conditions to query
        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        // Add sorting
        query.push_str(" ORDER BY ");
        match sorting {
            ExplorationSorting::Newest => query.push_str("av.created_at DESC"),
            ExplorationSorting::Oldest => query.push_str("av.created_at ASC"),
            ExplorationSorting::FundingProgress => query.push_str("(av.current_amount / av.total_goal) DESC"),
            ExplorationSorting::DaysRemaining => query.push_str("av.expires_at ASC"),
            ExplorationSorting::MinInvestment => query.push_str("av.min_investment ASC"),
            ExplorationSorting::ExpectedReturn => query.push_str("av.expected_return DESC"),
            ExplorationSorting::ArtistRating => query.push_str("av.artist_rating DESC"),
            ExplorationSorting::Popularity => query.push_str("av.current_investors DESC"),
        }

        query.push_str(&format!(" LIMIT {} OFFSET {}", page_size, offset));

        // Execute query (simplified for now)
        // TODO: Implement proper parameterized query execution
        
        // Mock response for now
        let mock_discoveries = vec![
            VentureDiscovery {
                venture_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                artist_name: "John Doe".to_string(),
                artist_avatar: Some("https://example.com/avatar.jpg".to_string()),
                title: "My First Album".to_string(),
                description: Some("An amazing debut album".to_string()),
                min_investment: 100.0,
                max_investment: Some(1000.0),
                funding_goal: 10000.0,
                current_funding: 5000.0,
                funding_progress: 50.0,
                total_investors: 25,
                status: VentureStatus::Open,
                end_date: Some(Utc::now() + chrono::Duration::days(30)),
                days_remaining: Some(30),
                created_at: Utc::now(),
                top_tiers: Vec::new(),
                tags: vec!["music".to_string(), "album".to_string()],
                category: VentureCategory::Music,
                risk_level: RiskLevel::Medium,
                expected_return: 15.0,
                artist_rating: 4.5,
                artist_previous_ventures: 2,
                artist_success_rate: 85.0,
            }
        ];

        Ok(mock_discoveries)
    }

    /// Get venture recommendations for a fan
    pub async fn get_venture_recommendations(
        &self,
        fan_id: Uuid,
        limit: u32,
    ) -> Result<Vec<VentureRecommendation>, AppError> {
        // TODO: Implement recommendation algorithm
        // For now, return mock recommendations
        let mock_recommendations = vec![
            VentureRecommendation {
                venture_id: Uuid::new_v4(),
                score: 0.85,
                reasons: vec![
                    "Matches your favorite category (Music)".to_string(),
                    "Artist has high success rate".to_string(),
                    "Risk level matches your tolerance".to_string(),
                ],
                match_percentage: 85.0,
            }
        ];

        Ok(mock_recommendations)
    }

    /// Save or update fan preferences
    pub async fn save_fan_preferences(&self, preferences: &FanPreferences) -> Result<(), AppError> {
        let category_strings: Vec<String> = preferences.favorite_categories.iter().map(|c| c.to_string()).collect();
        let investment_type_strings: Vec<String> = preferences.preferred_investment_types.iter().map(|t| t.to_string()).collect();
        let interest_strings = preferences.interests.clone();

        sqlx::query!(
            r#"
            INSERT INTO fan_preferences (
                fan_id, favorite_categories, preferred_investment_types, risk_tolerance,
                min_investment, max_investment, favorite_artists, interests, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (fan_id) DO UPDATE SET
                favorite_categories = EXCLUDED.favorite_categories,
                preferred_investment_types = EXCLUDED.preferred_investment_types,
                risk_tolerance = EXCLUDED.risk_tolerance,
                min_investment = EXCLUDED.min_investment,
                max_investment = EXCLUDED.max_investment,
                favorite_artists = EXCLUDED.favorite_artists,
                interests = EXCLUDED.interests,
                updated_at = EXCLUDED.updated_at
            "#,
            preferences.fan_id,
            &category_strings,
            &investment_type_strings,
            preferences.risk_tolerance.to_string(),
            preferences.min_investment,
            preferences.max_investment,
            &preferences.favorite_artists,
            &interest_strings,
            preferences.created_at,
            preferences.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Get fan preferences
    pub async fn get_fan_preferences(&self, fan_id: Uuid) -> Result<Option<FanPreferences>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM fan_preferences WHERE fan_id = $1
            "#,
            fan_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let favorite_categories: Vec<VentureCategory> = row.favorite_categories
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|s| VentureCategory::from_string(s.as_str()).ok())
                    .collect();
                
                let investment_types: Vec<InvestmentType> = row
                    .preferred_investment_types
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|s| InvestmentType::from_string(s.as_str()).ok())
                    .collect();

                Ok(Some(FanPreferences {
                    fan_id: row.fan_id,
                    favorite_categories,
                    preferred_investment_types: investment_types,
                    risk_tolerance: RiskLevel::from_string(&row.risk_tolerance.unwrap_or_else(|| "medium".to_string())).unwrap_or(RiskLevel::Medium),
                    min_investment: row.min_investment.unwrap_or(0.0),
                    max_investment: row.max_investment.unwrap_or(10000.0),
                    favorite_artists: row.favorite_artists.unwrap_or_default(),
                    interests: row.interests.unwrap_or_default(),
                    created_at: row.created_at.unwrap_or_else(|| Utc::now()),
                    updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
                }))
            }
            None => Ok(None)
        }
    }
}

// =============================================================================
// HELPER IMPLEMENTATIONS
// =============================================================================

impl VentureStatus {
    fn to_string(&self) -> String {
        match self {
            VentureStatus::Draft => "Draft".to_string(),
            VentureStatus::Open => "Open".to_string(),
            VentureStatus::Closed => "Closed".to_string(),
            VentureStatus::Cancelled => "Cancelled".to_string(),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "Draft" => Ok(VentureStatus::Draft),
            "Open" => Ok(VentureStatus::Open),
            "Closed" => Ok(VentureStatus::Closed),
            "Cancelled" => Ok(VentureStatus::Cancelled),
            _ => Err(AppError::DomainRuleViolation(format!("Invalid venture status: {}", s))),
        }
    }
}

impl InvestmentStatus {
    fn to_string(&self) -> String {
        match self {
            InvestmentStatus::Pending => "Pending".to_string(),
            InvestmentStatus::Active => "Active".to_string(),
            InvestmentStatus::Completed => "Completed".to_string(),
            InvestmentStatus::Cancelled => "Cancelled".to_string(),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "Pending" => Ok(InvestmentStatus::Pending),
            "Active" => Ok(InvestmentStatus::Active),
            "Completed" => Ok(InvestmentStatus::Completed),
            "Cancelled" => Ok(InvestmentStatus::Cancelled),
            _ => Err(AppError::DomainRuleViolation(format!("Invalid investment status: {}", s))),
        }
    }
}

impl InvestmentType {
    fn to_string(&self) -> String {
        match self {
            InvestmentType::EarlyAccess => "EarlyAccess".to_string(),
            InvestmentType::ExclusiveContent => "ExclusiveContent".to_string(),
            InvestmentType::Merchandise => "Merchandise".to_string(),
            InvestmentType::ConcertTickets => "ConcertTickets".to_string(),
            InvestmentType::MeetAndGreet => "MeetAndGreet".to_string(),
            InvestmentType::RevenueShare => "RevenueShare".to_string(),
            InvestmentType::Custom(s) => format!("Custom:{}", s),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "EarlyAccess" => Ok(InvestmentType::EarlyAccess),
            "ExclusiveContent" => Ok(InvestmentType::ExclusiveContent),
            "Merchandise" => Ok(InvestmentType::Merchandise),
            "ConcertTickets" => Ok(InvestmentType::ConcertTickets),
            "MeetAndGreet" => Ok(InvestmentType::MeetAndGreet),
            "RevenueShare" => Ok(InvestmentType::RevenueShare),
            s if s.starts_with("Custom:") => {
                let custom = s.strip_prefix("Custom:").unwrap_or("Unknown");
                Ok(InvestmentType::Custom(custom.to_string()))
            },
            _ => Err(AppError::DomainRuleViolation(format!("Invalid investment type: {}", s))),
        }
    }
}

impl BenefitType {
    fn to_string(&self) -> String {
        match self {
            BenefitType::DigitalContent => "DigitalContent".to_string(),
            BenefitType::PhysicalProduct => "PhysicalProduct".to_string(),
            BenefitType::Experience => "Experience".to_string(),
            BenefitType::RevenueShare => "RevenueShare".to_string(),
            BenefitType::Recognition => "Recognition".to_string(),
            BenefitType::Custom(s) => format!("Custom:{}", s),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "DigitalContent" => Ok(BenefitType::DigitalContent),
            "PhysicalProduct" => Ok(BenefitType::PhysicalProduct),
            "Experience" => Ok(BenefitType::Experience),
            "RevenueShare" => Ok(BenefitType::RevenueShare),
            "Recognition" => Ok(BenefitType::Recognition),
            s if s.starts_with("Custom:") => {
                let custom = s.strip_prefix("Custom:").unwrap_or("Unknown");
                Ok(BenefitType::Custom(custom.to_string()))
            },
            _ => Err(AppError::DomainRuleViolation(format!("Invalid benefit type: {}", s))),
        }
    }
}

impl DeliveryMethod {
    fn to_string(&self) -> String {
        match self {
            DeliveryMethod::Automatic => "automatic".to_string(),
            DeliveryMethod::Manual => "manual".to_string(),
            DeliveryMethod::Physical => "physical".to_string(),
            DeliveryMethod::Experience => "experience".to_string(),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "automatic" => Ok(DeliveryMethod::Automatic),
            "manual" => Ok(DeliveryMethod::Manual),
            "physical" => Ok(DeliveryMethod::Physical),
            "experience" => Ok(DeliveryMethod::Experience),
            _ => Err(AppError::DomainRuleViolation(format!("Invalid delivery method: {}", s))),
        }
    }
}

impl DeliveryStatus {
    fn to_string(&self) -> String {
        match self {
            DeliveryStatus::Pending => "pending".to_string(),
            DeliveryStatus::InProgress => "in_progress".to_string(),
            DeliveryStatus::Delivered => "delivered".to_string(),
            DeliveryStatus::Failed => "failed".to_string(),
            DeliveryStatus::Cancelled => "cancelled".to_string(),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "pending" => Ok(DeliveryStatus::Pending),
            "in_progress" => Ok(DeliveryStatus::InProgress),
            "delivered" => Ok(DeliveryStatus::Delivered),
            "failed" => Ok(DeliveryStatus::Failed),
            "cancelled" => Ok(DeliveryStatus::Cancelled),
            _ => Err(AppError::DomainRuleViolation(format!("Invalid delivery status: {}", s))),
        }
    }
}

impl VentureCategory {
    fn to_string(&self) -> String {
        match self {
            VentureCategory::Music => "music".to_string(),
            VentureCategory::VisualArts => "visual_arts".to_string(),
            VentureCategory::Film => "film".to_string(),
            VentureCategory::Gaming => "gaming".to_string(),
            VentureCategory::Technology => "technology".to_string(),
            VentureCategory::Fashion => "fashion".to_string(),
            VentureCategory::Food => "food".to_string(),
            VentureCategory::Travel => "travel".to_string(),
            VentureCategory::Education => "education".to_string(),
            VentureCategory::Health => "health".to_string(),
            VentureCategory::Other => "other".to_string(),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "music" => Ok(VentureCategory::Music),
            "visual_arts" => Ok(VentureCategory::VisualArts),
            "film" => Ok(VentureCategory::Film),
            "gaming" => Ok(VentureCategory::Gaming),
            "technology" => Ok(VentureCategory::Technology),
            "fashion" => Ok(VentureCategory::Fashion),
            "food" => Ok(VentureCategory::Food),
            "travel" => Ok(VentureCategory::Travel),
            "education" => Ok(VentureCategory::Education),
            "health" => Ok(VentureCategory::Health),
            "other" => Ok(VentureCategory::Other),
            _ => Err(AppError::DomainRuleViolation(format!("Invalid venture category: {}", s))),
        }
    }
}

impl RiskLevel {
    fn to_string(&self) -> String {
        match self {
            RiskLevel::Low => "low".to_string(),
            RiskLevel::Medium => "medium".to_string(),
            RiskLevel::High => "high".to_string(),
            RiskLevel::VeryHigh => "very_high".to_string(),
        }
    }

    fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "low" => Ok(RiskLevel::Low),
            "medium" => Ok(RiskLevel::Medium),
            "high" => Ok(RiskLevel::High),
            "very_high" => Ok(RiskLevel::VeryHigh),
            _ => Err(AppError::DomainRuleViolation(format!("Invalid risk level: {}", s))),
        }
    }
} 