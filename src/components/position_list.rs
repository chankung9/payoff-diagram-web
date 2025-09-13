use dioxus::prelude::*;
use crate::models::Position;

#[derive(Props, Clone, PartialEq)]
pub struct PositionListProps {
    pub positions: Vec<Position>,
    pub on_remove_position: EventHandler<usize>,
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
                            on_remove: move |idx| props.on_remove_position.call(idx)
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
}

pub fn PositionCard(props: PositionCardProps) -> Element {
    let (position_type_class, position_info) = match &props.position {
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
            
            div {
                class: "position-card-header",
                div {
                    class: "position-type-badge",
                    "{props.position.position_type():?}"
                }
                button {
                    class: "btn btn-danger btn-sm remove-btn",
                    onclick: move |_| props.on_remove.call(props.index),
                    "×"
                }
            }
            
            div {
                class: "position-card-body",
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
