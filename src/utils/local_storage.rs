// Local storage management for Web3 data sovereignty
use wasm_bindgen::prelude::*;
use web_sys::{window, Storage};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::models::{Portfolio, Position};

/// Local storage keys
const PORTFOLIO_LIST_KEY: &str = "payoff_portfolios_v1";
const CURRENT_PORTFOLIO_KEY: &str = "payoff_current_portfolio_v1";
const APP_SETTINGS_KEY: &str = "payoff_app_settings_v1";
const AUTO_SAVE_KEY: &str = "payoff_auto_save_v1";

/// Portfolio metadata for list management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfolioListItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub position_count: usize,
    pub tags: Vec<String>,
}

/// Application settings stored locally
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_save_enabled: bool,
    pub auto_save_interval: u64, // seconds
    pub default_export_format: crate::models::ExportFormat,
    pub theme: String,
    pub language: String,
    pub last_portfolio_id: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_save_enabled: true,
            auto_save_interval: 30, // 30 seconds
            default_export_format: crate::models::ExportFormat::JSON,
            theme: "auto".to_string(),
            language: "en".to_string(),
            last_portfolio_id: None,
        }
    }
}

/// Local storage manager for portfolios and app data
pub struct LocalStorageManager;

impl LocalStorageManager {
    /// Get browser local storage
    fn get_storage() -> Result<Storage, String> {
        window()
            .ok_or("No window available")?
            .local_storage()
            .map_err(|_| "Failed to access local storage")?
            .ok_or("Local storage not available".to_string())
    }

    /// Save portfolio to local storage
    pub fn save_portfolio(portfolio: &Portfolio) -> Result<(), String> {
        let storage = Self::get_storage()?;
        
        // Serialize portfolio
        let portfolio_json = serde_json::to_string(portfolio)
            .map_err(|e| format!("Failed to serialize portfolio: {}", e))?;

        // Save portfolio data
        let portfolio_key = format!("portfolio_{}", portfolio.id);
        storage
            .set_item(&portfolio_key, &portfolio_json)
            .map_err(|_| "Failed to save portfolio to storage")?;

        // Update portfolio list
        Self::update_portfolio_list(portfolio)?;

        // Update current portfolio if this is the active one
        if let Ok(current_id) = Self::get_current_portfolio_id() {
            if current_id == portfolio.id {
                Self::set_current_portfolio_id(&portfolio.id)?;
            }
        }

        web_sys::console::log_1(&format!("Portfolio '{}' saved to local storage", portfolio.name).into());
        Ok(())
    }

    /// Load portfolio from local storage
    pub fn load_portfolio(portfolio_id: &str) -> Result<Portfolio, String> {
        let storage = Self::get_storage()?;
        let portfolio_key = format!("portfolio_{}", portfolio_id);

        let portfolio_json = storage
            .get_item(&portfolio_key)
            .map_err(|_| "Failed to access storage")?
            .ok_or_else(|| format!("Portfolio '{}' not found in storage", portfolio_id))?;

        let portfolio: Portfolio = serde_json::from_str(&portfolio_json)
            .map_err(|e| format!("Failed to deserialize portfolio: {}", e))?;

        web_sys::console::log_1(&format!("Portfolio '{}' loaded from local storage", portfolio.name).into());
        Ok(portfolio)
    }

    /// Delete portfolio from local storage
    pub fn delete_portfolio(portfolio_id: &str) -> Result<(), String> {
        let storage = Self::get_storage()?;
        let portfolio_key = format!("portfolio_{}", portfolio_id);

        // Remove portfolio data
        storage
            .remove_item(&portfolio_key)
            .map_err(|_| "Failed to remove portfolio from storage")?;

        // Update portfolio list
        Self::remove_from_portfolio_list(portfolio_id)?;

        // Clear current portfolio if it was the deleted one
        if let Ok(current_id) = Self::get_current_portfolio_id() {
            if current_id == portfolio_id {
                storage
                    .remove_item(CURRENT_PORTFOLIO_KEY)
                    .map_err(|_| "Failed to clear current portfolio")?;
            }
        }

        web_sys::console::log_1(&format!("Portfolio '{}' deleted from local storage", portfolio_id).into());
        Ok(())
    }

    /// Get list of all portfolios
    pub fn get_portfolio_list() -> Result<Vec<PortfolioListItem>, String> {
        let storage = Self::get_storage()?;

        let list_json = storage
            .get_item(PORTFOLIO_LIST_KEY)
            .map_err(|_| "Failed to access storage")?
            .unwrap_or_else(|| "[]".to_string());

        let list: Vec<PortfolioListItem> = serde_json::from_str(&list_json)
            .map_err(|e| format!("Failed to deserialize portfolio list: {}", e))?;

        Ok(list)
    }

    /// Update portfolio list with current portfolio info
    fn update_portfolio_list(portfolio: &Portfolio) -> Result<(), String> {
        let mut list = Self::get_portfolio_list()?;

        // Remove existing entry if present
        list.retain(|item| item.id != portfolio.id);

        // Add updated entry
        let list_item = PortfolioListItem {
            id: portfolio.id.clone(),
            name: portfolio.name.clone(),
            description: portfolio.description.clone(),
            created_at: portfolio.created_at,
            updated_at: portfolio.updated_at,
            position_count: portfolio.positions.len(),
            tags: portfolio.tags.clone(),
        };
        list.push(list_item);

        // Sort by updated_at (most recent first)
        list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        // Save updated list
        Self::save_portfolio_list(&list)
    }

    /// Remove portfolio from list
    fn remove_from_portfolio_list(portfolio_id: &str) -> Result<(), String> {
        let mut list = Self::get_portfolio_list()?;
        list.retain(|item| item.id != portfolio_id);
        Self::save_portfolio_list(&list)
    }

    /// Save portfolio list to storage
    fn save_portfolio_list(list: &[PortfolioListItem]) -> Result<(), String> {
        let storage = Self::get_storage()?;
        let list_json = serde_json::to_string(list)
            .map_err(|e| format!("Failed to serialize portfolio list: {}", e))?;

        storage
            .set_item(PORTFOLIO_LIST_KEY, &list_json)
            .map_err(|_| "Failed to save portfolio list to storage")?;

        Ok(())
    }

    /// Set current portfolio ID
    pub fn set_current_portfolio_id(portfolio_id: &str) -> Result<(), String> {
        let storage = Self::get_storage()?;
        storage
            .set_item(CURRENT_PORTFOLIO_KEY, portfolio_id)
            .map_err(|_| "Failed to set current portfolio")?;

        // Also update app settings
        let mut settings = Self::get_app_settings()?;
        settings.last_portfolio_id = Some(portfolio_id.to_string());
        Self::save_app_settings(&settings)?;

        Ok(())
    }

    /// Get current portfolio ID
    pub fn get_current_portfolio_id() -> Result<String, String> {
        let storage = Self::get_storage()?;
        storage
            .get_item(CURRENT_PORTFOLIO_KEY)
            .map_err(|_| "Failed to access storage")?
            .ok_or("No current portfolio set".to_string())
    }

    /// Load current portfolio
    pub fn load_current_portfolio() -> Result<Portfolio, String> {
        let portfolio_id = Self::get_current_portfolio_id()?;
        Self::load_portfolio(&portfolio_id)
    }

    /// Get app settings
    pub fn get_app_settings() -> Result<AppSettings, String> {
        let storage = Self::get_storage()?;

        let settings_json = storage
            .get_item(APP_SETTINGS_KEY)
            .map_err(|_| "Failed to access storage")?
            .unwrap_or_else(|| {
                // Return default settings JSON
                serde_json::to_string(&AppSettings::default()).unwrap()
            });

        let settings: AppSettings = serde_json::from_str(&settings_json)
            .map_err(|e| format!("Failed to deserialize app settings: {}", e))?;

        Ok(settings)
    }

    /// Save app settings
    pub fn save_app_settings(settings: &AppSettings) -> Result<(), String> {
        let storage = Self::get_storage()?;
        let settings_json = serde_json::to_string(settings)
            .map_err(|e| format!("Failed to serialize app settings: {}", e))?;

        storage
            .set_item(APP_SETTINGS_KEY, &settings_json)
            .map_err(|_| "Failed to save app settings to storage")?;

        Ok(())
    }

    /// Auto-save portfolio (for background saving)
    pub fn auto_save_portfolio(portfolio: &Portfolio) -> Result<(), String> {
        // Check if auto-save is enabled
        let settings = Self::get_app_settings()?;
        if !settings.auto_save_enabled {
            return Ok(());
        }

        // Save with auto-save timestamp
        let mut portfolio_clone = portfolio.clone();
        portfolio_clone.updated_at = Utc::now();

        Self::save_portfolio(&portfolio_clone)?;

        // Save auto-save metadata
        let auto_save_data = serde_json::json!({
            "last_auto_save": Utc::now(),
            "portfolio_id": portfolio.id
        });

        let storage = Self::get_storage()?;
        storage
            .set_item(AUTO_SAVE_KEY, &auto_save_data.to_string())
            .map_err(|_| "Failed to save auto-save metadata")?;

        web_sys::console::log_1(&"Auto-saved portfolio".into());
        Ok(())
    }

    /// Get storage usage information
    pub fn get_storage_info() -> Result<StorageInfo, String> {
        let storage = Self::get_storage()?;
        let portfolios = Self::get_portfolio_list()?;
        
        let mut total_size = 0;
        let mut portfolio_sizes = Vec::new();

        for portfolio_item in &portfolios {
            let portfolio_key = format!("portfolio_{}", portfolio_item.id);
            if let Ok(Some(data)) = storage.get_item(&portfolio_key) {
                let size = data.len();
                total_size += size;
                portfolio_sizes.push((portfolio_item.name.clone(), size));
            }
        }

        Ok(StorageInfo {
            portfolio_count: portfolios.len(),
            total_size_bytes: total_size,
            portfolio_sizes,
            storage_available: true,
        })
    }

    /// Clear all data (for reset functionality)
    pub fn clear_all_data() -> Result<(), String> {
        let storage = Self::get_storage()?;
        let portfolios = Self::get_portfolio_list()?;

        // Remove all portfolios
        for portfolio in portfolios {
            let portfolio_key = format!("portfolio_{}", portfolio.id);
            storage
                .remove_item(&portfolio_key)
                .map_err(|_| "Failed to remove portfolio data")?;
        }

        // Remove metadata
        storage.remove_item(PORTFOLIO_LIST_KEY).ok();
        storage.remove_item(CURRENT_PORTFOLIO_KEY).ok();
        storage.remove_item(AUTO_SAVE_KEY).ok();
        // Keep app settings for user preferences

        web_sys::console::log_1(&"All portfolio data cleared from local storage".into());
        Ok(())
    }

    /// Migrate legacy data to new format
    pub fn migrate_legacy_data() -> Result<Option<Portfolio>, String> {
        let storage = Self::get_storage()?;
        
        // Check for legacy position data
        const LEGACY_POSITIONS_KEY: &str = "payoff_positions";
        
        if let Ok(Some(legacy_data)) = storage.get_item(LEGACY_POSITIONS_KEY) {
            web_sys::console::log_1(&"Found legacy position data, migrating...".into());
            
            // Try to parse legacy positions
            if let Ok(positions) = serde_json::from_str::<Vec<Position>>(&legacy_data) {
                // Convert to new portfolio format
                let portfolio = crate::utils::BrowserFileManager::convert_positions_to_portfolio(positions)?;
                
                // Save as new portfolio
                Self::save_portfolio(&portfolio)?;
                Self::set_current_portfolio_id(&portfolio.id)?;
                
                // Remove legacy data
                storage.remove_item(LEGACY_POSITIONS_KEY).ok();
                
                web_sys::console::log_1(&format!("Migrated {} positions to new portfolio format", portfolio.positions.len()).into());
                return Ok(Some(portfolio));
            }
        }
        
        Ok(None)
    }
}

/// Storage usage information
#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub portfolio_count: usize,
    pub total_size_bytes: usize,
    pub portfolio_sizes: Vec<(String, usize)>, // (name, size)
    pub storage_available: bool,
}

impl StorageInfo {
    pub fn total_size_mb(&self) -> f64 {
        self.total_size_bytes as f64 / (1024.0 * 1024.0)
    }
    
    pub fn largest_portfolio(&self) -> Option<&(String, usize)> {
        self.portfolio_sizes.iter().max_by_key(|(_, size)| size)
    }
}
