use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChartControlsProps {
    pub price_start: f64,
    pub price_end: f64,
    pub step_size: f64,
    pub on_price_range_change: EventHandler<(f64, f64)>,
    pub on_step_size_change: EventHandler<f64>,
    pub on_calculate: EventHandler<()>,
}

pub fn ChartControls(props: ChartControlsProps) -> Element {
    let mut price_start_input = use_signal(|| props.price_start.to_string());
    let mut price_end_input = use_signal(|| props.price_end.to_string());
    let mut step_size_input = use_signal(|| props.step_size.to_string());
    let mut error_message = use_signal(|| String::new());

    // Update inputs when props change
    use_effect(move || {
        price_start_input.set(props.price_start.to_string());
        price_end_input.set(props.price_end.to_string());
        step_size_input.set(props.step_size.to_string());
    });

    let handle_update = move |_| {
        error_message.set(String::new());
        
        let start = match price_start_input().parse::<f64>() {
            Ok(s) if s >= 0.0 => s,
            _ => {
                error_message.set("Start price must be a non-negative number".to_string());
                return;
            }
        };
        
        let end = match price_end_input().parse::<f64>() {
            Ok(e) if e > start => e,
            _ => {
                error_message.set("End price must be greater than start price".to_string());
                return;
            }
        };
        
        let step = match step_size_input().parse::<f64>() {
            Ok(s) if s > 0.0 && s <= (end - start) => s,
            _ => {
                error_message.set("Step size must be positive and reasonable".to_string());
                return;
            }
        };
        
        props.on_price_range_change.call((start, end));
        props.on_step_size_change.call(step);
        props.on_calculate.call(());
    };

    let suggested_ranges = vec![
        ("Auto (±20%)", None),
        ("0 - 100", Some((0.0, 100.0))),
        ("50 - 150", Some((50.0, 150.0))),
        ("80 - 120", Some((80.0, 120.0))),
        ("90 - 110", Some((90.0, 110.0))),
    ];

    rsx! {
        div {
            class: "chart-controls",
            h3 { "Chart Settings" }
            
            if !error_message().is_empty() {
                div {
                    class: "error-message",
                    "{error_message()}"
                }
            }
            
            div {
                class: "controls-grid",
                
                div {
                    class: "control-group",
                    h4 { "Price Range" }
                    
                    div {
                        class: "form-row",
                        div {
                            class: "form-group",
                            label { r#for: "price-start", "Start Price" }
                            input {
                                id: "price-start",
                                class: "form-control",
                                r#type: "number",
                                step: "0.01",
                                min: "0",
                                value: "{price_start_input()}",
                                oninput: move |e| price_start_input.set(e.value())
                            }
                        }
                        
                        div {
                            class: "form-group",
                            label { r#for: "price-end", "End Price" }
                            input {
                                id: "price-end",
                                class: "form-control",
                                r#type: "number",
                                step: "0.01",
                                min: "0",
                                value: "{price_end_input()}",
                                oninput: move |e| price_end_input.set(e.value())
                            }
                        }
                    }
                    
                    div {
                        class: "quick-ranges",
                        h5 { "Quick Ranges:" }
                        div {
                            class: "range-buttons",
                            for (label, range) in suggested_ranges.iter() {
                                button {
                                    class: "btn btn-outline-secondary btn-sm",
                                    onclick: move |_| {
                                        if let Some((start, end)) = range {
                                            price_start_input.set(start.to_string());
                                            price_end_input.set(end.to_string());
                                        }
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
                
                div {
                    class: "control-group",
                    h4 { "Resolution" }
                    
                    div {
                        class: "form-group",
                            label { r#for: "step-size", "Step Size" }
                            input {
                                id: "step-size",
                                class: "form-control",
                                r#type: "number",
                                step: "any",
                                min: "0.001",
                                value: "{step_size_input()}",
                                oninput: move |e| step_size_input.set(e.value())
                            }
                            small {
                                class: "form-text",
                                "Smaller values = higher resolution (more data points)"
                            }
                    }
                    
                    div {
                        class: "quick-steps",
                        h5 { "Quick Steps:" }
                        div {
                            class: "step-buttons",
                            for step in [0.01, 0.1, 0.5, 1.0, 5.0, 10.0] {
                                button {
                                    class: "btn btn-outline-secondary btn-sm",
                                    onclick: move |_| step_size_input.set(step.to_string()),
                                    "{step}"
                                }
                            }
                        }
                    }
                }
            }
            
            div {
                class: "control-actions",
                button {
                    class: "btn btn-primary btn-lg",
                    onclick: handle_update,
                    "📊 Calculate Payoff Diagram"
                }
                
                div {
                    class: "calculation-info",
                    small {
                        "Range: ${:.2} - ${:.2} | Step: {:.3} | Points: {:.0}",
                        props.price_start,
                        props.price_end,
                        props.step_size,
                        ((props.price_end - props.price_start) / props.step_size).ceil()
                    }
                }
            }
        }
    }
}
