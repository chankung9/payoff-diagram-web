use dioxus::prelude::*;
use crate::models::api_keys::{ApiKey, Platform};
use crate::components::{ApiKeyForm, PositionSyncDialog};
use crate::models::{Portfolio, Position};

#[derive(Props, Clone, PartialEq)]
pub struct ApiKeyManagerProps {
    #[props(optional)]
    pub on_key_selected: Option<EventHandler<String>>, // Called when user selects a key for import
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>, // Called when user closes the manager
    #[props(optional)]
    pub current_portfolio: Option<Signal<Option<Portfolio>>>, // Current portfolio for importing
}

#[component]
pub fn ApiKeyManager(props: ApiKeyManagerProps) -> Element {
    // State for API keys (load from local storage)
    let mut api_keys = use_signal(|| {
        // Load from local storage if available, otherwise empty vec
        match crate::utils::LocalStorageManager::load_api_keys() {
            Ok(keys) => keys,
            Err(_) => vec![], // Start with empty list if no keys stored
        }
    });

    // UI state
    let mut show_form = use_signal(|| false);
    let mut editing_key = use_signal(|| None::<ApiKey>);
    let mut view_mode = use_signal(|| ViewMode::List);
    let mut show_sync_dialog = use_signal(|| false);
    let mut selected_key_for_sync = use_signal(|| None::<ApiKey>);

    #[derive(PartialEq, Clone)]
    enum ViewMode {
        List,
        Form,
    }

    // Form handlers
    let handle_add_new = move |_| {
        editing_key.set(None);
        show_form.set(true);
        view_mode.set(ViewMode::Form);
    };

    let handle_edit = move |key_id: String| {
        if let Some(key) = api_keys.read().iter().find(|k| k.id == key_id).cloned() {
            editing_key.set(Some(key));
            show_form.set(true);
            view_mode.set(ViewMode::Form);
        }
    };

    let handle_save = {
        let mut api_keys = api_keys.clone();
        EventHandler::new(move |key: ApiKey| {
            api_keys.with_mut(|keys| {
                if let Some(existing) = keys.iter_mut().find(|k| k.id == key.id) {
                    *existing = key.clone(); // Update existing
                } else {
                    keys.push(key); // Add new
                }
                
                // Save to local storage
                if let Err(e) = crate::utils::LocalStorageManager::save_api_keys(keys) {
                    web_sys::console::error_1(&format!("Failed to save API keys: {}", e).into());
                }
            });
            show_form.set(false);
            editing_key.set(None);
            view_mode.set(ViewMode::List);
        })
    };    let handle_delete = {
        let mut api_keys = api_keys.clone();
        EventHandler::new(move |key_id: String| {
            api_keys.with_mut(|keys| {
                keys.retain(|k| k.id != key_id);
                
                // Save to local storage
                if let Err(e) = crate::utils::LocalStorageManager::save_api_keys(keys) {
                    web_sys::console::error_1(&format!("Failed to save API keys: {}", e).into());
                }
            });
        })
    };

    let handle_toggle = move |key_id: String| {
        api_keys.with_mut(|keys| {
            if let Some(key) = keys.iter_mut().find(|k| k.id == key_id) {
                key.is_active = !key.is_active;
                
                // Save to local storage
                if let Err(e) = crate::utils::LocalStorageManager::save_api_keys(keys) {
                    web_sys::console::error_1(&format!("Failed to save API keys: {}", e).into());
                }
            }
        });
    };

    let handle_test = move |key_id: String| {
        // In real implementation, this would test the API connection
        web_sys::console::log_1(&format!("Testing API key: {}", key_id).into());
    };

    let handle_cancel_form = move |_| {
        show_form.set(false);
        editing_key.set(None);
        view_mode.set(ViewMode::List);
    };

        let handle_sync = move |key_id: String| {
        if let Some(key) = api_keys.read().iter().find(|k| k.id == key_id) {
            selected_key_for_sync.set(Some(key.clone()));
            show_sync_dialog.set(true);
        }
    };

    // Clone props for closures
    let on_close_prop = props.on_close.clone();

    rsx! {
        div { class: "api-key-manager",
            // Header
            div { class: "manager-header",
                h2 { "API Key Management" }
                
                div { class: "header-actions",
                    if *view_mode.read() == ViewMode::List {
                        button {
                            class: "primary-btn",
                            onclick: handle_add_new,
                            "➕ Add New Key"
                        }
                    } else {
                        button {
                            class: "secondary-btn",
                            onclick: move |_| view_mode.set(ViewMode::List),
                            "◀️ Back to List"
                        }
                    }
                    
                    // Close button
                    if on_close_prop.is_some() {
                        button {
                            class: "close-btn",
                            onclick: {
                                let on_close = on_close_prop.clone();
                                move |_| {
                                    if let Some(ref handler) = on_close {
                                        handler.call(());
                                    }
                                }
                            },
                            "✕"
                        }
                    }
                }
            }

            // Quick stats
            div { class: "quick-stats",
                {
                    let active_keys = api_keys.read().iter().filter(|k| k.is_active).count();
                    let total_keys = api_keys.read().len();
                    let binance_keys = api_keys.read().iter().filter(|k| k.platform == Platform::Binance).count();
                    
                    rsx! {
                        div { class: "stat-card",
                            div { class: "stat-number", "{total_keys}" }
                            div { class: "stat-label", "Total Keys" }
                        }
                        
                        div { class: "stat-card",
                            div { class: "stat-number", "{active_keys}" }
                            div { class: "stat-label", "Active" }
                        }
                        
                        div { class: "stat-card",
                            div { class: "stat-number", "{binance_keys}" }
                            div { class: "stat-label", "Binance" }
                        }
                    }
                }
            }

            // Main content based on view mode
            match *view_mode.read() {
                ViewMode::List => rsx! {
                    crate::components::api_key_list::ApiKeyList {
                        api_keys: api_keys.read().clone(),
                        on_edit: handle_edit,
                        on_delete: handle_delete,
                        on_toggle: handle_toggle,
                        on_test: handle_test,
                        on_sync: handle_sync,
                    }
                },
                ViewMode::Form => rsx! {
                    div { class: "form-container",
                        if editing_key.read().is_some() {
                            crate::components::api_key_form::ApiKeyForm {
                                api_key: editing_key.read().clone(),
                                on_save: handle_save,
                                on_cancel: handle_cancel_form,
                                on_delete: Some(handle_delete.clone()),
                            }
                        } else {
                            crate::components::api_key_form::ApiKeyForm {
                                api_key: editing_key.read().clone(),
                                on_save: handle_save,
                                on_cancel: handle_cancel_form,
                                on_delete: None,
                            }
                        }
                    }
                }
            }

            // Position Sync Dialog
            if *show_sync_dialog.read() {
                if let Some(key) = selected_key_for_sync.read().as_ref() {
                    crate::components::position_sync_dialog::PositionSyncDialog {
                        api_key: key.clone(),
                        current_portfolio: props.current_portfolio,
                        on_close: move |_| {
                            show_sync_dialog.set(false);
                            selected_key_for_sync.set(None);
                        },
                        on_positions_imported: move |positions| {
                            // Handle imported positions - add them to the portfolio
                            if let Some(portfolio_signal) = props.current_portfolio {
                                let current_portfolio = portfolio_signal.read().clone();
                                if let Some(mut portfolio) = current_portfolio {
                                    for position in positions {
                                        portfolio.add_position(position);
                                    }
                                    // Clone the portfolio_signal for use in the async block
                                    let mut portfolio_sig = portfolio_signal.clone();
                                    portfolio_sig.set(Some(portfolio));
                                }
                            }
                            show_sync_dialog.set(false);
                            selected_key_for_sync.set(None);
                        },
                    }
                }
            }
        }
    }
}
