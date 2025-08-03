// Application layer - Casos de uso y orquestación de servicios de dominio

pub mod use_cases;
pub mod services;
pub mod dtos;
pub mod commands;
pub mod queries;
pub mod event_handlers;

// Re-exports - Specific exports to avoid conflicts
pub use use_cases::{CreateContractUseCase, PurchaseSharesUseCase, DistributeRevenueUseCase};
pub use services::{FractionalOwnershipApplicationService};
pub use dtos::{ContractDto, ShareDto, RevenueDto};
pub use event_handlers::{ContractEventHandler, ShareEventHandler}; 