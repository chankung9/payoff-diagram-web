use dioxus::prelude::*;
use crate::models::Position;
use crate::engine::{PayoffEngine, PayoffPoint};
use web_sys;

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
    let selected_engine = use_signal(|| ChartEngine::SvgNative);
    
    // Binance-style legend state (show on long press/hold)
    let mut legend_visible = use_signal(|| false);
    let mut legend_position = use_signal(|| (40.0, 20.0)); // Adjusted for larger Binance mobile popup
    let mut is_dragging = use_signal(|| false);
    let mut drag_offset = use_signal(|| (0.0, 0.0));
    
    // Mobile detection using effect to check CSS media query and touch capability
    let mut is_mobile = use_signal(|| false);
    
    // Check if mobile on component mount
    use_effect(move || {
        // More accurate mobile detection combining width and touch capability
        let window = web_sys::window().unwrap();
        
        // Check screen width
        let width_mobile = window
            .inner_width().ok()
            .and_then(|w| w.as_f64())
            .map(|width| width <= 480.0) // More conservative mobile threshold
            .unwrap_or(false);
            
        // Check if device has touch capability
        let touch_mobile = window
            .navigator()
            .max_touch_points() > 0;
            
        // Check user agent for mobile indicators
        let ua_mobile = window
            .navigator()
            .user_agent()
            .map(|ua| ua.to_lowercase().contains("mobile") || 
                     ua.to_lowercase().contains("android") ||
                     ua.to_lowercase().contains("iphone"))
            .unwrap_or(false);
            
        // Device is mobile if it meets width criteria AND has touch OR is detected via user agent
        let mobile = (width_mobile && touch_mobile) || ua_mobile;
        is_mobile.set(mobile);
    });
    
    // Hover data state for legend display
    let mut hover_data = use_signal(|| None::<(f64, f64, f64)>); // price, pnl, percent
    let mut last_hover_data = use_signal(|| (0.0, 0.0, 0.0)); // Keep last hovered data
    
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
            
            // Chart Engine Selector (hidden for now)
            // div {
            //     class: "chart-engine-selector",
            //     h3 { "📊 Chart Engine Selection" }
            //     
            //     div {
            //         class: "engine-dropdown",
            //         select {
            //             value: "{selected_engine().display_name()}",
            //             onchange: move |evt| {
            //                 if evt.value() == "SVG Native" {
            //                     selected_engine.set(ChartEngine::SvgNative);
            //                 }
            //             },
            //             
            //             option { value: "SVG Native", "SVG Native (Available)" }
            //             option { value: "Canvas Rust", "Canvas Rust (Coming Soon)" }
            //             option { value: "Chart.js", "Chart.js (Coming Soon)" }
            //             option { value: "TradingView", "TradingView (Coming Soon)" }
            //             option { value: "Plotters", "Plotters (Coming Soon)" }
            //         }
            //         
            //         div {
            //             class: "engine-info",
            //             "Current: {selected_engine().display_name()}"
            //         }
            //     }
            // }
            
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
                                        width: "100%",
                                        height: "100%", 
                                        view_box: "0 0 800 400",
                                        preserve_aspect_ratio: "xMidYMid meet",
                                        style: "border: 1px solid #dee2e6; background: #f8f9fa; min-height: 400px; max-height: 600px;",
                                        
                                        // Global mouse events for dragging
                                        onmousemove: move |evt| {
                                            if is_dragging() {
                                                let rect = evt.client_coordinates();
                                                let offset = drag_offset();
                                                let new_x = (rect.x as f64 - offset.0).max(0.0).min(660.0); // Constrain within SVG
                                                let new_y = (rect.y as f64 - offset.1).max(0.0).min(320.0);
                                                legend_position.set((new_x, new_y));
                                            }
                                        },
                                        onmouseup: move |_| {
                                            is_dragging.set(false);
                                        },
                                        onmouseleave: move |_| {
                                            is_dragging.set(false);
                                            hover_data.set(None); // Clear hover when leaving SVG
                                        },
                                        
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
                                                    {
                                                        let points = chart_data.payoff_points.clone();
                                                        let positions = props.positions.clone();
                                                        rsx! {
                                                            for (i, point) in points.iter().enumerate() {
                                                                if i % 3 == 0 { // Show every 3rd point to avoid clutter
                                                                    {
                                                                        let x = (point.price - min_price) / price_range * 640.0;
                                                                        let y = 320.0 - ((point.payoff - min_payoff) / payoff_range * 320.0);
                                                                        
                                                                        // Calculate percentage change
                                                                        let initial_value = positions.iter()
                                                                            .map(|pos| match pos {
                                                                                Position::Option(opt) => opt.premium * opt.quantity.abs(),
                                                                                Position::Spot(spot) => spot.entry_price * spot.quantity.abs(),
                                                                                Position::Futures(fut) => fut.entry_price * fut.quantity.abs() * fut.contract_size,
                                                                            })
                                                                            .sum::<f64>();
                                                                        let percent_change = if initial_value > 0.0 {
                                                                            (point.payoff / initial_value) * 100.0
                                                                        } else {
                                                                            0.0
                                                                        };
                                                                        
                                                                        let price = point.price;
                                                                        let payoff = point.payoff;
                                                                        
                                                                        rsx! {
                                                                            circle {
                                                                                cx: "{x}",
                                                                                cy: "{y}",
                                                                                r: "6", // Slightly larger for better interaction
                                                                                fill: if point.payoff >= 0.0 { "#28a745" } else { "#dc3545" },
                                                                                stroke: "#ffffff",
                                                                                stroke_width: "2",
                                                                                opacity: "0.8",
                                                                                class: "chart-point",
                                                                                style: "cursor: pointer; transition: all 0.2s ease;",
                                                                                
                                                                                // Binance-style long press interaction
                                                                                onmousedown: move |evt| {
                                                                                    evt.prevent_default();
                                                                                    evt.stop_propagation(); // Prevent event bubbling
                                                                                    let data = (price, payoff, percent_change);
                                                                                    hover_data.set(Some(data));
                                                                                    last_hover_data.set(data);
                                                                                    
                                                                                    // Set legend position near the clicked point (only for desktop)
                                                                                    if !is_mobile() {
                                                                                        let mouse_pos = evt.client_coordinates();
                                                                                        // Adjust position to keep legend within bounds
                                                                                        let x = (mouse_pos.x as f64 + 20.0).min(520.0); // Keep within SVG bounds
                                                                                        let y = (mouse_pos.y as f64 - 100.0).max(50.0);
                                                                                        legend_position.set((x, y));
                                                                                        legend_visible.set(true);
                                                                                    }
                                                                                },
                                                                                
                                                                                // Touch events for mobile
                                                                                ontouchstart: move |evt| {
                                                                                    if is_mobile() {
                                                                                        evt.prevent_default();
                                                                                        let data = (price, payoff, percent_change);
                                                                                        hover_data.set(Some(data));
                                                                                        last_hover_data.set(data);
                                                                                        
                                                                                        // Center mobile popup (Binance-style)
                                                                                        legend_position.set((40.0, 20.0));
                                                                                        legend_visible.set(true);
                                                                                    }
                                                                                },
                                                                                
                                                                                onclick: move |evt| {
                                                                                    if is_mobile() {
                                                                                        evt.prevent_default();
                                                                                        evt.stop_propagation();
                                                                                        let data = (price, payoff, percent_change);
                                                                                        hover_data.set(Some(data));
                                                                                        last_hover_data.set(data);
                                                                                        
                                                                                        // Center mobile popup (Binance-style)
                                                                                        legend_position.set((40.0, 20.0));
                                                                                        legend_visible.set(true);
                                                                                    }
                                                                                },
                                                                                
                                                                                onmouseup: move |_| {
                                                                                    // Keep legend visible for a moment
                                                                                },
                                                                                
                                                                                onmouseleave: move |_| {
                                                                                    // Hide legend when leaving dot (desktop only)
                                                                                    if !is_mobile() {
                                                                                        hover_data.set(None);
                                                                                        legend_visible.set(false);
                                                                                    }
                                                                                },
                                                                                
                                                                                // Quick hover feedback (without showing legend for desktop)
                                                                                onmouseenter: move |_| {
                                                                                    if !is_mobile() {
                                                                                        let data = (price, payoff, percent_change);
                                                                                        hover_data.set(Some(data));
                                                                                        last_hover_data.set(data);
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    
                                                    // Breakeven points markers
                                                    {
                                                        let breakeven_points = chart_data.breakeven_points.clone();
                                                        rsx! {
                                                            for be_point in breakeven_points.iter() {
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
                                                                        // Breakeven price label only
                                                                        text {
                                                                            x: "{x}",
                                                                            y: "{zero_y - 10.0}",
                                                                            text_anchor: "middle",
                                                                            font_size: "10",
                                                                            fill: "#6c757d",
                                                                            "${be_point:.0}"
                                                                        }
                                                                    }
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
                                        
                                        // Unified Binance-style legend for both desktop and mobile
                                        if legend_visible() {
                                            // Background overlay for mobile (semi-transparent)
                                            if is_mobile() {
                                                rect {
                                                    x: "0", y: "0", width: "800", height: "400",
                                                    fill: "rgba(0, 0, 0, 0.3)", // Semi-transparent overlay
                                                    style: "cursor: pointer;",
                                                    onclick: move |_| {
                                                        // Close popup when clicking outside
                                                        legend_visible.set(false);
                                                    }
                                                }
                                            }
                                            
                                            // Unified Binance-style legend (positioned at top-right)
                                            g {
                                                class: "binance-legend",
                                                style: "cursor: move;",
                                                
                                                // Drag events for both platforms
                                                onmousedown: move |evt| {
                                                    is_dragging.set(true);
                                                    let rect = evt.client_coordinates();
                                                    drag_offset.set((rect.x as f64, rect.y as f64));
                                                },
                                                
                                                // Compact legend background for top-right positioning
                                                rect {
                                                    x: "0", y: "0", 
                                                    width: if is_mobile() { "260" } else { "220" }, // Smaller for top-right corner
                                                    height: if is_mobile() { "320" } else { "280" }, // Compact height
                                                    fill: "rgba(30, 35, 41, 0.95)", // Binance dark theme
                                                    stroke: "#F0B90B", // Binance yellow
                                                    stroke_width: "2",
                                                    rx: "8",
                                                    style: "filter: drop-shadow(4px 4px 8px rgba(0,0,0,0.3));",
                                                    
                                                    // Double tap to close popup (mobile)
                                                    ondoubleclick: move |_| {
                                                        legend_visible.set(false);
                                                    },
                                                    
                                                    // Prevent closing when clicking inside popup
                                                    onclick: move |evt| {
                                                        evt.stop_propagation();
                                                    }
                                                }
                                                
                                                // Legend title (responsive text size)
                                                text { 
                                                    x: "20", // Left alignment for both mobile and desktop
                                                    y: "30",
                                                    text_anchor: "start", // Left align text
                                                    font_size: if is_mobile() { "18" } else { "16" }, // Responsive font size
                                                    font_weight: "bold",
                                                    fill: "#F0B90B", // Binance yellow
                                                    "📊 Payoff Analysis"
                                                }
                                                
                                                // Chart legend items (Binance-style with responsive sizing)
                                                line { 
                                                    x1: "20", y1: "55", x2: "45", y2: "55", 
                                                    stroke: "#02C076", // Binance green
                                                    stroke_width: "3" 
                                                }
                                                text { 
                                                    x: "50", y: "60", 
                                                    font_size: if is_mobile() { "14" } else { "12" }, 
                                                    fill: "#EAECEF", // Binance light gray
                                                    "Payoff Curve" 
                                                }
                                                
                                                line { 
                                                    x1: "20", y1: "80", x2: "45", y2: "80", 
                                                    stroke: "#F0B90B", stroke_width: "3", 
                                                    stroke_dasharray: "4,2" 
                                                }
                                                text { 
                                                    x: "50", y: "85", 
                                                    font_size: if is_mobile() { "14" } else { "12" }, 
                                                    fill: "#EAECEF", 
                                                    "Break Even" 
                                                }
                                                
                                                circle { 
                                                    cx: "32", cy: "105", r: "5", 
                                                    fill: "#02C076" // Binance green
                                                }
                                                text { 
                                                    x: "50", y: "110", 
                                                    font_size: if is_mobile() { "14" } else { "12" }, 
                                                    fill: "#EAECEF", 
                                                    "Profit Zone" 
                                                }
                                                
                                                circle { 
                                                    cx: "32", cy: "130", r: "5", 
                                                    fill: "#F6465D" // Binance red
                                                }
                                                text { 
                                                    x: "50", y: "135", 
                                                    font_size: if is_mobile() { "14" } else { "12" }, 
                                                    fill: "#EAECEF", 
                                                    "Loss Zone" 
                                                }
                                                
                                                // Separator line (responsive width)
                                                line { 
                                                    x1: "20", y1: "155", 
                                                    x2: if is_mobile() { "300" } else { "260" }, y2: "155", 
                                                    stroke: "#474D57", stroke_width: "1" 
                                                }
                                                
                                                // Real-time data display section (responsive)
                                                {
                                                    let display_data = if let Some(current_data) = hover_data() {
                                                        current_data
                                                    } else {
                                                        last_hover_data()
                                                    };
                                                    
                                                    let (price, payoff, percent) = display_data;
                                                    let status = if hover_data().is_some() { "LIVE" } else { "LAST" };
                                                    
                                                    rsx! {
                                                        // Data status indicator (responsive)
                                                        text { 
                                                            x: if is_mobile() { "160" } else { "140" }, 
                                                            y: "180", 
                                                            text_anchor: "middle",
                                                            font_size: if is_mobile() { "13" } else { "11" }, 
                                                            font_weight: "bold",
                                                            fill: if hover_data().is_some() { "#02C076" } else { "#848E9C" },
                                                            "● {status} DATA"
                                                        }
                                                        
                                                        // Price display (responsive font size)
                                                        text { 
                                                            x: "20", y: "210", 
                                                            font_size: if is_mobile() { "16" } else { "14" }, 
                                                            font_weight: "bold",
                                                            fill: "#EAECEF",
                                                            "Price: ${price:.2}"
                                                        }
                                                        
                                                        // P&L display (responsive font size)
                                                        text { 
                                                            x: "20", y: "240", 
                                                            font_size: if is_mobile() { "16" } else { "14" }, 
                                                            font_weight: "bold",
                                                            fill: if payoff >= 0.0 { "#02C076" } else { "#F6465D" },
                                                            "P&L: ${payoff:.2}"
                                                        }
                                                        
                                                        // Percentage change (responsive font size)
                                                        text { 
                                                            x: "20", y: "270", 
                                                            font_size: if is_mobile() { "14" } else { "12" }, 
                                                            font_weight: "bold",
                                                            fill: if payoff >= 0.0 { "#02C076" } else { "#F6465D" },
                                                            "({percent:+.1}%)"
                                                        }
                                                        
                                                        // Additional Binance-style info separator
                                                        line { 
                                                            x1: "20", y1: "290", 
                                                            x2: if is_mobile() { "300" } else { "260" }, y2: "290", 
                                                            stroke: "#474D57", stroke_width: "1" 
                                                        }
                                                        
                                                        // Portfolio Analysis label
                                                        text { 
                                                            x: "20", y: "315", 
                                                            font_size: if is_mobile() { "12" } else { "10" }, 
                                                            fill: "#848E9C",
                                                            "Portfolio Analysis"
                                                        }
                                                        
                                                        // Close instruction (responsive)
                                                        text { 
                                                            x: "20", y: if is_mobile() { "335" } else { "330" }, 
                                                            font_size: if is_mobile() { "11" } else { "9" }, 
                                                            fill: "#848E9C",
                                                            if is_mobile() { "Tap outside or double-tap to close" } else { "Click outside or double-click to close" }
                                                        }
                                                        
                                                        // Risk indicator (responsive)
                                                        {
                                                            let risk_color = if payoff < -100.0 { "#F6465D" } 
                                                                             else if payoff > 100.0 { "#02C076" } 
                                                                             else { "#F0B90B" };
                                                            let risk_text = if payoff < -100.0 { "HIGH RISK" } 
                                                                           else if payoff > 100.0 { "HIGH PROFIT" } 
                                                                           else { "MODERATE" };
                                                            rsx! {
                                                                text { 
                                                                    x: "20", 
                                                                    y: if is_mobile() { "365" } else { "350" }, 
                                                                    font_size: if is_mobile() { "11" } else { "10" }, 
                                                                    font_weight: "bold",
                                                                    fill: risk_color,
                                                                    "Risk Level: {risk_text}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                
                                                // Close button (responsive positioning)
                                                circle {
                                                    cx: if is_mobile() { "295" } else { "255" }, 
                                                    cy: "25", 
                                                    r: if is_mobile() { "15" } else { "12" },
                                                    fill: "#F6465D", // Binance red
                                                    style: "cursor: pointer;",
                                                    onclick: move |_| {
                                                        legend_visible.set(false);
                                                    }
                                                }
                                                text {
                                                    x: if is_mobile() { "295" } else { "255" }, 
                                                    y: if is_mobile() { "32" } else { "30" },
                                                    text_anchor: "middle",
                                                    font_size: if is_mobile() { "18" } else { "14" },
                                                    fill: "white",
                                                    font_weight: "bold",
                                                    style: "cursor: pointer; user-select: none;",
                                                    "×"
                                                }
                                            }
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
