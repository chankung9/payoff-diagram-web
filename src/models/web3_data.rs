// === Web3 Data Model Architecture ===
// This file defines the comprehensive data model for Web3 data sovereignty
// Supporting multiple storage providers, sync, and full user control

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::Position;

// === Portfolio Level (Top-level Container) ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    // Metadata
    pub id: String,                    // UUID v4
    pub name: String,                  // User-defined name
    pub description: Option<String>,   // Optional description
    pub created_at: DateTime<Utc>,     // Creation timestamp
    pub updated_at: DateTime<Utc>,     // Last modification
    pub version: String,               // Schema version (e.g., "1.0.0")
    
    // Core Data
    pub positions: Vec<EnhancedPosition>, // All positions with metadata
    pub settings: PortfolioSettings,   // Chart/calculation settings
    pub tags: Vec<String>,             // User-defined tags
    
    // Web3 Metadata
    pub storage_metadata: StorageMetadata,
    pub sync_metadata: SyncMetadata,
}

impl Portfolio {
    pub fn new(name: String, description: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description: Some(description),
            created_at: now,
            updated_at: now,
            version: "1.0.0".to_string(),
            positions: Vec::new(),
            settings: PortfolioSettings::default(),
            tags: Vec::new(),
            storage_metadata: StorageMetadata::default(),
            sync_metadata: SyncMetadata::default(),
        }
    }

    pub fn add_position(&mut self, position: crate::models::Position) {
        let enhanced_position = EnhancedPosition {
            id: uuid::Uuid::new_v4().to_string(),
            position,
            metadata: PositionMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                source: PositionSource::Manual,
                tags: Vec::new(),
                notes: None,
                external_id: None,
            },
        };
        self.positions.push(enhanced_position);
        self.update_timestamp();
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now();
    }

    // Compatibility field for last_modified
    pub fn last_modified(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}

// === Enhanced Position Model ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedPosition {
    pub id: String,                    // UUID for tracking
    pub position: Position,            // Core position data
    pub metadata: PositionMetadata,    // Additional metadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub source: PositionSource,        // How position was created
    pub tags: Vec<String>,             // User-defined tags
    pub notes: Option<String>,         // User notes
    pub external_id: Option<String>,   // Binance position ID, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionSource {
    Manual,                            // Manually entered
    BinanceAPI { account_id: String }, // Imported from Binance
    CSVImport { filename: String },    // Imported from CSV
    Template { template_id: String },  // From position template
}

// === Portfolio Settings ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSettings {
    // Chart Settings
    pub chart_settings: ChartSettings,
    
    // Calculation Settings
    pub calculation_settings: CalculationSettings,
    
    // Display Settings
    pub display_settings: DisplaySettings,
    
    // Export Settings
    pub export_settings: ExportSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSettings {
    pub price_range: Option<(f64, f64)>,  // Custom price range
    pub step_size: Option<f64>,           // Chart resolution
    pub chart_type: ChartType,            // SVG, Canvas, etc.
    pub theme: ChartTheme,                // Light, Dark, etc.
    pub legend_position: LegendPosition,  // Top-right, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationSettings {
    pub auto_range: bool,                 // Auto-calculate price range
    pub include_inactive: bool,           // Include disabled positions
    pub precision: u8,                    // Decimal precision
    pub currency: String,                 // USD, EUR, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub position_order: PositionOrder,    // Sort order
    pub show_descriptions: bool,          // Show position descriptions
    pub compact_mode: bool,               // Compact UI mode
    pub language: String,                 // UI language
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub default_format: ExportFormat,
    pub include_metadata: bool,
    pub include_settings: bool,
    pub encryption_enabled: bool,
}

// === Storage & Sync Metadata ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    pub storage_provider: StorageProvider,
    pub storage_location: Option<String>,  // File path, IPFS hash, etc.
    pub encryption_enabled: bool,
    pub backup_locations: Vec<BackupLocation>,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMetadata {
    pub last_sync: Option<DateTime<Utc>>,
    pub sync_conflicts: Vec<SyncConflict>,
    pub device_id: String,             // Unique device identifier
    pub sync_enabled: bool,
    pub auto_sync_interval: Option<u64>, // Minutes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageProvider {
    LocalStorage,
    GoogleDrive { account_id: String },
    IPFS { node_url: String },
    Dropbox { account_id: String },
    GitHub { repo: String, path: String },
    Custom { provider_name: String, endpoint: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupLocation {
    pub provider: StorageProvider,
    pub location: String,
    pub last_backup: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub conflict_id: String,
    pub occurred_at: DateTime<Utc>,
    pub conflict_type: ConflictType,
    pub local_version: String,
    pub remote_version: String,
    pub resolved: bool,
}

// === Enums ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    SVG,
    Canvas,
    WebGL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartTheme {
    Light,
    Dark,
    Auto, // Follow system theme
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionOrder {
    CreatedDate,
    UpdatedDate,
    Alphabetical,
    PositionType,
    ProfitLoss,
    Custom(Vec<String>), // Custom order by position IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    JSON,
    CSV,
    Excel,
    PDF,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    PositionModified,
    SettingsChanged,
    PositionAdded,
    PositionRemoved,
    MetadataConflict,
}

// === Position Templates ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: TemplateCategory,
    pub positions: Vec<TemplatePosition>,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateCategory {
    BasicStrategies,
    Options,
    Futures,
    Complex,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePosition {
    pub position_type: String,    // "Spot", "Option", "Futures"
    pub parameters: serde_json::Value, // Flexible parameters
    pub description: String,
}

// === External Data Integration ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDataSource {
    pub id: String,
    pub name: String,
    pub source_type: ExternalSourceType,
    pub credentials: Option<EncryptedCredentials>,
    pub last_sync: Option<DateTime<Utc>>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalSourceType {
    BinanceSpot,
    BinanceFutures,
    BinanceOptions,
    CoinbaseAPI,
    KrakenAPI,
    CustomAPI { endpoint: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    pub encrypted_data: String,  // AES-256 encrypted credentials
    pub salt: String,           // For key derivation
    pub iv: String,             // Initialization vector
}

// === Import/Export Data Exchange Format ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataExchangeFormat {
    // Format metadata
    pub format_version: String,
    pub exported_at: DateTime<Utc>,
    pub exported_by: String,        // Device/app identifier
    pub checksum: Option<String>,   // Data integrity check
    
    // Core data
    pub portfolios: Vec<Portfolio>,
    pub templates: Vec<PositionTemplate>,
    pub external_sources: Vec<ExternalDataSource>,
    
    // Export settings
    pub includes_settings: bool,
    pub includes_metadata: bool,
    pub encryption_used: bool,
}

// === Implementation helpers ===
impl Default for Portfolio {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Portfolio".to_string(),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: "1.0.0".to_string(),
            positions: Vec::new(),
            settings: PortfolioSettings::default(),
            tags: Vec::new(),
            storage_metadata: StorageMetadata::default(),
            sync_metadata: SyncMetadata::default(),
        }
    }
}

impl Default for PortfolioSettings {
    fn default() -> Self {
        Self {
            chart_settings: ChartSettings::default(),
            calculation_settings: CalculationSettings::default(),
            display_settings: DisplaySettings::default(),
            export_settings: ExportSettings::default(),
        }
    }
}

impl Default for ChartSettings {
    fn default() -> Self {
        Self {
            price_range: None,
            step_size: None,
            chart_type: ChartType::SVG,
            theme: ChartTheme::Auto,
            legend_position: LegendPosition::TopRight,
        }
    }
}

impl Default for CalculationSettings {
    fn default() -> Self {
        Self {
            auto_range: true,
            include_inactive: false,
            precision: 2,
            currency: "USD".to_string(),
        }
    }
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            position_order: PositionOrder::CreatedDate,
            show_descriptions: true,
            compact_mode: false,
            language: "en".to_string(),
        }
    }
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            default_format: ExportFormat::JSON,
            include_metadata: true,
            include_settings: true,
            encryption_enabled: false,
        }
    }
}

impl Default for StorageMetadata {
    fn default() -> Self {
        Self {
            storage_provider: StorageProvider::LocalStorage,
            storage_location: None,
            encryption_enabled: false,
            backup_locations: Vec::new(),
            compression_enabled: true,
        }
    }
}

impl Default for SyncMetadata {
    fn default() -> Self {
        Self {
            last_sync: None,
            sync_conflicts: Vec::new(),
            device_id: uuid::Uuid::new_v4().to_string(),
            sync_enabled: false,
            auto_sync_interval: Some(30), // 30 minutes default
        }
    }
}
