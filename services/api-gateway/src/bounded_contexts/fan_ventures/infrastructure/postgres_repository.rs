use crate::bounded_contexts::fan_ventures::domain::entities::{
    ArtistVenture, FanInvestment, VentureTier, VentureBenefit, VentureCategory, RiskLevel, VentureStatus, InvestmentType, InvestmentStatus, BenefitType, DeliveryMethod
};
use crate::shared::domain::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

// Helper functions for parsing enums from database strings
fn parse_benefit_type(s: &str) -> BenefitType {
    match s.to_lowercase().as_str() {
        "exclusive_content" | "digitalcontent" => BenefitType::DigitalContent,
        "physical_product" | "merchandise" => BenefitType::PhysicalProduct,
        "experience" | "meet_greet" | "concert_tickets" => BenefitType::Experience,
        "revenue_share" => BenefitType::RevenueShare,
        "recognition" | "voting_rights" => BenefitType::Recognition,
        _ => BenefitType::Custom(s.to_string()),
    }
}

fn parse_delivery_method(s: &str) -> DeliveryMethod {
    match s.to_lowercase().as_str() {
        "automatic" => DeliveryMethod::Automatic,
        "manual" => DeliveryMethod::Manual,
        "physical" => DeliveryMethod::Physical,
        "experience" => DeliveryMethod::Experience,
        _ => DeliveryMethod::Manual,
    }
}

fn parse_venture_category(s: &str) -> VentureCategory {
    match s.to_lowercase().as_str() {
        "music" => VentureCategory::Music,
        "visual_arts" => VentureCategory::VisualArts,
        "film" => VentureCategory::Film,
        "gaming" => VentureCategory::Gaming,
        "technology" => VentureCategory::Technology,
        "fashion" => VentureCategory::Fashion,
        "food" => VentureCategory::Food,
        "travel" => VentureCategory::Travel,
        "education" => VentureCategory::Education,
        "health" => VentureCategory::Health,
        _ => VentureCategory::Other,
    }
}

fn parse_risk_level(s: &str) -> RiskLevel {
    match s.to_lowercase().as_str() {
        "low" => RiskLevel::Low,
        "medium" => RiskLevel::Medium,
        "high" => RiskLevel::High,
        "very_high" => RiskLevel::VeryHigh,
        _ => RiskLevel::Medium,
    }
}

fn parse_venture_status(s: &str) -> VentureStatus {
    match s.to_lowercase().as_str() {
        "draft" => VentureStatus::Draft,
        "open" | "active" => VentureStatus::Open,
        "closed" | "funded" | "completed" => VentureStatus::Closed,
        "cancelled" => VentureStatus::Cancelled,
        _ => VentureStatus::Draft,
    }
}

fn parse_investment_status(s: &str) -> InvestmentStatus {
    match s.to_lowercase().as_str() {
        "pending" => InvestmentStatus::Pending,
        "confirmed" | "active" => InvestmentStatus::Active,
        "completed" => InvestmentStatus::Completed,
        "cancelled" | "refunded" => InvestmentStatus::Cancelled,
        _ => InvestmentStatus::Pending,
    }
}

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
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date,
                updated_at = EXCLUDED.updated_at,
                -- Auto-update status to 'funded' if goal is reached
                status = CASE 
                    WHEN EXCLUDED.current_funding >= EXCLUDED.funding_goal THEN 'funded'
                    WHEN EXCLUDED.status = 'draft' AND EXCLUDED.current_funding > 0 THEN 'active'
                    ELSE EXCLUDED.status
                END"#,
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

        // Persist benefits if any
        for benefit in &venture.benefits {
            self.create_venture_benefit(benefit).await?;
        }

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
                
                // Load benefits
                let benefits = self.get_venture_benefits(venture_id).await
                    .unwrap_or_else(|e| {
                         tracing::error!("Failed to load benefits for venture {}: {}", venture_id, e);
                         vec![]
                    });

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
                    benefits,
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

    pub async fn update_venture(&self, venture: &ArtistVenture) -> Result<(), AppError> {
        // Update venture with current timestamp
        let updated_at = Utc::now();
        
        sqlx::query!(
            r#"UPDATE artist_ventures SET
                title = $2,
                description = $3,
                category = $4,
                tags = $5,
                risk_level = $6,
                expected_return = $7,
                artist_rating = $8,
                artist_previous_ventures = $9,
                artist_success_rate = $10,
                funding_goal = $11,
                current_funding = $12,
                min_investment = $13,
                max_investment = $14,
                status = $15,
                start_date = $16,
                end_date = $17,
                updated_at = $18
            WHERE id = $1"#,
            venture.id,
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
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update venture: {}", e)))?;

        // Update benefits if any (delete old ones and create new ones)
        // Note: This is a simple approach - in production you might want to diff and update
        if !venture.benefits.is_empty() {
            // Delete existing benefits for this venture
            sqlx::query!(
                "DELETE FROM venture_benefits WHERE venture_id = $1",
                venture.id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete old benefits: {}", e)))?;

            // Create new benefits
            for benefit in &venture.benefits {
                self.create_venture_benefit(benefit).await?;
            }
        }

        Ok(())
    }

    pub async fn delete_venture(&self, venture_id: Uuid) -> Result<(), AppError> {
        // Soft delete: Update status to 'Cancelled' instead of hard delete
        // This preserves data for analytics and audit purposes
        sqlx::query!(
            r#"UPDATE artist_ventures 
               SET status = 'cancelled', updated_at = $2
               WHERE id = $1"#,
            venture_id,
            Utc::now()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete venture: {}", e)))?;

        // Optionally, you could also delete related investments and benefits
        // For now, we'll keep them for historical records
        
        Ok(())
    }

    pub async fn get_ventures_by_artist(&self, artist_id: Uuid) -> Result<Vec<ArtistVenture>, AppError> {
        let rows = sqlx::query!(
            r#"SELECT id, artist_id, title, description, category, tags, risk_level,
                      expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                      funding_goal, current_funding, min_investment, max_investment, status,
                      start_date, end_date, created_at, updated_at
               FROM artist_ventures 
               WHERE artist_id = $1
               ORDER BY created_at DESC"#,
            artist_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get ventures by artist: {}", e)))?;

        let mut ventures = Vec::new();
        for row in rows {
            let tags: Vec<String> = serde_json::from_value(row.tags)
                .map_err(|e| AppError::SerializationError(e.to_string()))?;
            
            // Load benefits for each venture
            let benefits = self.get_venture_benefits(row.id).await
                .unwrap_or_else(|e| {
                    tracing::error!("Failed to load benefits for venture {}: {}", row.id, e);
                    vec![]
                });
            
            let venture = ArtistVenture {
                id: row.id,
                artist_id: row.artist_id,
                title: row.title,
                description: row.description,
                category: parse_venture_category(&row.category),
                tags,
                risk_level: parse_risk_level(&row.risk_level),
                expected_return: row.expected_return,
                artist_rating: row.artist_rating,
                artist_previous_ventures: row.artist_previous_ventures,
                artist_success_rate: row.artist_success_rate,
                funding_goal: row.funding_goal,
                current_funding: row.current_funding,
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                status: parse_venture_status(&row.status),
                start_date: row.start_date,
                end_date: row.end_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
                benefits,
            };
            ventures.push(venture);
        }

        Ok(ventures)
    }

    pub async fn get_ventures_by_category(&self, category: &str) -> Result<Vec<ArtistVenture>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, artist_id, title, description, category, tags, risk_level,
                   expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                   funding_goal, current_funding, min_investment, max_investment, status,
                   start_date, end_date, created_at, updated_at
            FROM artist_ventures
            WHERE category = $1
            ORDER BY created_at DESC
            "#,
            category
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get ventures by category: {}", e)))?;

        let mut ventures = Vec::new();
        for row in rows {
            let tags: Vec<String> = serde_json::from_value(row.tags.unwrap_or(serde_json::Value::Array(vec![])))
                .unwrap_or_default();
            
            let benefits = self.get_venture_benefits(row.id).await?;
            
            let venture = ArtistVenture {
                id: row.id,
                artist_id: row.artist_id,
                title: row.title,
                description: row.description,
                category: parse_venture_category(&row.category),
                tags,
                risk_level: parse_risk_level(&row.risk_level),
                expected_return: row.expected_return.unwrap_or(0.0),
                artist_rating: row.artist_rating.unwrap_or(0.0),
                artist_previous_ventures: row.artist_previous_ventures.unwrap_or(0),
                artist_success_rate: row.artist_success_rate.unwrap_or(0.0),
                funding_goal: row.funding_goal,
                current_funding: row.current_funding.unwrap_or(0.0),
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                status: parse_venture_status(&row.status),
                start_date: row.start_date,
                end_date: row.end_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
                benefits,
            };
            ventures.push(venture);
        }
        
        Ok(ventures)
    }

    pub async fn get_ventures_by_status(&self, status: &str) -> Result<Vec<ArtistVenture>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, artist_id, title, description, category, tags, risk_level,
                   expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                   funding_goal, current_funding, min_investment, max_investment, status,
                   start_date, end_date, created_at, updated_at
            FROM artist_ventures
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
            status
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get ventures by status: {}", e)))?;

        let mut ventures = Vec::new();
        for row in rows {
            let tags: Vec<String> = serde_json::from_value(row.tags.unwrap_or(serde_json::Value::Array(vec![])))
                .unwrap_or_default();
            
            let benefits = self.get_venture_benefits(row.id).await?;
            
            let venture = ArtistVenture {
                id: row.id,
                artist_id: row.artist_id,
                title: row.title,
                description: row.description,
                category: parse_venture_category(&row.category),
                tags,
                risk_level: parse_risk_level(&row.risk_level),
                expected_return: row.expected_return.unwrap_or(0.0),
                artist_rating: row.artist_rating.unwrap_or(0.0),
                artist_previous_ventures: row.artist_previous_ventures.unwrap_or(0),
                artist_success_rate: row.artist_success_rate.unwrap_or(0.0),
                funding_goal: row.funding_goal,
                current_funding: row.current_funding.unwrap_or(0.0),
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                status: parse_venture_status(&row.status),
                start_date: row.start_date,
                end_date: row.end_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
                benefits,
            };
            ventures.push(venture);
        }
        
        Ok(ventures)
    }

    pub async fn search_ventures(&self, query: &str, limit: Option<i32>) -> Result<Vec<ArtistVenture>, AppError> {
        let limit = limit.unwrap_or(50).min(100); // Max 100 results
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, artist_id, title, description, category, tags, risk_level,
                   expected_return, artist_rating, artist_previous_ventures, artist_success_rate,
                   funding_goal, current_funding, min_investment, max_investment, status,
                   start_date, end_date, created_at, updated_at
            FROM artist_ventures
            WHERE title ILIKE $1 
               OR description ILIKE $1
               OR $2 = ANY(tags)
            ORDER BY 
                CASE 
                    WHEN title ILIKE $1 THEN 1
                    WHEN description ILIKE $1 THEN 2
                    ELSE 3
                END,
                created_at DESC
            LIMIT $3
            "#,
            search_pattern,
            query,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to search ventures: {}", e)))?;

        let mut ventures = Vec::new();
        for row in rows {
            let tags: Vec<String> = serde_json::from_value(row.tags.unwrap_or(serde_json::Value::Array(vec![])))
                .unwrap_or_default();
            
            let benefits = self.get_venture_benefits(row.id).await?;
            
            let venture = ArtistVenture {
                id: row.id,
                artist_id: row.artist_id,
                title: row.title,
                description: row.description,
                category: parse_venture_category(&row.category),
                tags,
                risk_level: parse_risk_level(&row.risk_level),
                expected_return: row.expected_return.unwrap_or(0.0),
                artist_rating: row.artist_rating.unwrap_or(0.0),
                artist_previous_ventures: row.artist_previous_ventures.unwrap_or(0),
                artist_success_rate: row.artist_success_rate.unwrap_or(0.0),
                funding_goal: row.funding_goal,
                current_funding: row.current_funding.unwrap_or(0.0),
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                status: parse_venture_status(&row.status),
                start_date: row.start_date,
                end_date: row.end_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
                benefits,
            };
            ventures.push(venture);
        }
        
        Ok(ventures)
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
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT as count
            FROM artist_ventures
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get venture count: {}", e)))?;
        
        Ok(count.unwrap_or(0) as u64)
    }

    pub async fn get_venture_revenue(&self, venture_id: Uuid) -> Result<f64, AppError> {
        // Sum all confirmed investments for this venture
        let revenue = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(investment_amount), 0.0) as revenue
            FROM fan_investments
            WHERE venture_id = $1 AND status = 'confirmed'
            "#,
            venture_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get venture revenue: {}", e)))?;
        
        Ok(revenue.unwrap_or(0.0))
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

    pub async fn get_fan_investments(&self, fan_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, fan_id, venture_id, investment_amount, investment_type, status, created_at, updated_at
            FROM fan_investments
            WHERE fan_id = $1
            ORDER BY created_at DESC
            "#,
            fan_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get fan investments: {}", e)))?;

        let mut investments = Vec::new();
        for row in rows {
            let investment_type: InvestmentType = serde_json::from_value(row.investment_type)
                .map_err(|e| AppError::SerializationError(format!("Failed to parse investment_type: {}", e)))?;
            
            let status = parse_investment_status(&row.status);
            
            let investment = FanInvestment {
                id: row.id,
                fan_id: row.fan_id,
                venture_id: row.venture_id,
                investment_amount: row.investment_amount,
                investment_type,
                status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            investments.push(investment);
        }
        
        Ok(investments)
    }

    pub async fn get_fan_investments_by_venture(&self, venture_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, fan_id, venture_id, investment_amount, investment_type, status, created_at, updated_at
            FROM fan_investments
            WHERE venture_id = $1 AND status = 'confirmed'
            ORDER BY created_at DESC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get investments by venture: {}", e)))?;

        let mut investments = Vec::new();
        for row in rows {
            let investment_type: InvestmentType = serde_json::from_value(row.investment_type)
                .map_err(|e| AppError::SerializationError(format!("Failed to parse investment_type: {}", e)))?;
            
            let status = parse_investment_status(&row.status);
            
            let investment = FanInvestment {
                id: row.id,
                fan_id: row.fan_id,
                venture_id: row.venture_id,
                investment_amount: row.investment_amount,
                investment_type,
                status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            investments.push(investment);
        }
        
        Ok(investments)
    }

    pub async fn update_fan_investment(&self, investment: &FanInvestment) -> Result<(), AppError> {
        let status_str = match investment.status {
            InvestmentStatus::Pending => "pending",
            InvestmentStatus::Active => "active",
            InvestmentStatus::Completed => "completed",
            InvestmentStatus::Cancelled => "cancelled",
        };

        sqlx::query!(
            r#"UPDATE fan_investments SET
                investment_amount = $2,
                investment_type = $3,
                status = $4,
                updated_at = $5
            WHERE id = $1"#,
            investment.id,
            investment.investment_amount,
            serde_json::to_value(&investment.investment_type)
                .map_err(|e| AppError::SerializationError(e.to_string()))?,
            status_str,
            investment.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update investment: {}", e)))?;

        Ok(())
    }

    pub async fn delete_fan_investment(&self, _investment_id: Uuid) -> Result<(), AppError> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(())
    }

    pub async fn get_investment_by_id(&self, investment_id: Uuid) -> Result<Option<FanInvestment>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT id, fan_id, venture_id, investment_amount, created_at, status, updated_at
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
                row.investment_amount,
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
            SELECT id, fan_id, venture_id, investment_amount, created_at, status, updated_at
            FROM fan_investments
            WHERE venture_id = $1
            ORDER BY created_at DESC
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
                row.investment_amount,
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

    pub async fn get_total_invested_investment_amount(&self) -> Result<f64, AppError> {
        let total = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(investment_amount), 0) FROM fan_investments WHERE status = 'active'"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get total invested investment_amount: {}", e)))?;
        
        Ok(total.unwrap_or(0.0))
    }

    // =============================================================================
    // VENTURE TIERS
    // =============================================================================

    pub async fn create_venture_tier(&self, tier: &VentureTier) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO venture_tiers (id, venture_id, name, min_investment, max_investment, 
                                     description, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
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
        .map_err(|e| AppError::DatabaseError(format!("Failed to create venture tier: {}", e)))?;
        
        Ok(())
    }

    pub async fn get_venture_tiers(&self, venture_id: Uuid) -> Result<Vec<VentureTier>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, venture_id, name, min_investment, max_investment, 
                   description, created_at
            FROM venture_tiers
            WHERE venture_id = $1
            ORDER BY min_investment ASC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get venture tiers: {}", e)))?;
        
        let mut tiers = Vec::new();
        for row in rows {
            // Load benefits for this tier
            let benefits = self.get_venture_benefits_by_tier(row.id).await?;
            
            let tier = VentureTier {
                id: row.id,
                venture_id: row.venture_id,
                tier_name: row.name,
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                description: row.description,
                benefits,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            tiers.push(tier);
            }
        
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
            SELECT id, venture_id, name, min_investment, max_investment, 
                   description, created_at
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
                tier_name: row.name,
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

    pub async fn create_venture_benefit(&self, benefit: &VentureBenefit) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO venture_benefits (
                id, venture_id, tier_id, title, description, benefit_type,
                delivery_method, estimated_delivery_date, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            benefit.id,
            benefit.venture_id,
            benefit.tier_id,
            benefit.title,
            benefit.description,
            benefit.benefit_type.to_string(),
            benefit.delivery_method.to_string(),
            benefit.estimated_delivery_date,
            benefit.created_at,
            benefit.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create venture benefit: {}", e)))?;

        Ok(())
    }

    pub async fn get_venture_benefits(&self, venture_id: Uuid) -> Result<Vec<VentureBenefit>, AppError> {
        let rows = sqlx::query!(
            r#"SELECT id, venture_id, tier_id, title, description, benefit_type,
                      delivery_method, estimated_delivery_date, created_at, updated_at
               FROM venture_benefits
               WHERE venture_id = $1"#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get venture benefits: {}", e)))?;

        let benefits = rows.into_iter().map(|row| VentureBenefit {
            id: row.id,
            venture_id: row.venture_id,
            tier_id: row.tier_id,
            title: row.title,
            description: row.description,
            benefit_type: parse_benefit_type(&row.benefit_type),
            delivery_method: parse_delivery_method(&row.delivery_method.unwrap_or_else(|| "manual".to_string())),
            estimated_delivery_date: row.estimated_delivery_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(benefits)
    }

    pub async fn get_venture_benefits_by_tier(&self, tier_id: Uuid) -> Result<Vec<VentureBenefit>, AppError> {
        let rows = sqlx::query!(
            r#"SELECT id, venture_id, tier_id, title, description, benefit_type,
                      delivery_method, estimated_delivery_date, created_at, updated_at
               FROM venture_benefits
               WHERE tier_id = $1"#,
            tier_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get venture benefits by tier: {}", e)))?;

        let benefits = rows.into_iter().map(|row| VentureBenefit {
            id: row.id,
            venture_id: row.venture_id,
            tier_id: row.tier_id,
            title: row.title,
            description: row.description,
            benefit_type: parse_benefit_type(&row.benefit_type),
            delivery_method: parse_delivery_method(&row.delivery_method.unwrap_or_else(|| "manual".to_string())),
            estimated_delivery_date: row.estimated_delivery_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(benefits)
    }

    pub async fn update_venture_benefit(&self, benefit: &VentureBenefit) -> Result<(), AppError> {
        // TODO: Implement update
        Ok(())
    }

    pub async fn delete_venture_benefit(&self, benefit_id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM venture_benefits WHERE id = $1", benefit_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn get_benefit_by_id(&self, benefit_id: Uuid) -> Result<Option<VentureBenefit>, AppError> {
        let row = sqlx::query!(
            r#"SELECT id, venture_id, tier_id, title, description, benefit_type,
                      delivery_method, estimated_delivery_date, created_at, updated_at
               FROM venture_benefits
               WHERE id = $1"#,
            benefit_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get benefit by ID: {}", e)))?;

        if let Some(row) = row {
            Ok(Some(VentureBenefit {
                id: row.id,
                venture_id: row.venture_id,
                tier_id: row.tier_id,
                title: row.title,
                description: row.description,
                benefit_type: row.benefit_type.parse().unwrap_or_default(),
                delivery_method: row.delivery_method.unwrap_or_default().parse().unwrap_or_default(),
                estimated_delivery_date: row.estimated_delivery_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
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
    pub async fn create_revenue_distribution(&self, distribution: &crate::bounded_contexts::fan_ventures::domain::entities::RevenueDistribution) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO revenue_distributions (
                id, venture_id, total_revenue, artist_share, fan_share, platform_fee,
                distributed_at, period_start, period_end, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())"#,
            distribution.id,
            distribution.venture_id,
            distribution.total_revenue,
            distribution.artist_share,
            distribution.fan_share,
            distribution.platform_fee,
            distribution.distributed_at,
            distribution.period_start,
            distribution.period_end
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create revenue distribution: {}", e)))?;
        Ok(())
    }

    /// Get venture distributions
    pub async fn get_venture_distributions(&self, venture_id: Uuid) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::RevenueDistribution>, AppError> {
        let rows = sqlx::query!(
            r#"SELECT id, venture_id, total_revenue, artist_share, fan_share, platform_fee,
                      distributed_at, period_start, period_end
               FROM revenue_distributions
               WHERE venture_id = $1
               ORDER BY distributed_at DESC"#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get distributions: {}", e)))?;

        let distributions = rows.into_iter().map(|row| crate::bounded_contexts::fan_ventures::domain::entities::RevenueDistribution {
            id: row.id,
            venture_id: row.venture_id,
            total_revenue: row.total_revenue,
            artist_share: row.artist_share,
            fan_share: row.fan_share,
            platform_fee: row.platform_fee,
            distributed_at: row.distributed_at,
            period_start: row.period_start,
            period_end: row.period_end,
        }).collect();

        Ok(distributions)
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
    pub async fn create_benefit_delivery(&self, delivery: &crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO benefit_deliveries (
                id, benefit_id, venture_id, fan_id, tier_id, delivery_status,
                delivery_method, delivery_date, tracking_info, notes, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
            delivery.id,
            delivery.benefit_id,
            delivery.venture_id,
            delivery.fan_id,
            delivery.tier_id,
            format!("{:?}", delivery.delivery_status),
            format!("{:?}", delivery.delivery_method),
            delivery.delivery_date,
            serde_json::to_value(&delivery.tracking_info).unwrap_or(serde_json::Value::Null),
            delivery.notes,
            delivery.created_at,
            delivery.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create benefit delivery: {}", e)))?;
        Ok(())
    }

    /// Get benefit delivery
    pub async fn get_benefit_delivery(&self, delivery_id: Uuid) -> Result<Option<crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery>, AppError> {
        let row = sqlx::query!(
            r#"SELECT id, benefit_id, venture_id, fan_id, tier_id, delivery_status,
                      delivery_method, delivery_date, tracking_info, notes, created_at, updated_at
               FROM benefit_deliveries
               WHERE id = $1"#,
            delivery_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get benefit delivery: {}", e)))?;

        if let Some(row) = row {
             // Basic parsing (Assuming enums match name, otherwise need robust parsing)
             // Using simplified parsing assuming data integrity from create
             let delivery_status = match row.delivery_status.as_str() {
                 "Pending" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Pending,
                 "InProgress" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::InProgress,
                 "Delivered" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Delivered,
                 "Failed" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Failed,
                 "Cancelled" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Cancelled,
                 _ => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Pending,
             };
             
             let delivery_method = match row.delivery_method.as_str() {
                 "Automatic" => DeliveryMethod::Automatic,
                 "Manual" => DeliveryMethod::Manual,
                 "Physical" => DeliveryMethod::Physical,
                 "Experience" => DeliveryMethod::Experience,
                 _ => DeliveryMethod::Manual, 
             };

             Ok(Some(crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery {
                 id: row.id,
                 benefit_id: row.benefit_id,
                 venture_id: row.venture_id,
                 fan_id: row.fan_id,
                 tier_id: row.tier_id,
                 delivery_status,
                 delivery_method,
                 delivery_date: row.delivery_date,
                 tracking_info: serde_json::from_value(row.tracking_info.unwrap_or(serde_json::Value::Null)).ok(),
                 notes: row.notes,
                 created_at: row.created_at.expect("Valid date"),
                 updated_at: row.updated_at.expect("Valid date"),
             }))
        } else {
            Ok(None)
        }
    }

    /// Update benefit delivery
    pub async fn update_benefit_delivery(&self, delivery_id: Uuid, delivery: &crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery) -> Result<(), AppError> {
         sqlx::query!(
            r#"UPDATE benefit_deliveries 
               SET delivery_status = $1, tracking_info = $2, notes = $3, updated_at = NOW()
               WHERE id = $4"#,
            format!("{:?}", delivery.delivery_status),
            serde_json::to_value(&delivery.tracking_info).unwrap_or(serde_json::Value::Null),
            delivery.notes,
            delivery_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update benefit delivery: {}", e)))?;
        Ok(())
    }

    /// Get fan deliveries
    pub async fn get_fan_deliveries(&self, fan_id: Uuid) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery>, AppError> {
         let rows = sqlx::query!(
            r#"SELECT id, benefit_id, venture_id, fan_id, tier_id, delivery_status,
                      delivery_method, delivery_date, tracking_info, notes, created_at, updated_at
               FROM benefit_deliveries
               WHERE fan_id = $1
               ORDER BY created_at DESC"#,
            fan_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get fan deliveries: {}", e)))?;

        // Mapping similar to get_benefit_delivery (can be refactored to helper)
        let deliveries = rows.into_iter().map(|row| {
             let delivery_status = match row.delivery_status.as_str() {
                 "Pending" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Pending,
                 "InProgress" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::InProgress,
                 "Delivered" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Delivered,
                 "Failed" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Failed,
                 "Cancelled" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Cancelled,
                 _ => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Pending,
             };
             
             let delivery_method = match row.delivery_method.as_str() {
                 "Automatic" => DeliveryMethod::Automatic,
                 "Manual" => DeliveryMethod::Manual,
                 "Physical" => DeliveryMethod::Physical,
                 "Experience" => DeliveryMethod::Experience,
                 _ => DeliveryMethod::Manual, 
             };

             crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery {
                 id: row.id,
                 benefit_id: row.benefit_id,
                 venture_id: row.venture_id,
                 fan_id: row.fan_id,
                 tier_id: row.tier_id,
                 delivery_status,
                 delivery_method,
                 delivery_date: row.delivery_date,
                 tracking_info: serde_json::from_value(row.tracking_info.unwrap_or(serde_json::Value::Null)).ok(),
                 notes: row.notes,
                 created_at: row.created_at.expect("Valid date"),
                 updated_at: row.updated_at.expect("Valid date"),
             }
        }).collect();
        
        Ok(deliveries)
    }

    /// Get venture deliveries
    pub async fn get_venture_deliveries(&self, venture_id: Uuid) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery>, AppError> {
         let rows = sqlx::query!(
            r#"SELECT id, benefit_id, venture_id, fan_id, tier_id, delivery_status,
                      delivery_method, delivery_date, tracking_info, notes, created_at, updated_at
               FROM benefit_deliveries
               WHERE venture_id = $1
               ORDER BY created_at DESC"#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get venture deliveries: {}", e)))?;

        let deliveries = rows.into_iter().map(|row| {
             // Simplified map
             let delivery_status = match row.delivery_status.as_str() {
                 "Pending" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Pending,
                 "InProgress" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::InProgress,
                 "Delivered" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Delivered,
                 "Failed" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Failed,
                 "Cancelled" => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Cancelled,
                 _ => crate::bounded_contexts::fan_ventures::domain::entities::DeliveryStatus::Pending,
             };
             
             let delivery_method = match row.delivery_method.as_str() {
                 "Automatic" => DeliveryMethod::Automatic,
                 "Manual" => DeliveryMethod::Manual,
                 "Physical" => DeliveryMethod::Physical,
                 "Experience" => DeliveryMethod::Experience,
                 _ => DeliveryMethod::Manual, 
             };

             crate::bounded_contexts::fan_ventures::domain::entities::BenefitDelivery {
                 id: row.id,
                 benefit_id: row.benefit_id,
                 venture_id: row.venture_id,
                 fan_id: row.fan_id,
                 tier_id: row.tier_id,
                 delivery_status,
                 delivery_method,
                 delivery_date: row.delivery_date,
                 tracking_info: serde_json::from_value(row.tracking_info.unwrap_or(serde_json::Value::Null)).ok(),
                 notes: row.notes,
                 created_at: row.created_at.expect("Valid date"),
                 updated_at: row.updated_at.expect("Valid date"),
             }
        }).collect();
        
        Ok(deliveries)
    }

    /// Get venture recommendations
    pub async fn get_venture_recommendations(&self, _fan_id: Uuid, _limit: u32) -> Result<Vec<crate::bounded_contexts::fan_ventures::domain::entities::VentureRecommendation>, AppError> {
        // Keeps stub
        Ok(vec![])
    }

    /// Save fan preferences
    pub async fn save_fan_preferences(&self, _preferences: &crate::bounded_contexts::fan_ventures::domain::entities::FanPreferences) -> Result<(), AppError> {
        // Keeps stub
        Ok(())
    }

    /// Get fan preferences
    pub async fn get_fan_preferences(&self, _fan_id: Uuid) -> Result<Option<crate::bounded_contexts::fan_ventures::domain::entities::FanPreferences>, AppError> {
        // Keeps stub
        Ok(None)
    }
} 