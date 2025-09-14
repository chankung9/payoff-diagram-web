use crate::models::{FuturesPosition, OptionPosition, Position, SpotPosition};

/// Validation result for position inputs
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { is_valid: true, errors: Vec::new(), warnings: Vec::new() }
    }

    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn is_ok(&self) -> bool {
        self.is_valid && self.errors.is_empty()
    }
}

/// Position validation engine (WASM-compatible)
pub struct ValidationEngine;

impl ValidationEngine {
    /// Validate a single position
    pub fn validate_position(position: &Position) -> ValidationResult {
        let mut result = ValidationResult::new();

        match position {
            Position::Spot(spot) => Self::validate_spot_position(spot, &mut result),
            Position::Option(option) => Self::validate_option_position(option, &mut result),
            Position::Futures(futures) => Self::validate_futures_position(futures, &mut result),
        }

        result
    }

    /// Validate multiple positions as a portfolio
    pub fn validate_portfolio(positions: &[Position]) -> ValidationResult {
        let mut result = ValidationResult::new();

        if positions.is_empty() {
            result.add_warning("Portfolio is empty".to_string());
            return result;
        }

        // Validate each position
        for (i, position) in positions.iter().enumerate() {
            let pos_result = Self::validate_position(position);

            for error in pos_result.errors {
                result.add_error(format!("Position {}: {}", i + 1, error));
            }

            for warning in pos_result.warnings {
                result.add_warning(format!("Position {}: {}", i + 1, warning));
            }
        }

        // Portfolio-level validations
        Self::validate_portfolio_risk(positions, &mut result);

        result
    }

    /// Validate chart parameters
    pub fn validate_chart_parameters(
        price_start: f64,
        price_end: f64,
        step_size: f64,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();

        if price_start <= 0.0 {
            result.add_error("Start price must be positive".to_string());
        }

        if price_end <= 0.0 {
            result.add_error("End price must be positive".to_string());
        }

        if price_start >= price_end {
            result.add_error("End price must be greater than start price".to_string());
        }

        if step_size <= 0.0 {
            result.add_error("Step size must be positive".to_string());
        }

        if step_size > (price_end - price_start) {
            result.add_error("Step size is too large for the price range".to_string());
        }

        // Performance warnings
        let total_steps = ((price_end - price_start) / step_size) as usize;
        if total_steps > 10000 {
            result.add_warning(format!("Large number of data points ({}). Consider increasing step size for better performance.", total_steps));
        }

        result
    }

    // === Private validation functions ===

    fn validate_spot_position(spot: &SpotPosition, result: &mut ValidationResult) {
        if spot.quantity == 0.0 {
            result.add_error("Quantity cannot be zero".to_string());
        }

        if spot.entry_price <= 0.0 {
            result.add_error("Entry price must be positive".to_string());
        }

        // Warnings
        if spot.quantity.abs() > 10000.0 {
            result.add_warning("Large position size detected".to_string());
        }
    }

    fn validate_option_position(option: &OptionPosition, result: &mut ValidationResult) {
        if option.quantity == 0.0 {
            result.add_error("Quantity cannot be zero".to_string());
        }

        if option.strike_price <= 0.0 {
            result.add_error("Strike price must be positive".to_string());
        }

        if option.premium < 0.0 {
            result.add_error("Premium cannot be negative".to_string());
        }

        // Warnings
        if option.premium == 0.0 {
            result.add_warning("Zero premium option - verify this is correct".to_string());
        }

        if option.quantity.abs() > 1000.0 {
            result.add_warning("Large option position detected".to_string());
        }

        if option.premium > option.strike_price * 0.5 {
            result.add_warning("Premium seems unusually high relative to strike price".to_string());
        }
    }

    fn validate_futures_position(futures: &FuturesPosition, result: &mut ValidationResult) {
        if futures.quantity == 0.0 {
            result.add_error("Quantity cannot be zero".to_string());
        }

        if futures.contract_size <= 0.0 {
            result.add_error("Contract size must be positive".to_string());
        }

        // Warnings
        if futures.quantity.abs() > 100.0 {
            result.add_warning("Large futures position detected".to_string());
        }

        if futures.contract_size > 100000.0 {
            result.add_warning("Very large contract size detected".to_string());
        }
    }

    fn validate_portfolio_risk(positions: &[Position], result: &mut ValidationResult) {
        // Check for excessive leverage
        let total_notional = positions
            .iter()
            .map(|pos| match pos {
                Position::Spot(spot) => spot.quantity.abs() * spot.entry_price,
                Position::Option(option) => option.quantity.abs() * option.strike_price,
                Position::Futures(futures) => {
                    futures.quantity.abs() * futures.entry_price * futures.contract_size
                }
            })
            .sum::<f64>();

        if total_notional > 1_000_000.0 {
            result.add_warning("Portfolio has very large notional exposure".to_string());
        }

        // Check for portfolio complexity
        if positions.len() > 10 {
            result.add_warning(
                "Complex portfolio with many positions - consider simplification".to_string(),
            );
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_valid_spot_position() {
        let spot = SpotPosition::new(100.0, 50.0, None);
        let position = Position::Spot(spot);

        let result = ValidationEngine::validate_position(&position);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_spot_position() {
        let spot = SpotPosition::new(0.0, -10.0, None); // Invalid quantity and price
        let position = Position::Spot(spot);

        let result = ValidationEngine::validate_position(&position);
        assert!(!result.is_ok());
        assert_eq!(result.errors.len(), 2);
    }

    #[test]
    fn test_chart_parameter_validation() {
        // Valid parameters
        let result = ValidationEngine::validate_chart_parameters(50.0, 150.0, 1.0);
        assert!(result.is_ok());

        // Invalid parameters
        let result = ValidationEngine::validate_chart_parameters(150.0, 50.0, 1.0); // Start > End
        assert!(!result.is_ok());
    }
}
