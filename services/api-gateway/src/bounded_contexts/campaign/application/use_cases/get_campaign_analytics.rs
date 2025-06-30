use serde::{Deserialize, Serialize};

use crate::bounded_contexts::campaign::domain::value_objects::CampaignId;
use crate::bounded_contexts::campaign::domain::aggregates::{CampaignAnalytics, NFTDistribution};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCampaignAnalyticsQuery {
    pub campaign_id: String,
    pub include_predictions: bool,
    pub include_optimization_suggestions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCampaignAnalyticsResponse {
    pub success: bool,
    pub campaign_id: String,
    pub analytics: CampaignAnalytics,
    pub predictions: Option<CampaignPredictions>,
    pub optimization_suggestions: Option<Vec<OptimizationSuggestion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignPredictions {
    pub projected_completion_date: Option<chrono::DateTime<chrono::Utc>>,
    pub projected_final_sales: u32,
    pub projected_revenue: f64,
    pub confidence_score: f64,
    pub key_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub suggestion_type: String,
    pub description: String,
    pub expected_impact: String,
    pub confidence: f64,
    pub implementation_effort: String,
}

pub struct GetCampaignAnalyticsUseCase {}

impl GetCampaignAnalyticsUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, query: GetCampaignAnalyticsQuery) -> Result<GetCampaignAnalyticsResponse, String> {
        // Validate query
        self.validate_query(&query)?;

        // Parse campaign ID
        let _campaign_id = CampaignId::from_string(&query.campaign_id)
            .map_err(|e| format!("Invalid campaign ID: {}", e))?;

        // Simulate analytics data
        let analytics = self.simulate_analytics();
        
        let predictions = if query.include_predictions {
            Some(self.generate_predictions(&analytics))
        } else {
            None
        };

        let optimization_suggestions = if query.include_optimization_suggestions {
            Some(self.generate_optimization_suggestions(&analytics))
        } else {
            None
        };

        Ok(GetCampaignAnalyticsResponse {
            success: true,
            campaign_id: query.campaign_id,
            analytics,
            predictions,
            optimization_suggestions,
        })
    }

    fn validate_query(&self, query: &GetCampaignAnalyticsQuery) -> Result<(), String> {
        if query.campaign_id.trim().is_empty() {
            return Err("Campaign ID is required".to_string());
        }
        Ok(())
    }

    fn simulate_analytics(&self) -> CampaignAnalytics {
        CampaignAnalytics {
            campaign_id: uuid::Uuid::new_v4(),
            total_nfts_sold: 450,
            total_revenue: 4500.0,
            completion_percentage: 45.0,
            unique_buyers: 380,
            average_purchase_amount: 11.84,
            sales_velocity: 30.0,
            days_remaining: 15,
            is_successful: false,
            boost_efficiency: 2.1,
            nft_distribution: NFTDistribution {
                total_owners: 380,
                max_nfts_per_owner: 5,
                average_nfts_per_owner: 1.18,
                distribution_fairness: 0.85,
            },
            average_nfts_per_buyer: 1.18,
            sales_velocity_per_day: 30.0,
            distribution_fairness_score: 0.85,
            milestone_progress: 0.45,
            predicted_final_sales: 850,
        }
    }

    fn generate_predictions(&self, analytics: &CampaignAnalytics) -> CampaignPredictions {
        let days_to_completion = if analytics.sales_velocity_per_day > 0.0 {
            let remaining_sales = if analytics.predicted_final_sales > analytics.total_nfts_sold {
                analytics.predicted_final_sales - analytics.total_nfts_sold
            } else {
                0
            };
            (remaining_sales as f64 / analytics.sales_velocity_per_day).ceil() as i64
        } else {
            analytics.days_remaining as i64
        };

        let projected_completion_date = if days_to_completion <= analytics.days_remaining as i64 {
            Some(chrono::Utc::now() + chrono::Duration::days(days_to_completion))
        } else {
            None
        };

        CampaignPredictions {
            projected_completion_date,
            projected_final_sales: analytics.predicted_final_sales,
            projected_revenue: analytics.predicted_final_sales as f64 * 10.0,
            confidence_score: if analytics.total_nfts_sold > 100 { 0.85 } else { 0.65 },
            key_factors: vec![
                "Current sales velocity".to_string(),
                "Time remaining".to_string(),
                "Market sentiment".to_string(),
                "Price optimization".to_string(),
            ],
        }
    }

    fn generate_optimization_suggestions(&self, analytics: &CampaignAnalytics) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        if analytics.sales_velocity_per_day < 20.0 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: "price_reduction".to_string(),
                description: "Consider reducing NFT price by 10-15% to boost sales velocity".to_string(),
                expected_impact: "30-50% increase in daily sales".to_string(),
                confidence: 0.78,
                implementation_effort: "Low".to_string(),
            });
        }

        if analytics.unique_buyers < 200 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: "marketing_boost".to_string(),
                description: "Increase social media marketing and influencer partnerships".to_string(),
                expected_impact: "25-40% increase in unique buyers".to_string(),
                confidence: 0.72,
                implementation_effort: "Medium".to_string(),
            });
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_query() -> GetCampaignAnalyticsQuery {
        GetCampaignAnalyticsQuery {
            campaign_id: uuid::Uuid::new_v4().to_string(),
            include_predictions: true,
            include_optimization_suggestions: true,
        }
    }

    #[test]
    fn test_get_analytics_success() {
        let use_case = GetCampaignAnalyticsUseCase::new();
        let query = create_valid_query();
        
        let result = use_case.execute(query.clone());
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert_eq!(response.campaign_id, query.campaign_id);
        assert!(response.analytics.total_nfts_sold > 0);
    }

    #[test]
    fn test_get_analytics_empty_campaign_id() {
        let use_case = GetCampaignAnalyticsUseCase::new();
        let mut query = create_valid_query();
        query.campaign_id = "".to_string();
        
        let result = use_case.execute(query);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Campaign ID is required"));
    }
}