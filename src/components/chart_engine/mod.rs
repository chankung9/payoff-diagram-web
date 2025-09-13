// Chart Engine Module - Modular chart rendering system
// Similar to Binance's chart selection (Binance Chart vs TradingView)

pub mod svg_engine;
pub mod canvas_engine;
pub mod chart_selector;

use crate::models::payoff::PayoffPoint;

#[derive(Debug, Clone, PartialEq)]
pub enum ChartEngine {
    SvgNative,      // Our pure SVG implementation
    CanvasRust,     // Canvas-based Rust implementation  
    ChartJs,        // Chart.js integration (future)
    TradingView,    // TradingView widgets (future)
    Plotters,       // Plotters library (future)
}

impl ChartEngine {
    pub fn display_name(&self) -> &str {
        match self {
            ChartEngine::SvgNative => "SVG Native",
            ChartEngine::CanvasRust => "Canvas Rust", 
            ChartEngine::ChartJs => "Chart.js",
            ChartEngine::TradingView => "TradingView",
            ChartEngine::Plotters => "Plotters",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ChartEngine::SvgNative => "Pure SVG, lightweight, customizable",
            ChartEngine::CanvasRust => "Canvas-based, high performance",
            ChartEngine::ChartJs => "Feature-rich JavaScript charts",
            ChartEngine::TradingView => "Professional trading charts",
            ChartEngine::Plotters => "Rust-native chart library",
        }
    }

    pub fn is_available(&self) -> bool {
        match self {
            ChartEngine::SvgNative => true,        // Always available
            ChartEngine::CanvasRust => true,       // Will implement next
            ChartEngine::ChartJs => false,         // Future implementation
            ChartEngine::TradingView => false,     // Future implementation
            ChartEngine::Plotters => false,        // Future implementation
        }
    }

    pub fn features(&self) -> Vec<&str> {
        match self {
            ChartEngine::SvgNative => vec![
                "Lightweight", "Scalable", "Customizable", "Interactive", "No dependencies"
            ],
            ChartEngine::CanvasRust => vec![
                "High performance", "Smooth animations", "Complex interactions"
            ],
            ChartEngine::ChartJs => vec![
                "Rich features", "Beautiful defaults", "Zoom/Pan", "Multiple chart types"
            ],
            ChartEngine::TradingView => vec![
                "Professional", "Advanced analysis", "Trading tools", "Market data"
            ],
            ChartEngine::Plotters => vec![
                "Pure Rust", "Multiple backends", "Scientific charts", "High precision"
            ],
        }
    }
}

// Chart configuration for all engines
#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub width: u32,
    pub height: u32,
    pub margin: ChartMargin,
    pub colors: ChartColors,
    pub grid: GridConfig,
    pub axes: AxesConfig,
    pub interactive: bool,
    pub animation: bool,
}

#[derive(Debug, Clone)]
pub struct ChartMargin {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

#[derive(Debug, Clone)]
pub struct ChartColors {
    pub background: String,
    pub grid: String,
    pub axis: String,
    pub profit_line: String,
    pub loss_line: String,
    pub zero_line: String,
    pub breakeven: String,
}

#[derive(Debug, Clone)]
pub struct GridConfig {
    pub show_major: bool,
    pub show_minor: bool,
    pub major_color: String,
    pub minor_color: String,
}

#[derive(Debug, Clone)]
pub struct AxesConfig {
    pub show_labels: bool,
    pub show_ticks: bool,
    pub tick_count_x: u32,
    pub tick_count_y: u32,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 400,
            margin: ChartMargin {
                top: 40,
                right: 40,
                bottom: 60,
                left: 80,
            },
            colors: ChartColors {
                background: "#f8f9fa".to_string(),
                grid: "#e9ecef".to_string(),
                axis: "#6c757d".to_string(),
                profit_line: "#28a745".to_string(),
                loss_line: "#dc3545".to_string(),
                zero_line: "#ffc107".to_string(),
                breakeven: "#fd7e14".to_string(),
            },
            grid: GridConfig {
                show_major: true,
                show_minor: false,
                major_color: "#e9ecef".to_string(),
                minor_color: "#f8f9fa".to_string(),
            },
            axes: AxesConfig {
                show_labels: true,
                show_ticks: true,
                tick_count_x: 10,
                tick_count_y: 8,
            },
            interactive: true,
            animation: true,
        }
    }
}

// Chart data for all engines
#[derive(Debug, Clone)]
pub struct ChartData {
    pub payoff_points: Vec<PayoffPoint>,
    pub breakeven_points: Vec<f64>,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub price_range: (f64, f64),
}

// Trait for all chart engines to implement
pub trait ChartRenderer {
    fn render_chart(&self, data: &ChartData, config: &ChartConfig) -> String;
    fn supports_interaction(&self) -> bool;
    fn supports_animation(&self) -> bool;
    fn bundle_size_impact(&self) -> &str; // "None", "Small", "Medium", "Large"
}
