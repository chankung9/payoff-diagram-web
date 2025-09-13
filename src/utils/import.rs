use crate::models::Position;
use crate::utils::export::{ExportData, ExportSettings};
use wasm_bindgen::prelude::*;
use web_sys::{FileReader, HtmlInputElement, Event};

/// Import data from JSON string
pub fn import_from_json(json_data: &str) -> Result<(Vec<Position>, ExportSettings), ImportError> {
    let export_data: ExportData = serde_json::from_str(json_data)
        .map_err(|e| ImportError::ParseError(e.to_string()))?;
    
    // Validate version compatibility (basic check)
    if !is_version_compatible(&export_data.metadata.version) {
        return Err(ImportError::VersionError(export_data.metadata.version));
    }
    
    // Validate data integrity
    validate_positions(&export_data.positions)?;
    validate_settings(&export_data.settings)?;
    
    Ok((export_data.positions, export_data.settings))
}

/// Simplified import data from file input element
pub fn import_data(
    file_input: &HtmlInputElement,
    callback: Box<dyn Fn(Result<(Vec<Position>, ExportSettings), ImportError>)>,
) -> Result<(), JsValue> {
    let files = file_input.files().ok_or("No files selected")?;
    let file = files.get(0).ok_or("No file selected")?;
    
    // Validate file type
    if !file.type_().starts_with("application/json") && !file.name().ends_with(".json") {
        callback(Err(ImportError::InvalidFileType(file.type_())));
        return Ok(());
    }
    
    let file_reader = FileReader::new()?;
    
    // Create closure to handle file reading
    let onload_callback = Closure::wrap(Box::new(move |event: Event| {
        let file_reader = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        let content = file_reader.result().unwrap().as_string().unwrap();
        
        let result = import_from_json(&content);
        callback(result);
    }) as Box<dyn Fn(Event)>);
    
    file_reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
    file_reader.read_as_text(&file)?;
    
    // Keep closure alive
    onload_callback.forget();
    
    Ok(())
}

/// Simplified import data from clipboard
pub fn import_from_clipboard(
    callback: Box<dyn Fn(Result<(Vec<Position>, ExportSettings), ImportError>)>,
) -> Result<(), JsValue> {
    // Simplified clipboard reading using eval
    let clipboard_text = js_sys::eval("navigator.clipboard.readText()")
        .map_err(|_| JsValue::from_str("Clipboard not supported"))?;
    
    // This is a simplified approach - in production you'd want proper promise handling
    callback(Err(ImportError::ClipboardNotSupported));
    
    Ok(())
}

/// Validate that the version is compatible
fn is_version_compatible(version: &str) -> bool {
    // Simple version compatibility check
    // For now, accept any version that starts with "0."
    version.starts_with("0.") || version == env!("CARGO_PKG_VERSION")
}

/// Validate positions data
fn validate_positions(positions: &[Position]) -> Result<(), ImportError> {
    for (i, position) in positions.iter().enumerate() {
        match position {
            Position::Spot(spot) => {
                if spot.entry_price < 0.0 {
                    return Err(ImportError::ValidationError(
                        format!("Position {}: Entry price cannot be negative", i)
                    ));
                }
                if spot.quantity == 0.0 {
                    return Err(ImportError::ValidationError(
                        format!("Position {}: Quantity cannot be zero", i)
                    ));
                }
            }
            Position::Option(option) => {
                if option.strike_price < 0.0 {
                    return Err(ImportError::ValidationError(
                        format!("Position {}: Strike price cannot be negative", i)
                    ));
                }
                if option.premium < 0.0 {
                    return Err(ImportError::ValidationError(
                        format!("Position {}: Premium cannot be negative", i)
                    ));
                }
                if option.quantity == 0.0 {
                    return Err(ImportError::ValidationError(
                        format!("Position {}: Quantity cannot be zero", i)
                    ));
                }
            }
            Position::Futures(futures) => {
                if futures.contract_size <= 0.0 {
                    return Err(ImportError::ValidationError(
                        format!("Position {}: Contract size must be positive", i)
                    ));
                }
                if futures.quantity == 0.0 {
                    return Err(ImportError::ValidationError(
                        format!("Position {}: Quantity cannot be zero", i)
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Validate settings data
fn validate_settings(settings: &ExportSettings) -> Result<(), ImportError> {
    if settings.price_range_start >= settings.price_range_end {
        return Err(ImportError::ValidationError(
            "Price range start must be less than end".to_string()
        ));
    }
    
    if settings.step_size <= 0.0 {
        return Err(ImportError::ValidationError(
            "Step size must be positive".to_string()
        ));
    }
    
    if settings.decimal_precision > 10 {
        return Err(ImportError::ValidationError(
            "Decimal precision cannot exceed 10".to_string()
        ));
    }
    
    Ok(())
}

/// Import error types
#[derive(Debug, Clone)]
pub enum ImportError {
    ParseError(String),
    VersionError(String),
    ValidationError(String),
    InvalidFileType(String),
    ClipboardError,
    ClipboardNotSupported,
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ImportError::VersionError(version) => write!(f, "Incompatible version: {}", version),
            ImportError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ImportError::InvalidFileType(file_type) => write!(f, "Invalid file type: {}", file_type),
            ImportError::ClipboardError => write!(f, "Failed to read from clipboard"),
            ImportError::ClipboardNotSupported => write!(f, "Clipboard API not supported"),
        }
    }
}

impl std::error::Error for ImportError {}
