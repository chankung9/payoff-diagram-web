use dioxus::prelude::*;
use crate::models::{Position, PayoffCalculator, PayoffPoint};

#[derive(Props, Clone, PartialEq)]
pub struct PayoffChartProps {
    pub positions: Vec<Position>,
    pub price_start: f64,
    pub price_end: f64,
    pub step_size: f64,
}

pub fn PayoffChart(props: PayoffChartProps) -> Element {
    // Calculate payoff data
    let payoff_data = if props.positions.is_empty() {
        Vec::new()
    } else {
        PayoffCalculator::generate_payoff_curve(
            &props.positions,
            props.price_start,
            props.price_end,
            props.step_size,
        )
    };

    // Find breakeven points
    let breakeven_points = if !props.positions.is_empty() {
        PayoffCalculator::find_breakeven_points(
            &props.positions,
            props.price_start,
            props.price_end,
            props.step_size,
        )
    } else {
        Vec::new()
    };

    // Calculate max profit and loss
    let max_profit = PayoffCalculator::calculate_max_profit(
        &props.positions,
        props.price_start,
        props.price_end,
        props.step_size,
    );
    
    let max_loss = PayoffCalculator::calculate_max_loss(
        &props.positions,
        props.price_start,
        props.price_end,
        props.step_size,
    );

    let element = rsx! {
        div {
            class: "payoff-chart-container",
            h3 { "Payoff Diagram" }
            
            if props.positions.is_empty() {
                div {
                    class: "chart-empty-state",
                    div {
                        class: "empty-chart-placeholder",
                        "📊"
                    }
                    p { "Add positions to see the payoff diagram" }
                    small { "The chart will show profit/loss across different underlying prices" }
                }
            } else {
                div {
                    class: "chart-content",
                    
                    // Chart statistics
                    div {
                        class: "chart-stats",
                        div {
                            class: "stat-grid",
                            div {
                                class: "stat-item profit",
                                span { class: "stat-label", "Max Profit:" }
                                span { 
                                    class: "stat-value",
                                    match max_profit {
                                        Some(profit) if profit.is_finite() => format!("${:.2}", profit),
                                        _ => "Unlimited".to_string(),
                                    }
                                }
                            }
                            div {
                                class: "stat-item loss",
                                span { class: "stat-label", "Max Loss:" }
                                span { 
                                    class: "stat-value",
                                    match max_loss {
                                        Some(loss) if loss.is_finite() => format!("${:.2}", loss),
                                        _ => "Unlimited".to_string(),
                                    }
                                }
                            }
                            div {
                                class: "stat-item breakeven",
                                span { class: "stat-label", "Breakeven Points:" }
                                span { 
                                    class: "stat-value",
                                    {if breakeven_points.is_empty() {
                                        "None".to_string()
                                    } else {
                                        breakeven_points.iter()
                                            .map(|p| format!("${:.2}", p))
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    }}
                                }
                            }
                            div {
                                class: "stat-item data-points",
                                span { class: "stat-label", "Data Points:" }
                                span { class: "stat-value", "{payoff_data.len()}" }
                            }
                        }
                    }
                    
                    // ASCII Chart placeholder (will be replaced with real chart later)
                    div {
                        class: "chart-placeholder",
                        ASCIIChart {
                            data: payoff_data.clone(),
                            price_start: props.price_start,
                            price_end: props.price_end,
                            breakeven_points: breakeven_points.clone()
                        }
                    }
                    
                    // Data table (for debugging/verification)
                    if !payoff_data.is_empty() {
                        details {
                            class: "chart-data-details",
                            summary { "📊 View Raw Data ({payoff_data.len()} points)" }
                            div {
                                class: "data-table-container",
                                table {
                                    class: "data-table",
                                    thead {
                                        tr {
                                            th { "Price" }
                                            th { "P&L" }
                                            th { "Status" }
                                        }
                                    }
                                    tbody {
                                        for (i, point) in payoff_data.iter().enumerate() {
                                            if i % 5 == 0 || point.profit_loss.abs() < 0.1 { // Show every 5th point or near breakeven
                                                tr {
                                                    class: if point.profit_loss.abs() < 0.1 { "breakeven-row" } else { "" },
                                                    td { "${:.2}", point.underlying_price }
                                                    td { 
                                                        class: if point.profit_loss >= 0.0 { "profit" } else { "loss" },
                                                        "${:.2}", point.profit_loss 
                                                    }
                                                    td {
                                                        if point.profit_loss > 0.1 { "Profit" }
                                                        else if point.profit_loss < -0.1 { "Loss" }
                                                        else { "Breakeven" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Simple ASCII chart for now (will be replaced with proper charting library)
#[derive(Props, Clone, PartialEq)]
pub struct ASCIIChartProps {
    pub data: Vec<PayoffPoint>,
    pub price_start: f64,
    pub price_end: f64,
    pub breakeven_points: Vec<f64>,
}

pub fn ASCIIChart(props: ASCIIChartProps) -> Element {
    if props.data.is_empty() {
        return rsx! {
            div { class: "ascii-chart", "No data to display" }
        };
    }

    let min_pnl = props.data.iter().map(|p| p.profit_loss).fold(f64::INFINITY, f64::min);
    let max_pnl = props.data.iter().map(|p| p.profit_loss).fold(f64::NEG_INFINITY, f64::max);
    
    let chart_height = 20;
    let chart_width = 60;
    
    // Create a simple text visualization
    let mut chart_lines = Vec::new();
    
    // Title
    chart_lines.push(format!("Payoff Diagram (${:.2} - ${:.2})", props.price_start, props.price_end));
    chart_lines.push("".to_string());
    
    // Y-axis labels and chart
    for row in 0..chart_height {
        let y_value = max_pnl - (row as f64 / (chart_height - 1) as f64) * (max_pnl - min_pnl);
        let mut line = format!("{:>8.1} ", y_value);
        
        for col in 0..chart_width {
            let price_ratio = col as f64 / (chart_width - 1) as f64;
            let price = props.price_start + price_ratio * (props.price_end - props.price_start);
            
            // Find closest data point
            let closest_point = props.data.iter()
                .min_by(|a, b| (a.underlying_price - price).abs().partial_cmp(&(b.underlying_price - price).abs()).unwrap());
            
            if let Some(point) = closest_point {
                let normalized_pnl = if max_pnl != min_pnl {
                    (point.profit_loss - min_pnl) / (max_pnl - min_pnl)
                } else {
                    0.5
                };
                
                let expected_y = 1.0 - (row as f64 / (chart_height - 1) as f64);
                
                if (normalized_pnl - expected_y).abs() < 0.05 {
                    if point.profit_loss > 0.1 {
                        line.push('▲');
                    } else if point.profit_loss < -0.1 {
                        line.push('▼');
                    } else {
                        line.push('●');
                    }
                } else if row == chart_height / 2 {
                    line.push('━'); // Zero line
                } else {
                    line.push(' ');
                }
            } else {
                line.push(' ');
            }
        }
        
        chart_lines.push(line);
    }
    
    // X-axis
    let mut x_axis = "         ".to_string();
    for i in 0..chart_width {
        if i % 10 == 0 {
            x_axis.push('┼');
        } else {
            x_axis.push('─');
        }
    }
    chart_lines.push(x_axis);
    
    // X-axis labels
    let mut x_labels = "         ".to_string();
    for i in 0..chart_width {
        if i % 20 == 0 {
            let price = props.price_start + (i as f64 / (chart_width - 1) as f64) * (props.price_end - props.price_start);
            x_labels.push_str(&format!("{:.0}", price));
            // Pad to next position
            for _ in 0..(20 - format!("{:.0}", price).len()) {
                if i + 1 < chart_width { x_labels.push(' '); }
            }
        }
    }
    chart_lines.push(x_labels);
    
    // Legend
    chart_lines.push("".to_string());
    chart_lines.push("Legend: ▲ Profit | ▼ Loss | ● Breakeven | ━ Zero Line".to_string());
    
    if !props.breakeven_points.is_empty() {
        chart_lines.push(format!("Breakeven: {}", 
            props.breakeven_points.iter()
                .map(|p| format!("${:.2}", p))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    rsx! {
        div {
            class: "ascii-chart",
            pre {
                class: "chart-display",
                code {
                    for line in chart_lines {
                        "{line}\n"
                    }
                }
            }
        }
    }
}
