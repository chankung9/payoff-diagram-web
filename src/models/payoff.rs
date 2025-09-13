use super::position::{Position, SpotPosition, OptionPosition, FuturesPosition, OptionType};
use serde::{Deserialize, Serialize};

/// A single point on the payoff diagram
#[derive(Debug, Clone, PartialEq)]
pub struct PayoffPoint {
    pub underlying_price: f64,
    pub profit_loss: f64,
}

/// Calculator for payoff diagrams
pub struct PayoffCalculator;

impl PayoffCalculator {
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
            let profit_loss = Self::calculate_portfolio_payoff(positions, current_price);
            points.push(PayoffPoint {
                underlying_price: current_price,
                profit_loss,
            });
            current_price += step_size;
        }

        points
    }

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

        // For long positions: P&L = Quantity * (Intrinsic Value - Premium)
        // For short positions: P&L = Quantity * (Premium - Intrinsic Value) 
        // Note: quantity can be negative for short positions
        let net_payoff = if option.quantity >= 0.0 {
            // Long position: pay premium, receive intrinsic value
            intrinsic_value - option.premium
        } else {
            // Short position: receive premium, pay intrinsic value
            option.premium - intrinsic_value
        };

        option.quantity.abs() * net_payoff * if option.quantity >= 0.0 { 1.0 } else { -1.0 }
    }

    /// Calculate futures position payoff
    fn calculate_futures_payoff(futures: &FuturesPosition, underlying_price: f64) -> f64 {
        // P&L = Quantity * Contract Size * (Current Price - Entry Price)
        futures.quantity * futures.contract_size * (underlying_price - futures.entry_price)
    }

    /// Find break-even points for a portfolio
    pub fn find_breakeven_points(
        positions: &[Position],
        price_start: f64,
        price_end: f64,
        precision: f64,
    ) -> Vec<f64> {
        let mut breakeven_points = Vec::new();
        let step_size = precision;
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

    /// Linear interpolation to find where P&L crosses zero
    fn interpolate_zero_crossing(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        if (y2 - y1).abs() < f64::EPSILON {
            return x1; // Avoid division by zero
        }
        
        // Linear interpolation: x = x1 + (0 - y1) * (x2 - x1) / (y2 - y1)
        x1 + (0.0 - y1) * (x2 - x1) / (y2 - y1)
    }

    /// Calculate maximum profit for a portfolio (if bounded)
    pub fn calculate_max_profit(
        positions: &[Position],
        price_start: f64,
        price_end: f64,
        step_size: f64,
    ) -> Option<f64> {
        let points = Self::generate_payoff_curve(positions, price_start, price_end, step_size);
        points.iter().map(|p| p.profit_loss).max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Calculate maximum loss for a portfolio (if bounded)
    pub fn calculate_max_loss(
        positions: &[Position],
        price_start: f64,
        price_end: f64,
        step_size: f64,
    ) -> Option<f64> {
        let points = Self::generate_payoff_curve(positions, price_start, price_end, step_size);
        points.iter().map(|p| p.profit_loss).min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spot_payoff() {
        let spot = SpotPosition::new(100.0, 50.0, None);
        let position = Position::Spot(spot);
        
        // At entry price: no profit/loss
        assert_eq!(PayoffCalculator::calculate_single_payoff(&position, 50.0), 0.0);
        
        // Price goes up: profit
        assert_eq!(PayoffCalculator::calculate_single_payoff(&position, 60.0), 1000.0);
        
        // Price goes down: loss
        assert_eq!(PayoffCalculator::calculate_single_payoff(&position, 40.0), -1000.0);
    }

    #[test]
    fn test_option_payoff() {
        // Long call option
        let call = OptionPosition::new(OptionType::Call, 1.0, 50.0, 5.0, None);
        let position = Position::Option(call);
        
        // Below strike: lose premium
        assert_eq!(PayoffCalculator::calculate_single_payoff(&position, 45.0), -5.0);
        
        // At strike: still lose premium
        assert_eq!(PayoffCalculator::calculate_single_payoff(&position, 50.0), -5.0);
        
        // Above breakeven (strike + premium): profit
        assert_eq!(PayoffCalculator::calculate_single_payoff(&position, 60.0), 5.0);
    }
}
