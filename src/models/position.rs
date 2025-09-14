use serde::{Deserialize, Serialize};

/// Types of financial positions supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionType {
    Spot,
    Option,
    Futures,
}

/// Option types (Call or Put)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

/// Main position enum that contains all position types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Position {
    Spot(SpotPosition),
    Option(OptionPosition),
    Futures(FuturesPosition),
}

/// Spot position (direct ownership of underlying asset)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotPosition {
    pub quantity: f64,       // Number of units (positive = long, negative = short)
    pub entry_price: f64,    // Price at which position was entered
    pub description: String, // Optional description
    pub active: bool,        // Whether position is active (included in calculations)
}

/// Option position (Call or Put)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionPosition {
    pub option_type: OptionType, // Call or Put
    pub quantity: f64,           // Number of contracts (positive = long, negative = short)
    pub strike_price: f64,       // Strike price of the option
    pub premium: f64,            // Premium paid/received per contract
    pub expiry_price: f64,       // Current price at expiry (for calculation)
    pub description: String,     // Optional description
    pub active: bool,            // Whether position is active (included in calculations)
}

/// Futures position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuturesPosition {
    pub quantity: f64,       // Number of contracts (positive = long, negative = short)
    pub entry_price: f64,    // Price at which futures was entered
    pub contract_size: f64,  // Size of each contract (multiplier)
    pub description: String, // Optional description
    pub active: bool,        // Whether position is active (included in calculations)
}

impl Position {
    /// Get the position type
    pub fn position_type(&self) -> PositionType {
        match self {
            Position::Spot(_) => PositionType::Spot,
            Position::Option(_) => PositionType::Option,
            Position::Futures(_) => PositionType::Futures,
        }
    }

    /// Get a description of the position
    pub fn description(&self) -> &str {
        match self {
            Position::Spot(pos) => &pos.description,
            Position::Option(pos) => &pos.description,
            Position::Futures(pos) => &pos.description,
        }
    }

    /// Get the quantity (for display purposes)
    pub fn quantity(&self) -> f64 {
        match self {
            Position::Spot(pos) => pos.quantity,
            Position::Option(pos) => pos.quantity,
            Position::Futures(pos) => pos.quantity,
        }
    }

    /// Check if position is active
    pub fn is_active(&self) -> bool {
        match self {
            Position::Spot(pos) => pos.active,
            Position::Option(pos) => pos.active,
            Position::Futures(pos) => pos.active,
        }
    }

    /// Toggle position active state
    pub fn toggle_active(&mut self) {
        match self {
            Position::Spot(ref mut pos) => pos.active = !pos.active,
            Position::Option(ref mut pos) => pos.active = !pos.active,
            Position::Futures(ref mut pos) => pos.active = !pos.active,
        }
    }

    /// Set position active state
    pub fn set_active(&mut self, active: bool) {
        match self {
            Position::Spot(ref mut pos) => pos.active = active,
            Position::Option(ref mut pos) => pos.active = active,
            Position::Futures(ref mut pos) => pos.active = active,
        }
    }
}

impl SpotPosition {
    pub fn new(quantity: f64, entry_price: f64, description: Option<String>) -> Self {
        Self {
            quantity,
            entry_price,
            description: description.unwrap_or_else(|| {
                let direction = if quantity >= 0.0 { "Long" } else { "Short" };
                format!("{} {} units @ {}", direction, quantity.abs(), entry_price)
            }),
            active: true, // Default to active
        }
    }
}

impl OptionPosition {
    pub fn new(
        option_type: OptionType,
        quantity: f64,
        strike_price: f64,
        premium: f64,
        description: Option<String>,
    ) -> Self {
        Self {
            option_type,
            quantity,
            strike_price,
            premium,
            expiry_price: strike_price, // Default to strike price
            description: description.unwrap_or_else(|| {
                let direction = if quantity >= 0.0 { "Long" } else { "Short" };
                let opt_type = match option_type {
                    OptionType::Call => "Call",
                    OptionType::Put => "Put",
                };
                format!(
                    "{} {} {} @ Strike {} Premium {}",
                    direction,
                    quantity.abs(),
                    opt_type,
                    strike_price,
                    premium
                )
            }),
            active: true, // Default to active
        }
    }
}

impl FuturesPosition {
    pub fn new(
        quantity: f64,
        entry_price: f64,
        contract_size: f64,
        description: Option<String>,
    ) -> Self {
        Self {
            quantity,
            entry_price,
            contract_size,
            description: description.unwrap_or_else(|| {
                let direction = if quantity >= 0.0 { "Long" } else { "Short" };
                format!(
                    "{} {} Futures @ {} (Size: {})",
                    direction,
                    quantity.abs(),
                    entry_price,
                    contract_size
                )
            }),
            active: true, // Default to active
        }
    }
}
