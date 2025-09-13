// SVG Chart Engine - Pure SVG implementation
// First implementation of our modular chart system

use dioxus::prelude::*;
use super::{ChartRenderer, ChartData, ChartConfig};

pub struct SvgChartEngine;

impl ChartRenderer for SvgChartEngine {
    fn render_chart(&self, data: &ChartData, config: &ChartConfig) -> String {
        // This will be called by the component to generate SVG
        // For now, return placeholder - we'll implement the actual SVG generation
        format!("SVG chart with {} points", data.payoff_points.len())
    }

    fn supports_interaction(&self) -> bool {
        true
    }

    fn supports_animation(&self) -> bool {
        true // CSS animations
    }

    fn bundle_size_impact(&self) -> &str {
        "None"
    }
}

impl SvgChartEngine {
    pub fn new() -> Self {
        Self
    }

    // Calculate X position in SVG coordinates
    pub fn calculate_x_position(&self, price: f64, data: &ChartData, config: &ChartConfig) -> f64 {
        let chart_width = (config.width - config.margin.left - config.margin.right) as f64;
        let price_range = data.price_range.1 - data.price_range.0;
        
        if price_range <= 0.0 {
            return config.margin.left as f64;
        }
        
        config.margin.left as f64 + ((price - data.price_range.0) / price_range) * chart_width
    }

    // Calculate Y position in SVG coordinates  
    pub fn calculate_y_position(&self, payoff: f64, data: &ChartData, config: &ChartConfig) -> f64 {
        let chart_height = (config.height - config.margin.top - config.margin.bottom) as f64;
        
        // Find min/max payoff for scaling
        let min_payoff = data.payoff_points.iter()
            .map(|p| p.payoff)
            .fold(f64::INFINITY, f64::min)
            .min(0.0); // Ensure zero is visible
            
        let max_payoff = data.payoff_points.iter()
            .map(|p| p.payoff)
            .fold(f64::NEG_INFINITY, f64::max)
            .max(0.0); // Ensure zero is visible
            
        let payoff_range = max_payoff - min_payoff;
        
        if payoff_range <= 0.0 {
            return config.margin.top as f64 + chart_height / 2.0;
        }
        
        // Invert Y coordinate (SVG Y grows downward, but we want profit to go up)
        config.margin.top as f64 + ((max_payoff - payoff) / payoff_range) * chart_height
    }

    // Generate SVG path points for polyline
    pub fn generate_svg_points(&self, data: &ChartData, config: &ChartConfig) -> String {
        data.payoff_points
            .iter()
            .map(|point| {
                format!(
                    "{:.1},{:.1}",
                    self.calculate_x_position(point.price, data, config),
                    self.calculate_y_position(point.payoff, data, config)
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    // Generate tick positions for X axis
    pub fn generate_x_ticks(&self, data: &ChartData, config: &ChartConfig) -> Vec<(f64, f64)> {
        let tick_count = config.axes.tick_count_x as usize;
        let price_range = data.price_range.1 - data.price_range.0;
        let step = price_range / (tick_count - 1) as f64;
        
        (0..tick_count)
            .map(|i| {
                let price = data.price_range.0 + (i as f64 * step);
                let x = self.calculate_x_position(price, data, config);
                (x, price)
            })
            .collect()
    }

    // Generate tick positions for Y axis
    pub fn generate_y_ticks(&self, data: &ChartData, config: &ChartConfig) -> Vec<(f64, f64)> {
        let tick_count = config.axes.tick_count_y as usize;
        
        let min_payoff = data.payoff_points.iter()
            .map(|p| p.payoff)
            .fold(f64::INFINITY, f64::min)
            .min(0.0);
            
        let max_payoff = data.payoff_points.iter()
            .map(|p| p.payoff)
            .fold(f64::NEG_INFINITY, f64::max)
            .max(0.0);
            
        let payoff_range = max_payoff - min_payoff;
        let step = payoff_range / (tick_count - 1) as f64;
        
        (0..tick_count)
            .map(|i| {
                let payoff = min_payoff + (i as f64 * step);
                let y = self.calculate_y_position(payoff, data, config);
                (y, payoff)
            })
            .collect()
    }
}

// Dioxus component that uses SVG engine
#[derive(Props, Clone, PartialEq)]
pub struct SvgChartProps {
    pub data: ChartData,
    pub config: ChartConfig,
}

pub fn SvgChart(props: SvgChartProps) -> Element {
    let engine = SvgChartEngine::new();
    
    rsx! {
        div {
            class: "svg-chart-container",
            
            svg {
                class: "payoff-chart-svg",
                width: "{props.config.width}",
                height: "{props.config.height}",
                viewBox: "0 0 {props.config.width} {props.config.height}",
                
                // Chart background
                rect {
                    x: "0",
                    y: "0",
                    width: "{props.config.width}",
                    height: "{props.config.height}",
                    fill: "{props.config.colors.background}",
                    stroke: "{props.config.colors.grid}",
                    stroke_width: "1"
                }
                
                // Chart area group
                g {
                    transform: "translate({props.config.margin.left}, {props.config.margin.top})",
                    
                    // Grid lines (if enabled)
                    if props.config.grid.show_major {
                        g {
                            class: "grid-lines",
                            
                            // Vertical grid lines
                            for (x, _price) in engine.generate_x_ticks(&props.data, &props.config) {
                                line {
                                    x1: "{x - props.config.margin.left as f64}",
                                    y1: "0",
                                    x2: "{x - props.config.margin.left as f64}",
                                    y2: "{props.config.height - props.config.margin.top - props.config.margin.bottom}",
                                    stroke: "{props.config.grid.major_color}",
                                    stroke_width: "1",
                                    opacity: "0.5"
                                }
                            }
                            
                            // Horizontal grid lines
                            for (y, _payoff) in engine.generate_y_ticks(&props.data, &props.config) {
                                line {
                                    x1: "0",
                                    y1: "{y - props.config.margin.top as f64}",
                                    x2: "{props.config.width - props.config.margin.left - props.config.margin.right}",
                                    y2: "{y - props.config.margin.top as f64}",
                                    stroke: "{props.config.grid.major_color}",
                                    stroke_width: "1",
                                    opacity: "0.5"
                                }
                            }
                        }
                    }
                    
                    // Zero line (highlighted)
                    line {
                        x1: "0",
                        y1: "{engine.calculate_y_position(0.0, &props.data, &props.config) - props.config.margin.top as f64}",
                        x2: "{props.config.width - props.config.margin.left - props.config.margin.right}",
                        y2: "{engine.calculate_y_position(0.0, &props.data, &props.config) - props.config.margin.top as f64}",
                        stroke: "{props.config.colors.zero_line}",
                        stroke_width: "2",
                        stroke_dasharray: "5,5"
                    }
                    
                    // Main payoff curve
                    if !props.data.payoff_points.is_empty() {
                        polyline {
                            points: "{engine.generate_svg_points(&props.data, &props.config)}",
                            fill: "none",
                            stroke: "#007bff",
                            stroke_width: "3",
                            stroke_linejoin: "round"
                        }
                        
                        // Data points
                        for (i, point) in props.data.payoff_points.iter().enumerate() {
                            if i % 5 == 0 { // Show every 5th point
                                circle {
                                    cx: "{engine.calculate_x_position(point.price, &props.data, &props.config) - props.config.margin.left as f64}",
                                    cy: "{engine.calculate_y_position(point.payoff, &props.data, &props.config) - props.config.margin.top as f64}",
                                    r: "4",
                                    fill: if point.payoff >= 0.0 { "{props.config.colors.profit_line}" } else { "{props.config.colors.loss_line}" },
                                    stroke: "#ffffff",
                                    stroke_width: "2",
                                    
                                    // Tooltip
                                    title { "Price: ${point.price:.2}, P&L: ${point.payoff:.2}" }
                                }
                            }
                        }
                    }
                    
                    // Breakeven points
                    for be_point in props.data.breakeven_points.iter() {
                        line {
                            x1: "{engine.calculate_x_position(*be_point, &props.data, &props.config) - props.config.margin.left as f64}",
                            y1: "0",
                            x2: "{engine.calculate_x_position(*be_point, &props.data, &props.config) - props.config.margin.left as f64}",
                            y2: "{props.config.height - props.config.margin.top - props.config.margin.bottom}",
                            stroke: "{props.config.colors.breakeven}",
                            stroke_width: "2",
                            stroke_dasharray: "3,3"
                        }
                        text {
                            x: "{engine.calculate_x_position(*be_point, &props.data, &props.config) - props.config.margin.left as f64}",
                            y: "{engine.calculate_y_position(0.0, &props.data, &props.config) - props.config.margin.top as f64 - 10.0}",
                            text_anchor: "middle",
                            font_size: "12",
                            fill: "{props.config.colors.breakeven}",
                            "BE: ${be_point:.0}"
                        }
                    }
                }
                
                // Axes
                if props.config.axes.show_labels {
                    g {
                        class: "axes",
                        
                        // X-axis
                        line {
                            x1: "{props.config.margin.left}",
                            y1: "{props.config.height - props.config.margin.bottom}",
                            x2: "{props.config.width - props.config.margin.right}",
                            y2: "{props.config.height - props.config.margin.bottom}",
                            stroke: "{props.config.colors.axis}",
                            stroke_width: "2"
                        }
                        
                        // Y-axis
                        line {
                            x1: "{props.config.margin.left}",
                            y1: "{props.config.margin.top}",
                            x2: "{props.config.margin.left}",
                            y2: "{props.config.height - props.config.margin.bottom}",
                            stroke: "{props.config.colors.axis}",
                            stroke_width: "2"
                        }
                        
                        // X-axis labels
                        for (x, price) in engine.generate_x_ticks(&props.data, &props.config) {
                            text {
                                x: "{x}",
                                y: "{props.config.height - props.config.margin.bottom + 20}",
                                text_anchor: "middle",
                                font_size: "12",
                                fill: "{props.config.colors.axis}",
                                "${price:.0}"
                            }
                        }
                        
                        // Y-axis labels  
                        for (y, payoff) in engine.generate_y_ticks(&props.data, &props.config) {
                            text {
                                x: "{props.config.margin.left - 10}",
                                y: "{y + 4}",
                                text_anchor: "end",
                                font_size: "12",
                                fill: "{props.config.colors.axis}",
                                "${payoff:.0}"
                            }
                        }
                        
                        // Axis titles
                        text {
                            x: "{props.config.width / 2}",
                            y: "{props.config.height - 10}",
                            text_anchor: "middle",
                            font_size: "14",
                            fill: "{props.config.colors.axis}",
                            "Stock Price ($)"
                        }
                        
                        text {
                            x: "20",
                            y: "{props.config.height / 2}",
                            text_anchor: "middle",
                            font_size: "14",
                            fill: "{props.config.colors.axis}",
                            transform: "rotate(-90, 20, {props.config.height / 2})",
                            "Profit / Loss ($)"
                        }
                    }
                }
                
                // Chart title
                text {
                    x: "{props.config.width / 2}",
                    y: "25",
                    text_anchor: "middle",
                    font_size: "16",
                    font_weight: "bold",
                    fill: "#212529",
                    "Options Portfolio Payoff Diagram"
                }
            }
        }
    }
}
