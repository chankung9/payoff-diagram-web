use crate::models::Position;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, Url, HtmlAnchorElement};

/// Data structure for export/import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub positions: Vec<Position>,
    pub settings: ExportSettings,
    pub metadata: ExportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub price_range_start: f64,
    pub price_range_end: f64,
    pub step_size: f64,
    pub decimal_precision: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub version: String,
    pub exported_at: String,
    pub description: Option<String>,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            price_range_start: 0.0,
            price_range_end: 100.0,
            step_size: 1.0,
            decimal_precision: 2,
        }
    }
}

impl ExportMetadata {
    pub fn new(description: Option<String>) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            exported_at: js_sys::Date::new_0().to_iso_string().as_string().unwrap(),
            description,
        }
    }
}

/// Export data to JSON string
pub fn export_to_json(
    positions: &[Position],
    settings: &ExportSettings,
    description: Option<String>,
) -> Result<String, serde_json::Error> {
    let export_data = ExportData {
        positions: positions.to_vec(),
        settings: settings.clone(),
        metadata: ExportMetadata::new(description),
    };

    serde_json::to_string_pretty(&export_data)
}

/// Export data and trigger download in browser (simplified version)
pub fn export_data(
    positions: &[Position],
    settings: &ExportSettings,
    filename: Option<String>,
    description: Option<String>,
) -> Result<(), JsValue> {
    let json_data = export_to_json(positions, settings, description)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Create blob with JSON data
    let data_array = js_sys::Array::new();
    data_array.push(&JsValue::from_str(&json_data));
    
    let mut blob_options = BlobPropertyBag::new();
    blob_options.set_type("application/json");
    
    let blob = Blob::new_with_str_sequence_and_options(&data_array, &blob_options)?;
    
    // Create download URL
    let url = Url::create_object_url_with_blob(&blob)?;
    
    // Create temporary anchor element and trigger download
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let anchor = document
        .create_element("a")?
        .dyn_into::<HtmlAnchorElement>()?;
    
    let filename = filename.unwrap_or_else(|| {
        format!("payoff-diagram-{}.json", 
            js_sys::Date::new_0().to_iso_string().as_string().unwrap()
                .split('T').next().unwrap_or("export"))
    });
    
    anchor.set_href(&url);
    anchor.set_download(&filename);
    
    // Note: Simplified approach without styling
    document.body().unwrap().append_child(&anchor)?;
    anchor.click();
    document.body().unwrap().remove_child(&anchor)?;
    
    // Clean up object URL
    Url::revoke_object_url(&url)?;
    
    Ok(())
}

/// Export data to clipboard as JSON (simplified version)
pub fn export_to_clipboard(
    positions: &[Position],
    settings: &ExportSettings,
    description: Option<String>,
) -> Result<(), JsValue> {
    let json_data = export_to_json(positions, settings, description)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Use the Clipboard API to copy to clipboard if available
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    
    // Try to use clipboard writeText (simplified approach)
    // This is a basic implementation - in production you'd want better error handling
    js_sys::eval(&format!("navigator.clipboard.writeText(`{}`)", json_data))
        .map_err(|_| JsValue::from_str("Clipboard not supported or failed"))?;
    
    Ok(())
}
