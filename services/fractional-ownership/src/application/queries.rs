// CQRS Queries para fractional ownership
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetUserPortfolioQuery {
    pub user_id: Uuid,
} 