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

    // Update inputs when props change
    use_effect(move || {
        price_start_input.set(props.price_start.to_string());
        price_end_input.set(props.price_end.to_string());
        step_size_input.set(props.step_size.to_string());
    });

    // Handle realtime updates
    let mut handle_price_start_change = move |value: String| {
        price_start_input.set(value.clone());
        if let Ok(start) = value.parse::<f64>() {
            let end = props.price_end;
            if start >= 0.0 && end > start {
                props.on_price_range_change.call((start, end));
                props.on_calculate.call(());
            }
        }
    };

    let mut handle_price_end_change = move |value: String| {
        price_end_input.set(value.clone());
        if let Ok(end) = value.parse::<f64>() {
            let start = props.price_start;
            if start >= 0.0 && end > start {
                props.on_price_range_change.call((start, end));
                props.on_calculate.call(());
            }
        }
    };

    let mut handle_step_size_change = move |value: String| {
        step_size_input.set(value.clone());
        if let Ok(step) = value.parse::<f64>() {
            if step > 0.0 {
                props.on_step_size_change.call(step);
                props.on_calculate.call(());
            }
        }
    };

    rsx! {
        div {
            class: "chart-controls",
            h3 { "Chart Settings" }
            
            div {
                class: "controls-grid",
                
                div {
                    class: "control-section",
                    h4 { "Price Range" }
                    
                    div {
                        class: "range-inputs",
                        div {
                            class: "form-group",
                            label { "Start Price" }
                            input {
                                class: "form-control",
                                r#type: "number",
                                step: "0.01",
                                value: "{price_start_input()}",
                                oninput: move |e| handle_price_start_change(e.value())
                            }
                        }
                        
                        div {
                            class: "form-group",
                            label { "End Price" }
                            input {
                                class: "form-control",
                                r#type: "number", 
                                step: "0.01",
                                value: "{price_end_input()}",
                                oninput: move |e| handle_price_end_change(e.value())
                            }
                        }
                    }
                }
                
                div {
                    class: "control-section",
                    h4 { "Resolution" }
                    
                    div {
                        class: "form-group",
                        label { "Step Size" }
                        input {
                            class: "form-control",
                            r#type: "number",
                            step: "0.01",
                            min: "0.01",
                            value: "{step_size_input()}",
                            oninput: move |e| handle_step_size_change(e.value())
                        }
                    }
                }
                
                div {
                    class: "control-section",
                    
                    div {
                        class: "chart-info",
                        small {
                            "Range: ${props.price_start:.2} - ${props.price_end:.2} | Step: {props.step_size:.2}"
                        }
                    }
                }
            }
        }
    }
}
