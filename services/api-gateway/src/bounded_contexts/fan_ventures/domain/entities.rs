use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// =============================================================================
// FAN VENTURES - ENTITIES (Reemplazando Fractional Ownership)
// =============================================================================

/// Representa una inversión de un fan en un artista
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanInvestment {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub fan_id: Uuid,
    pub investment_amount: f64,
    pub investment_type: InvestmentType,
    pub created_at: DateTime<Utc>,
    pub status: InvestmentStatus,
    pub expected_return: f64,
    pub duration_months: i32,
}

/// Tipo de inversión del fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvestmentType {
    EarlyAccess,    // Acceso temprano a contenido
    ExclusiveContent, // Contenido exclusivo
    Merchandise,    // Productos exclusivos
    ConcertTickets, // Entradas a conciertos
    MeetAndGreet,   // Encuentros con el artista
    RevenueShare,   // Participación en ingresos
    Custom(String), // Beneficio personalizado
}

/// Estado de la inversión
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InvestmentStatus {
    Pending,    // Pendiente de confirmación
    Active,     // Activa
    Completed,  // Completada
    Cancelled,  // Cancelada
}

/// Venture creado por un artista para sus fans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistVenture {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub investment_type: InvestmentType,
    pub min_investment: f64,
    pub max_investment: Option<f64>,
    pub total_goal: f64,
    pub current_amount: f64,
    pub max_investors: Option<i32>,
    pub current_investors: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: VentureStatus,
    pub benefits: Vec<VentureBenefit>,
}

/// Estado del venture
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VentureStatus {
    Draft,      // Borrador
    Open,       // Abierto para inversiones
    Closed,     // Cerrado
    Cancelled,  // Cancelado
}

/// Tier de inversión para un venture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureTier {
    pub id: Uuid,
    pub venture_id: Uuid,
    pub name: String,           // "Bronze", "Silver", "Gold", "Platinum"
    pub min_investment: f64,
    pub max_investment: Option<f64>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub benefits: Vec<VentureBenefit>,
}

/// Beneficio específico del venture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureBenefit {
    pub id: Uuid,
    pub venture_id: Uuid,
    pub tier_id: Option<Uuid>,  // Referencia al tier
    pub title: String,
    pub description: Option<String>,
    pub benefit_type: BenefitType,
    pub value: f64,
    pub delivery_method: DeliveryMethod,
    pub delivery_date: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Tipo de beneficio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenefitType {
    DigitalContent,    // Contenido digital
    PhysicalProduct,   // Producto físico
    Experience,        // Experiencia (concierto, meet&greet)
    RevenueShare,      // Participación en ingresos
    Recognition,       // Reconocimiento público
    Custom(String),    // Beneficio personalizado
}

/// Método de entrega del beneficio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryMethod {
    Automatic,     // Entrega automática (digital)
    Manual,        // Entrega manual (requiere acción del artista)
    Physical,      // Entrega física (envío postal)
    Experience,    // Entrega de experiencia (evento)
}

/// Distribución de ingresos para ventures con revenue sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistribution {
    pub id: Uuid,
    pub venture_id: Uuid,
    pub total_revenue: f64,
    pub artist_share: f64,
    pub fan_share: f64,
    pub platform_fee: f64,
    pub distributed_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

/// Portfolio de un fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanPortfolio {
    pub fan_id: Uuid,
    pub total_invested: f64,
    pub total_returned: f64,
    pub active_investments: i32,
    pub completed_investments: i32,
    pub favorite_artists: Vec<Uuid>,
    pub total_benefits_received: i32,
}

/// Estadísticas de un venture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureAnalytics {
    pub venture_id: Uuid,
    pub total_investors: i32,
    pub average_investment: f64,
    pub funding_progress: f64,
    pub total_revenue_generated: f64,
    pub total_benefits_delivered: i32,
    pub investor_satisfaction: f64,
    pub completion_rate: f64,
}

// =============================================================================
// REQUEST/RESPONSE STRUCTURES
// =============================================================================

/// Request para crear un tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTierRequest {
    pub name: String,
    pub min_investment: f64,
    pub max_investment: Option<f64>,
    pub description: Option<String>,
    pub benefits: Vec<CreateBenefitRequest>,
}

/// Request para crear un beneficio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBenefitRequest {
    pub title: String,
    pub description: Option<String>,
    pub benefit_type: BenefitType,
    pub value: f64,
    pub delivery_method: DeliveryMethod,
    pub delivery_date: Option<DateTime<Utc>>,
}

/// Response para un tier con sus beneficios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierResponse {
    pub id: Uuid,
    pub name: String,
    pub min_investment: f64,
    pub max_investment: Option<f64>,
    pub description: Option<String>,
    pub benefits: Vec<VentureBenefit>,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// DASHBOARD ENTITIES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistDashboard {
    pub artist_id: Uuid,
    pub total_ventures: u32,
    pub active_ventures: u32,
    pub total_funding_raised: f64,
    pub total_investors: u32,
    pub recent_ventures: Vec<VentureSummary>,
    pub top_performing_ventures: Vec<VentureSummary>,
    pub recent_investments: Vec<InvestmentSummary>,
    pub monthly_stats: MonthlyStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureSummary {
    pub venture_id: Uuid,
    pub title: String,
    pub status: VentureStatus,
    pub current_amount: f64,
    pub total_goal: f64,
    pub funding_progress: f64,
    pub total_investors: u32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentSummary {
    pub investment_id: Uuid,
    pub venture_id: Uuid,
    pub venture_title: String,
    pub fan_id: Uuid,
    pub fan_name: String,
    pub investment_amount: f64,
    pub investment_type: InvestmentType,
    pub status: InvestmentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStats {
    pub month: String, // "2024-01"
    pub new_ventures: u32,
    pub new_investments: u32,
    pub funding_raised: f64,
    pub new_investors: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureDashboard {
    pub venture_id: Uuid,
    pub venture: ArtistVenture,
    pub tiers: Vec<VentureTier>,
    pub investors: Vec<InvestorSummary>,
    pub funding_progress: FundingProgress,
    pub recent_activity: Vec<ActivityItem>,
    pub analytics: VentureAnalytics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorSummary {
    pub fan_id: Uuid,
    pub fan_name: String,
    pub investment_amount: f64,
    pub investment_date: DateTime<Utc>,
    pub tier_qualification: Option<String>,
    pub total_benefits_received: u32,
    pub status: InvestmentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingProgress {
    pub current_amount: f64,
    pub total_goal: f64,
    pub percentage_complete: f64,
    pub days_remaining: Option<i32>,
    pub average_investment: f64,
    pub largest_investment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    pub id: Uuid,
    pub activity_type: ActivityType,
    pub description: String,
    pub amount: Option<f64>,
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    VentureCreated,
    InvestmentMade,
    TierCreated,
    BenefitDelivered,
    RevenueDistributed,
    VentureActivated,
    VentureClosed,
}

// =============================================================================
// BENEFIT DELIVERY ENTITIES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitDelivery {
    pub id: Uuid,
    pub benefit_id: Uuid,
    pub venture_id: Uuid,
    pub fan_id: Uuid,
    pub tier_id: Option<Uuid>,
    pub delivery_status: DeliveryStatus,
    pub delivery_method: DeliveryMethod,
    pub delivery_date: Option<DateTime<Utc>>,
    pub tracking_info: Option<TrackingInfo>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeliveryStatus {
    Pending,
    InProgress,
    Delivered,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingInfo {
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub delivery_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryRequest {
    pub benefit_id: Uuid,
    pub fan_id: Uuid,
    pub delivery_method: DeliveryMethod,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeliveryRequest {
    pub delivery_status: DeliveryStatus,
    pub tracking_info: Option<TrackingInfo>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverySummary {
    pub delivery_id: Uuid,
    pub benefit_title: String,
    pub venture_title: String,
    pub fan_name: String,
    pub delivery_status: DeliveryStatus,
    pub delivery_method: DeliveryMethod,
    pub delivery_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanDeliveryHistory {
    pub fan_id: Uuid,
    pub total_deliveries: u32,
    pub pending_deliveries: u32,
    pub completed_deliveries: u32,
    pub deliveries: Vec<DeliverySummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureDeliveryStats {
    pub venture_id: Uuid,
    pub total_benefits: u32,
    pub pending_deliveries: u32,
    pub completed_deliveries: u32,
    pub delivery_success_rate: f64,
    pub average_delivery_time_days: f64,
}

// =============================================================================
// VENTURE EXPLORATION ENTITIES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureExploration {
    pub ventures: Vec<VentureDiscovery>,
    pub total_count: u32,
    pub page: u32,
    pub page_size: u32,
    pub filters: ExplorationFilters,
    pub sorting: ExplorationSorting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureDiscovery {
    pub venture_id: Uuid,
    pub artist_id: Uuid,
    pub artist_name: String,
    pub artist_avatar: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub investment_type: InvestmentType,
    pub min_investment: f64,
    pub max_investment: Option<f64>,
    pub total_goal: f64,
    pub current_amount: f64,
    pub funding_progress: f64,
    pub total_investors: u32,
    pub status: VentureStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub days_remaining: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub top_tiers: Vec<VentureTier>,
    pub tags: Vec<String>,
    pub category: VentureCategory,
    pub risk_level: RiskLevel,
    pub expected_return: f64,
    pub artist_rating: f64,
    pub artist_previous_ventures: u32,
    pub artist_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationFilters {
    pub categories: Option<Vec<VentureCategory>>,
    pub investment_types: Option<Vec<InvestmentType>>,
    pub risk_levels: Option<Vec<RiskLevel>>,
    pub min_investment: Option<f64>,
    pub max_investment: Option<f64>,
    pub min_funding_progress: Option<f64>,
    pub max_funding_progress: Option<f64>,
    pub min_artist_rating: Option<f64>,
    pub tags: Option<Vec<String>>,
    pub search_query: Option<String>,
    pub expires_within_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExplorationSorting {
    Newest,
    Oldest,
    FundingProgress,
    DaysRemaining,
    MinInvestment,
    ExpectedReturn,
    ArtistRating,
    Popularity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VentureCategory {
    Music,
    VisualArts,
    Film,
    Gaming,
    Technology,
    Fashion,
    Food,
    Travel,
    Education,
    Health,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureRecommendation {
    pub venture_id: Uuid,
    pub score: f64,
    pub reasons: Vec<String>,
    pub match_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanPreferences {
    pub fan_id: Uuid,
    pub favorite_categories: Vec<VentureCategory>,
    pub preferred_investment_types: Vec<InvestmentType>,
    pub risk_tolerance: RiskLevel,
    pub min_investment: f64,
    pub max_investment: f64,
    pub favorite_artists: Vec<Uuid>,
    pub interests: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureSearchRequest {
    pub query: Option<String>,
    pub filters: Option<ExplorationFilters>,
    pub sorting: Option<ExplorationSorting>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureRecommendationsRequest {
    pub fan_id: Uuid,
    pub limit: Option<u32>,
    pub include_explanation: Option<bool>,
} 