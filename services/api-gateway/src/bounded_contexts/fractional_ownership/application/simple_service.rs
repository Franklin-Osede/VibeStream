use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::fractional_ownership::domain::entities::{
    ArtistVenture, FanInvestment, RevenueDistribution, VentureBenefit,
    VentureStatus, InvestmentStatus, InvestmentType, BenefitType
};
use crate::bounded_contexts::fractional_ownership::infrastructure::postgres_repository::PostgresFanVenturesRepository;

// =============================================================================
// FAN VENTURES SERVICE (Reemplazando Fractional Ownership)
// =============================================================================

pub struct FanVenturesService {
    repository: PostgresFanVenturesRepository,
}

impl FanVenturesService {
    pub fn new(repository: PostgresFanVenturesRepository) -> Self {
        Self { repository }
    }

    // =============================================================================
    // ARTIST VENTURES MANAGEMENT
    // =============================================================================

    /// Create a new venture for an artist
    pub async fn create_venture(
        &self,
        artist_id: Uuid,
        title: String,
        description: String,
        investment_type: InvestmentType,
        min_investment: f64,
        max_investment: f64,
        total_goal: f64,
        max_investors: Option<u32>,
        expires_at: DateTime<Utc>,
        benefits: Vec<VentureBenefit>,
    ) -> Result<ArtistVenture, AppError> {
        // Validate venture parameters
        if min_investment <= 0.0 || max_investment <= 0.0 || total_goal <= 0.0 {
            return Err(AppError::DomainRuleViolation(
                "Investment amounts must be greater than 0".to_string()
            ));
        }

        if max_investment < min_investment {
            return Err(AppError::DomainRuleViolation(
                "Max investment cannot be less than min investment".to_string()
            ));
        }

        if expires_at <= Utc::now() {
            return Err(AppError::DomainRuleViolation(
                "Venture must expire in the future".to_string()
            ));
        }

        let venture = ArtistVenture {
            id: Uuid::new_v4(),
            artist_id,
            title,
            description: Some(description),
            investment_type,
            min_investment,
            max_investment: Some(max_investment),
            total_goal,
            current_amount: 0.0,
            max_investors: max_investors.map(|v| v as i32),
            current_investors: 0,
            created_at: Utc::now(),
            expires_at: Some(expires_at),
            status: VentureStatus::Draft,
            benefits,
        };

        self.repository.create_venture(&venture).await?;
        Ok(venture)
    }

    /// Get a venture by ID
    pub async fn get_venture(&self, venture_id: Uuid) -> Result<Option<ArtistVenture>, AppError> {
        self.repository.get_venture(venture_id).await
    }

    /// List all open ventures
    pub async fn list_open_ventures(&self, limit: Option<i32>) -> Result<Vec<ArtistVenture>, AppError> {
        self.repository.list_open_ventures(limit).await
    }

    /// Activate a venture (change status from Draft to Open)
    pub async fn activate_venture(&self, venture_id: Uuid) -> Result<ArtistVenture, AppError> {
        let mut venture = self.repository.get_venture(venture_id).await?
            .ok_or_else(|| AppError::NotFound("Venture not found".to_string()))?;

        if venture.status != VentureStatus::Draft {
            return Err(AppError::DomainRuleViolation(
                "Only draft ventures can be activated".to_string()
            ));
        }

        venture.status = VentureStatus::Open;
        // TODO: Update venture status in database
        Ok(venture)
    }

    // =============================================================================
    // FAN INVESTMENTS
    // =============================================================================

    /// Make an investment in a venture
    pub async fn make_investment(
        &self,
        venture_id: Uuid,
        fan_id: Uuid,
        investment_amount: f64,
        investment_type: InvestmentType,
        expected_return: f64,
        duration_months: u32,
    ) -> Result<FanInvestment, AppError> {
        // Get venture to validate
        let venture = self.repository.get_venture(venture_id).await?
            .ok_or_else(|| AppError::NotFound("Venture not found".to_string()))?;

        // Validate investment
        if venture.status != VentureStatus::Open {
            return Err(AppError::DomainRuleViolation(
                "Venture is not open for investments".to_string()
            ));
        }

        if investment_amount < venture.min_investment {
            return Err(AppError::DomainRuleViolation(
                format!("Investment amount must be at least ${}", venture.min_investment)
            ));
        }

        if let Some(max_inv) = venture.max_investment {
            if investment_amount > max_inv {
                return Err(AppError::DomainRuleViolation(
                    format!("Investment amount must be at most ${}", max_inv)
                ));
            }
        }

        if venture.current_amount + investment_amount > venture.total_goal {
            return Err(AppError::DomainRuleViolation(
                "Investment would exceed venture goal".to_string()
            ));
        }

        if let Some(max_investors) = venture.max_investors {
            if venture.current_investors >= max_investors {
                return Err(AppError::DomainRuleViolation(
                    "Venture has reached maximum number of investors".to_string()
                ));
            }
        }

        let investment = FanInvestment {
            id: Uuid::new_v4(),
            artist_id: venture.artist_id,
            fan_id,
            investment_amount,
            investment_type,
            created_at: Utc::now(),
            status: InvestmentStatus::Pending,
            expected_return,
            duration_months: duration_months as i32,
        };

        self.repository.create_fan_investment(&investment).await?;
        Ok(investment)
    }

    /// Get all investments for a fan
    pub async fn get_fan_investments(&self, fan_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
        self.repository.get_fan_investments(fan_id).await
    }

    /// Get all investments for an artist
    pub async fn get_artist_investments(&self, artist_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
        // TODO: Implement this method in repository
        Ok(Vec::new())
    }

    // =============================================================================
    // REVENUE DISTRIBUTION
    // =============================================================================

    /// Distribute revenue for a venture
    pub async fn distribute_revenue(
        &self,
        venture_id: Uuid,
        total_revenue: f64,
        platform_fee_percentage: f64,
        artist_share_percentage: f64,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<RevenueDistribution, AppError> {
        // Validate percentages
        if platform_fee_percentage + artist_share_percentage > 100.0 {
            return Err(AppError::DomainRuleViolation(
                "Total percentages cannot exceed 100%".to_string()
            ));
        }

        if period_start >= period_end {
            return Err(AppError::DomainRuleViolation(
                "Period start must be before period end".to_string()
            ));
        }

        let platform_fee = total_revenue * (platform_fee_percentage / 100.0);
        let artist_share = total_revenue * (artist_share_percentage / 100.0);
        let fan_share = total_revenue - platform_fee - artist_share;

        let distribution = RevenueDistribution {
            id: Uuid::new_v4(),
            venture_id,
            total_revenue,
            artist_share,
            fan_share,
            platform_fee,
            distributed_at: Utc::now(),
            period_start,
            period_end,
        };

        self.repository.create_revenue_distribution(&distribution).await?;
        Ok(distribution)
    }

    /// Get revenue distributions for a venture
    pub async fn get_venture_distributions(&self, venture_id: Uuid) -> Result<Vec<RevenueDistribution>, AppError> {
        self.repository.get_venture_distributions(venture_id).await
    }

    // =============================================================================
    // VENTURE BENEFITS
    // =============================================================================

    /// Add a benefit to a venture
    pub async fn add_venture_benefit(
        &self,
        venture_id: Uuid,
        title: String,
        description: String,
        benefit_type: BenefitType,
        delivery_date: Option<DateTime<Utc>>,
    ) -> Result<VentureBenefit, AppError> {
        let benefit = VentureBenefit {
            id: Uuid::new_v4(),
            venture_id,
            title,
            description,
            benefit_type,
            delivery_date,
            is_delivered: false,
        };

        self.repository.create_venture_benefit(&benefit).await?;
        Ok(benefit)
    }

    /// Mark a benefit as delivered
    pub async fn mark_benefit_delivered(&self, benefit_id: Uuid) -> Result<(), AppError> {
        // TODO: Implement this method in repository
        Ok(())
    }

    // =============================================================================
    // ANALYTICS & QUERIES
    // =============================================================================

    /// Get fan portfolio summary
    pub async fn get_fan_portfolio(&self, fan_id: Uuid) -> Result<FanPortfolio, AppError> {
        let investments = self.get_fan_investments(fan_id).await?;
        
        let total_invested: f64 = investments.iter().map(|i| i.investment_amount).sum();
        let total_returned: f64 = investments.iter().map(|i| i.expected_return).sum();
        let active_investments = investments.iter().filter(|i| i.status == InvestmentStatus::Active).count() as u32;
        let completed_investments = investments.iter().filter(|i| i.status == InvestmentStatus::Completed).count() as u32;
        
        // Get unique artists
        let favorite_artists: Vec<Uuid> = investments.iter()
            .map(|i| i.artist_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        Ok(FanPortfolio {
            fan_id,
            total_invested,
            total_returned,
            active_investments,
            completed_investments,
            favorite_artists,
            total_benefits_received: 0, // TODO: Implement benefits tracking
        })
    }

    /// Get venture analytics
    pub async fn get_venture_analytics(&self, venture_id: Uuid) -> Result<VentureAnalytics, AppError> {
        let venture = self.get_venture(venture_id).await?
            .ok_or_else(|| AppError::NotFound("Venture not found".to_string()))?;

        let investments = self.get_artist_investments(venture.artist_id).await?;
        let distributions = self.get_venture_distributions(venture_id).await?;
        
        let total_investors = venture.current_investors;
        let average_investment = if total_investors > 0 {
            venture.current_amount / total_investors as f64
        } else {
            0.0
        };

        let funding_progress = if venture.total_goal > 0.0 {
            (venture.current_amount / venture.total_goal) * 100.0
        } else {
            0.0
        };

        let total_revenue_generated: f64 = distributions.iter().map(|d| d.total_revenue).sum();
        let total_benefits_delivered = venture.benefits.iter().filter(|b| b.is_delivered).count() as u32;

        Ok(VentureAnalytics {
            venture_id,
            total_investors: total_investors as u32,
            average_investment,
            funding_progress,
            total_revenue_generated,
            total_benefits_delivered,
            investor_satisfaction: 0.0, // TODO: Implement satisfaction tracking
            completion_rate: 0.0, // TODO: Implement completion rate calculation
        })
    }
}

// =============================================================================
// ANALYTICS STRUCTURES
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FanPortfolio {
    pub fan_id: Uuid,
    pub total_invested: f64,
    pub total_returned: f64,
    pub active_investments: u32,
    pub completed_investments: u32,
    pub favorite_artists: Vec<Uuid>,
    pub total_benefits_received: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VentureAnalytics {
    pub venture_id: Uuid,
    pub total_investors: u32,
    pub average_investment: f64,
    pub funding_progress: f64,
    pub total_revenue_generated: f64,
    pub total_benefits_delivered: u32,
    pub investor_satisfaction: f64,
    pub completion_rate: f64,
} 