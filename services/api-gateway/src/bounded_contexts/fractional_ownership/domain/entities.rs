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
    pub duration_months: u32,
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
    pub description: String,
    pub investment_type: InvestmentType,
    pub min_investment: f64,
    pub max_investment: f64,
    pub total_goal: f64,
    pub current_amount: f64,
    pub max_investors: Option<u32>,
    pub current_investors: u32,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
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

/// Beneficio específico del venture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureBenefit {
    pub id: Uuid,
    pub venture_id: Uuid,
    pub title: String,
    pub description: String,
    pub benefit_type: BenefitType,
    pub delivery_date: Option<DateTime<Utc>>,
    pub is_delivered: bool,
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
    pub active_investments: u32,
    pub completed_investments: u32,
    pub favorite_artists: Vec<Uuid>,
    pub total_benefits_received: u32,
}

/// Estadísticas de un venture
#[derive(Debug, Clone, Serialize, Deserialize)]
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