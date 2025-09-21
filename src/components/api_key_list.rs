use dioxus::prelude::*;
use crate::models::api_keys::{ApiKey, Platform};
use chrono::{DateTime, Utc};

#[derive(Props, Clone, PartialEq)]
pub struct ApiKeyListProps {
    pub api_keys: Vec<ApiKey>,
    pub on_edit: EventHandler<String>,     // Edit key by ID
    pub on_delete: EventHandler<String>,   // Delete key by ID
    pub on_toggle: EventHandler<String>,   // Toggle active status by ID
    pub on_test: EventHandler<String>,     // Test connection by ID
    pub on_sync: EventHandler<String>,     // Sync positions by ID
}

#[component]
pub fn ApiKeyList(props: ApiKeyListProps) -> Element {
    let mut testing_keys = use_signal(|| std::collections::HashSet::<String>::new());

    let mut handle_test = move |key_id: String| {
        testing_keys.with_mut(|set| set.insert(key_id.clone()));
        
        // Simulate API test
        let test_id = key_id.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(2000).await;
            testing_keys.with_mut(|set| set.remove(&test_id));
        });
        
        props.on_test.call(key_id);
    };

    let format_date = |date: DateTime<Utc>| -> String {
        date.format("%Y-%m-%d %H:%M").to_string()
    };

    let get_platform_icon = |platform: &Platform| -> &str {
        match platform {
            Platform::Binance => "🟡",
        }
    };

    let get_status_icon = |key: &ApiKey| -> (&str, &str) {
        if key.is_active {
            ("🟢", "Active")
        } else {
            ("🔴", "Inactive")
        }
    };

    rsx! {
        div { class: "api-key-list",
            if props.api_keys.is_empty() {
                div { class: "empty-state",
                    div { class: "empty-icon", "🔑" }
                    h3 { "No API Keys" }
                    p { "Add your first API key to start importing trading data." }
                }
            } else {
                div { class: "key-grid",
                    for key in &props.api_keys {
                        div { 
                            key: "{key.id}",
                            class: if key.is_active { "key-card active" } else { "key-card inactive" },
                            
                            // Header
                            div { class: "key-header",
                                div { class: "key-title",
                                    span { class: "platform-icon", "{get_platform_icon(&key.platform)}" }
                                    h4 { "{key.name}" }
                                    span { class: "platform-name", "{key.platform:?}" }
                                }
                                
                                div { class: "key-status",
                                    {
                                        let (icon, status) = get_status_icon(key);
                                        rsx! {
                                            span { class: "status-indicator", "{icon}" }
                                            span { class: "status-text", "{status}" }
                                        }
                                    }
                                }
                            }

                            // Key info
                            div { class: "key-info",
                                div { class: "key-field",
                                    label { "API Key:" }
                                    code { "{&key.api_key[..8]}...{&key.api_key[key.api_key.len()-4..]}" }
                                }
                                
                                if let Some(ref description) = key.description {
                                    div { class: "key-field",
                                        label { "Description:" }
                                        span { "{description}" }
                                    }
                                }

                                div { class: "key-field",
                                    label { "Created:" }
                                    span { "{format_date(key.created_at)}" }
                                }

                                if let Some(last_used) = key.last_used {
                                    div { class: "key-field",
                                        label { "Last Used:" }
                                        span { "{format_date(last_used)}" }
                                    }
                                }
                            }

                            // Permissions
                            div { class: "key-permissions",
                                label { "Permissions:" }
                                div { class: "permission-tags",
                                    if key.permissions.can_read_account {
                                        span { class: "permission-tag", "Account" }
                                    }
                                    if key.permissions.can_read_orders {
                                        span { class: "permission-tag", "Orders" }
                                    }
                                    if key.permissions.can_read_futures {
                                        span { class: "permission-tag", "Futures" }
                                    }
                                    if key.permissions.can_read_options {
                                        span { class: "permission-tag", "Options" }
                                    }
                                }
                            }

                            // Actions
                            div { class: "key-actions",
                                button {
                                    class: "action-btn sync-btn",
                                    disabled: !key.is_active,
                                    onclick: {
                                        let key_id = key.id.clone();
                                        move |_| props.on_sync.call(key_id.clone())
                                    },
                                    "📊 Sync"
                                }
                                
                                button {
                                    class: "action-btn test-btn",
                                    disabled: testing_keys.read().contains(&key.id),
                                    onclick: {
                                        let key_id = key.id.clone();
                                        move |_| handle_test(key_id.clone())
                                    },
                                    if testing_keys.read().contains(&key.id) {
                                        "🔄 Testing..."
                                    } else {
                                        "🔍 Test"
                                    }
                                }
                                
                                button {
                                    class: "action-btn toggle-btn",
                                    onclick: {
                                        let key_id = key.id.clone();
                                        move |_| props.on_toggle.call(key_id.clone())
                                    },
                                    if key.is_active { "⏸️ Disable" } else { "▶️ Enable" }
                                }
                                
                                button {
                                    class: "action-btn edit-btn",
                                    onclick: {
                                        let key_id = key.id.clone();
                                        move |_| props.on_edit.call(key_id.clone())
                                    },
                                    "✏️ Edit"
                                }
                                
                                button {
                                    class: "action-btn delete-btn",
                                    onclick: {
                                        let key_id = key.id.clone();
                                        move |_| {
                                            // Simple confirmation - in real app might use a modal
                                            if web_sys::window()
                                                .unwrap()
                                                .confirm_with_message("Are you sure you want to delete this API key?")
                                                .unwrap_or(false)
                                            {
                                                props.on_delete.call(key_id.clone());
                                            }
                                        }
                                    },
                                    "🗑️"
                                }
                            }
                        }
                    }
                }

                // Summary
                div { class: "list-summary",
                    {
                        let active_count = props.api_keys.iter().filter(|k| k.is_active).count();
                        let total_count = props.api_keys.len();
                        
                        rsx! {
                            p { 
                                "Total: {total_count} API keys ({active_count} active, {total_count - active_count} inactive)"
                            }
                        }
                    }
                }
            }
        }
    }
}
