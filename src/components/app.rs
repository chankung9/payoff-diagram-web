use dioxus::prelude::*;
use crate::models::Position;
use crate::components::{PositionForm, PositionList, ChartControls, PayoffChart};

pub fn App() -> Element {
    let mut positions = use_signal(|| Vec::<Position>::new());
    let mut price_start = use_signal(|| 0.0);    
    let mut price_end = use_signal(|| 300.0);   

    // Auto-adjust price range based on positions
    let auto_range = use_memo(move || {
        if positions().is_empty() {
            return (0.0, 300.0);
        }
        
        let mut min_relevant = f64::INFINITY;
        let mut max_relevant = f64::NEG_INFINITY;
        
        for position in positions() {
            match position {
                Position::Option(ref option) => {
                    // Extend range around strike price
                    let range_padding = option.strike_price * 0.5; // 50% padding
                    min_relevant = min_relevant.min(option.strike_price - range_padding);
                    max_relevant = max_relevant.max(option.strike_price + range_padding);
                }
                Position::Spot(ref spot) => {
                    let range_padding = spot.entry_price * 0.3; // 30% padding
                    min_relevant = min_relevant.min(spot.entry_price - range_padding);
                    max_relevant = max_relevant.max(spot.entry_price + range_padding);
                }
                Position::Futures(ref futures) => {
                    let range_padding = futures.entry_price * 0.3; // 30% padding
                    min_relevant = min_relevant.min(futures.entry_price - range_padding);
                    max_relevant = max_relevant.max(futures.entry_price + range_padding);
                }
            }
        }
        
        // Ensure minimum range and floor at 0
        let start = (min_relevant.max(0.0)).max(0.0);
        let end = max_relevant.max(start + 100.0);
        
        (start, end)
    });

    // Update price range when positions change
    use_effect(move || {
        let (start, end) = auto_range();
        price_start.set(start);
        price_end.set(end);
    });
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
                
                // Chart and Position Form - Side by Side on Desktop, Stacked on Mobile
                div {
                    class: "chart-and-form-section responsive-layout",
                    
                    // Chart Section - 9/12 width on desktop, full width on mobile (order: 1)
                    div {
                        class: "section chart-section-side mobile-order-1",
                        PayoffChart {
                            positions: positions.read().clone(),
                            price_start: price_start(),
                            price_end: price_end(),
                            step_size: step_size()
                        }
                    }
                    
                    // Position Form - 3/12 width on desktop, full width on mobile (order: 2)
                    div {
                        class: "section position-form-side mobile-order-2",
                        PositionForm {
                            on_add_position: move |position: Position| {
                                positions.write().push(position);
                            }
                        }
                    }
                }
                
                // Position List and Chart Controls - Grid Layout Below (order: 3 on mobile)
                div {
                    class: "app-grid-bottom mobile-order-3",
                    
                    // Left Column: Position List Only
                    div {
                        class: "left-column",
                        
                        div {
                            class: "section position-list-section",
                            PositionList {
                                positions: positions.read().clone(),
                                on_remove_position: move |index: usize| {
                                    if index < positions.read().len() {
                                        positions.write().remove(index);
                                    }
                                },
                                on_update_position: move |(index, updated_position): (usize, Position)| {
                                    if index < positions.read().len() {
                                        positions.write()[index] = updated_position;
                                    }
                                },
                                on_toggle_position: move |index: usize| {
                                    if index < positions.read().len() {
                                        positions.write()[index].toggle_active();
                                    }
                                },
                                on_clear_all: move |_| {
                                    positions.write().clear();
                                }
                            }
                        }
                    }
                    
                    // Right Column: Chart Controls Only
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
