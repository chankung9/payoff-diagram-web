// Model modules
pub mod position;
pub mod payoff;

// Re-export main types
pub use position::{Position, PositionType, SpotPosition, OptionPosition, FuturesPosition, OptionType};
pub use payoff::{PayoffCalculator, PayoffPoint};
