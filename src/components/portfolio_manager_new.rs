// Portfolio management UI component
use dioxus::prelude::*;
use crate::models::{Portfolio, ExportFormat};
use crate::utils::{LocalStorageManager, PortfolioListItem, BrowserFileManager, StorageInfo};

#[derive(Props, Clone, PartialEq)]
pub struct PortfolioManagerProps {
    pub current_portfolio: Signal<Option<Portfolio>>,
    pub on_portfolio_change: EventHandler<Portfolio>,
    pub on_delete_portfolio: EventHandler<()>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn PortfolioManager(props: PortfolioManagerProps) -> Element {
    let mut portfolio_list = use_signal(|| Vec::<PortfolioListItem>::new());
    let mut show_create_form = use_signal(|| false);
    let mut show_import_dialog = use_signal(|| false);
    let mut storage_info = use_signal(|| None::<StorageInfo>);
    let mut error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);

    // Load portfolio list on mount
    use_effect(move || {
        match LocalStorageManager::get_portfolio_list() {
            Ok(list) => portfolio_list.set(list),
            Err(e) => error_message.set(Some(format!("Error loading portfolios: {}", e))),
        }

        match LocalStorageManager::get_storage_info() {
            Ok(info) => storage_info.set(Some(info)),
            Err(_) => {}  // Silently ignore storage info errors
        }
    });

    let create_portfolio = move |name: String| {
        let new_portfolio = Portfolio::new(name);
        match LocalStorageManager::save_portfolio(&new_portfolio) {
            Ok(_) => {
                props.on_portfolio_change.call(new_portfolio);
                show_create_form.set(false);
                success_message.set(Some("Portfolio created successfully".to_string()));
                
                // Refresh list
                if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                    portfolio_list.set(list);
                }
            },
            Err(e) => error_message.set(Some(format!("Error creating portfolio: {}", e))),
        }
    };

    let load_portfolio = move |id: String| {
        match LocalStorageManager::load_portfolio(&id) {
            Ok(portfolio) => {
                props.on_portfolio_change.call(portfolio);
                success_message.set(Some("Portfolio loaded successfully".to_string()));
            },
            Err(e) => error_message.set(Some(format!("Error loading portfolio: {}", e))),
        }
    };

    let export_portfolio = move |(id, format): (String, ExportFormat)| {
        match LocalStorageManager::load_portfolio(&id) {
            Ok(portfolio) => {
                match BrowserFileManager::export_portfolio(&portfolio, format) {
                    Ok(_) => success_message.set(Some("Portfolio exported successfully".to_string())),
                    Err(e) => error_message.set(Some(format!("Export error: {}", e))),
                }
            },
            Err(e) => error_message.set(Some(format!("Error loading portfolio for export: {}", e))),
        }
    };

    let delete_portfolio = move |id: String| {
        match LocalStorageManager::delete_portfolio(&id) {
            Ok(_) => {
                success_message.set(Some("Portfolio deleted successfully".to_string()));
                
                // Check if deleted portfolio was current
                if let Some(current) = props.current_portfolio.read().as_ref() {
                    if current.id == id {
                        props.on_delete_portfolio.call(());
                    }
                }
                
                // Refresh list
                if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                    portfolio_list.set(list);
                }
            },
            Err(e) => error_message.set(Some(format!("Error deleting portfolio: {}", e))),
        }
    };

    let import_portfolio = move |file_content: String| {
        match BrowserFileManager::import_portfolio(&file_content) {
            Ok(portfolio) => {
                match LocalStorageManager::save_portfolio(&portfolio) {
                    Ok(_) => {
                        props.on_portfolio_change.call(portfolio);
                        show_import_dialog.set(false);
                        success_message.set(Some("Portfolio imported successfully".to_string()));
                        
                        // Refresh list
                        if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                            portfolio_list.set(list);
                        }
                    },
                    Err(e) => error_message.set(Some(format!("Error saving imported portfolio: {}", e))),
                }
            },
            Err(e) => error_message.set(Some(format!("Import error: {}", e))),
        }
    };

    let current_portfolio_id = props.current_portfolio.read().as_ref().map(|p| p.id.clone());

    rsx! {
        div { class: "portfolio-manager",
            div { class: "portfolio-header",
                h2 { "Portfolio Manager" }
                button {
                    class: "close-btn",
                    onclick: move |_| props.on_close.call(()),
                    "✕"
                }
            }

            // Success/Error Messages
            if let Some(msg) = success_message.read().as_ref() {
                div { class: "message success", 
                    "{msg}"
                    button {
                        onclick: move |_| success_message.set(None),
                        "✕"
                    }
                }
            }

            if let Some(msg) = error_message.read().as_ref() {
                div { class: "message error", 
                    "{msg}"
                    button {
                        onclick: move |_| error_message.set(None),
                        "✕"
                    }
                }
            }

            // Action Buttons
            div { class: "portfolio-actions",
                button {
                    class: "action-btn primary",
                    onclick: move |_| show_create_form.set(true),
                    "📝 Create Portfolio"
                }
                button {
                    class: "action-btn secondary",
                    onclick: move |_| show_import_dialog.set(true),
                    "📂 Import Portfolio"
                }
            }

            // Create Form
            if show_create_form() {
                CreatePortfolioForm {
                    on_create: create_portfolio,
                    on_cancel: move |_| show_create_form.set(false),
                }
            }

            // Import Dialog
            if show_import_dialog() {
                ImportPortfolioDialog {
                    on_import: import_portfolio,
                    on_cancel: move |_| show_import_dialog.set(false),
                }
            }

            // Portfolio List
            div { class: "portfolio-list",
                h3 { "Saved Portfolios" }
                if portfolio_list.read().is_empty() {
                    div { class: "empty-state",
                        "No saved portfolios. Create your first portfolio!"
                    }
                } else {
                    for item in portfolio_list.read().iter() {
                        PortfolioCard {
                            key: "{item.id}",
                            portfolio_item: item.clone(),
                            is_current: current_portfolio_id.as_ref() == Some(&item.id),
                            on_load: load_portfolio,
                            on_export: export_portfolio,
                            on_delete: delete_portfolio,
                        }
                    }
                }
            }

            // Storage Info
            if let Some(info) = storage_info.read().as_ref() {
                div { class: "storage-info",
                    h4 { "Storage Information" }
                    div { "Total portfolios: {info.portfolio_count}" }
                    div { "Storage used: {info.storage_size_kb} KB" }
                    div { "Last backup: {info.last_backup.as_ref().map(|d| d.format(\"%Y-%m-%d %H:%M\").to_string()).unwrap_or_else(|| \"Never\".to_string())}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CreatePortfolioFormProps {
    on_create: EventHandler<String>,
    on_cancel: EventHandler<()>,
}

#[component]
fn CreatePortfolioForm(props: CreatePortfolioFormProps) -> Element {
    let mut portfolio_name = use_signal(|| String::new());

    rsx! {
        div { class: "create-form-overlay",
            div { class: "create-form",
                h3 { "Create New Portfolio" }
                div { class: "form-group",
                    label { "Portfolio Name:" }
                    input {
                        r#type: "text",
                        value: "{portfolio_name.read()}",
                        oninput: move |e| portfolio_name.set(e.value()),
                        placeholder: "Enter portfolio name...",
                        autofocus: true
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "action-btn primary",
                        disabled: portfolio_name.read().trim().is_empty(),
                        onclick: move |_| {
                            let name = portfolio_name.read().trim().to_string();
                            if !name.is_empty() {
                                props.on_create.call(name);
                            }
                        },
                        "Create"
                    }
                    button {
                        class: "action-btn secondary",
                        onclick: move |_| props.on_cancel.call(()),
                        "Cancel"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ImportPortfolioDialogProps {
    on_import: EventHandler<String>,
    on_cancel: EventHandler<()>,
}

#[component]
fn ImportPortfolioDialog(props: ImportPortfolioDialogProps) -> Element {
    let mut is_loading = use_signal(|| false);

    let handle_file_select = move |evt: Event<FileData>| {
        if let Some(file_engine) = evt.files() {
            if let Some(file) = file_engine.files().get(0) {
                is_loading.set(true);
                let on_import = props.on_import.clone();
                spawn(async move {
                    match BrowserFileManager::read_file_content(file).await {
                        Ok(content) => on_import.call(content),
                        Err(_) => {}, // Error will be handled by parent
                    }
                    is_loading.set(false);
                });
            }
        }
    };

    rsx! {
        div { class: "import-dialog-overlay",
            div { class: "import-dialog",
                h3 { "Import Portfolio" }
                
                if is_loading() {
                    div { class: "loading", "Loading file..." }
                } else {
                    div { class: "file-input-container",
                        input {
                            r#type: "file",
                            accept: ".json,.csv",
                            onchange: handle_file_select
                        }
                        div { class: "file-help",
                            "Select a JSON or CSV file exported from this app"
                        }
                    }
                }

                div { class: "dialog-actions",
                    button {
                        class: "action-btn secondary",
                        onclick: move |_| props.on_cancel.call(()),
                        "Cancel"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PortfolioCardProps {
    portfolio_item: PortfolioListItem,
    is_current: bool,
    on_load: EventHandler<String>,
    on_export: EventHandler<(String, ExportFormat)>,
    on_delete: EventHandler<String>,
}

#[component]
fn PortfolioCard(props: PortfolioCardProps) -> Element {
    let mut show_actions = use_signal(|| false);

    rsx! {
        div { 
            class: "portfolio-card {if props.is_current { \"current\" } else { \"\" }}",
            onclick: {
                let portfolio_id = props.portfolio_item.id.clone();
                let on_load = props.on_load.clone();
                move |_| on_load.call(portfolio_id.clone())
            },

            div { class: "portfolio-info",
                h4 { "{props.portfolio_item.name}" }
                div { class: "portfolio-meta",
                    span { class: "position-count", 
                        "{props.portfolio_item.position_count} positions" 
                    }
                    span { class: "update-time", 
                        {format!("Updated {}", props.portfolio_item.updated_at.format("%m/%d %H:%M"))}
                    }
                }
            }

            div { class: "portfolio-actions",
                button {
                    class: "action-btn more-btn",
                    onclick: move |e| {
                        e.stop_propagation();
                        show_actions.set(!show_actions());
                    },
                    "⋮"
                }

                if show_actions() {
                    div { class: "portfolio-actions-menu",
                        button {
                            class: "action-btn export-btn",
                            onclick: {
                                let portfolio_id = props.portfolio_item.id.clone();
                                let on_export = props.on_export.clone();
                                move |e| {
                                    e.stop_propagation();
                                    on_export.call((portfolio_id.clone(), ExportFormat::JSON));
                                    show_actions.set(false);
                                }
                            },
                            "📄 Export JSON"
                        }
                        button {
                            class: "action-btn export-btn",
                            onclick: {
                                let portfolio_id = props.portfolio_item.id.clone();
                                let on_export = props.on_export.clone();
                                move |e| {
                                    e.stop_propagation();
                                    on_export.call((portfolio_id.clone(), ExportFormat::CSV));
                                    show_actions.set(false);
                                }
                            },
                            "📊 Export CSV"
                        }
                        if !props.is_current {
                            button {
                                class: "action-btn delete-btn",
                                onclick: {
                                    let portfolio_id = props.portfolio_item.id.clone();
                                    let portfolio_name = props.portfolio_item.name.clone();
                                    let on_delete = props.on_delete.clone();
                                    move |e| {
                                        e.stop_propagation();
                                        let confirmed = web_sys::window()
                                            .map(|w| w.confirm_with_message(&format!("Delete portfolio '{}'?", portfolio_name)).unwrap_or(false))
                                            .unwrap_or(false);
                                        if confirmed {
                                            on_delete.call(portfolio_id.clone());
                                        }
                                        show_actions.set(false);
                                    }
                                },
                                "🗑️ Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
