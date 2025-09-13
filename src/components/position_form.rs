use dioxus::prelude::*;
use crate::models::{Position, PositionType, SpotPosition, OptionPosition, FuturesPosition, OptionType};

#[derive(Props, Clone, PartialEq)]
pub struct PositionFormProps {
    pub on_add_position: EventHandler<Position>,
}

pub fn PositionForm(props: PositionFormProps) -> Element {
    let mut position_type = use_signal(|| PositionType::Spot);
    let mut quantity = use_signal(|| String::new());
    let mut entry_price = use_signal(|| String::new());
    let mut strike_price = use_signal(|| String::new());
    let mut premium = use_signal(|| String::new());
    let mut contract_size = use_signal(|| String::new());
    let mut option_type = use_signal(|| OptionType::Call);
    let mut description = use_signal(|| String::new());
    let mut error_message = use_signal(|| String::new());

    let mut reset_form = move || {
        quantity.set(String::new());
        entry_price.set(String::new());
        strike_price.set(String::new());
        premium.set(String::new());
        contract_size.set(String::new());
        description.set(String::new());
        error_message.set(String::new());
    };

    let mut handle_submit = move |_| {
        error_message.set(String::new());
        
        // Parse and validate inputs
        let qty = match quantity().parse::<f64>() {
            Ok(q) if q != 0.0 => q,
            _ => {
                error_message.set("Quantity must be a non-zero number".to_string());
                return;
            }
        };

        let position = match position_type() {
            PositionType::Spot => {
                let price = match entry_price().parse::<f64>() {
                    Ok(p) if p > 0.0 => p,
                    _ => {
                        error_message.set("Entry price must be a positive number".to_string());
                        return;
                    }
                };
                
                Position::Spot(SpotPosition::new(qty, price, Some(description())))
            }
            PositionType::Option => {
                let strike = match strike_price().parse::<f64>() {
                    Ok(s) if s > 0.0 => s,
                    _ => {
                        error_message.set("Strike price must be a positive number".to_string());
                        return;
                    }
                };
                
                let prem = match premium().parse::<f64>() {
                    Ok(p) if p >= 0.0 => p,
                    _ => {
                        error_message.set("Premium must be a non-negative number".to_string());
                        return;
                    }
                };
                
                Position::Option(OptionPosition::new(
                    option_type(),
                    qty,
                    strike,
                    prem,
                    Some(description())
                ))
            }
            PositionType::Futures => {
                let price = match entry_price().parse::<f64>() {
                    Ok(p) => p,
                    _ => {
                        error_message.set("Entry price must be a number".to_string());
                        return;
                    }
                };
                
                let size = match contract_size().parse::<f64>() {
                    Ok(s) if s > 0.0 => s,
                    _ => {
                        error_message.set("Contract size must be a positive number".to_string());
                        return;
                    }
                };
                
                Position::Futures(FuturesPosition::new(qty, price, size, Some(description())))
            }
        };

        props.on_add_position.call(position);
        reset_form();
    };

    rsx! {
        div {
            class: "position-form",
            h3 { "Add Position" }
            
            if !error_message().is_empty() {
                div {
                    class: "error-message",
                    "{error_message()}"
                }
            }
            
            form {
                onsubmit: move |e| {
                    handle_submit(e);
                },
                
                div {
                    class: "form-row",
                    
                    div {
                        class: "form-group",
                        label { r#for: "position-type", "Position Type" }
                        select {
                            id: "position-type",
                            class: "form-control",
                            value: "{position_type():?}",
                            onchange: move |e| {
                                match e.value().as_str() {
                                    "Spot" => position_type.set(PositionType::Spot),
                                    "Option" => position_type.set(PositionType::Option),
                                    "Futures" => position_type.set(PositionType::Futures),
                                    _ => {}
                                }
                            },
                            option { value: "Spot", "Spot" }
                            option { value: "Option", "Option" }
                            option { value: "Futures", "Futures" }
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { r#for: "quantity", "Quantity" }
                        input {
                            id: "quantity",
                            class: "form-control",
                            r#type: "number",
                            step: "any",
                            placeholder: "e.g., 100 (positive = long, negative = short)",
                            value: "{quantity()}",
                            oninput: move |e| quantity.set(e.value())
                        }
                    }
                }
                
                // Conditional fields based on position type
                match position_type() {
                    PositionType::Spot => rsx! {
                        div {
                            class: "form-row",
                            div {
                                class: "form-group",
                                label { r#for: "entry-price", "Entry Price" }
                                input {
                                    id: "entry-price",
                                    class: "form-control",
                                    r#type: "number",
                                    step: "0.01",
                                    placeholder: "e.g., 100.50",
                                    value: "{entry_price()}",
                                    oninput: move |e| entry_price.set(e.value())
                                }
                            }
                        }
                    },
                    PositionType::Option => rsx! {
                        div {
                            class: "form-row",
                            div {
                                class: "form-group",
                                label { r#for: "option-type", "Option Type" }
                                select {
                                    id: "option-type",
                                    class: "form-control",
                                    value: "{option_type():?}",
                                    onchange: move |e| {
                                        match e.value().as_str() {
                                            "Call" => option_type.set(OptionType::Call),
                                            "Put" => option_type.set(OptionType::Put),
                                            _ => {}
                                        }
                                    },
                                    option { value: "Call", "Call" }
                                    option { value: "Put", "Put" }
                                }
                            }
                            
                            div {
                                class: "form-group",
                                label { r#for: "strike-price", "Strike Price" }
                                input {
                                    id: "strike-price",
                                    class: "form-control",
                                    r#type: "number",
                                    step: "0.01",
                                    placeholder: "e.g., 105.00",
                                    value: "{strike_price()}",
                                    oninput: move |e| strike_price.set(e.value())
                                }
                            }
                            
                            div {
                                class: "form-group",
                                label { r#for: "premium", "Premium" }
                                input {
                                    id: "premium",
                                    class: "form-control",
                                    r#type: "number",
                                    step: "0.01",
                                    placeholder: "e.g., 5.25",
                                    value: "{premium()}",
                                    oninput: move |e| premium.set(e.value())
                                }
                            }
                        }
                    },
                    PositionType::Futures => rsx! {
                        div {
                            class: "form-row",
                            div {
                                class: "form-group",
                                label { r#for: "entry-price-futures", "Entry Price" }
                                input {
                                    id: "entry-price-futures",
                                    class: "form-control",
                                    r#type: "number",
                                    step: "0.01",
                                    placeholder: "e.g., 98.75",
                                    value: "{entry_price()}",
                                    oninput: move |e| entry_price.set(e.value())
                                }
                            }
                            
                            div {
                                class: "form-group",
                                label { r#for: "contract-size", "Contract Size" }
                                input {
                                    id: "contract-size",
                                    class: "form-control",
                                    r#type: "number",
                                    step: "any",
                                    placeholder: "e.g., 1000",
                                    value: "{contract_size()}",
                                    oninput: move |e| contract_size.set(e.value())
                                }
                            }
                        }
                    }
                }
                
                div {
                    class: "form-group",
                    label { r#for: "description", "Description (Optional)" }
                    input {
                        id: "description",
                        class: "form-control",
                        r#type: "text",
                        placeholder: "e.g., Long AAPL stock",
                        value: "{description()}",
                        oninput: move |e| description.set(e.value())
                    }
                }
                
                div {
                    class: "form-actions",
                    button {
                        r#type: "submit",
                        class: "btn btn-primary",
                        "Add Position"
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-secondary",
                        onclick: move |_| reset_form(),
                        "Clear"
                    }
                }
            }
        }
    }
}
