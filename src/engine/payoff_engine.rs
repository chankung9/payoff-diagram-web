use crate::models::{Position, SpotPosition, OptionPosition, FuturesPosition, OptionType};

/// A single point on the payoff diagram
#[derive(Debug, Clone, PartialEq)]
pub struct PayoffPoint {
    pub price: f64,
    pub payoff: f64,
}

/// Core payoff calculation engine (WASM-compatible)
pub struct PayoffEngine;

impl PayoffEngine {
    /// Calculate payoff for a single position at a given underlying price
    pub fn calculate_single_payoff(position: &Position, underlying_price: f64) -> f64 {
        match position {
            Position::Spot(spot) => Self::calculate_spot_payoff(spot, underlying_price),
            Position::Option(option) => Self::calculate_option_payoff(option, underlying_price),
            Position::Futures(futures) => Self::calculate_futures_payoff(futures, underlying_price),
        }
    }

    /// Calculate payoff for multiple positions (portfolio)
    pub fn calculate_portfolio_payoff(positions: &[Position], underlying_price: f64) -> f64 {
        positions
            .iter()
            .filter(|pos| pos.is_active())  // Only include active positions
            .map(|pos| Self::calculate_single_payoff(pos, underlying_price))
            .sum()
    }

    /// Generate payoff points across a price range
    pub fn generate_payoff_curve(
        positions: &[Position],
        price_start: f64,
        price_end: f64,
        step_size: f64,
    ) -> Vec<PayoffPoint> {
        let mut points = Vec::new();
        let mut current_price = price_start;

        while current_price <= price_end {
            let payoff = Self::calculate_portfolio_payoff(positions, current_price);
            points.push(PayoffPoint {
                price: current_price,
                payoff,
            });
            current_price += step_size;
        }

        points
    }

    /// Find breakeven points for the portfolio
    pub fn find_breakeven_points(
        positions: &[Position], 
        price_start: f64, 
        price_end: f64, 
        step_size: f64
    ) -> Vec<f64> {
        let active_positions: Vec<&Position> = positions.iter().filter(|pos| pos.is_active()).collect();
        if active_positions.is_empty() {
            return Vec::new();
        }
        
        let mut breakeven_points = Vec::new();
        let mut prev_pnl = Self::calculate_portfolio_payoff(positions, price_start);
        let mut current_price = price_start + step_size;

        while current_price <= price_end {
            let current_pnl = Self::calculate_portfolio_payoff(positions, current_price);
            
            // Check if we crossed zero (sign change)
            if (prev_pnl <= 0.0 && current_pnl >= 0.0) || (prev_pnl >= 0.0 && current_pnl <= 0.0) {
                // Use linear interpolation to find more precise breakeven point
                let breakeven = Self::interpolate_zero_crossing(
                    current_price - step_size,
                    prev_pnl,
                    current_price,
                    current_pnl,
                );
                breakeven_points.push(breakeven);
            }

            prev_pnl = current_pnl;
            current_price += step_size;
        }

        breakeven_points
    }

    /// Calculate maximum profit for a portfolio (if bounded)
    pub fn calculate_max_profit(
        positions: &[Position],
        price_start: f64,
        price_end: f64,
        step_size: f64,
    ) -> Option<f64> {
        let active_positions: Vec<&Position> = positions.iter().filter(|pos| pos.is_active()).collect();
        if active_positions.is_empty() {
            return None;
        }
        
        let points = Self::generate_payoff_curve(positions, price_start, price_end, step_size);
        let max_profit = points.iter().map(|p| p.payoff).max_by(|a, b| a.partial_cmp(b).unwrap());
        
        #[cfg(debug_assertions)]
        if let Some(max) = max_profit {
            web_sys::console::log_1(&format!("Max Profit calculated: ${:.2}", max).into());
        }
        
        max_profit
    }

    /// Calculate maximum loss for a portfolio (if bounded)
    pub fn calculate_max_loss(
        positions: &[Position],
        price_start: f64,
        price_end: f64,
        step_size: f64,
    ) -> Option<f64> {
        let active_positions: Vec<&Position> = positions.iter().filter(|pos| pos.is_active()).collect();
        if active_positions.is_empty() {
            return None;
        }
        
        let points = Self::generate_payoff_curve(positions, price_start, price_end, step_size);
        let max_loss = points.iter().map(|p| p.payoff).min_by(|a, b| a.partial_cmp(b).unwrap());
        
        #[cfg(debug_assertions)]
        if let Some(min) = max_loss {
            web_sys::console::log_1(&format!("Max Loss calculated: ${:.2}", min).into());
        }
        
        max_loss
    }

    // === Private helper functions ===

    /// Calculate spot position payoff
    fn calculate_spot_payoff(spot: &SpotPosition, underlying_price: f64) -> f64 {
        // P&L = Quantity * (Current Price - Entry Price)
        spot.quantity * (underlying_price - spot.entry_price)
    }

    /// Calculate option position payoff
    fn calculate_option_payoff(option: &OptionPosition, underlying_price: f64) -> f64 {
        let intrinsic_value = match option.option_type {
            OptionType::Call => (underlying_price - option.strike_price).max(0.0),
            OptionType::Put => (option.strike_price - underlying_price).max(0.0),
        };

        // Calculate payoff per contract (always from long perspective first)
        let long_payoff_per_contract = intrinsic_value - option.premium;
        
        // Apply quantity (negative quantity automatically makes it short)
        let total_payoff = option.quantity * long_payoff_per_contract;
        
        // Debug logging for troubleshooting
        #[cfg(debug_assertions)]
        if underlying_price == 200.0 || underlying_price == 0.0 || underlying_price == 250.0 {
            web_sys::console::log_1(&format!(
                "Option Debug - Underlying: ${:.2}, Intrinsic: ${:.2}, Long P&L: ${:.2}, Quantity: {}, Total: ${:.2}",
                underlying_price, intrinsic_value, long_payoff_per_contract, option.quantity, total_payoff
            ).into());
        }
        
        total_payoff
    }

    /// Calculate futures position payoff
    fn calculate_futures_payoff(futures: &FuturesPosition, underlying_price: f64) -> f64 {
        // P&L = Quantity * Contract Size * (Current Price - Entry Price)
        futures.quantity * futures.contract_size * (underlying_price - futures.entry_price)
    }

    /// Linear interpolation to find where P&L crosses zero
    fn interpolate_zero_crossing(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        if (y2 - y1).abs() < f64::EPSILON {
            return x1; // Avoid division by zero
        }
        
        // Linear interpolation: x = x1 + (0 - y1) * (x2 - x1) / (y2 - y1)
        x1 + (0.0 - y1) * (x2 - x1) / (y2 - y1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_spot_payoff() {
        let spot = SpotPosition::new(100.0, 50.0, None);
        let position = Position::Spot(spot);
        
        // At entry price: no profit/loss
        assert_eq!(PayoffEngine::calculate_single_payoff(&position, 50.0), 0.0);
        
        // Price goes up: profit
        assert_eq!(PayoffEngine::calculate_single_payoff(&position, 60.0), 1000.0);
        
        // Price goes down: loss
        assert_eq!(PayoffEngine::calculate_single_payoff(&position, 40.0), -1000.0);
    }

    #[test]
    fn test_option_payoff() {
        // Long call option
        let call = OptionPosition::new(OptionType::Call, 1.0, 50.0, 5.0, None);
        let position = Position::Option(call);
        
        // Below strike: lose premium
        assert_eq!(PayoffEngine::calculate_single_payoff(&position, 45.0), -5.0);
        
        // At strike: still lose premium
        assert_eq!(PayoffEngine::calculate_single_payoff(&position, 50.0), -5.0);
        
        // Above breakeven (strike + premium): profit
        assert_eq!(PayoffEngine::calculate_single_payoff(&position, 60.0), 5.0);
    }

    #[test]
    fn test_payoff_curve_generation() {
        let spot = SpotPosition::new(100.0, 50.0, None);
        let position = Position::Spot(spot);
        let positions = vec![position];
        
        let curve = PayoffEngine::generate_payoff_curve(&positions, 40.0, 60.0, 5.0);
        
        assert_eq!(curve.len(), 5); // 40, 45, 50, 55, 60
        assert_eq!(curve[0].price, 40.0);
        assert_eq!(curve[0].payoff, -1000.0); // (40-50) * 100 = -1000
        assert_eq!(curve[2].price, 50.0);
        assert_eq!(curve[2].payoff, 0.0); // breakeven
        assert_eq!(curve[4].price, 60.0);
        assert_eq!(curve[4].payoff, 1000.0); // (60-50) * 100 = 1000
    }
}
