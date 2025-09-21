use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trading platform enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Binance,
    // Future platforms: Coinbase, Kraken, etc.
}

/// API key permissions for different trading types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiKeyPermissions {
    pub can_read_account: bool,   // Account balance and info
    pub can_read_orders: bool,    // Open orders
    pub can_read_futures: bool,   // Futures positions
    pub can_read_options: bool,   // Options positions
    pub can_trade: bool,          // Trading permissions (always false for this app)
}

impl Default for ApiKeyPermissions {
    fn default() -> Self {
        Self {
            can_read_account: true,
            can_read_orders: true,
            can_read_futures: true,
            can_read_options: false,
            can_trade: false,
        }
    }
}

/// API key model for secure storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiKey {
    pub id: String,               // Unique identifier
    pub name: String,             // User-friendly name
    pub platform: Platform,       // Exchange platform
    pub api_key: String,          // Plain API key (for display)
    pub encrypted_secret: String, // Encrypted secret key
    pub permissions: ApiKeyPermissions,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub is_active: bool,
}

impl ApiKey {
    /// Create a new API key instance
    pub fn new(
        name: String,
        platform: Platform,
        api_key: String,
        secret: String,
        permissions: ApiKeyPermissions,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: format!("key_{}", now.timestamp()),
            name,
            platform,
            api_key,
            encrypted_secret: secret, // TODO: Implement encryption
            permissions,
            description: None,
            created_at: now,
            last_used: None,
            is_active: true,
        }
    }

    /// Update the last used timestamp
    pub fn mark_as_used(&mut self) {
        self.last_used = Some(Utc::now());
    }
}

/// Result of API key validation test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyTestResult {
    pub success: bool,
    pub message: String,
    pub tested_permissions: Vec<String>,
    pub tested_at: DateTime<Utc>,
}
