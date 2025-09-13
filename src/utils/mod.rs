// Placeholder for utils module
// Will be implemented later with proper Dioxus patterns

pub mod export {
    use serde_json;
    use crate::models::Position;

    pub fn export_to_json_string(positions: &[Position]) -> Result<String, String> {
        serde_json::to_string_pretty(positions)
            .map_err(|e| format!("Serialization error: {}", e))
    }
}

pub mod import {
    use serde_json;
    use crate::models::Position;

    pub fn import_from_json_string(json_data: &str) -> Result<Vec<Position>, String> {
        serde_json::from_str(json_data)
            .map_err(|e| format!("Deserialization error: {}", e))
    }
}
