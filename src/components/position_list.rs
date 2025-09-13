use dioxus::prelude::*;
use crate::models::Position;

#[derive(Props, Clone, PartialEq)]
pub struct PositionListProps {
    pub positions: Vec<Position>,
    pub on_remove_position: EventHandler<usize>,
    pub on_update_position: EventHandler<(usize, Position)>, // New: for updating positions
    pub on_clear_all: EventHandler<()>,
}

pub fn PositionList(props: PositionListProps) -> Element {
    rsx! {
        div {
            class: "position-list",
            div {
                class: "position-list-header",
                h3 { "Current Positions ({props.positions.len()})" }
                if !props.positions.is_empty() {
                    button {
                        class: "btn btn-warning btn-sm",
                        onclick: move |_| props.on_clear_all.call(()),
                        "Clear All"
                    }
                }
            }
            
            if props.positions.is_empty() {
                div {
                    class: "empty-state",
                    p { "No positions added yet. Use the form above to add your first position." }
                }
            } else {
                div {
                    class: "position-cards",
                    for (index, position) in props.positions.iter().enumerate() {
                        PositionCard {
                            key: "{index}",
                            position: position.clone(),
                            index,
                            on_remove: move |idx| props.on_remove_position.call(idx),
                            on_update: move |(idx, pos): (usize, Position)| props.on_update_position.call((idx, pos))
                        }
                    }
                }
                
                div {
                    class: "position-summary",
                    h4 { "Portfolio Summary" }
                    div {
                        class: "summary-stats",
                        div {
                            class: "stat-item",
                            span { class: "stat-label", "Total Positions:" }
                            span { class: "stat-value", "{props.positions.len()}" }
                        }
                        div {
                            class: "stat-item",
                            span { class: "stat-label", "Spot Positions:" }
                            span { 
                                class: "stat-value", 
                                "{props.positions.iter().filter(|p| matches!(p, Position::Spot(_))).count()}" 
                            }
                        }
                        div {
                            class: "stat-item",
                            span { class: "stat-label", "Option Positions:" }
                            span { 
                                class: "stat-value", 
                                "{props.positions.iter().filter(|p| matches!(p, Position::Option(_))).count()}" 
                            }
                        }
                        div {
                            class: "stat-item",
                            span { class: "stat-label", "Futures Positions:" }
                            span { 
                                class: "stat-value", 
                                "{props.positions.iter().filter(|p| matches!(p, Position::Futures(_))).count()}" 
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PositionCardProps {
    pub position: Position,
    pub index: usize,
    pub on_remove: EventHandler<usize>,
    pub on_update: EventHandler<(usize, Position)>, // New: for updating positions
}

pub fn PositionCard(props: PositionCardProps) -> Element {
    // Edit mode state
    let mut is_editing = use_signal(|| false);
    
    // Editable fields state
    let mut edit_quantity = use_signal(|| props.position.quantity());
    let mut edit_entry_price = use_signal(|| match &props.position {
        Position::Spot(spot) => spot.entry_price,
        Position::Option(option) => option.premium,
        Position::Futures(futures) => futures.entry_price,
    });
    let mut edit_strike_price = use_signal(|| match &props.position {
        Position::Option(option) => option.strike_price,
        _ => 0.0,
    });
    let mut edit_description = use_signal(|| props.position.description().to_string());

    // Clone position to avoid ownership issues
    let position_clone = props.position.clone();
    
    // Reset edit fields when entering edit mode
    let mut enter_edit_mode = {
        let pos = position_clone.clone();
        move || {
            edit_quantity.set(pos.quantity());
            edit_entry_price.set(match &pos {
                Position::Spot(spot) => spot.entry_price,
                Position::Option(option) => option.premium,
                Position::Futures(futures) => futures.entry_price,
            });
            edit_strike_price.set(match &pos {
                Position::Option(option) => option.strike_price,
                _ => 0.0,
            });
            edit_description.set(pos.description().to_string());
            is_editing.set(true);
        }
    };

    // Save changes
    let mut save_changes = {
        let pos = position_clone.clone();
        move || {
            let updated_position = match &pos {
                Position::Spot(_) => {
                    use crate::models::SpotPosition;
                    Position::Spot(SpotPosition {
                        quantity: edit_quantity(),
                        entry_price: edit_entry_price(),
                        description: edit_description(),
                    })
                }
                Position::Option(option) => {
                    use crate::models::OptionPosition;
                    Position::Option(OptionPosition {
                        option_type: option.option_type,
                        strike_price: edit_strike_price(),
                        quantity: edit_quantity(),
                        premium: edit_entry_price(),
                        expiry_price: option.expiry_price, // Keep existing expiry_price
                        description: edit_description(),
                    })
                }
                Position::Futures(_) => {
                    use crate::models::FuturesPosition;
                    Position::Futures(FuturesPosition {
                        quantity: edit_quantity(),
                        entry_price: edit_entry_price(),
                        contract_size: 1.0, // Default contract size - should make this editable too
                        description: edit_description(),
                    })
                }
            };
            
            props.on_update.call((props.index, updated_position));
            is_editing.set(false);
        }
    };

    let mut cancel_edit = move || {
        is_editing.set(false);
    };
    let (position_type_class, position_info) = match &position_clone {
        Position::Spot(spot) => {
            let direction = if spot.quantity >= 0.0 { "Long" } else { "Short" };
            let info = format!(
                "{} {} units @ ${:.2}",
                direction,
                spot.quantity.abs(),
                spot.entry_price
            );
            ("spot-position", info)
        }
        Position::Option(option) => {
            let direction = if option.quantity >= 0.0 { "Long" } else { "Short" };
            let option_type = match option.option_type {
                crate::models::OptionType::Call => "Call",
                crate::models::OptionType::Put => "Put",
            };
            let info = format!(
                "{} {} {} @ Strike ${:.2}, Premium ${:.2}",
                direction,
                option.quantity.abs(),
                option_type,
                option.strike_price,
                option.premium
            );
            ("option-position", info)
        }
        Position::Futures(futures) => {
            let direction = if futures.quantity >= 0.0 { "Long" } else { "Short" };
            let info = format!(
                "{} {} contracts @ ${:.2} (Size: {})",
                direction,
                futures.quantity.abs(),
                futures.entry_price,
                futures.contract_size
            );
            ("futures-position", info)
        }
    };

    rsx! {
        div {
            class: "position-card {position_type_class}",
            class: if is_editing() { "editing" } else { "" },
            
            div {
                class: "position-card-header",
                div {
                    class: "position-type-badge",
                    "{props.position.position_type():?}"
                }
                div {
                    class: "position-actions",
                    if is_editing() {
                        // Edit mode buttons
                        button {
                            class: "btn btn-success btn-sm",
                            onclick: move |_| save_changes(),
                            "💾 Save"
                        }
                        button {
                            class: "btn btn-secondary btn-sm",
                            onclick: move |_| cancel_edit(),
                            "❌ Cancel"
                        }
                    } else {
                        // View mode buttons
                        button {
                            class: "btn btn-primary btn-sm edit-btn",
                            onclick: move |_| enter_edit_mode(),
                            "✏️ Edit"
                        }
                        button {
                            class: "btn btn-danger btn-sm remove-btn",
                            onclick: move |_| props.on_remove.call(props.index),
                            "🗑️"
                        }
                    }
                }
            }
            
            div {
                class: "position-card-body",
                
                if is_editing() {
                    // Edit mode form
                    div {
                        class: "position-edit-form",
                        
                        div {
                            class: "form-row",
                            label { 
                                class: "form-label",
                                "Quantity:"
                            }
                            input {
                                r#type: "number",
                                class: "form-input",
                                value: "{edit_quantity()}",
                                step: "0.01",
                                oninput: move |evt| {
                                    if let Ok(val) = evt.value().parse::<f64>() {
                                        edit_quantity.set(val);
                                    }
                                }
                            }
                        }
                        
                        div {
                            class: "form-row",
                            label { 
                                class: "form-label",
                                match &props.position {
                                    Position::Spot(_) => "Entry Price:",
                                    Position::Option(_) => "Premium:",
                                    Position::Futures(_) => "Entry Price:",
                                }
                            }
                            input {
                                r#type: "number",
                                class: "form-input",
                                value: "{edit_entry_price()}",
                                step: "0.01",
                                min: "0",
                                oninput: move |evt| {
                                    if let Ok(val) = evt.value().parse::<f64>() {
                                        edit_entry_price.set(val);
                                    }
                                }
                            }
                        }
                        
                        // Strike price for options
                        if matches!(&props.position, Position::Option(_)) {
                            div {
                                class: "form-row",
                                label { 
                                    class: "form-label",
                                    "Strike Price:"
                                }
                                input {
                                    r#type: "number",
                                    class: "form-input",
                                    value: "{edit_strike_price()}",
                                    step: "0.01",
                                    min: "0",
                                    oninput: move |evt| {
                                        if let Ok(val) = evt.value().parse::<f64>() {
                                            edit_strike_price.set(val);
                                        }
                                    }
                                }
                            }
                        }
                        
                        div {
                            class: "form-row",
                            label { 
                                class: "form-label",
                                "Description:"
                            }
                            input {
                                r#type: "text",
                                class: "form-input",
                                value: "{edit_description()}",
                                placeholder: "Optional description",
                                oninput: move |evt| edit_description.set(evt.value())
                            }
                        }
                    }
                } else {
                    // View mode display
                    div {
                        class: "position-info",
                        "{position_info}"
                    }
                    
                    if !props.position.description().is_empty() {
                        div {
                            class: "position-description",
                            "\"{props.position.description()}\""
                        }
                    }
                }
            }
        }
    }
}
