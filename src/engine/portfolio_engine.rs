use super::payoff_engine::{PayoffEngine, PayoffPoint};
use crate::models::Position;

/// Portfolio analysis metrics
#[derive(Debug, Clone, PartialEq)]
pub struct PortfolioMetrics {
    pub total_positions: usize,
    pub breakeven_points: Vec<f64>,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub profit_probability: Option<f64>,
    pub expected_value: Option<f64>,
}

/// Portfolio analysis engine (WASM-compatible)
pub struct PortfolioEngine;

impl PortfolioEngine {
    /// Analyze portfolio and return comprehensive metrics
    pub fn analyze_portfolio(
        positions: &[Position],
        price_start: f64,
        price_end: f64,
        step_size: f64,
    ) -> PortfolioMetrics {
        let breakeven_points =
            PayoffEngine::find_breakeven_points(positions, price_start, price_end, step_size);

        let max_profit =
            PayoffEngine::calculate_max_profit(positions, price_start, price_end, step_size);

        let max_loss =
            PayoffEngine::calculate_max_loss(positions, price_start, price_end, step_size);

        let payoff_curve =
            PayoffEngine::generate_payoff_curve(positions, price_start, price_end, step_size);

        let profit_probability = Self::calculate_profit_probability(&payoff_curve);
        let expected_value = Self::calculate_expected_value(&payoff_curve);

        PortfolioMetrics {
            total_positions: positions.len(),
            breakeven_points,
            max_profit,
            max_loss,
            profit_probability,
            expected_value,
        }
    }

    /// Calculate probability of profit (assuming uniform price distribution)
    fn calculate_profit_probability(payoff_curve: &[PayoffPoint]) -> Option<f64> {
        if payoff_curve.is_empty() {
            return None;
        }

        let profitable_points = payoff_curve
            .iter()
            .filter(|point| point.payoff > 0.0)
            .count();

        Some(profitable_points as f64 / payoff_curve.len() as f64)
    }

    /// Calculate expected value (assuming uniform price distribution)
    fn calculate_expected_value(payoff_curve: &[PayoffPoint]) -> Option<f64> {
        if payoff_curve.is_empty() {
            return None;
        }

        let total_payoff: f64 = payoff_curve.iter().map(|point| point.payoff).sum();

        Some(total_payoff / payoff_curve.len() as f64)
    }

    /// Check if portfolio has unlimited profit potential
    pub fn has_unlimited_profit(positions: &[Position]) -> bool {
        // Simple heuristic: check if any position has unlimited upside
        positions.iter().any(|pos| match pos {
            crate::models::Position::Spot(spot) => spot.quantity > 0.0,
            crate::models::Position::Option(option) => {
                matches!(option.option_type, crate::models::OptionType::Call)
                    && option.quantity > 0.0
            }
            crate::models::Position::Futures(futures) => futures.quantity > 0.0,
        })
    }

    /// Check if portfolio has unlimited loss potential
    pub fn has_unlimited_loss(positions: &[Position]) -> bool {
        // Simple heuristic: check if any position has unlimited downside
        positions.iter().any(|pos| match pos {
            crate::models::Position::Spot(spot) => spot.quantity > 0.0, // Long spot has unlimited downside to 0
            crate::models::Position::Option(option) => {
                // Short calls or long puts can have significant losses
                (matches!(option.option_type, crate::models::OptionType::Call)
                    && option.quantity < 0.0)
                    || (matches!(option.option_type, crate::models::OptionType::Put)
                        && option.quantity > 0.0)
            }
            crate::models::Position::Futures(_futures) => true, // Futures always have unlimited risk
        })
    }

    /// Get portfolio risk classification
    pub fn get_risk_level(positions: &[Position]) -> RiskLevel {
        let has_unlimited_profit = Self::has_unlimited_profit(positions);
        let has_unlimited_loss = Self::has_unlimited_loss(positions);

        match (has_unlimited_profit, has_unlimited_loss) {
            (true, true) => RiskLevel::High,
            (true, false) => RiskLevel::Medium,
            (false, true) => RiskLevel::High,
            (false, false) => RiskLevel::Low,
        }
    }
}

/// Portfolio risk classification
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,    // Limited profit, limited loss
    Medium, // Unlimited profit, limited loss
    High,   // Unlimited loss potential
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_portfolio_analysis() {
        let spot = SpotPosition::new(100.0, 50.0, None);
        let position = Position::Spot(spot);
        let positions = vec![position];

        let metrics = PortfolioEngine::analyze_portfolio(&positions, 40.0, 60.0, 5.0);

        assert_eq!(metrics.total_positions, 1);
        assert!(metrics.profit_probability.is_some());
        assert!(metrics.expected_value.is_some());
    }

    #[test]
    fn test_risk_level_classification() {
        // Long spot position - high risk (unlimited downside to 0)
        let spot = SpotPosition::new(100.0, 50.0, None);
        let position = Position::Spot(spot);
        let positions = vec![position];

        let risk = PortfolioEngine::get_risk_level(&positions);
        assert_eq!(risk, RiskLevel::High);
    }
}
