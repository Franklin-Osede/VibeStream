// TODO: Implement ShareTradingService, RevenueDistributionService 

use crate::domain::entities::{FractionalSong, ShareOwnership};
use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount};
use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

/// Servicio de dominio para cálculos financieros complejos de participaciones fraccionadas
pub struct FractionalOwnershipPricingService;

impl FractionalOwnershipPricingService {
    /// Calcular precio dinámico de acciones basado en:
    /// - Ingresos históricos de la canción
    /// - Demanda del mercado (% de acciones vendidas)
    /// - Popularidad del artista
    /// - Tendencias del mercado
    pub fn calculate_dynamic_share_price(
        current_price: &SharePrice,
        total_revenue: &RevenueAmount,
        shares_sold_percentage: f64,
        market_demand_multiplier: f64,
        artist_popularity_score: f64, // 0.0 - 10.0
        days_since_creation: i64,
    ) -> Result<SharePrice, FractionalOwnershipError> {
        // Factor base: ingresos por acción
        let revenue_per_share = if shares_sold_percentage > 0.0 {
            total_revenue.value() / (1000.0 * shares_sold_percentage / 100.0) // Asumimos 1000 acciones máximo
        } else {
            0.0
        };

        // Multiplicador por demanda (más vendidas = más caras)
        let demand_multiplier = 1.0 + (shares_sold_percentage / 100.0) * 0.5; // Hasta 50% más caro

        // Multiplicador por popularidad del artista
        let popularity_multiplier = 1.0 + (artist_popularity_score / 10.0) * 0.3; // Hasta 30% más caro

        // Multiplicador temporal (precio aumenta con el tiempo hasta un límite)
        let time_multiplier = if days_since_creation <= 365 {
            1.0 + (days_since_creation as f64 / 365.0) * 0.2 // Hasta 20% más caro en un año
        } else {
            1.2 // Máximo 20% después del primer año
        };

        // Factor de estabilización de ingresos
        let revenue_stability_factor = if revenue_per_share > current_price.value() * 0.1 {
            1.0 + (revenue_per_share / current_price.value()).min(2.0) * 0.1 // Hasta 20% más por buenos ingresos
        } else {
            1.0
        };

        let new_price = current_price.value() 
            * demand_multiplier 
            * popularity_multiplier 
            * time_multiplier 
            * market_demand_multiplier
            * revenue_stability_factor;

        // Aplicar límites razonables
        let final_price = new_price.max(0.01).min(10000.0); // Entre $0.01 y $10,000

        SharePrice::new(final_price)
    }

    /// Calcular valor justo de mercado para una transferencia entre usuarios
    pub fn calculate_fair_market_value(
        aggregate: &FractionalOwnershipAggregate,
        shares_quantity: u32,
        artist_popularity_score: f64,
        market_conditions: MarketConditions,
    ) -> Result<SharePrice, FractionalOwnershipError> {
        let current_price = aggregate.fractional_song().share_price();
        let sold_percentage = aggregate.fractional_song().sold_percentage();
        
        // Descuento por volumen (para ventas grandes)
        let volume_discount = if shares_quantity > 50 {
            0.95 // 5% descuento
        } else if shares_quantity > 20 {
            0.98 // 2% descuento
        } else {
            1.0
        };

        // Ajuste por condiciones del mercado
        let market_multiplier = match market_conditions {
            MarketConditions::Bull => 1.1,    // 10% más caro en mercado alcista
            MarketConditions::Bear => 0.9,    // 10% más barato en mercado bajista
            MarketConditions::Stable => 1.0,
            MarketConditions::Volatile => 0.95, // 5% descuento por volatilidad
        };

        let fair_price = current_price.value() * volume_discount * market_multiplier;
        SharePrice::new(fair_price)
    }

    /// Evaluar si una canción es una buena inversión
    pub fn evaluate_investment_potential(
        aggregate: &FractionalOwnershipAggregate,
        artist_historical_performance: ArtistPerformanceMetrics,
        genre_market_trends: GenreMarketTrends,
    ) -> InvestmentRating {
        let mut score = 0.0;

        // Factor 1: Ratio precio/ingresos actual
        let current_revenue = aggregate.fractional_song().total_revenue().value();
        let market_value = aggregate.calculate_market_value().value();
        
        let pe_ratio = if current_revenue > 0.0 {
            market_value / current_revenue
        } else {
            100.0 // Penalizar canciones sin ingresos
        };

        score += match pe_ratio {
            ratio if ratio < 5.0 => 25.0,   // Excelente valor
            ratio if ratio < 10.0 => 20.0,  // Buen valor
            ratio if ratio < 20.0 => 15.0,  // Valor promedio
            ratio if ratio < 50.0 => 10.0,  // Sobrevalorado
            _ => 0.0,                       // Muy sobrevalorado
        };

        // Factor 2: Performance histórica del artista
        score += artist_historical_performance.average_revenue_growth * 10.0;

        // Factor 3: Tendencias del género
        score += match genre_market_trends.growth_trend {
            TrendDirection::Growing => 15.0,
            TrendDirection::Stable => 10.0,
            TrendDirection::Declining => 0.0,
        };

        // Factor 4: Diversificación del ownership
        let ownership_count = aggregate.ownerships().len();
        score += match ownership_count {
            count if count > 20 => 15.0,    // Bien diversificado
            count if count > 10 => 10.0,    // Moderadamente diversificado
            count if count > 5 => 5.0,      // Poco diversificado
            _ => 0.0,                       // Concentrado
        };

        // Factor 5: Liquidez (% de acciones disponibles)
        let liquidity_score = aggregate.fractional_song().sold_percentage();
        score += (liquidity_score / 100.0) * 10.0;

        match score {
            s if s >= 70.0 => InvestmentRating::Excellent,
            s if s >= 55.0 => InvestmentRating::Good,
            s if s >= 40.0 => InvestmentRating::Average,
            s if s >= 25.0 => InvestmentRating::Poor,
            _ => InvestmentRating::VeryPoor,
        }
    }
}

/// Servicio de dominio para análisis de riesgo de inversión
pub struct RiskAnalysisService;

impl RiskAnalysisService {
    /// Calcular riesgo de una inversión en acciones fraccionadas
    pub fn calculate_investment_risk(
        aggregate: &FractionalOwnershipAggregate,
        artist_metrics: ArtistPerformanceMetrics,
        market_volatility: f64, // 0.0 - 1.0
    ) -> RiskAssessment {
        let mut risk_factors = Vec::new();
        let mut total_risk_score = 0.0;

        // Factor 1: Concentración de ownership
        let ownership_concentration = Self::calculate_ownership_concentration(aggregate);
        if ownership_concentration > 0.7 {
            risk_factors.push(RiskFactor::HighOwnershipConcentration);
            total_risk_score += 20.0;
        }

        // Factor 2: Volatilidad de ingresos del artista
        if artist_metrics.revenue_volatility > 0.5 {
            risk_factors.push(RiskFactor::HighRevenueVolatility);
            total_risk_score += 15.0;
        }

        // Factor 3: Edad de la canción
        let days_old = Utc::now().signed_duration_since(aggregate.fractional_song().created_at()).num_days();
        if days_old < 30 {
            risk_factors.push(RiskFactor::NewAsset);
            total_risk_score += 10.0;
        }

        // Factor 4: Volatilidad del mercado
        if market_volatility > 0.7 {
            risk_factors.push(RiskFactor::HighMarketVolatility);
            total_risk_score += 15.0;
        }

        // Factor 5: Liquidez limitada
        let sold_percentage = aggregate.fractional_song().sold_percentage();
        if sold_percentage < 20.0 {
            risk_factors.push(RiskFactor::LowLiquidity);
            total_risk_score += 10.0;
        }

        // Factor 6: Dependencia de un solo artista
        risk_factors.push(RiskFactor::SingleArtistDependency);
        total_risk_score += 5.0;

        let risk_level = match total_risk_score {
            score if score < 20.0 => RiskLevel::Low,
            score if score < 40.0 => RiskLevel::Medium,
            score if score < 60.0 => RiskLevel::High,
            _ => RiskLevel::VeryHigh,
        };

        RiskAssessment {
            risk_level,
            risk_score: total_risk_score,
            risk_factors,
            recommendation: Self::generate_risk_recommendation(risk_level, &risk_factors),
        }
    }

    fn calculate_ownership_concentration(aggregate: &FractionalOwnershipAggregate) -> f64 {
        if aggregate.ownerships().is_empty() {
            return 1.0;
        }

        let mut ownership_percentages: Vec<f64> = aggregate.ownerships()
            .values()
            .map(|ownership| ownership.ownership_percentage().value())
            .collect();

        ownership_percentages.sort_by(|a, b| b.partial_cmp(a).unwrap());

        // Calcular el índice de Herfindahl-Hirschman (HHI)
        let hhi: f64 = ownership_percentages.iter()
            .map(|percentage| (percentage / 100.0).powi(2))
            .sum();

        hhi
    }

    fn generate_risk_recommendation(risk_level: RiskLevel, risk_factors: &[RiskFactor]) -> String {
        match risk_level {
            RiskLevel::Low => "Inversión de bajo riesgo. Adecuada para portafolios conservadores.".to_string(),
            RiskLevel::Medium => format!(
                "Inversión de riesgo moderado. Factores a considerar: {}. Adecuada para inversores con tolerancia media al riesgo.",
                risk_factors.len()
            ),
            RiskLevel::High => "Inversión de alto riesgo. Solo para inversores experimentados con alta tolerancia al riesgo.".to_string(),
            RiskLevel::VeryHigh => "Inversión de riesgo muy alto. No recomendada para la mayoría de inversores.".to_string(),
        }
    }
}

/// Servicio de dominio para optimización de portfolios
pub struct PortfolioOptimizationService;

impl PortfolioOptimizationService {
    /// Sugerir diversificación de portfolio basada en género, artista y riesgo
    pub fn suggest_portfolio_diversification(
        current_holdings: Vec<FractionalOwnershipSummary>,
        investment_amount: RevenueAmount,
        risk_tolerance: RiskTolerance,
    ) -> DiversificationSuggestion {
        // Analizar concentración actual por artista
        let mut artist_concentration = HashMap::new();
        let mut genre_concentration = HashMap::new();
        let mut total_value = 0.0;

        for holding in &current_holdings {
            *artist_concentration.entry(holding.artist_id).or_insert(0.0) += holding.investment_value;
            *genre_concentration.entry(holding.genre.clone()).or_insert(0.0) += holding.investment_value;
            total_value += holding.investment_value;
        }

        let mut recommendations = Vec::new();

        // Verificar concentración por artista
        for (artist_id, value) in artist_concentration {
            let concentration_percentage = (value / total_value) * 100.0;
            if concentration_percentage > 30.0 {
                recommendations.push(DiversificationRecommendation::ReduceArtistConcentration {
                    artist_id,
                    current_percentage: concentration_percentage,
                    suggested_max: 25.0,
                });
            }
        }

        // Verificar concentración por género
        for (genre, value) in genre_concentration {
            let concentration_percentage = (value / total_value) * 100.0;
            if concentration_percentage > 40.0 {
                recommendations.push(DiversificationRecommendation::ReduceGenreConcentration {
                    genre,
                    current_percentage: concentration_percentage,
                    suggested_max: 35.0,
                });
            }
        }

        // Sugerir nuevas inversiones basadas en tolerancia al riesgo
        let suggested_allocations = match risk_tolerance {
            RiskTolerance::Conservative => vec![
                ("Established Artists", 60.0),
                ("Indie Folk", 20.0),
                ("Classical", 20.0),
            ],
            RiskTolerance::Moderate => vec![
                ("Established Artists", 40.0),
                ("Rising Artists", 30.0),
                ("Popular Genres", 30.0),
            ],
            RiskTolerance::Aggressive => vec![
                ("Rising Artists", 50.0),
                ("New Releases", 30.0),
                ("Experimental Genres", 20.0),
            ],
        };

        DiversificationSuggestion {
            recommendations,
            suggested_allocations: suggested_allocations.into_iter()
                .map(|(category, percentage)| (category.to_string(), percentage))
                .collect(),
            max_single_investment: match risk_tolerance {
                RiskTolerance::Conservative => investment_amount.value() * 0.1,
                RiskTolerance::Moderate => investment_amount.value() * 0.15,
                RiskTolerance::Aggressive => investment_amount.value() * 0.25,
            },
        }
    }
}

// Tipos de apoyo para los servicios

#[derive(Debug, Clone, PartialEq)]
pub enum MarketConditions {
    Bull,      // Mercado alcista
    Bear,      // Mercado bajista
    Stable,    // Mercado estable
    Volatile,  // Mercado volátil
}

#[derive(Debug, Clone)]
pub struct ArtistPerformanceMetrics {
    pub average_revenue_growth: f64,  // Crecimiento promedio anual de ingresos
    pub revenue_volatility: f64,      // Volatilidad de ingresos (0.0 - 1.0)
    pub hit_rate: f64,               // Porcentaje de canciones exitosas
    pub market_presence_years: u32,   // Años en el mercado
}

#[derive(Debug, Clone)]
pub struct GenreMarketTrends {
    pub growth_trend: TrendDirection,
    pub market_share_percentage: f64,
    pub volatility_index: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Growing,
    Stable,
    Declining,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvestmentRating {
    Excellent,
    Good,
    Average,
    Poor,
    VeryPoor,
}

#[derive(Debug, Clone)]
pub struct RiskAssessment {
    pub risk_level: RiskLevel,
    pub risk_score: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub recommendation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskFactor {
    HighOwnershipConcentration,
    HighRevenueVolatility,
    NewAsset,
    HighMarketVolatility,
    LowLiquidity,
    SingleArtistDependency,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskTolerance {
    Conservative,
    Moderate,
    Aggressive,
}

#[derive(Debug, Clone)]
pub struct FractionalOwnershipSummary {
    pub fractional_song_id: Uuid,
    pub artist_id: Uuid,
    pub genre: String,
    pub shares_owned: u32,
    pub investment_value: f64,
}

#[derive(Debug, Clone)]
pub struct DiversificationSuggestion {
    pub recommendations: Vec<DiversificationRecommendation>,
    pub suggested_allocations: Vec<(String, f64)>, // (categoria, porcentaje)
    pub max_single_investment: f64,
}

#[derive(Debug, Clone)]
pub enum DiversificationRecommendation {
    ReduceArtistConcentration {
        artist_id: Uuid,
        current_percentage: f64,
        suggested_max: f64,
    },
    ReduceGenreConcentration {
        genre: String,
        current_percentage: f64,
        suggested_max: f64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_dynamic_price_with_demand() {
        let current_price = SharePrice::new(10.0).unwrap();
        let total_revenue = RevenueAmount::new(1000.0).unwrap();
        
        let new_price = FractionalOwnershipPricingService::calculate_dynamic_share_price(
            &current_price,
            &total_revenue,
            50.0, // 50% vendido
            1.2,  // Alta demanda del mercado
            8.0,  // Artista popular
            180,  // 6 meses desde creación
        ).unwrap();

        // El precio debe haber aumentado significativamente
        assert!(new_price.value() > current_price.value());
    }

    #[test]
    fn should_evaluate_investment_as_good_with_favorable_metrics() {
        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let share_price = SharePrice::new(10.0).unwrap();

        let fractional_song = FractionalSong::new(
            song_id,
            artist_id,
            "Test Song".to_string(),
            1000,
            share_price,
        ).unwrap();

        let aggregate = FractionalOwnershipAggregate::new(fractional_song);

        let artist_metrics = ArtistPerformanceMetrics {
            average_revenue_growth: 0.3, // 30% crecimiento anual
            revenue_volatility: 0.2,     // Baja volatilidad
            hit_rate: 0.7,              // 70% de éxito
            market_presence_years: 5,
        };

        let genre_trends = GenreMarketTrends {
            growth_trend: TrendDirection::Growing,
            market_share_percentage: 25.0,
            volatility_index: 0.3,
        };

        let rating = FractionalOwnershipPricingService::evaluate_investment_potential(
            &aggregate,
            artist_metrics,
            genre_trends,
        );

        // Con métricas favorables, debe ser al menos una buena inversión
        assert!(matches!(rating, InvestmentRating::Good | InvestmentRating::Excellent));
    }

    #[test]
    fn should_identify_high_risk_for_concentrated_ownership() {
        // Crear aggregate con ownership concentrado
        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let share_price = SharePrice::new(10.0).unwrap();

        let fractional_song = FractionalSong::new(
            song_id,
            artist_id,
            "Test Song".to_string(),
            1000,
            share_price,
        ).unwrap();

        let aggregate = FractionalOwnershipAggregate::new(fractional_song);

        let artist_metrics = ArtistPerformanceMetrics {
            average_revenue_growth: 0.1,
            revenue_volatility: 0.8, // Alta volatilidad
            hit_rate: 0.3,
            market_presence_years: 1, // Artista nuevo
        };

        let risk_assessment = RiskAnalysisService::calculate_investment_risk(
            &aggregate,
            artist_metrics,
            0.8, // Alta volatilidad de mercado
        );

        // Debe identificar alto riesgo
        assert!(matches!(risk_assessment.risk_level, RiskLevel::High | RiskLevel::VeryHigh));
        assert!(!risk_assessment.risk_factors.is_empty());
    }
} 