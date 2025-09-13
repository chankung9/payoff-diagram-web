use dioxus::prelude::*;
use crate::models::Position;
use crate::engine::PayoffEngine;

#[derive(Props, Clone, PartialEq)]
pub struct PayoffChartProps {
    pub positions: Vec<Position>,
    pub price_start: f64,
    pub price_end: f64,
    pub step_size: f64,
}

pub fn PayoffChart(props: PayoffChartProps) -> Element {
    // Calculate payoff data directly in component body
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

    rsx! {
        div {
            class: "payoff-chart-container",
            h3 { "Payoff Diagram" }
            
            if props.positions.is_empty() {
                div {
                    class: "chart-empty-state",
                    p { "Add positions to see the payoff diagram" }
                }
            } else {
                div {
                    class: "chart-content",
                    
                    // Chart statistics
                    div {
                        class: "chart-stats",
                        div {
                            class: "stat-item",
                            strong { "Data Points: " }
                            span { "{payoff_data.len()}" }
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
                        if let Some(max_p) = max_profit {
                            div {
                                class: "stat-item profit",
                                strong { "Max Profit: " }
                                span { "${max_p:.2}" }
                            }
                        }
                        if let Some(max_l) = max_loss {
                            div {
                                class: "stat-item loss",
                                strong { "Max Loss: " }
                                span { "${max_l:.2}" }
                            }
                        }
                        if !breakeven_points.is_empty() {
                            div {
                                class: "stat-item",
                                strong { "Breakeven Points: " }
                                span {
                                    for (i, point) in breakeven_points.iter().enumerate() {
                                        if i > 0 { ", " }
                                        "${point:.2}"
                                    }
                                }
                            }
                        }
                    }
                    
                    // Simple ASCII chart (placeholder for now)
                    div {
                        class: "chart-placeholder",
                        h4 { "Chart Preview (Data Generated)" }
                        div {
                            class: "ascii-chart",
                            for point in payoff_data.iter().take(10) {
                                div {
                                    class: "chart-row",
                                    span { class: "price", "${point.price:.2}" }
                                    span { class: "separator", " → " }
                                    span { 
                                        class: if point.payoff >= 0.0 { "payoff profit" } else { "payoff loss" },
                                        "${point.payoff:.2}"
                                    }
                                }
                            }
                            if payoff_data.len() > 10 {
                                div {
                                    class: "chart-row ellipsis",
                                    "... and {payoff_data.len() - 10} more data points"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
