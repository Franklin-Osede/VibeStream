use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::fractional_ownership::domain::entities::{
    FanInvestment, ArtistVenture, RevenueDistribution, VentureBenefit, 
    InvestmentStatus, VentureStatus, InvestmentType, BenefitType
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
                id, artist_id, title, description, investment_type, 
                min_investment, max_investment, total_goal, current_amount,
                max_investors, current_investors, created_at, expires_at, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
            venture.id,
            venture.artist_id,
            venture.title,
            venture.description,
            venture.investment_type.to_string(),
            venture.min_investment,
            venture.max_investment,
            venture.total_goal,
            venture.current_amount,
            venture.max_investors,
            venture.current_investors as i32,
            venture.created_at,
            venture.expires_at,
            venture.status.to_string()
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
                    investment_type: InvestmentType::from_string(&row.investment_type)?,
                    min_investment: row.min_investment,
                    max_investment: row.max_investment,
                    total_goal: row.total_goal,
                    current_amount: row.current_amount,
                    max_investors: row.max_investors.map(|v| v as u32),
                    current_investors: row.current_investors as u32,
                    created_at: row.created_at,
                    expires_at: row.expires_at,
                    status: VentureStatus::from_string(&row.status)?,
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
            WHERE status = 'Open' AND expires_at > NOW()
            ORDER BY created_at DESC 
            LIMIT $1
            "#,
            limit
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
                investment_type: InvestmentType::from_string(&row.investment_type).unwrap_or(InvestmentType::Custom("Unknown".to_string())),
                min_investment: row.min_investment,
                max_investment: row.max_investment,
                total_goal: row.total_goal,
                current_amount: row.current_amount,
                max_investors: row.max_investors.map(|v| v as u32),
                current_investors: row.current_investors as u32,
                created_at: row.created_at,
                expires_at: row.expires_at,
                status: VentureStatus::from_string(&row.status).unwrap_or(VentureStatus::Draft),
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
                id, artist_id, fan_id, investment_amount, investment_type,
                created_at, status, expected_return, duration_months
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            investment.id,
            investment.artist_id,
            investment.fan_id,
            investment.investment_amount,
            investment.investment_type.to_string(),
            investment.created_at,
            investment.status.to_string(),
            investment.expected_return,
            investment.duration_months as i32
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update venture current amount and investors count
        sqlx::query!(
            r#"
            UPDATE artist_ventures 
            SET current_amount = current_amount + $1,
                current_investors = current_investors + 1
            WHERE artist_id = $2
            "#,
            investment.investment_amount,
            investment.artist_id
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
            ORDER BY created_at DESC
            "#,
            fan_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let investments = rows.into_iter().map(|row| FanInvestment {
            id: row.id,
            artist_id: row.artist_id,
            fan_id: row.fan_id,
            investment_amount: row.investment_amount,
            investment_type: InvestmentType::from_string(&row.investment_type).unwrap_or(InvestmentType::Custom("Unknown".to_string())),
            created_at: row.created_at,
            status: InvestmentStatus::from_string(&row.status).unwrap_or(InvestmentStatus::Pending),
            expected_return: row.expected_return,
            duration_months: row.duration_months as u32,
        }).collect();

        Ok(investments)
    }

    // =============================================================================
    // VENTURE BENEFITS
    // =============================================================================

    pub async fn create_venture_benefit(&self, benefit: &VentureBenefit) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO venture_benefits (
                id, venture_id, title, description, benefit_type,
                delivery_date, is_delivered
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            benefit.id,
            benefit.venture_id,
            benefit.title,
            benefit.description,
            benefit.benefit_type.to_string(),
            benefit.delivery_date,
            benefit.is_delivered
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
            ORDER BY created_at ASC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let benefits = rows.into_iter().map(|row| VentureBenefit {
            id: row.id,
            venture_id: row.venture_id,
            title: row.title,
            description: row.description,
            benefit_type: BenefitType::from_string(&row.benefit_type).unwrap_or(BenefitType::Custom("Unknown".to_string())),
            delivery_date: row.delivery_date,
            is_delivered: row.is_delivered,
        }).collect();

        Ok(benefits)
    }

    // =============================================================================
    // REVENUE DISTRIBUTIONS
    // =============================================================================

    pub async fn create_revenue_distribution(&self, distribution: &RevenueDistribution) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO revenue_distributions (
                id, venture_id, total_revenue, artist_share, 
                fan_share, platform_fee, distributed_at, period_start, period_end
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
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
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_venture_distributions(&self, venture_id: Uuid) -> Result<Vec<RevenueDistribution>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM revenue_distributions 
            WHERE venture_id = $1
            ORDER BY distributed_at DESC
            "#,
            venture_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let distributions = rows.into_iter().map(|row| RevenueDistribution {
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