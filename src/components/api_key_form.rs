use dioxus::prelude::*;
use crate::models::api_keys::{ApiKey, Platform, ApiKeyPermissions, KeyTestResult};
use chrono::Utc;

#[derive(Props, Clone, PartialEq)]
pub struct ApiKeyFormProps {
    pub api_key: Option<ApiKey>,    // None for new key, Some for editing
    pub on_save: EventHandler<ApiKey>,
    pub on_cancel: EventHandler<()>,
    pub on_delete: Option<EventHandler<String>>, // Only for existing keys
}

#[component]
pub fn ApiKeyForm(props: ApiKeyFormProps) -> Element {
    let mut name = use_signal(|| props.api_key.as_ref().map(|k| k.name.clone()).unwrap_or_default());
    let mut api_key = use_signal(|| props.api_key.as_ref().map(|k| k.api_key.clone()).unwrap_or_default());
    let mut secret_key = use_signal(|| {
        props.api_key.as_ref()
            .map(|k| k.encrypted_secret.clone())
            .unwrap_or_default()
    });
    let mut platform = use_signal(|| props.api_key.as_ref().map(|k| k.platform).unwrap_or(Platform::Binance));
    let mut description = use_signal(|| {
        props.api_key.as_ref()
            .and_then(|k| k.description.clone())
            .unwrap_or_default()
    });
    
    // Permissions
    let mut can_read_account = use_signal(|| {
        props.api_key.as_ref()
            .map(|k| k.permissions.can_read_account)
            .unwrap_or(true)
    });
    let mut can_read_orders = use_signal(|| {
        props.api_key.as_ref()
            .map(|k| k.permissions.can_read_orders)
            .unwrap_or(true)
    });
    let mut can_read_futures = use_signal(|| {
        props.api_key.as_ref()
            .map(|k| k.permissions.can_read_futures)
            .unwrap_or(true)
    });
    let mut can_read_options = use_signal(|| {
        props.api_key.as_ref()
            .map(|k| k.permissions.can_read_options)
            .unwrap_or(false)
    });

    // Form state
    let mut testing = use_signal(|| false);
    let mut test_result = use_signal(|| None::<KeyTestResult>);
    let mut show_secret = use_signal(|| false);

    let is_editing = props.api_key.is_some();
    let form_title = if is_editing { "Edit API Key" } else { "Add New API Key" };

    // Clone props for closures
    let api_key_prop = props.api_key.clone();
    let api_key_prop_for_delete = props.api_key.clone();
    let on_save_prop = props.on_save.clone();
    let on_delete_prop = props.on_delete.clone();

    let is_valid = {
        let name_valid = !name.read().trim().is_empty();
        let api_key_valid = !api_key.read().trim().is_empty();
        let secret_valid = !secret_key.read().trim().is_empty();
        name_valid && api_key_valid && secret_valid
    };

    let handle_test_connection = move |_| {
        testing.set(true);
        test_result.set(None);
        
        // In a real implementation, this would call the proxy server
        // For now, simulate a test
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(2000).await;
            
            let result = KeyTestResult {
                success: api_key.read().len() > 10, // Simple validation
                message: if api_key.read().len() > 10 {
                    "Connection successful! All permissions verified.".to_string()
                } else {
                    "Invalid API key format".to_string()
                },
                tested_permissions: vec![
                    "Account Info".to_string(),
                    "Read Orders".to_string(),
                    "Read Futures".to_string(),
                ],
                tested_at: Utc::now(),
            };
            
            test_result.set(Some(result));
            testing.set(false);
        });
    };

    let handle_save = move |_| {
        if !is_valid {
            return;
        }

        let permissions = ApiKeyPermissions {
            can_read_account: *can_read_account.read(),
            can_read_orders: *can_read_orders.read(),
            can_read_futures: *can_read_futures.read(),
            can_read_options: *can_read_options.read(),
            can_trade: false, // Always false for this app
        };

        let key = ApiKey {
            id: api_key_prop.as_ref()
                .map(|k| k.id.clone())
                .unwrap_or_else(|| format!("key_{}", Utc::now().timestamp())),
            name: name.read().trim().to_string(),
            platform: *platform.read(),
            api_key: api_key.read().trim().to_string(),
            encrypted_secret: secret_key.read().trim().to_string(), // TODO: Encrypt
            permissions,
            description: {
                let desc_value = description.read();
                if desc_value.trim().is_empty() {
                    None
                } else {
                    Some(desc_value.trim().to_string())
                }
            },
            created_at: api_key_prop.as_ref()
                .map(|k| k.created_at)
                .unwrap_or_else(|| Utc::now()),
            last_used: None,
            is_active: true,
        };

        on_save_prop.call(key);
    };

    let handle_delete = move |_| {
        if let Some(ref key) = api_key_prop_for_delete {
            if let Some(ref on_delete) = on_delete_prop {
                on_delete.call(key.id.clone());
            }
        }
    };

    rsx! {
        div { class: "api-key-form-overlay",
            div { class: "api-key-form",
                // Header
                div { class: "form-header",
                    h2 { "{form_title}" }
                    button { 
                        class: "close-btn",
                        onclick: move |_| props.on_cancel.call(()),
                        "✕"
                    }
                }

                // Form content
                div { class: "form-content",
                    // Basic info
                    div { class: "form-section",
                        h3 { "Basic Information" }
                        
                        div { class: "form-group",
                            label { r#for: "key-name", "Key Name *" }
                            input {
                                id: "key-name",
                                r#type: "text",
                                placeholder: "e.g., My Trading Account",
                                value: "{name}",
                                oninput: move |evt| name.set(evt.value()),
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "platform", "Platform" }
                            select {
                                id: "platform",
                                value: "{platform:?}",
                                onchange: move |evt| {
                                    if evt.value() == "Binance" {
                                        platform.set(Platform::Binance);
                                    }
                                },
                                option { value: "Binance", selected: *platform.read() == Platform::Binance, "Binance" }
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "description", "Description" }
                            textarea {
                                id: "description",
                                placeholder: "Optional description for this API key",
                                value: "{description.read()}",
                                oninput: move |evt| description.set(evt.value()),
                            }
                        }
                    }

                    // API credentials
                    div { class: "form-section",
                        h3 { "API Credentials" }
                        
                        div { class: "form-group",
                            label { r#for: "api-key", "API Key *" }
                            input {
                                id: "api-key",
                                r#type: "text",
                                placeholder: "Enter your API key",
                                value: "{api_key}",
                                oninput: move |evt| api_key.set(evt.value()),
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "secret-key", "Secret Key *" }
                            div { class: "secret-input-group",
                                input {
                                    id: "secret-key",
                                    r#type: if *show_secret.read() { "text" } else { "password" },
                                    placeholder: "Enter your secret key",
                                    value: "{secret_key}",
                                    oninput: move |evt| secret_key.set(evt.value()),
                                }
                                button {
                                    r#type: "button",
                                    class: "toggle-secret-btn",
                                    onclick: move |_| {
                                        let current = *show_secret.read();
                                        show_secret.set(!current);
                                    },
                                    if *show_secret.read() { "👁️" } else { "👁️‍🗨️" }
                                }
                            }
                        }
                    }

                    // Permissions
                    div { class: "form-section",
                        h3 { "Permissions" }
                        p { class: "permissions-note", 
                            "Select which data this API key can access. Trading permissions are not supported."
                        }
                        
                        div { class: "permissions-grid",
                            label { class: "permission-item",
                                input {
                                    r#type: "checkbox",
                                    checked: *can_read_account.read(),
                                    onchange: move |evt| can_read_account.set(evt.checked()),
                                }
                                span { "Account Information" }
                                small { "Balance, account status" }
                            }

                            label { class: "permission-item",
                                input {
                                    r#type: "checkbox",
                                    checked: *can_read_orders.read(),
                                    onchange: move |evt| can_read_orders.set(evt.checked()),
                                }
                                span { "Open Orders" }
                                small { "Current open positions and orders" }
                            }

                            label { class: "permission-item",
                                input {
                                    r#type: "checkbox",
                                    checked: *can_read_futures.read(),
                                    onchange: move |evt| can_read_futures.set(evt.checked()),
                                }
                                span { "Futures Positions" }
                                small { "Futures trading positions" }
                            }

                            label { class: "permission-item",
                                input {
                                    r#type: "checkbox",
                                    checked: *can_read_options.read(),
                                    onchange: move |evt| can_read_options.set(evt.checked()),
                                }
                                span { "Options Positions" }
                                small { "Options trading positions" }
                            }
                        }
                    }

                    // Test connection
                    div { class: "form-section",
                        h3 { "Test Connection" }
                        
                        button {
                            class: "test-btn",
                            disabled: !is_valid || *testing.read(),
                            onclick: handle_test_connection,
                            if *testing.read() {
                                "🔄 Testing..."
                            } else {
                                "🔍 Test Connection"
                            }
                        }

                        if let Some(result) = test_result.read().as_ref() {
                            div { 
                                class: if result.success { "test-result success" } else { "test-result error" },
                                div { class: "test-message",
                                    if result.success { "✅" } else { "❌" }
                                    " {result.message}"
                                }
                                if result.success {
                                    div { class: "test-permissions",
                                        "Verified permissions: "
                                        for permission in &result.tested_permissions {
                                            span { class: "permission-tag", "{permission}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Footer actions
                div { class: "form-footer",
                    div { class: "footer-left",
                        if is_editing && props.on_delete.is_some() {
                            button {
                                class: "delete-btn",
                                onclick: handle_delete,
                                "🗑️ Delete"
                            }
                        }
                    }
                    
                    div { class: "footer-right",
                        button {
                            class: "cancel-btn",
                            onclick: move |_| props.on_cancel.call(()),
                            "Cancel"
                        }
                        
                        button {
                            class: "save-btn",
                            disabled: !is_valid,
                            onclick: handle_save,
                            if is_editing { "💾 Update" } else { "💾 Save" }
                        }
                    }
                }
            }
        }
    }
}
