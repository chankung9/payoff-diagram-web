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
    
    // Interactive legend dragging state
    let mut legend_position = use_signal(|| (500.0, 50.0)); // Default position (moved left for larger legend)
    let mut is_dragging = use_signal(|| false);
    let mut drag_offset = use_signal(|| (0.0, 0.0));
    
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
                                                                                r: "5",
                                                                                fill: if point.payoff >= 0.0 { "#28a745" } else { "#dc3545" },
                                                                                stroke: "#ffffff",
                                                                                stroke_width: "2",
                                                                                opacity: "0.8",
                                                                                class: "chart-point",
                                                                                style: "cursor: pointer; transition: all 0.2s ease;",
                                                                                
                                                                                // Enhanced hover effects with legend update
                                                                                onmouseenter: move |_| {
                                                                                    let data = (price, payoff, percent_change);
                                                                                    hover_data.set(Some(data));
                                                                                    last_hover_data.set(data);
                                                                                },
                                                                                onmouseleave: move |_| {
                                                                                    hover_data.set(None);
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
                                        
                                        // Interactive legend with hover data display (draggable)
                                        g {
                                            transform: "translate({legend_position().0}, {legend_position().1})",
                                            class: "draggable-legend",
                                            style: "cursor: move;",
                                            
                                            // Legend background (expanded to show data)
                                            rect {
                                                x: "0", y: "0", width: "180", height: "120",
                                                fill: "rgba(255, 255, 255, 0.95)",
                                                stroke: "#dee2e6",
                                                stroke_width: "1",
                                                rx: "4",
                                                style: "filter: drop-shadow(2px 2px 4px rgba(0,0,0,0.1));",
                                                
                                                // Drag events
                                                onmousedown: move |evt| {
                                                    is_dragging.set(true);
                                                    let rect = evt.client_coordinates();
                                                    let current_pos = legend_position();
                                                    drag_offset.set((rect.x as f64 - current_pos.0, rect.y as f64 - current_pos.1));
                                                }
                                            }
                                            
                                            // Legend title with drag handle indicator
                                            text { 
                                                x: "90", y: "12", 
                                                text_anchor: "middle",
                                                font_size: "10", 
                                                font_weight: "bold",
                                                fill: "#6c757d",
                                                "📊 Interactive Legend (Drag Me)"
                                            }
                                            
                                            // Chart legend items
                                            line { x1: "10", y1: "25", x2: "30", y2: "25", stroke: "#007bff", stroke_width: "3" }
                                            text { x: "35", y: "29", font_size: "9", fill: "#495057", "Payoff Curve" }
                                            
                                            line { x1: "10", y1: "38", x2: "30", y2: "38", stroke: "#ffc107", stroke_width: "3", stroke_dasharray: "4,2" }
                                            text { x: "35", y: "42", font_size: "9", fill: "#495057", "Break Even" }
                                            
                                            circle { cx: "20", cy: "51", r: "3", fill: "#28a745" }
                                            text { x: "35", y: "55", font_size: "9", fill: "#495057", "Profit Zone" }
                                            
                                            circle { cx: "20", cy: "64", r: "3", fill: "#dc3545" }
                                            text { x: "35", y: "68", font_size: "9", fill: "#495057", "Loss Zone" }
                                            
                                            // Separator line
                                            line { x1: "10", y1: "75", x2: "170", y2: "75", stroke: "#dee2e6", stroke_width: "1" }
                                            
                                            // Hover data display section
                                            {
                                                let display_data = if let Some(current_data) = hover_data() {
                                                    current_data
                                                } else {
                                                    last_hover_data()
                                                };
                                                
                                                let (price, payoff, percent) = display_data;
                                                let status = if hover_data().is_some() { "LIVE" } else { "LAST" };
                                                
                                                rsx! {
                                                    // Data status indicator
                                                    text { 
                                                        x: "90", y: "88", 
                                                        text_anchor: "middle",
                                                        font_size: "8", 
                                                        font_weight: "bold",
                                                        fill: if hover_data().is_some() { "#28a745" } else { "#6c757d" },
                                                        "{status} DATA"
                                                    }
                                                    
                                                    // Price display
                                                    text { 
                                                        x: "10", y: "100", 
                                                        font_size: "10", 
                                                        font_weight: "bold",
                                                        fill: "#212529",
                                                        "Price: ${price:.2}"
                                                    }
                                                    
                                                    // P&L display
                                                    text { 
                                                        x: "10", y: "112", 
                                                        font_size: "10", 
                                                        font_weight: "bold",
                                                        fill: if payoff >= 0.0 { "#28a745" } else { "#dc3545" },
                                                        "P&L: ${payoff:.2} ({percent:+.1}%)"
                                                    }
                                                }
                                            }
                                            
                                            // Mini close button to minimize legend
                                            circle {
                                                cx: "170", cy: "10", r: "6",
                                                fill: "#f8f9fa",
                                                stroke: "#dee2e6",
                                                style: "cursor: pointer;",
                                                onclick: move |_| {
                                                    // Move legend to corner when minimized
                                                    legend_position.set((650.0, 50.0));
                                                }
                                            }
                                            text {
                                                x: "170", y: "13",
                                                text_anchor: "middle",
                                                font_size: "8",
                                                fill: "#6c757d",
                                                style: "cursor: pointer; user-select: none;",
                                                "×"
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
