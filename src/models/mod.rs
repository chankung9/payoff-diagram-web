// Model modules - Pure data structures only
pub mod position;

// Re-export main types
pub use position::{Position, PositionType, SpotPosition, OptionPosition, FuturesPosition, OptionType};
