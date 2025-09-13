use dioxus::prelude::*;
use crate::models::Position;
use crate::components::{PositionForm, PositionList, ChartControls, PayoffChart};

pub fn App() -> Element {
    let mut positions = use_signal(|| Vec::<Position>::new());
    let mut price_start = use_signal(|| 50.0);
    let mut price_end = use_signal(|| 150.0);
    let mut step_size = use_signal(|| 1.0);

    rsx! {
        div {
            class: "app-container",
            header {
                class: "app-header",
                h1 { "Payoff Diagram Web Application" }
                p { "Create and visualize payoff diagrams for financial positions" }
            }
            
            main {
                class: "app-main",
                
                div {
                    class: "app-grid",
                    
                    // Left Column: Position Management
                    div {
                        class: "left-column",
                        
                        div {
                            class: "section position-input-section",
                            PositionForm {
                                on_add_position: move |position: Position| {
                                    positions.write().push(position);
                                }
                            }
                        }
                        
                        div {
                            class: "section position-list-section",
                            PositionList {
                                positions: positions.read().clone(),
                                on_remove_position: move |index: usize| {
                                    if index < positions.read().len() {
                                        positions.write().remove(index);
                                    }
                                },
                                on_clear_all: move |_| {
                                    positions.write().clear();
                                }
                            }
                        }
                    }
                    
                    // Right Column: Chart and Controls
                    div {
                        class: "right-column",
                        
                        div {
                            class: "section chart-controls-section",
                            ChartControls {
                                price_start: price_start(),
                                price_end: price_end(),
                                step_size: step_size(),
                                on_price_range_change: move |(start, end): (f64, f64)| {
                                    price_start.set(start);
                                    price_end.set(end);
                                },
                                on_step_size_change: move |step: f64| {
                                    step_size.set(step);
                                },
                                on_calculate: move |_| {
                                    // Force re-render of chart
                                    // The chart will automatically update due to reactive signals
                                }
                            }
                        }
                        
                        div {
                            class: "section chart-section",
                            PayoffChart {
                                positions: positions.read().clone(),
                                price_start: price_start(),
                                price_end: price_end(),
                                step_size: step_size()
                            }
                        }
                    }
                }
                
                // Footer with helpful information
                div {
                    class: "app-info",
                    details {
                        class: "help-section",
                        summary { "ℹ️ How to use this application" }
                        div {
                            class: "help-content",
                            h4 { "Getting Started:" }
                            ol {
                                li { "Choose a position type (Spot, Option, or Futures)" }
                                li { "Fill in the required fields" }
                                li { "Click 'Add Position' to add it to your portfolio" }
                                li { "Adjust the price range and resolution as needed" }
                                li { "Click 'Calculate Payoff Diagram' to see the results" }
                            }
                            
                            h4 { "Position Types:" }
                            ul {
                                li { strong { "Spot: " } "Direct ownership of an asset (stocks, commodities, etc.)" }
                                li { strong { "Option: " } "Call or Put options with strike price and premium" }
                                li { strong { "Futures: " } "Futures contracts with contract size multiplier" }
                            }
                            
                            h4 { "Tips:" }
                            ul {
                                li { "Use negative quantities for short positions" }
                                li { "Start with simple positions to understand the payoff patterns" }
                                li { "Adjust the step size for smoother or more detailed charts" }
                                li { "Breakeven points show where profit/loss crosses zero" }
                            }
                        }
                    }
                }
            }
            
            footer {
                class: "app-footer",
                p { "Built with Rust + Dioxus + WebAssembly | © 2025 Payoff Diagram Web" }
            }
        }
    }
}
