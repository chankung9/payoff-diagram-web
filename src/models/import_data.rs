// Data Import Models
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::{Position, api_keys::Platform};

/// Import operation mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImportMode {
    Replace,  // Replace all existing positions
    Append,   // Add to existing positions
}

impl ImportMode {
    pub fn description(&self) -> &str {
        match self {
            ImportMode::Replace => "Replace all existing positions with imported data",
            ImportMode::Append => "Add imported positions to existing portfolio",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            ImportMode::Replace => "🔄",
            ImportMode::Append => "➕",
        }
    }
}

/// Asset types to import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSelection {
    pub spot_positions: bool,
    pub futures_positions: bool,
    pub options_positions: bool,
    pub open_orders: bool,
}

impl Default for AssetSelection {
    fn default() -> Self {
        Self {
            spot_positions: true,
            futures_positions: true,
            options_positions: true,
            open_orders: false, // Orders are not positions
        }
    }
}

/// Symbol filter for import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolFilter {
    pub symbols: Vec<String>,     // Specific symbols to include
    pub base_assets: Vec<String>, // Base assets like SOL, BTC, ETH
    pub quote_assets: Vec<String>, // Quote assets like USDT, USDC
    pub exclude_dust: bool,       // Exclude very small positions
    pub min_value_usd: f64,       // Minimum position value in USD
}

impl Default for SymbolFilter {
    fn default() -> Self {
        Self {
            symbols: vec![],
            base_assets: vec!["SOL".to_string(), "BTC".to_string(), "ETH".to_string()],
            quote_assets: vec!["USDT".to_string(), "USDC".to_string()],
            exclude_dust: true,
            min_value_usd: 1.0,
        }
    }
}

/// Import configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportConfig {
    pub api_key_id: String,           // Which API key to use
    pub mode: ImportMode,             // Replace or append
    pub asset_selection: AssetSelection, // What to import
    pub symbol_filter: SymbolFilter,  // Which symbols to include
    pub create_backup: bool,          // Backup before import
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            api_key_id: String::new(),
            mode: ImportMode::Append,
            asset_selection: AssetSelection::default(),
            symbol_filter: SymbolFilter::default(),
            create_backup: true,
        }
    }
}

/// Import operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub success: bool,
    pub message: String,
    pub positions_imported: usize,
    pub positions_skipped: usize,
    pub errors: Vec<String>,
    pub backup_created: Option<String>, // Backup file name
    pub imported_at: DateTime<Utc>,
}

/// Raw position data from exchange API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPositionData {
    pub platform: Platform,
    pub position_type: String,        // "spot", "futures", "options"
    pub symbol: String,
    pub size: f64,
    pub entry_price: Option<f64>,
    pub mark_price: Option<f64>,
    pub unrealized_pnl: Option<f64>,
    pub raw_data: serde_json::Value, // Original API response
}

impl RawPositionData {
    /// Convert raw position data to app Position model
    pub fn to_position(&self) -> Result<Position, String> {
        // This will be implemented based on each platform's data format
        match self.platform {
            Platform::Binance => self.convert_binance_position(),
        }
    }

    fn convert_binance_position(&self) -> Result<Position, String> {
        // Convert Binance-specific position data to Position enum
        // Implementation will vary based on position type
        Err("Not implemented yet".to_string())
    }
}

/// Import operation status
#[derive(Debug, Clone)]
pub enum ImportStatus {
    NotStarted,
    FetchingData,
    ProcessingData,
    CreatingBackup,
    ImportingPositions,
    Completed(ImportResult),
    Failed(String),
}
