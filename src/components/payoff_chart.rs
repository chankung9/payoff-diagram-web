use dioxus::prelude::*;
use crate::models::Position;
use crate::engine::{PayoffEngine, PayoffPoint};

#[derive(Debug, Clone, PartialEq)]
pub enum ChartEngine {
    SvgNative,
    CanvasRust,
    ChartJs,
    TradingView,
    Plotters,
}

#[derive(Debug, Clone)]
pub struct ChartData {
    pub payoff_points: Vec<PayoffPoint>,
    pub breakeven_points: Vec<f64>,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub price_range: (f64, f64),
}

#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub width: u32,
    pub height: u32,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 400,
        }
    }
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

    pub fn is_available(&self) -> bool {
        match self {
            ChartEngine::SvgNative => true,
            _ => false, // Others coming soon
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PayoffChartProps {
    pub positions: Vec<Position>,
    pub price_start: f64,
    pub price_end: f64,
    pub step_size: f64,
}

pub fn PayoffChart(props: PayoffChartProps) -> Element {
    // Chart engine selection state (like Binance's chart selector)
    let mut selected_engine = use_signal(|| ChartEngine::SvgNative);
    
    // Calculate payoff data
    let payoff_data = if props.positions.is_empty() {
        Vec::new()
    } else {
        PayoffEngine::generate_payoff_curve(
            &props.positions,
            props.price_start,
            props.price_end,
            props.step_size,
        )
    };

    let breakeven_points = if props.positions.is_empty() {
        Vec::new()
    } else {
        PayoffEngine::find_breakeven_points(
            &props.positions, 
            props.price_start, 
            props.price_end, 
            props.step_size
        )
    };

    let max_profit = if props.positions.is_empty() {
        None
    } else {
        PayoffEngine::calculate_max_profit(
            &props.positions, 
            props.price_start, 
            props.price_end, 
            props.step_size
        )
    };

    let max_loss = if props.positions.is_empty() {
        None
    } else {
        PayoffEngine::calculate_max_loss(
            &props.positions, 
            props.price_start, 
            props.price_end, 
            props.step_size
        )
    };

    // Prepare chart data
    let chart_data = ChartData {
        payoff_points: payoff_data,
        breakeven_points,
        max_profit,
        max_loss,
        price_range: (props.price_start, props.price_end),
    };

    // Chart configuration
    let chart_config = ChartConfig::default();

    rsx! {
        div {
            class: "payoff-chart-container",
            
            // Chart Engine Selector (simplified version)
            div {
                class: "chart-engine-selector",
                h3 { "📊 Chart Engine Selection" }
                
                div {
                    class: "engine-dropdown",
                    select {
                        value: "{selected_engine().display_name()}",
                        onchange: move |evt| {
                            if evt.value() == "SVG Native" {
                                selected_engine.set(ChartEngine::SvgNative);
                            }
                        },
                        
                        option { value: "SVG Native", "SVG Native (Available)" }
                        option { value: "Canvas Rust", "Canvas Rust (Coming Soon)" }
                        option { value: "Chart.js", "Chart.js (Coming Soon)" }
                        option { value: "TradingView", "TradingView (Coming Soon)" }
                        option { value: "Plotters", "Plotters (Coming Soon)" }
                    }
                    
                    div {
                        class: "engine-info",
                        "Current: {selected_engine().display_name()}"
                    }
                }
            }
            
            // Chart statistics (keep existing)
            div {
                class: "chart-stats",
                div {
                    class: "stat-item",
                    strong { "Data Points: " }
                    span { "{chart_data.payoff_points.len()}" }
                }
                div {
                    class: "stat-item",
                    strong { "Price Range: " }
                    span { "${props.price_start:.2} - ${props.price_end:.2}" }
                }
                div {
                    class: "stat-item",
                    strong { "Step Size: " }
                    span { "${props.step_size:.2}" }
                }
                if let Some(max_p) = chart_data.max_profit {
                    div {
                        class: "stat-item profit",
                        strong { "Max Profit: " }
                        span { "${max_p:.2}" }
                    }
                }
                if let Some(max_l) = chart_data.max_loss {
                    div {
                        class: "stat-item loss",
                        strong { "Max Loss: " }
                        span { "${max_l:.2}" }
                    }
                }
                if !chart_data.breakeven_points.is_empty() {
                    div {
                        class: "stat-item",
                        strong { "Breakeven Points: " }
                        span {
                            for (i, point) in chart_data.breakeven_points.iter().enumerate() {
                                if i > 0 { ", " }
                                "${point:.2}"
                            }
                        }
                    }
                }
            }
            
            // Chart Rendering Area
            div {
                class: "chart-render-area",
                
                match selected_engine() {
                    ChartEngine::SvgNative => rsx! {
                        div {
                            class: "svg-chart-container",
                            h4 { "🎯 Interactive SVG Payoff Chart" }
                            
                            if chart_data.payoff_points.is_empty() {
                                div {
                                    class: "chart-empty-state",
                                    p { "Add positions to see the interactive SVG payoff diagram" }
                                    
                                    div {
                                        class: "feature-preview",
                                        h5 { "SVG Chart Features:" }
                                        ul {
                                            li { "✅ Lightweight (no external dependencies)" }
                                            li { "✅ Scalable vector graphics" }
                                            li { "✅ Interactive tooltips" }
                                            li { "✅ Customizable styling" }
                                            li { "✅ Responsive design" }
                                        }
                                    }
                                }
                            } else {
                                // Actual SVG Chart Implementation
                                div {
                                    class: "svg-chart-wrapper",
                                    
                                    svg {
                                        class: "payoff-chart-svg",
                                        width: "800",
                                        height: "400", 
                                        view_box: "0 0 800 400",
                                        style: "border: 1px solid #dee2e6; background: #f8f9fa;",
                                        
                                        // Define chart dimensions
                                        defs {
                                            // Gradient for profit area
                                            linearGradient {
                                                id: "profitGradient",
                                                x1: "0%", y1: "0%", x2: "0%", y2: "100%",
                                                stop { offset: "0%", stop_color: "#28a745", stop_opacity: "0.3" }
                                                stop { offset: "100%", stop_color: "#28a745", stop_opacity: "0.1" }
                                            }
                                            // Gradient for loss area
                                            linearGradient {
                                                id: "lossGradient", 
                                                x1: "0%", y1: "0%", x2: "0%", y2: "100%",
                                                stop { offset: "0%", stop_color: "#dc3545", stop_opacity: "0.1" }
                                                stop { offset: "100%", stop_color: "#dc3545", stop_opacity: "0.3" }
                                            }
                                        }
                                        
                                        // Chart area group with margins
                                        g {
                                            transform: "translate(80, 40)",
                                            
                                            // Background grid
                                            g {
                                                class: "grid",
                                                // Vertical grid lines
                                                for i in 0..=10 {
                                                    line {
                                                        x1: "{i * 64}",  // 640px / 10 = 64px spacing
                                                        y1: "0",
                                                        x2: "{i * 64}",
                                                        y2: "320",       // 400 - 80 (margins) = 320px
                                                        stroke: "#e9ecef",
                                                        stroke_width: "1",
                                                        opacity: "0.5"
                                                    }
                                                }
                                                // Horizontal grid lines  
                                                for i in 0..=8 {
                                                    line {
                                                        x1: "0",
                                                        y1: "{i * 40}",  // 320px / 8 = 40px spacing
                                                        x2: "640",
                                                        y2: "{i * 40}",
                                                        stroke: "#e9ecef", 
                                                        stroke_width: "1",
                                                        opacity: "0.5"
                                                    }
                                                }
                                            }
                                            
                                            // Calculate scale for positioning
                                            {
                                                let min_price = chart_data.payoff_points.iter().map(|p| p.price).fold(f64::INFINITY, f64::min);
                                                let max_price = chart_data.payoff_points.iter().map(|p| p.price).fold(f64::NEG_INFINITY, f64::max);
                                                let min_payoff = chart_data.payoff_points.iter().map(|p| p.payoff).fold(f64::INFINITY, f64::min).min(0.0);
                                                let max_payoff = chart_data.payoff_points.iter().map(|p| p.payoff).fold(f64::NEG_INFINITY, f64::max).max(0.0);
                                                
                                                let price_range = if max_price > min_price { max_price - min_price } else { 1.0 };
                                                let payoff_range = if max_payoff > min_payoff { max_payoff - min_payoff } else { 1.0 };
                                                
                                                // Zero line (profit/loss boundary)
                                                let zero_y = 320.0 - ((0.0 - min_payoff) / payoff_range * 320.0);
                                                
                                                rsx! {
                                                    // Zero line (highlighted)
                                                    line {
                                                        x1: "0",
                                                        y1: "{zero_y}",
                                                        x2: "640",
                                                        y2: "{zero_y}",
                                                        stroke: "#ffc107",
                                                        stroke_width: "3",
                                                        stroke_dasharray: "8,4",
                                                        opacity: "0.8"
                                                    }
                                                    
                                                    // Main payoff curve
                                                    polyline {
                                                        points: {
                                                            chart_data.payoff_points.iter()
                                                                .map(|point| {
                                                                    let x = (point.price - min_price) / price_range * 640.0;
                                                                    let y = 320.0 - ((point.payoff - min_payoff) / payoff_range * 320.0);
                                                                    format!("{:.1},{:.1}", x, y)
                                                                })
                                                                .collect::<Vec<_>>()
                                                                .join(" ")
                                                        },
                                                        fill: "none",
                                                        stroke: "#007bff",
                                                        stroke_width: "3",
                                                        stroke_linejoin: "round",
                                                        stroke_linecap: "round"
                                                    }
                                                    
                                                    // Data points (interactive)
                                                    for (i, point) in chart_data.payoff_points.iter().enumerate() {
                                                        if i % 3 == 0 { // Show every 3rd point to avoid clutter
                                                            {
                                                                let x = (point.price - min_price) / price_range * 640.0;
                                                                let y = 320.0 - ((point.payoff - min_payoff) / payoff_range * 320.0);
                                                                rsx! {
                                                                    circle {
                                                                        cx: "{x}",
                                                                        cy: "{y}",
                                                                        r: "5",
                                                                        fill: if point.payoff >= 0.0 { "#28a745" } else { "#dc3545" },
                                                                        stroke: "#ffffff",
                                                                        stroke_width: "2",
                                                                        opacity: "0.8",
                                                                        class: "chart-point",
                                                                        style: "cursor: pointer; transition: all 0.2s ease;",
                                                                        
                                                                        // Hover effects
                                                                        onmouseenter: move |_| {},
                                                                        onmouseleave: move |_| {},
                                                                        
                                                                        // Tooltip
                                                                        title { 
                                                                            "Price: ${point.price:.2} | P&L: ${point.payoff:.2}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    
                                                    // Breakeven points markers
                                                    for be_point in chart_data.breakeven_points.iter() {
                                                        {
                                                            let x = (*be_point - min_price) / price_range * 640.0;
                                                            rsx! {
                                                                // Vertical line for breakeven
                                                                line {
                                                                    x1: "{x}",
                                                                    y1: "0",
                                                                    x2: "{x}",
                                                                    y2: "320",
                                                                    stroke: "#fd7e14",
                                                                    stroke_width: "2",
                                                                    stroke_dasharray: "6,3",
                                                                    opacity: "0.8"
                                                                }
                                                                // Breakeven point marker
                                                                circle {
                                                                    cx: "{x}",
                                                                    cy: "{zero_y}",
                                                                    r: "8",
                                                                    fill: "#fd7e14",
                                                                    stroke: "#ffffff",
                                                                    stroke_width: "3"
                                                                }
                                                                // Breakeven label
                                                                text {
                                                                    x: "{x}",
                                                                    y: "{zero_y - 15.0}",
                                                                    text_anchor: "middle",
                                                                    font_size: "11",
                                                                    font_weight: "bold",
                                                                    fill: "#fd7e14",
                                                                    "BE"
                                                                }
                                                                text {
                                                                    x: "{x}",
                                                                    y: "{zero_y - 4.0}",
                                                                    text_anchor: "middle",
                                                                    font_size: "10",
                                                                    fill: "#6c757d",
                                                                    "${be_point:.0}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                    
                                                    // Axis labels (X-axis)
                                                    for i in 0..=5 {
                                                        {
                                                            let price = min_price + (i as f64 / 5.0) * price_range;
                                                            let x = i as f64 * 128.0; // 640 / 5 = 128
                                                            rsx! {
                                                                text {
                                                                    x: "{x}",
                                                                    y: "340",
                                                                    text_anchor: "middle",
                                                                    font_size: "12",
                                                                    fill: "#6c757d",
                                                                    "${price:.0}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                    
                                                    // Axis labels (Y-axis)
                                                    for i in 0..=5 {
                                                        {
                                                            let payoff = min_payoff + (i as f64 / 5.0) * payoff_range;
                                                            let y = 320.0 - (i as f64 * 64.0); // 320 / 5 = 64
                                                            rsx! {
                                                                text {
                                                                    x: "-15",
                                                                    y: "{y + 4.0}",
                                                                    text_anchor: "end",
                                                                    font_size: "12",
                                                                    fill: "#6c757d",
                                                                    "${payoff:.0}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        
                                        // Chart axes
                                        g {
                                            class: "axes",
                                            // X-axis
                                            line {
                                                x1: "80",
                                                y1: "360",
                                                x2: "720", 
                                                y2: "360",
                                                stroke: "#495057",
                                                stroke_width: "2"
                                            }
                                            // Y-axis
                                            line {
                                                x1: "80",
                                                y1: "40",
                                                x2: "80",
                                                y2: "360",
                                                stroke: "#495057",
                                                stroke_width: "2"
                                            }
                                            
                                            // Axis titles
                                            text {
                                                x: "400",
                                                y: "390",
                                                text_anchor: "middle",
                                                font_size: "14",
                                                font_weight: "bold",
                                                fill: "#495057",
                                                "Stock Price ($)"
                                            }
                                            text {
                                                x: "25",
                                                y: "200",
                                                text_anchor: "middle",
                                                font_size: "14",
                                                font_weight: "bold",
                                                fill: "#495057",
                                                transform: "rotate(-90, 25, 200)",
                                                "Profit / Loss ($)"
                                            }
                                        }
                                        
                                        // Chart title
                                        text {
                                            x: "400",
                                            y: "25",
                                            text_anchor: "middle",
                                            font_size: "16",
                                            font_weight: "bold",
                                            fill: "#212529",
                                            "Options Portfolio Payoff Diagram"
                                        }
                                        
                                        // Interactive legend
                                        g {
                                            transform: "translate(550, 50)",
                                            // Legend background
                                            rect {
                                                x: "0", y: "0", width: "140", height: "80",
                                                fill: "#ffffff",
                                                stroke: "#dee2e6",
                                                stroke_width: "1",
                                                rx: "4"
                                            }
                                            // Legend items
                                            line { x1: "10", y1: "15", x2: "30", y2: "15", stroke: "#007bff", stroke_width: "3" }
                                            text { x: "35", y: "19", font_size: "11", fill: "#495057", "Payoff Curve" }
                                            
                                            line { x1: "10", y1: "30", x2: "30", y2: "30", stroke: "#ffc107", stroke_width: "3", stroke_dasharray: "4,2" }
                                            text { x: "35", y: "34", font_size: "11", fill: "#495057", "Break Even" }
                                            
                                            circle { cx: "20", cy: "45", r: "4", fill: "#28a745" }
                                            text { x: "35", y: "49", font_size: "11", fill: "#495057", "Profit" }
                                            
                                            circle { cx: "20", cy: "60", r: "4", fill: "#dc3545" }
                                            text { x: "35", y: "64", font_size: "11", fill: "#495057", "Loss" }
                                        }
                                    }
                                    
                                    // Chart info panel
                                    div {
                                        class: "chart-info-panel",
                                        style: "margin-top: 15px; padding: 15px; background: #f8f9fa; border-radius: 8px; border: 1px solid #dee2e6;",
                                        
                                        div {
                                            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px;",
                                            
                                            div {
                                                strong { "📊 Chart Statistics" }
                                                div { style: "font-size: 0.9em; color: #6c757d; margin-top: 5px;", 
                                                    "Data Points: {chart_data.payoff_points.len()}" }
                                                div { style: "font-size: 0.9em; color: #6c757d;", 
                                                    "Breakeven Points: {chart_data.breakeven_points.len()}" }
                                            }
                                            
                                            if let Some(max_p) = chart_data.max_profit {
                                                div {
                                                    strong { style: "color: #28a745;", "💰 Max Profit" }
                                                    div { style: "font-size: 1.1em; font-weight: bold; color: #28a745; margin-top: 5px;", 
                                                        "${max_p:.2}" }
                                                }
                                            }
                                            
                                            if let Some(max_l) = chart_data.max_loss {
                                                div {
                                                    strong { style: "color: #dc3545;", "⚠️ Max Loss" }
                                                    div { style: "font-size: 1.1em; font-weight: bold; color: #dc3545; margin-top: 5px;", 
                                                        "${max_l:.2}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => rsx! {
                        div {
                            class: "chart-coming-soon",
                            h4 { "🚧 {selected_engine().display_name()} Chart" }
                            p { "This chart engine is under development" }
                            
                            div {
                                class: "coming-soon-notice",
                                "🎯 For now, please use SVG Native for chart visualization"
                            }
                        }
                    }
                }
            }
        }
    }
}
