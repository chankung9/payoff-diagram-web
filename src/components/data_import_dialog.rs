use dioxus::prelude::*;
use crate::models::{
    import_data::{ImportConfig, ImportMode, AssetSelection, SymbolFilter, ImportStatus, ImportResult},
    api_keys::ApiKey,
};

#[derive(Props, Clone, PartialEq)]
pub struct DataImportDialogProps {
    pub selected_api_key: ApiKey,
    pub on_import: EventHandler<ImportConfig>,
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn DataImportDialog(props: DataImportDialogProps) -> Element {
    let mut config = use_signal(|| {
        let mut cfg = ImportConfig::default();
        cfg.api_key_id = props.selected_api_key.id.clone();
        cfg
    });
    
    let mut import_status = use_signal(|| ImportStatus::NotStarted);
    let mut show_advanced = use_signal(|| false);

    // Update config helpers
    let mut update_mode = move |mode: ImportMode| {
        config.with_mut(|c| c.mode = mode);
    };

    let mut update_asset_selection = move |field: &str, value: bool| {
        config.with_mut(|c| {
            match field {
                "spot" => c.asset_selection.spot_positions = value,
                "futures" => c.asset_selection.futures_positions = value,
                "options" => c.asset_selection.options_positions = value,
                "orders" => c.asset_selection.open_orders = value,
                _ => {}
            }
        });
    };

    let handle_import = move |_| {
        import_status.set(ImportStatus::FetchingData);
        let import_config = config.read().clone();
        
        // Simulate import process
        spawn(async move {
            import_status.set(ImportStatus::ProcessingData);
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            import_status.set(ImportStatus::CreatingBackup);
            gloo_timers::future::TimeoutFuture::new(500).await;
            
            import_status.set(ImportStatus::ImportingPositions);
            gloo_timers::future::TimeoutFuture::new(1500).await;
            
            let result = ImportResult {
                success: true,
                message: "Successfully imported trading data".to_string(),
                positions_imported: 15,
                positions_skipped: 2,
                errors: vec![],
                backup_created: Some("portfolio_backup_20241223.json".to_string()),
                imported_at: chrono::Utc::now(),
            };
            
            import_status.set(ImportStatus::Completed(result));
        });
        
        props.on_import.call(import_config);
    };

    let can_import = match *import_status.read() {
        ImportStatus::NotStarted => {
            config.read().asset_selection.spot_positions ||
            config.read().asset_selection.futures_positions ||
            config.read().asset_selection.options_positions ||
            config.read().asset_selection.open_orders
        },
        _ => false,
    };

    rsx! {
        div { class: "import-dialog-overlay",
            div { class: "import-dialog",
                // Header
                div { class: "dialog-header",
                    h2 { "Import Trading Data" }
                    button { 
                        class: "close-btn",
                        onclick: move |_| props.on_cancel.call(()),
                        "✕"
                    }
                }

                // Selected API Key info
                div { class: "selected-key-info",
                    h3 { "Using API Key" }
                    div { class: "key-summary",
                        span { class: "platform-icon", "🟡" }
                        strong { "{props.selected_api_key.name}" }
                        span { class: "platform", "({props.selected_api_key.platform:?})" }
                    }
                }

                // Import configuration
                div { class: "import-config",
                    // Import mode
                    div { class: "config-section",
                        h3 { "Import Mode" }
                        div { class: "mode-options",
                            for mode in [ImportMode::Append, ImportMode::Replace] {
                                label { 
                                    class: if config.read().mode == mode { "mode-option selected" } else { "mode-option" },
                                    input {
                                        r#type: "radio",
                                        name: "import-mode",
                                        checked: config.read().mode == mode,
                                        onchange: move |_| update_mode(mode.clone()),
                                    }
                                    div { class: "mode-info",
                                        div { class: "mode-title",
                                            span { class: "mode-icon", "{mode.icon()}" }
                                            strong { 
                                                match mode {
                                                    ImportMode::Append => "Append to Portfolio",
                                                    ImportMode::Replace => "Replace Portfolio",
                                                }
                                            }
                                        }
                                        p { class: "mode-description", "{mode.description()}" }
                                    }
                                }
                            }
                        }
                    }

                    // Asset selection
                    div { class: "config-section",
                        h3 { "Data to Import" }
                        div { class: "asset-selection",
                            label { class: "asset-option",
                                input {
                                    r#type: "checkbox",
                                    checked: config.read().asset_selection.spot_positions,
                                    onchange: move |evt| update_asset_selection("spot", evt.checked()),
                                    disabled: !props.selected_api_key.permissions.can_read_account,
                                }
                                span { "💰 Spot Positions" }
                                small { "Current spot trading balances" }
                            }

                            label { class: "asset-option",
                                input {
                                    r#type: "checkbox",
                                    checked: config.read().asset_selection.futures_positions,
                                    onchange: move |evt| update_asset_selection("futures", evt.checked()),
                                    disabled: !props.selected_api_key.permissions.can_read_futures,
                                }
                                span { "📈 Futures Positions" }
                                small { "Active futures contracts" }
                            }

                            label { class: "asset-option",
                                input {
                                    r#type: "checkbox",
                                    checked: config.read().asset_selection.options_positions,
                                    onchange: move |evt| update_asset_selection("options", evt.checked()),
                                    disabled: !props.selected_api_key.permissions.can_read_options,
                                }
                                span { "🎯 Options Positions" }
                                small { "Active options contracts" }
                            }

                            label { class: "asset-option",
                                input {
                                    r#type: "checkbox",
                                    checked: config.read().asset_selection.open_orders,
                                    onchange: move |evt| update_asset_selection("orders", evt.checked()),
                                    disabled: !props.selected_api_key.permissions.can_read_orders,
                                }
                                span { "📋 Open Orders" }
                                small { "Pending buy/sell orders" }
                            }
                        }
                    }

                    // Advanced options
                    div { class: "config-section",
                        button {
                            class: "toggle-advanced",
                            onclick: move |_| {
                                let current = *show_advanced.read();
                                show_advanced.set(!current);
                            },
                            if *show_advanced.read() { "⏷ Hide Advanced Options" } else { "⏵ Show Advanced Options" }
                        }

                        if *show_advanced.read() {
                            div { class: "advanced-options",
                                // Symbol filter
                                div { class: "option-group",
                                    h4 { "Symbol Filter" }
                                    
                                    label { class: "checkbox-option",
                                        input {
                                            r#type: "checkbox",
                                            checked: config.read().symbol_filter.exclude_dust,
                                            onchange: move |evt| {
                                                config.with_mut(|c| c.symbol_filter.exclude_dust = evt.checked());
                                            },
                                        }
                                        "Exclude dust positions (< $1 USD)"
                                    }

                                    div { class: "input-group",
                                        label { "Minimum position value (USD):" }
                                        input {
                                            r#type: "number",
                                            min: "0",
                                            step: "0.01",
                                            value: "{config.read().symbol_filter.min_value_usd}",
                                            oninput: move |evt| {
                                                if let Ok(value) = evt.value().parse::<f64>() {
                                                    config.with_mut(|c| c.symbol_filter.min_value_usd = value);
                                                }
                                            },
                                        }
                                    }
                                }

                                // Backup option
                                div { class: "option-group",
                                    h4 { "Backup" }
                                    label { class: "checkbox-option",
                                        input {
                                            r#type: "checkbox",
                                            checked: config.read().create_backup,
                                            onchange: move |evt| {
                                                config.with_mut(|c| c.create_backup = evt.checked());
                                            },
                                        }
                                        "Create backup before import"
                                    }
                                }
                            }
                        }
                    }
                }

                // Import status
                if !matches!(*import_status.read(), ImportStatus::NotStarted) {
                    div { class: "import-status",
                        match import_status.read().clone() {
                            ImportStatus::FetchingData => rsx! {
                                div { class: "status-item active",
                                    "🔄 Fetching data from exchange..."
                                }
                            },
                            ImportStatus::ProcessingData => rsx! {
                                div { class: "status-item completed", "✅ Data fetched" }
                                div { class: "status-item active", "🔄 Processing positions..." }
                            },
                            ImportStatus::CreatingBackup => rsx! {
                                div { class: "status-item completed", "✅ Data processed" }
                                div { class: "status-item active", "🔄 Creating backup..." }
                            },
                            ImportStatus::ImportingPositions => rsx! {
                                div { class: "status-item completed", "✅ Backup created" }
                                div { class: "status-item active", "🔄 Importing positions..." }
                            },
                            ImportStatus::Completed(result) => rsx! {
                                div { class: "status-item completed", "✅ Import completed!" }
                                div { class: "import-result success",
                                    h4 { "Import Successful" }
                                    p { "{result.message}" }
                                    ul { class: "result-details",
                                        li { "Positions imported: {result.positions_imported}" }
                                        if result.positions_skipped > 0 {
                                            li { "Positions skipped: {result.positions_skipped}" }
                                        }
                                        if let Some(ref backup) = result.backup_created {
                                            li { "Backup saved: {backup}" }
                                        }
                                    }
                                }
                            },
                            ImportStatus::Failed(error) => rsx! {
                                div { class: "import-result error",
                                    h4 { "Import Failed" }
                                    p { "{error}" }
                                }
                            },
                            _ => rsx! { div {} }
                        }
                    }
                }

                // Footer actions
                div { class: "dialog-footer",
                    button {
                        class: "cancel-btn",
                        onclick: move |_| props.on_cancel.call(()),
                        if matches!(*import_status.read(), ImportStatus::Completed(_)) {
                            "Close"
                        } else {
                            "Cancel"
                        }
                    }
                    
                    if matches!(*import_status.read(), ImportStatus::NotStarted) {
                        button {
                            class: "import-btn",
                            disabled: !can_import,
                            onclick: handle_import,
                            "📥 Start Import"
                        }
                    }
                }
            }
        }
    }
}
