// Model modules - Pure data structures only
pub mod position;
pub mod web3_data;

// Re-export main types
pub use position::{
    FuturesPosition, OptionPosition, OptionType, Position, PositionType, SpotPosition,
};
pub use web3_data::{
    BackupLocation, CalculationSettings, ChartSettings, ChartTheme, ChartType, ConflictType,
    DataExchangeFormat, DisplaySettings, EnhancedPosition, ExportFormat, ExportSettings,
    ExternalDataSource, LegendPosition, Portfolio, PortfolioSettings, PositionMetadata,
    PositionOrder, PositionSource, PositionTemplate, StorageMetadata, StorageProvider,
    SyncConflict, SyncMetadata,
};
