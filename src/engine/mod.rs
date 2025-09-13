// Payoff Engine Module
// Core calculation engine for financial position analysis, independent of UI

pub mod payoff_engine;
pub mod portfolio_engine;
pub mod validation_engine;

// Re-export main interfaces
pub use payoff_engine::{PayoffEngine, PayoffPoint};
pub use portfolio_engine::{PortfolioEngine, PortfolioMetrics};
pub use validation_engine::{ValidationEngine, ValidationResult};
