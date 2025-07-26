use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::fan_ventures::domain::entities::{
    ArtistVenture, FanInvestment, RevenueDistribution, VentureBenefit,
    VentureStatus, InvestmentStatus, InvestmentType, BenefitType, DeliveryMethod,
    VentureTier, CreateBenefitRequest, ArtistDashboard, VentureSummary, InvestmentSummary,
    MonthlyStats, VentureDashboard, InvestorSummary, FundingProgress, ActivityItem, ActivityType,
    BenefitDelivery, DeliveryStatus, CreateDeliveryRequest, UpdateDeliveryRequest,
    DeliverySummary, FanDeliveryHistory, VentureDeliveryStats,
    VentureDiscovery, ExplorationFilters, ExplorationSorting, VentureRecommendation,
    FanPreferences, VentureExploration, VentureCategory, RiskLevel
};
use crate::bounded_contexts::fan_ventures::infrastructure::postgres_repository::PostgresFanVenturesRepository;

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
    pub async fn get_artist_investments(&self, _artist_id: Uuid) -> Result<Vec<FanInvestment>, AppError> {
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
            tier_id: None,
            title,
            description: Some(description),
            benefit_type,
            value: 0.0, // Default value
            delivery_method: DeliveryMethod::Manual,
            delivery_date,
            created_at: Some(Utc::now()),
        };

        self.repository.create_venture_benefit(&benefit).await?;
        Ok(benefit)
    }

    /// Mark a benefit as delivered
    pub async fn mark_benefit_delivered(&self, _benefit_id: Uuid) -> Result<(), AppError> {
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

        let _investments = self.get_artist_investments(venture.artist_id).await?;
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
        let total_benefits_delivered = venture.benefits.len() as u32; // TODO: Implement proper delivery tracking

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

    // =============================================================================
    // VENTURE TIERS MANAGEMENT
    // =============================================================================

    /// Create a new tier for a venture
    pub async fn create_venture_tier(
        &self,
        venture_id: Uuid,
        name: String,
        min_investment: f64,
        max_investment: Option<f64>,
        description: Option<String>,
        benefits: Vec<CreateBenefitRequest>,
    ) -> Result<VentureTier, AppError> {
        // Validate tier parameters
        if min_investment <= 0.0 {
            return Err(AppError::DomainRuleViolation(
                "Minimum investment must be greater than 0".to_string()
            ));
        }

        if let Some(max) = max_investment {
            if max <= min_investment {
                return Err(AppError::DomainRuleViolation(
                    "Maximum investment must be greater than minimum investment".to_string()
                ));
            }
        }

        // Convert CreateBenefitRequest to VentureBenefit
        let venture_benefits: Vec<VentureBenefit> = benefits.into_iter().map(|b| VentureBenefit {
            id: Uuid::new_v4(),
            venture_id,
            tier_id: None, // Will be set after tier creation
            title: b.title,
            description: b.description,
            benefit_type: b.benefit_type,
            value: b.value,
            delivery_method: b.delivery_method,
            delivery_date: b.delivery_date,
            created_at: Some(Utc::now()),
        }).collect();

        let tier = VentureTier {
            id: Uuid::new_v4(),
            venture_id,
            name,
            min_investment,
            max_investment,
            description,
            created_at: Utc::now(),
            benefits: venture_benefits,
        };

        self.repository.create_venture_tier(&tier).await?;
        Ok(tier)
    }

    /// Get all tiers for a venture
    pub async fn get_venture_tiers(&self, venture_id: Uuid) -> Result<Vec<VentureTier>, AppError> {
        self.repository.get_venture_tiers(venture_id).await
    }

    /// Get a specific tier
    pub async fn get_venture_tier(&self, tier_id: Uuid) -> Result<Option<VentureTier>, AppError> {
        self.repository.get_venture_tier(tier_id).await
    }

    /// Update a tier
    pub async fn update_venture_tier(
        &self,
        tier_id: Uuid,
        name: Option<String>,
        min_investment: Option<f64>,
        max_investment: Option<f64>,
        description: Option<String>,
    ) -> Result<VentureTier, AppError> {
        let mut tier = self.repository.get_venture_tier(tier_id).await?
            .ok_or_else(|| AppError::NotFound("Tier not found".to_string()))?;

        if let Some(name) = name {
            tier.name = name;
        }
        if let Some(min_inv) = min_investment {
            if min_inv <= 0.0 {
                return Err(AppError::DomainRuleViolation(
                    "Minimum investment must be greater than 0".to_string()
                ));
            }
            tier.min_investment = min_inv;
        }
        if let Some(max_inv) = max_investment {
            if max_inv <= tier.min_investment {
                return Err(AppError::DomainRuleViolation(
                    "Maximum investment must be greater than minimum investment".to_string()
                ));
            }
            tier.max_investment = Some(max_inv);
        }
        if let Some(desc) = description {
            tier.description = Some(desc);
        }

        self.repository.update_venture_tier(&tier).await?;
        Ok(tier)
    }

    /// Delete a tier
    pub async fn delete_venture_tier(&self, tier_id: Uuid) -> Result<(), AppError> {
        self.repository.delete_venture_tier(tier_id).await
    }

    /// Get benefits for a specific investment amount
    pub async fn get_benefits_for_investment(
        &self,
        venture_id: Uuid,
        investment_amount: f64,
    ) -> Result<Vec<VentureBenefit>, AppError> {
        let tiers = self.get_venture_tiers(venture_id).await?;
        
        // Find the highest tier that the investment qualifies for
        let qualifying_tier = tiers.into_iter()
            .filter(|tier| investment_amount >= tier.min_investment)
            .filter(|tier| tier.max_investment.is_none() || investment_amount <= tier.max_investment.unwrap())
            .max_by(|a, b| a.min_investment.partial_cmp(&b.min_investment).unwrap_or(std::cmp::Ordering::Equal));

        match qualifying_tier {
            Some(tier) => Ok(tier.benefits),
            None => Ok(Vec::new()), // No qualifying tier found
        }
    }

    // =============================================================================
    // DASHBOARD SERVICES
    // =============================================================================

    /// Get comprehensive dashboard for an artist
    pub async fn get_artist_dashboard(&self, artist_id: Uuid) -> Result<ArtistDashboard, AppError> {
        // Get all ventures for the artist
        let ventures = self.repository.list_ventures_by_artist(artist_id).await?;
        
        let total_ventures = ventures.len() as u32;
        let active_ventures = ventures.iter()
            .filter(|v| v.status == VentureStatus::Open)
            .count() as u32;
        
        let total_funding_raised: f64 = ventures.iter()
            .map(|v| v.current_amount)
            .sum();
        
        let total_investors: u32 = ventures.iter()
            .map(|v| v.current_investors as u32)
            .sum::<u32>();
        
        // Get recent ventures (last 5)
        let recent_ventures: Vec<VentureSummary> = ventures.iter()
            .take(5)
            .map(|v| VentureSummary {
                venture_id: v.id,
                title: v.title.clone(),
                status: v.status.clone(),
                current_amount: v.current_amount,
                total_goal: v.total_goal,
                funding_progress: if v.total_goal > 0.0 {
                    (v.current_amount / v.total_goal) * 100.0
                } else {
                    0.0
                },
                total_investors: v.current_investors as u32,
                created_at: v.created_at,
                expires_at: v.expires_at,
            })
            .collect();
        
        // Get top performing ventures (by funding progress)
        let mut top_performing: Vec<VentureSummary> = ventures.iter()
            .filter(|v| v.status == VentureStatus::Open)
            .map(|v| VentureSummary {
                venture_id: v.id,
                title: v.title.clone(),
                status: v.status.clone(),
                current_amount: v.current_amount,
                total_goal: v.total_goal,
                funding_progress: if v.total_goal > 0.0 {
                    (v.current_amount / v.total_goal) * 100.0
                } else {
                    0.0
                },
                total_investors: v.current_investors as u32,
                created_at: v.created_at,
                expires_at: v.expires_at,
            })
            .collect();
        
        top_performing.sort_by(|a, b| b.funding_progress.partial_cmp(&a.funding_progress).unwrap_or(std::cmp::Ordering::Equal));
        top_performing.truncate(3);
        
        // Mock recent investments (TODO: Implement in repository)
        let recent_investments = vec![
            InvestmentSummary {
                investment_id: Uuid::new_v4(),
                venture_id: ventures.first().map(|v| v.id).unwrap_or_default(),
                venture_title: "My First Album".to_string(),
                fan_id: Uuid::new_v4(),
                fan_name: "John Doe".to_string(),
                investment_amount: 500.0,
                investment_type: InvestmentType::RevenueShare,
                status: InvestmentStatus::Active,
                created_at: Utc::now(),
            }
        ];
        
        // Mock monthly stats (TODO: Implement in repository)
        let monthly_stats = MonthlyStats {
            month: "2024-01".to_string(),
            new_ventures: 2,
            new_investments: 15,
            funding_raised: 2500.0,
            new_investors: 12,
        };
        
        Ok(ArtistDashboard {
            artist_id,
            total_ventures,
            active_ventures,
            total_funding_raised,
            total_investors,
            recent_ventures,
            top_performing_ventures: top_performing,
            recent_investments,
            monthly_stats,
        })
    }

    /// Get detailed dashboard for a specific venture
    pub async fn get_venture_dashboard(&self, venture_id: Uuid) -> Result<VentureDashboard, AppError> {
        let venture = self.get_venture(venture_id).await?
            .ok_or_else(|| AppError::NotFound("Venture not found".to_string()))?;
        
        let tiers = self.get_venture_tiers(venture_id).await?;
        
        // Mock investors (TODO: Implement in repository)
        let investors = vec![
            InvestorSummary {
                fan_id: Uuid::new_v4(),
                fan_name: "Alice Smith".to_string(),
                investment_amount: 1000.0,
                investment_date: Utc::now(),
                tier_qualification: Some("Gold".to_string()),
                total_benefits_received: 3,
                status: InvestmentStatus::Active,
            },
            InvestorSummary {
                fan_id: Uuid::new_v4(),
                fan_name: "Bob Johnson".to_string(),
                investment_amount: 500.0,
                investment_date: Utc::now(),
                tier_qualification: Some("Silver".to_string()),
                total_benefits_received: 1,
                status: InvestmentStatus::Active,
            }
        ];
        
        let funding_progress = FundingProgress {
            current_amount: venture.current_amount,
            total_goal: venture.total_goal,
            percentage_complete: if venture.total_goal > 0.0 {
                (venture.current_amount / venture.total_goal) * 100.0
            } else {
                0.0
            },
            days_remaining: venture.expires_at.map(|expires| {
                let now = Utc::now();
                if expires > now {
                    (expires - now).num_days() as i32
                } else {
                    0
                }
            }),
            average_investment: if venture.current_investors > 0 {
                venture.current_amount / venture.current_investors as f64
            } else {
                0.0
            },
            largest_investment: 1000.0, // TODO: Calculate from actual investments
        };
        
        // Mock recent activity (TODO: Implement activity tracking)
        let recent_activity = vec![
            ActivityItem {
                id: Uuid::new_v4(),
                activity_type: ActivityType::InvestmentMade,
                description: "Alice Smith invested $1000".to_string(),
                amount: Some(1000.0),
                user_id: Some(Uuid::new_v4()),
                user_name: Some("Alice Smith".to_string()),
                created_at: Utc::now(),
            },
            ActivityItem {
                id: Uuid::new_v4(),
                activity_type: ActivityType::TierCreated,
                description: "Gold tier created".to_string(),
                amount: None,
                user_id: None,
                user_name: None,
                created_at: Utc::now(),
            }
        ];
        
        let analytics = self.get_venture_analytics(venture_id).await?;
        
        Ok(VentureDashboard {
            venture_id,
            venture,
            tiers,
            investors,
            funding_progress,
            recent_activity,
            analytics: crate::bounded_contexts::fan_ventures::domain::entities::VentureAnalytics {
                venture_id: analytics.venture_id,
                total_investors: analytics.total_investors as i32,
                average_investment: analytics.average_investment,
                funding_progress: analytics.funding_progress,
                total_revenue_generated: analytics.total_revenue_generated,
                total_benefits_delivered: analytics.total_benefits_delivered as i32,
                investor_satisfaction: analytics.investor_satisfaction,
                completion_rate: analytics.completion_rate,
            },
        })
    }

    // =============================================================================
    // BENEFIT DELIVERY SERVICES
    // =============================================================================

    /// Create a new benefit delivery
    pub async fn create_benefit_delivery(
        &self,
        venture_id: Uuid,
        request: CreateDeliveryRequest,
    ) -> Result<BenefitDelivery, AppError> {
        // Get the benefit to validate it exists and belongs to the venture
        let benefit = self.get_venture_benefit(request.benefit_id).await?
            .ok_or_else(|| AppError::NotFound("Benefit not found".to_string()))?;
        
        if benefit.venture_id != venture_id {
            return Err(AppError::DomainRuleViolation(
                "Benefit does not belong to this venture".to_string()
            ));
        }

        let delivery = BenefitDelivery {
            id: Uuid::new_v4(),
            benefit_id: request.benefit_id,
            venture_id,
            fan_id: request.fan_id,
            tier_id: benefit.tier_id,
            delivery_status: DeliveryStatus::Pending,
            delivery_method: request.delivery_method,
            delivery_date: None,
            tracking_info: None,
            notes: request.notes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repository.create_benefit_delivery(&delivery).await?;
        Ok(delivery)
    }

    /// Update benefit delivery status
    pub async fn update_benefit_delivery(
        &self,
        delivery_id: Uuid,
        request: UpdateDeliveryRequest,
    ) -> Result<BenefitDelivery, AppError> {
        let mut delivery = self.repository.get_benefit_delivery(delivery_id).await?
            .ok_or_else(|| AppError::NotFound("Delivery not found".to_string()))?;

        delivery.delivery_status = request.delivery_status;
        delivery.tracking_info = request.tracking_info;
        delivery.notes = request.notes;
        delivery.updated_at = Utc::now();

        // Set delivery date if status is delivered
        if delivery.delivery_status == DeliveryStatus::Delivered && delivery.delivery_date.is_none() {
            delivery.delivery_date = Some(Utc::now());
        }

        self.repository.update_benefit_delivery(delivery_id, &delivery).await?;
        Ok(delivery)
    }

    /// Get delivery by ID
    pub async fn get_benefit_delivery(&self, delivery_id: Uuid) -> Result<Option<BenefitDelivery>, AppError> {
        self.repository.get_benefit_delivery(delivery_id).await
    }

    /// Get all deliveries for a fan
    pub async fn get_fan_delivery_history(&self, fan_id: Uuid) -> Result<FanDeliveryHistory, AppError> {
        let deliveries = self.repository.get_fan_deliveries(fan_id).await?;
        
        let total_deliveries = deliveries.len() as u32;
        let pending_deliveries = deliveries.iter()
            .filter(|d| d.delivery_status == DeliveryStatus::Pending)
            .count() as u32;
        let completed_deliveries = deliveries.iter()
            .filter(|d| d.delivery_status == DeliveryStatus::Delivered)
            .count() as u32;

        // Convert to DeliverySummary (TODO: Get benefit and venture titles)
        let delivery_summaries = deliveries.into_iter().map(|d| DeliverySummary {
            delivery_id: d.id,
            benefit_title: "Benefit Title".to_string(), // TODO: Get from benefit
            venture_title: "Venture Title".to_string(), // TODO: Get from venture
            fan_name: "Fan Name".to_string(), // TODO: Get from user service
            delivery_status: d.delivery_status,
            delivery_method: d.delivery_method,
            delivery_date: d.delivery_date,
            created_at: d.created_at,
        }).collect();

        Ok(FanDeliveryHistory {
            fan_id,
            total_deliveries,
            pending_deliveries,
            completed_deliveries,
            deliveries: delivery_summaries,
        })
    }

    /// Get delivery statistics for a venture
    pub async fn get_venture_delivery_stats(&self, venture_id: Uuid) -> Result<VentureDeliveryStats, AppError> {
        let deliveries = self.repository.get_venture_deliveries(venture_id).await?;
        
        let total_benefits = deliveries.len() as u32;
        let pending_deliveries = deliveries.iter()
            .filter(|d| d.delivery_status == DeliveryStatus::Pending)
            .count() as u32;
        let completed_deliveries = deliveries.iter()
            .filter(|d| d.delivery_status == DeliveryStatus::Delivered)
            .count() as u32;

        let delivery_success_rate = if total_benefits > 0 {
            completed_deliveries as f64 / total_benefits as f64 * 100.0
        } else {
            0.0
        };

        // Calculate average delivery time (TODO: Implement proper calculation)
        let average_delivery_time_days = 3.5; // Mock value

        Ok(VentureDeliveryStats {
            venture_id,
            total_benefits,
            pending_deliveries,
            completed_deliveries,
            delivery_success_rate,
            average_delivery_time_days,
        })
    }

    /// Get venture benefit by ID (helper method)
    async fn get_venture_benefit(&self, _benefit_id: Uuid) -> Result<Option<VentureBenefit>, AppError> {
        // TODO: Implement in repository
        // For now, return None to avoid compilation errors
        Ok(None)
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
        self.repository.search_ventures(filters, sorting, page, page_size).await
    }

    /// Get venture recommendations for a fan
    pub async fn get_venture_recommendations(
        &self,
        fan_id: Uuid,
        limit: u32,
    ) -> Result<Vec<VentureRecommendation>, AppError> {
        self.repository.get_venture_recommendations(fan_id, limit).await
    }

    /// Save fan preferences
    pub async fn save_fan_preferences(&self, preferences: &FanPreferences) -> Result<(), AppError> {
        // Validate preferences
        if preferences.min_investment > preferences.max_investment {
            return Err(AppError::DomainRuleViolation(
                "Minimum investment cannot be greater than maximum investment".to_string()
            ));
        }

        if preferences.min_investment < 0.0 || preferences.max_investment < 0.0 {
            return Err(AppError::DomainRuleViolation(
                "Investment amounts must be positive".to_string()
            ));
        }

        self.repository.save_fan_preferences(preferences).await
    }

    /// Get fan preferences
    pub async fn get_fan_preferences(&self, fan_id: Uuid) -> Result<Option<FanPreferences>, AppError> {
        self.repository.get_fan_preferences(fan_id).await
    }

    /// Get venture exploration data
    pub async fn get_venture_exploration(&self, fan_id: Option<Uuid>) -> Result<VentureExploration, AppError> {
        // Get featured ventures (mock for now)
        let featured_ventures = vec![
            VentureDiscovery {
                venture_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                artist_name: "Featured Artist".to_string(),
                artist_avatar: Some("https://example.com/featured.jpg".to_string()),
                title: "Featured Venture".to_string(),
                description: Some("A featured venture for exploration".to_string()),
                investment_type: InvestmentType::RevenueShare,
                min_investment: 50.0,
                max_investment: Some(500.0),
                total_goal: 5000.0,
                current_amount: 2500.0,
                funding_progress: 50.0,
                total_investors: 15,
                status: VentureStatus::Open,
                expires_at: Some(Utc::now() + chrono::Duration::days(45)),
                days_remaining: Some(45),
                created_at: Utc::now(),
                top_tiers: Vec::new(),
                tags: vec!["featured".to_string(), "music".to_string()],
                category: VentureCategory::Music,
                risk_level: RiskLevel::Low,
                expected_return: 12.0,
                artist_rating: 4.8,
                artist_previous_ventures: 5,
                artist_success_rate: 92.0,
            }
        ];

        // Get trending ventures (mock for now)
        let trending_ventures = vec![
            VentureDiscovery {
                venture_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                artist_name: "Trending Artist".to_string(),
                artist_avatar: Some("https://example.com/trending.jpg".to_string()),
                title: "Trending Venture".to_string(),
                description: Some("A trending venture with high interest".to_string()),
                investment_type: InvestmentType::ExclusiveContent,
                min_investment: 25.0,
                max_investment: Some(250.0),
                total_goal: 2500.0,
                current_amount: 2000.0,
                funding_progress: 80.0,
                total_investors: 40,
                status: VentureStatus::Open,
                expires_at: Some(Utc::now() + chrono::Duration::days(15)),
                days_remaining: Some(15),
                created_at: Utc::now(),
                top_tiers: Vec::new(),
                tags: vec!["trending".to_string(), "exclusive".to_string()],
                category: VentureCategory::VisualArts,
                risk_level: RiskLevel::Medium,
                expected_return: 18.0,
                artist_rating: 4.6,
                artist_previous_ventures: 3,
                artist_success_rate: 88.0,
            }
        ];

        // Combine featured and trending ventures
        let mut all_ventures = Vec::new();
        all_ventures.extend(featured_ventures);
        all_ventures.extend(trending_ventures);

        // Get personalized recommendations if fan_id is provided
        if let Some(fan_id) = fan_id {
            let recommendations = self.get_venture_recommendations(fan_id, 5).await.unwrap_or_default();
            // Add recommended ventures to the list
            for recommendation in recommendations {
                // Mock venture for recommendation
                let recommended_venture = VentureDiscovery {
                    venture_id: recommendation.venture_id,
                    artist_id: Uuid::new_v4(),
                    artist_name: "Recommended Artist".to_string(),
                    artist_avatar: Some("https://example.com/recommended.jpg".to_string()),
                    title: "Recommended Venture".to_string(),
                    description: Some("A venture recommended for you".to_string()),
                    investment_type: InvestmentType::RevenueShare,
                    min_investment: 75.0,
                    max_investment: Some(750.0),
                    total_goal: 7500.0,
                    current_amount: 3750.0,
                    funding_progress: 50.0,
                    total_investors: 20,
                    status: VentureStatus::Open,
                    expires_at: Some(Utc::now() + chrono::Duration::days(30)),
                    days_remaining: Some(30),
                    created_at: Utc::now(),
                    top_tiers: Vec::new(),
                    tags: vec!["recommended".to_string(), "personalized".to_string()],
                    category: VentureCategory::Music,
                    risk_level: RiskLevel::Medium,
                    expected_return: 14.0,
                    artist_rating: 4.7,
                    artist_previous_ventures: 4,
                    artist_success_rate: 90.0,
                };
                all_ventures.push(recommended_venture);
            }
        }

        let total_count = all_ventures.len() as u32;
        let page_size = all_ventures.len() as u32;

        Ok(VentureExploration {
            ventures: all_ventures,
            total_count,
            page: 1,
            page_size,
            filters: ExplorationFilters {
                categories: None,
                investment_types: None,
                risk_levels: None,
                min_investment: None,
                max_investment: None,
                min_funding_progress: None,
                max_funding_progress: None,
                min_artist_rating: None,
                tags: None,
                search_query: None,
                expires_within_days: None,
            },
            sorting: ExplorationSorting::Newest,
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