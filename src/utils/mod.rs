// Utility modules
pub mod web3_export_import;
pub mod browser_file;
pub mod local_storage;

// Re-export for convenience
pub use web3_export_import::export as web3_export;
pub use web3_export_import::import as web3_import;
pub use web3_export_import::file_utils;
pub use web3_export_import::{ImportResult};
pub use browser_file::BrowserFileManager;
pub use local_storage::{LocalStorageManager, AppSettings, PortfolioListItem, StorageInfo};

// Legacy export/import functions (kept for backward compatibility)
pub fn export_to_json_string(positions: &[crate::models::Position]) -> Result<String, String> {
    serde_json::to_string_pretty(positions)
        .map_err(|e| format!("Serialization error: {}", e))
}

pub fn import_from_json_string(json_data: &str) -> Result<Vec<crate::models::Position>, String> {
    serde_json::from_str(json_data)
        .map_err(|e| format!("Deserialization error: {}", e))
}
