use dioxus::prelude::*;
use crate::models::{Position, PayoffPoint};
use crate::models::payoff::PayoffCalculator;

#[derive(Props, Clone, PartialEq)]
pub struct PayoffChartProps {
    pub positions: Vec<Position>,
    pub price_start: f64,
    pub price_end: f64,
    pub step_size: f64,
}

pub fn PayoffChart(props: PayoffChartProps) -> Element {
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
                    p { "Chart will be displayed here" }
                    p { "Positions: {props.positions.len()}" }
                    p { "Price Range: ${props.price_start:.2} - ${props.price_end:.2}" }
                }
            }
        }
    }
}
