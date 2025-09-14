// File import/export utilities for browser
use crate::models::{web3_data, Portfolio};
use crate::utils::web3_export_import::{export, file_utils, import};
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::*;
use web_sys::{window, Blob, BlobPropertyBag, Document, File, FileReader, HtmlAnchorElement, Url};

/// Browser file operations for import/export
pub struct BrowserFileManager;

impl BrowserFileManager {
    /// Download a file to the user's device
    pub fn download_file(content: &str, filename: &str, mime_type: &str) -> Result<(), String> {
        let window = window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;

        // Create blob with content
        let blob = create_blob(content, mime_type)?;

        // Create download URL
        let url =
            Url::create_object_url_with_blob(&blob).map_err(|_| "Failed to create object URL")?;

        // Create and trigger download
        let anchor = document
            .create_element("a")
            .map_err(|_| "Failed to create anchor element")?
            .dyn_into::<HtmlAnchorElement>()
            .map_err(|_| "Failed to cast to HtmlAnchorElement")?;

        anchor.set_href(&url);
        anchor.set_download(filename);
        anchor
            .set_attribute("style", "display: none")
            .map_err(|e| format!("Failed to set style: {:?}", e))?;

        // Add to body, click, and remove
        let body = document.body().ok_or("No body element")?;
        body.append_child(&anchor)
            .map_err(|_| "Failed to append anchor to body")?;

        anchor.click();

        body.remove_child(&anchor)
            .map_err(|_| "Failed to remove anchor from body")?;

        // Clean up object URL
        Url::revoke_object_url(&url).map_err(|_| "Failed to revoke object URL")?;

        Ok(())
    }

    /// Read file content from File object
    pub async fn read_file_content(file: &File) -> Result<String, String> {
        let file_reader = FileReader::new().map_err(|_| "Failed to create FileReader")?;

        // Create a promise to handle async file reading
        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            let reader_clone = file_reader.clone();
            let reject_clone = reject.clone();

            // Set up onload callback
            let onload_callback = Closure::wrap(Box::new(move |_| {
                if let Ok(result) = reader_clone.result() {
                    resolve.call1(&JsValue::NULL, &result).unwrap_or_default();
                } else {
                    reject_clone
                        .call1(&JsValue::NULL, &JsValue::from_str("Failed to read file"))
                        .unwrap_or_default();
                }
            }) as Box<dyn FnMut(JsValue)>);

            file_reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
            onload_callback.forget();

            let reject_clone2 = reject.clone();
            // Set up onerror callback
            let onerror_callback = Closure::wrap(Box::new(move |_| {
                reject_clone2
                    .call1(&JsValue::NULL, &JsValue::from_str("Error reading file"))
                    .unwrap_or_default();
            }) as Box<dyn FnMut(JsValue)>);

            file_reader.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();

            // Start reading the file
            if let Err(_) = file_reader.read_as_text(file) {
                reject
                    .call1(
                        &JsValue::NULL,
                        &JsValue::from_str("Failed to start reading file"),
                    )
                    .unwrap_or_default();
            }
        });

        // Convert Promise to Rust Future
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        let result = future.await.map_err(|_| "File reading failed")?;

        result
            .as_string()
            .ok_or_else(|| "File content is not a string".to_string())
    }

    /// Export portfolio to file and trigger download
    pub fn export_portfolio_to_file(
        portfolio: &Portfolio,
        format: crate::models::ExportFormat,
    ) -> Result<(), String> {
        let (content, filename, mime_type) = export::generate_download_content(portfolio, format)?;
        Self::download_file(&content, &filename, &mime_type)
    }

    /// Import portfolio from file content
    pub fn import_portfolio_from_content(content: &str) -> Result<Portfolio, String> {
        match import::import_smart(content)? {
            crate::utils::ImportResult::Portfolio(portfolio) => Ok(portfolio),
            crate::utils::ImportResult::Positions(positions) => {
                // Convert legacy positions to new portfolio format
                Self::convert_positions_to_portfolio(positions)
            }
        }
    }

    /// Convert legacy positions to new portfolio format
    pub fn convert_positions_to_portfolio(
        positions: Vec<crate::models::Position>,
    ) -> Result<Portfolio, String> {
        use crate::models::{EnhancedPosition, PositionMetadata, PositionSource};
        use chrono::Utc;

        let enhanced_positions: Vec<EnhancedPosition> = positions
            .into_iter()
            .enumerate()
            .map(|(index, position)| EnhancedPosition {
                id: uuid::Uuid::new_v4().to_string(),
                position,
                metadata: PositionMetadata {
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    source: PositionSource::Manual,
                    tags: vec!["imported".to_string()],
                    notes: Some(format!(
                        "Imported from legacy format (position #{})",
                        index + 1
                    )),
                    external_id: None,
                },
            })
            .collect();

        let mut portfolio = Portfolio::default();
        portfolio.name = "Imported Portfolio".to_string();
        portfolio.description = Some("Portfolio imported from legacy format".to_string());
        portfolio.positions = enhanced_positions;

        Ok(portfolio)
    }

    /// Validate file before import
    pub fn validate_import_file(file: &File) -> Result<(), String> {
        let filename = file.name();
        let file_size = file.size() as usize;

        // Check file extension
        let format = file_utils::detect_format_from_filename(&filename)
            .ok_or_else(|| format!("Unsupported file format: {}", filename))?;

        // Check file size (max 10MB)
        if file_size > 10 * 1024 * 1024 {
            return Err("File too large. Maximum size is 10MB".to_string());
        }

        // Additional format-specific validation
        match format {
            crate::models::ExportFormat::JSON => {
                if !filename.to_lowercase().ends_with(".json") {
                    return Err("JSON files must have .json extension".to_string());
                }
            }
            crate::models::ExportFormat::CSV => {
                if !filename.to_lowercase().ends_with(".csv") {
                    return Err("CSV files must have .csv extension".to_string());
                }
            }
            _ => {
                return Err(format!("Import not yet supported for {:?} format", format));
            }
        }

        Ok(())
    }
}

/// Create a Blob with the given content and MIME type
fn create_blob(content: &str, mime_type: &str) -> Result<Blob, String> {
    let uint8_array = Uint8Array::new_with_length(content.len() as u32);
    uint8_array.copy_from(content.as_bytes());

    let array = Array::new();
    array.push(&uint8_array);

    let mut blob_options = BlobPropertyBag::new();
    blob_options.set_type(mime_type);

    Blob::new_with_u8_array_sequence_and_options(&array, &blob_options)
        .map_err(|_| "Failed to create blob".to_string())
}

/// File drag and drop utilities
pub mod drag_drop {
    use super::*;
    use web_sys::{DataTransfer, DragEvent, FileList};

    /// Extract files from a drop event
    pub fn get_files_from_drop_event(event: &DragEvent) -> Result<Vec<File>, String> {
        let data_transfer = event
            .data_transfer()
            .ok_or("No data transfer in drop event")?;

        let file_list = data_transfer.files().ok_or("No files in drop event")?;

        let mut files = Vec::new();
        for i in 0..file_list.length() {
            if let Some(file) = file_list.get(i) {
                files.push(file);
            }
        }

        if files.is_empty() {
            return Err("No files found in drop event".to_string());
        }

        Ok(files)
    }

    /// Check if drag event contains files
    pub fn drag_event_has_files(event: &DragEvent) -> bool {
        if let Some(data_transfer) = event.data_transfer() {
            let types = data_transfer.types();
            if types.length() > 0 {
                for i in 0..types.length() {
                    let type_str = types.get(i).as_string().unwrap_or_default();
                    if type_str == "Files" {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Validate dropped files
    pub fn validate_dropped_files(files: &[File]) -> Result<(), String> {
        if files.is_empty() {
            return Err("No files to import".to_string());
        }

        if files.len() > 1 {
            return Err("Please drop only one file at a time".to_string());
        }

        let file = &files[0];
        BrowserFileManager::validate_import_file(file)
    }
}
