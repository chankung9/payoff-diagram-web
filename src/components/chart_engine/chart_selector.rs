// Chart Selector Component - Like Binance's chart engine selector
// Allows users to switch between different chart rendering engines

use dioxus::prelude::*;
use super::{ChartEngine, ChartData, ChartConfig};
use super::svg_engine::{SvgChart, SvgChartProps};

#[derive(Props, Clone, PartialEq)]
pub struct ChartSelectorProps {
    pub data: ChartData,
    pub config: ChartConfig,
    pub selected_engine: ChartEngine,
    pub on_engine_change: EventHandler<ChartEngine>,
}

pub fn ChartSelector(props: ChartSelectorProps) -> Element {
    let available_engines = vec![
        ChartEngine::SvgNative,
        ChartEngine::CanvasRust,
        ChartEngine::ChartJs,
        ChartEngine::TradingView,
        ChartEngine::Plotters,
    ];

    rsx! {
        div {
            class: "chart-selector-container",
            
            // Engine selector dropdown (like Binance)
            div {
                class: "chart-engine-selector",
                
                div {
                    class: "selector-header",
                    h4 { "📊 Chart Engine" }
                    p { class: "selector-description", "Choose your preferred chart rendering engine" }
                }
                
                div {
                    class: "engine-options",
                    
                    for engine in available_engines.iter() {
                        div {
                            class: "engine-option",
                            class: if *engine == props.selected_engine { "selected" } else { "" },
                            class: if !engine.is_available() { "disabled" } else { "" },
                            
                            onclick: move |_| {
                                if engine.is_available() {
                                    props.on_engine_change.call(engine.clone());
                                }
                            },
                            
                            div {
                                class: "engine-header",
                                div {
                                    class: "engine-name",
                                    span { 
                                        class: "engine-title",
                                        "{engine.display_name()}"
                                    }
                                    if !engine.is_available() {
                                        span { class: "coming-soon", " (Coming Soon)" }
                                    }
                                    if *engine == props.selected_engine {
                                        span { class: "selected-indicator", " ✓" }
                                    }
                                }
                                div {
                                    class: "engine-description",
                                    "{engine.description()}"
                                }
                            }
                            
                            div {
                                class: "engine-features",
                                for feature in engine.features() {
                                    span { class: "feature-tag", "{feature}" }
                                }
                            }
                            
                            div {
                                class: "engine-stats",
                                span { 
                                    class: "bundle-impact",
                                    "Bundle size: {engine.bundle_size_impact()}"
                                }
                            }
                        }
                    }
                }
            }
            
            // Chart rendering area
            div {
                class: "chart-render-area",
                
                div {
                    class: "chart-header",
                    h3 { "Payoff Diagram" }
                    div {
                        class: "chart-info",
                        span { 
                            class: "engine-badge",
                            "Rendered by: {props.selected_engine.display_name()}"
                        }
                        span {
                            class: "data-points",
                            "{props.data.payoff_points.len()} data points"
                        }
                    }
                }
                
                // Render chart based on selected engine
                match props.selected_engine {
                    ChartEngine::SvgNative => rsx! {
                        SvgChart {
                            data: props.data.clone(),
                            config: props.config.clone()
                        }
                    },
                    ChartEngine::CanvasRust => rsx! {
                        div {
                            class: "chart-placeholder coming-soon",
                            div {
                                class: "placeholder-content",
                                h4 { "🚧 Canvas Engine" }
                                p { "Canvas-based chart engine is under development" }
                                p { "Will provide high-performance rendering with smooth animations" }
                                
                                div {
                                    class: "placeholder-features",
                                    h5 { "Planned Features:" }
                                    ul {
                                        li { "High-performance canvas rendering" }
                                        li { "Smooth animations and transitions" }
                                        li { "Advanced interaction (zoom, pan)" }
                                        li { "Real-time data updates" }
                                    }
                                }
                            }
                        }
                    },
                    ChartEngine::ChartJs => rsx! {
                        div {
                            class: "chart-placeholder coming-soon",
                            div {
                                class: "placeholder-content",
                                h4 { "📈 Chart.js Integration" }
                                p { "Professional chart library with rich features" }
                                
                                div {
                                    class: "placeholder-features",
                                    h5 { "Planned Features:" }
                                    ul {
                                        li { "Beautiful default styling" }
                                        li { "Zoom and pan capabilities" }
                                        li { "Rich tooltip system" }
                                        li { "Multiple chart types" }
                                        li { "Animation and transitions" }
                                    }
                                }
                            }
                        }
                    },
                    ChartEngine::TradingView => rsx! {
                        div {
                            class: "chart-placeholder coming-soon",
                            div {
                                class: "placeholder-content",
                                h4 { "📊 TradingView Integration" }
                                p { "Professional trading charts with advanced analysis tools" }
                                
                                div {
                                    class: "placeholder-features",
                                    h5 { "Planned Features:" }
                                    ul {
                                        li { "Professional trading interface" }
                                        li { "Technical analysis tools" }
                                        li { "Multiple timeframes" }
                                        li { "Drawing tools and indicators" }
                                        li { "Market data integration" }
                                    }
                                }
                            }
                        }
                    },
                    ChartEngine::Plotters => rsx! {
                        div {
                            class: "chart-placeholder coming-soon",
                            div {
                                class: "placeholder-content",
                                h4 { "🦀 Plotters (Rust)" }
                                p { "Pure Rust chart library with WASM backend" }
                                
                                div {
                                    class: "placeholder-features",
                                    h5 { "Planned Features:" }
                                    ul {
                                        li { "Pure Rust implementation" }
                                        li { "High precision calculations" }
                                        li { "Scientific chart types" }
                                        li { "Cross-platform compatibility" }
                                        li { "No JavaScript dependencies" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Chart engine comparison (when no positions)
            if props.data.payoff_points.is_empty() {
                div {
                    class: "engine-comparison",
                    h4 { "⚖️ Chart Engine Comparison" }
                    
                    div {
                        class: "comparison-table",
                        div { class: "comparison-header",
                            div { "Engine" }
                            div { "Bundle Size" }
                            div { "Performance" }
                            div { "Features" }
                            div { "Status" }
                        }
                        
                        for engine in available_engines.iter() {
                            div { 
                                class: "comparison-row",
                                class: if *engine == props.selected_engine { "selected" } else { "" },
                                
                                div { class: "engine-name", "{engine.display_name()}" }
                                div { class: "bundle-size", "{engine.bundle_size_impact()}" }
                                div { class: "performance", 
                                    match engine {
                                        ChartEngine::SvgNative => "⚡ Fast",
                                        ChartEngine::CanvasRust => "🚀 Very Fast", 
                                        ChartEngine::ChartJs => "⚡ Fast",
                                        ChartEngine::TradingView => "📊 Professional",
                                        ChartEngine::Plotters => "🦀 Native",
                                    }
                                }
                                div { class: "feature-count", "{engine.features().len()} features" }
                                div { class: "status", 
                                    if engine.is_available() { "✅ Available" } else { "🚧 Coming Soon" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
