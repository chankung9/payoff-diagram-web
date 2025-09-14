// === Enhanced Export/Import System for Web3 Data Sovereignty ===
// This module provides comprehensive data export/import functionality
// with support for multiple formats, encryption, and metadata preservation

use serde_json;
use chrono::Utc;
use crate::models::{Position, Portfolio, DataExchangeFormat, ExportFormat, PositionTemplate, ExternalDataSource};

// === Enhanced Export Functions ===
pub mod export {
    use super::*;

    /// Export complete portfolio with all metadata and settings
    pub fn export_portfolio_complete(portfolio: &Portfolio) -> Result<String, String> {
        let exchange_format = DataExchangeFormat {
            format_version: "1.0.0".to_string(),
            exported_at: Utc::now(),
            exported_by: format!("payoff-diagram-web-{}", env!("CARGO_PKG_VERSION")),
            checksum: None, // TODO: Implement checksums
            portfolios: vec![portfolio.clone()],
            templates: Vec::new(), // TODO: Load user templates
            external_sources: Vec::new(), // TODO: Load external sources
            includes_settings: true,
            includes_metadata: true,
            encryption_used: false,
        };

        serde_json::to_string_pretty(&exchange_format)
            .map_err(|e| format!("Export serialization error: {}", e))
    }

    /// Export only positions (legacy format for backward compatibility)
    pub fn export_positions_only(positions: &[Position]) -> Result<String, String> {
        serde_json::to_string_pretty(positions)
            .map_err(|e| format!("Position export error: {}", e))
    }

    /// Export portfolio to specific format
    pub fn export_portfolio_format(
        portfolio: &Portfolio, 
        format: ExportFormat
    ) -> Result<String, String> {
        match format {
            ExportFormat::JSON => export_portfolio_complete(portfolio),
            ExportFormat::CSV => export_to_csv(portfolio),
            ExportFormat::Excel => Err("Excel export not yet implemented".to_string()),
            ExportFormat::PDF => Err("PDF export not yet implemented".to_string()),
        }
    }

    /// Export portfolio settings only
    pub fn export_settings_only(portfolio: &Portfolio) -> Result<String, String> {
        serde_json::to_string_pretty(&portfolio.settings)
            .map_err(|e| format!("Settings export error: {}", e))
    }

    /// Export to CSV format (positions only)
    fn export_to_csv(portfolio: &Portfolio) -> Result<String, String> {
        let mut csv_content = String::new();
        
        // CSV Header
        csv_content.push_str("ID,Type,Quantity,Entry_Price,Strike_Price,Premium,Contract_Size,Description,Active,Created_At,Tags\n");
        
        // CSV Data
        for enhanced_pos in &portfolio.positions {
            let pos = &enhanced_pos.position;
            let meta = &enhanced_pos.metadata;
            
            let row = match pos {
                Position::Spot(spot) => format!(
                    "{},Spot,{},{},,,,,\"{}\",{},{},\"{}\"\n",
                    enhanced_pos.id,
                    spot.quantity,
                    spot.entry_price,
                    spot.description.replace("\"", "\"\""),
                    spot.active,
                    meta.created_at.format("%Y-%m-%d %H:%M:%S"),
                    meta.tags.join(";")
                ),
                Position::Option(opt) => format!(
                    "{},Option,{},{},{},{},,,\"{}\",{},{},\"{}\"\n",
                    enhanced_pos.id,
                    opt.quantity,
                    opt.expiry_price,
                    opt.strike_price,
                    opt.premium,
                    opt.description.replace("\"", "\"\""),
                    opt.active,
                    meta.created_at.format("%Y-%m-%d %H:%M:%S"),
                    meta.tags.join(";")
                ),
                Position::Futures(fut) => format!(
                    "{},Futures,{},{},,,{},\"{}\",{},{},\"{}\"\n",
                    enhanced_pos.id,
                    fut.quantity,
                    fut.entry_price,
                    fut.contract_size,
                    fut.description.replace("\"", "\"\""),
                    fut.active,
                    meta.created_at.format("%Y-%m-%d %H:%M:%S"),
                    meta.tags.join(";")
                ),
            };
            csv_content.push_str(&row);
        }
        
        Ok(csv_content)
    }

    /// Generate file download content with proper MIME type
    pub fn generate_download_content(
        portfolio: &Portfolio,
        format: ExportFormat
    ) -> Result<(String, String, String), String> {
        let content = export_portfolio_format(portfolio, format.clone())?;
        let filename = generate_filename(&portfolio.name, &format);
        let mime_type = get_mime_type(&format);
        
        Ok((content, filename, mime_type))
    }

    fn generate_filename(portfolio_name: &str, format: &ExportFormat) -> String {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let clean_name = portfolio_name
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect::<String>();
        
        match format {
            ExportFormat::JSON => format!("{}_{}.json", clean_name, timestamp),
            ExportFormat::CSV => format!("{}_{}.csv", clean_name, timestamp),
            ExportFormat::Excel => format!("{}_{}.xlsx", clean_name, timestamp),
            ExportFormat::PDF => format!("{}_{}.pdf", clean_name, timestamp),
        }
    }

    fn get_mime_type(format: &ExportFormat) -> String {
        match format {
            ExportFormat::JSON => "application/json".to_string(),
            ExportFormat::CSV => "text/csv".to_string(),
            ExportFormat::Excel => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
            ExportFormat::PDF => "application/pdf".to_string(),
        }
    }
}

// === Enhanced Import Functions ===
pub mod import {
    use super::*;

    /// Import complete portfolio from exchange format
    pub fn import_portfolio_complete(json_data: &str) -> Result<Portfolio, String> {
        let exchange_format: DataExchangeFormat = serde_json::from_str(json_data)
            .map_err(|e| format!("Import deserialization error: {}", e))?;

        // Validate format version
        if !is_compatible_version(&exchange_format.format_version) {
            return Err(format!(
                "Incompatible format version: {}. Expected 1.x.x", 
                exchange_format.format_version
            ));
        }

        // Extract first portfolio (for now, support single portfolio)
        exchange_format.portfolios
            .into_iter()
            .next()
            .ok_or_else(|| "No portfolios found in import data".to_string())
    }

    /// Import positions only (legacy format)
    pub fn import_positions_only(json_data: &str) -> Result<Vec<Position>, String> {
        serde_json::from_str(json_data)
            .map_err(|e| format!("Position import error: {}", e))
    }

    /// Smart import - detect format automatically
    pub fn import_smart(data: &str) -> Result<ImportResult, String> {
        // Try complete portfolio format first
        if let Ok(portfolio) = import_portfolio_complete(data) {
            return Ok(ImportResult::Portfolio(portfolio));
        }

        // Try legacy positions format
        if let Ok(positions) = import_positions_only(data) {
            return Ok(ImportResult::Positions(positions));
        }

        // Try CSV format
        if let Ok(positions) = import_from_csv(data) {
            return Ok(ImportResult::Positions(positions));
        }

        Err("Unable to parse import data in any supported format".to_string())
    }

    /// Import from CSV format
    pub fn import_from_csv(csv_data: &str) -> Result<Vec<Position>, String> {
        let mut positions = Vec::new();
        let lines: Vec<&str> = csv_data.lines().collect();
        
        if lines.is_empty() {
            return Err("Empty CSV data".to_string());
        }

        // Skip header line
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let position = parse_csv_line(line)
                .map_err(|e| format!("Error on line {}: {}", line_num + 2, e))?;
            
            positions.push(position);
        }

        Ok(positions)
    }

    fn parse_csv_line(line: &str) -> Result<Position, String> {
        let fields: Vec<&str> = line.split(',').collect();
        
        if fields.len() < 9 {
            return Err("Insufficient fields in CSV line".to_string());
        }

        let position_type = fields[1].trim();
        let quantity: f64 = fields[2].trim().parse()
            .map_err(|_| "Invalid quantity")?;
        let entry_price: f64 = fields[3].trim().parse()
            .map_err(|_| "Invalid entry price")?;
        let description = fields[8].trim().trim_matches('"').to_string();

        match position_type {
            "Spot" => {
                Ok(Position::Spot(crate::models::SpotPosition::new(
                    quantity,
                    entry_price,
                    Some(description),
                )))
            },
            "Option" => {
                let strike_price: f64 = fields[4].trim().parse()
                    .map_err(|_| "Invalid strike price")?;
                let premium: f64 = fields[5].trim().parse()
                    .map_err(|_| "Invalid premium")?;
                
                // Default to Call for now - CSV format could be enhanced
                Ok(Position::Option(crate::models::OptionPosition::new(
                    crate::models::OptionType::Call,
                    quantity,
                    strike_price,
                    premium,
                    Some(description),
                )))
            },
            "Futures" => {
                let contract_size: f64 = fields[6].trim().parse()
                    .map_err(|_| "Invalid contract size")?;
                
                Ok(Position::Futures(crate::models::FuturesPosition::new(
                    quantity,
                    entry_price,
                    contract_size,
                    Some(description),
                )))
            },
            _ => Err(format!("Unknown position type: {}", position_type)),
        }
    }

    fn is_compatible_version(version: &str) -> bool {
        // For now, accept any 1.x.x version
        version.starts_with("1.")
    }

    /// Validate imported data integrity
    pub fn validate_import_data(portfolio: &Portfolio) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Basic validation
        if portfolio.name.trim().is_empty() {
            errors.push("Portfolio name cannot be empty".to_string());
        }

        if portfolio.positions.is_empty() {
            errors.push("Portfolio contains no positions".to_string());
        }

        // Position validation
        for (index, enhanced_pos) in portfolio.positions.iter().enumerate() {
            let pos = &enhanced_pos.position;
            match pos {
                Position::Spot(spot) => {
                    if spot.entry_price <= 0.0 {
                        errors.push(format!("Position {}: Invalid entry price", index + 1));
                    }
                },
                Position::Option(opt) => {
                    if opt.strike_price <= 0.0 {
                        errors.push(format!("Position {}: Invalid strike price", index + 1));
                    }
                    if opt.premium < 0.0 {
                        errors.push(format!("Position {}: Invalid premium", index + 1));
                    }
                },
                Position::Futures(fut) => {
                    if fut.entry_price <= 0.0 {
                        errors.push(format!("Position {}: Invalid entry price", index + 1));
                    }
                    if fut.contract_size <= 0.0 {
                        errors.push(format!("Position {}: Invalid contract size", index + 1));
                    }
                },
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// === Import Result Types ===
#[derive(Debug)]
pub enum ImportResult {
    Portfolio(Portfolio),
    Positions(Vec<Position>),
}

// === File Handling Utilities ===
pub mod file_utils {
    /// Generate a safe filename from user input
    pub fn sanitize_filename(input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
            .collect::<String>()
            .trim_matches('.')
            .to_string()
    }

    /// Detect file format from extension
    pub fn detect_format_from_filename(filename: &str) -> Option<super::ExportFormat> {
        let extension = filename.split('.').last()?.to_lowercase();
        match extension.as_str() {
            "json" => Some(super::ExportFormat::JSON),
            "csv" => Some(super::ExportFormat::CSV),
            "xlsx" | "xls" => Some(super::ExportFormat::Excel),
            "pdf" => Some(super::ExportFormat::PDF),
            _ => None,
        }
    }

    /// Estimate file size for export
    pub fn estimate_export_size(portfolio: &super::Portfolio) -> usize {
        // Rough estimation based on position count and metadata
        let base_size = 1024; // Base JSON structure
        let position_size = portfolio.positions.len() * 512; // ~512 bytes per position
        let metadata_size = 2048; // Settings and metadata
        
        base_size + position_size + metadata_size
    }
}
