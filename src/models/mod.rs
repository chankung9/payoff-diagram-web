// Model modules - Pure data structures only
pub mod position;
pub mod web3_data;

// Re-export main types
pub use position::{Position, PositionType, SpotPosition, OptionPosition, FuturesPosition, OptionType};
pub use web3_data::{
    Portfolio, EnhancedPosition, PositionMetadata, PositionSource,
    PortfolioSettings, ChartSettings, CalculationSettings, DisplaySettings, ExportSettings,
    StorageMetadata, SyncMetadata, StorageProvider, BackupLocation, SyncConflict,
    PositionTemplate, ExternalDataSource, DataExchangeFormat,
    ChartType, ChartTheme, LegendPosition, PositionOrder, ExportFormat, ConflictType
};
