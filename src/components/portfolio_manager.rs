// Simple Portfolio management UI component
use crate::models::{ExportFormat, Portfolio};
use crate::utils::{BrowserFileManager, LocalStorageManager, PortfolioListItem, StorageInfo};
use dioxus::prelude::*;

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
    let mut error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);

    // Load portfolio list on mount และสร้าง default portfolio หากไม่มี
    use_effect(move || {
        match LocalStorageManager::get_portfolio_list() {
            Ok(list) => {
                if list.is_empty() {
                    // สร้าง default portfolio
                    let default_portfolio = Portfolio::new("Default".to_string(), String::new());
                    match LocalStorageManager::save_portfolio(&default_portfolio) {
                        Ok(_) => {
                            props.on_portfolio_change.call(default_portfolio.clone());
                            // Refresh list to include the new default portfolio
                            if let Ok(updated_list) = LocalStorageManager::get_portfolio_list() {
                                portfolio_list.set(updated_list);
                            }
                        }
                        Err(e) => error_message
                            .set(Some(format!("Error creating default portfolio: {}", e))),
                    }
                } else {
                    portfolio_list.set(list);
                }
            }
            Err(e) => error_message.set(Some(format!("Error loading portfolios: {}", e))),
        }
    });

    let create_portfolio = move |name: String| {
        let new_portfolio = Portfolio::new(name.clone(), String::new());
        match LocalStorageManager::save_portfolio(&new_portfolio) {
            Ok(_) => {
                props.on_portfolio_change.call(new_portfolio);
                show_create_form.set(false);
                success_message.set(Some("Portfolio created successfully".to_string()));

                // Refresh list
                if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                    portfolio_list.set(list);
                }
            }
            Err(e) => error_message.set(Some(format!("Error creating portfolio: {}", e))),
        }
    };

    let load_portfolio = move |id: String| match LocalStorageManager::load_portfolio(&id) {
        Ok(portfolio) => {
            props.on_portfolio_change.call(portfolio);
            success_message.set(Some("Portfolio loaded successfully".to_string()));
        }
        Err(e) => error_message.set(Some(format!("Error loading portfolio: {}", e))),
    };

    let export_portfolio = move |(id, format): (String, ExportFormat)| {
        match LocalStorageManager::load_portfolio(&id) {
            Ok(portfolio) => {
                match BrowserFileManager::export_portfolio_to_file(&portfolio, format) {
                    Ok(_) => {
                        success_message.set(Some("Portfolio exported successfully".to_string()))
                    }
                    Err(e) => error_message.set(Some(format!("Export error: {}", e))),
                }
            }
            Err(e) => error_message.set(Some(format!("Error loading portfolio for export: {}", e))),
        }
    };

    let delete_portfolio = move |id: String| {
        match LocalStorageManager::delete_portfolio(&id) {
            Ok(_) => {
                success_message.set(Some("Portfolio deleted successfully".to_string()));

                // Check if deleted portfolio was current และแก้ไขการ handle ด้วยการใช้ spawn
                spawn(async move {
                    if let Some(current) = props.current_portfolio.read().as_ref() {
                        if current.id == id {
                            props.on_delete_portfolio.call(());
                        }
                    }
                });

                // Refresh list
                if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                    portfolio_list.set(list);
                }
            }
            Err(e) => error_message.set(Some(format!("Error deleting portfolio: {}", e))),
        }
    };

    let import_portfolio = move |file_content: String| {
        match BrowserFileManager::import_portfolio_from_content(&file_content) {
            Ok(portfolio) => {
                match LocalStorageManager::save_portfolio(&portfolio) {
                    Ok(_) => {
                        props.on_portfolio_change.call(portfolio);
                        show_import_dialog.set(false);
                        success_message.set(Some("Portfolio imported successfully".to_string()));

                        // Refresh portfolio list
                        if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                            portfolio_list.set(list);
                        }
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Error saving imported portfolio: {}", e)));
                    }
                }
            }
            Err(e) => {
                error_message.set(Some(format!("Import error: {}", e)));
            }
        }
    };

    // ใช้ use_memo เพื่อป้องกัน borrow conflict
    let current_portfolio_id = use_memo(move || {
        props
            .current_portfolio
            .read()
            .as_ref()
            .map(|p| p.id.clone())
    });

    rsx! {
        div { class: "portfolio-manager",
            div { class: "portfolio-manager-content",
                div { class: "portfolio-header",
                    h2 { "Portfolio Manager" }
                    button {
                        class: "close-btn",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }

            // Messages
            {match success_message.read().as_ref() {
                Some(msg) => rsx! {
                    div { class: "message success",
                        "{msg}"
                        button {
                            onclick: move |_| success_message.set(None),
                            "✕"
                        }
                    }
                },
                None => rsx! { div {} }
            }}

            {match error_message.read().as_ref() {
                Some(msg) => rsx! {
                    div { class: "message error",
                        "{msg}"
                        button {
                            onclick: move |_| error_message.set(None),
                            "✕"
                        }
                    }
                },
                None => rsx! { div {} }
            }}

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
            {match show_create_form() {
                true => rsx! {
                    CreatePortfolioForm {
                        on_create: create_portfolio,
                        on_cancel: move |_| show_create_form.set(false),
                    }
                },
                false => rsx! { div {} }
            }}

            // Import Dialog
            {match show_import_dialog() {
                true => rsx! {
                    ImportPortfolioDialog {
                        on_import: import_portfolio,
                        on_cancel: move |_| show_import_dialog.set(false),
                    }
                },
                false => rsx! { div {} }
            }}

            // Portfolio List
            div { class: "portfolio-list-section",
                div { class: "portfolio-list",
                {match portfolio_list.read().is_empty() {
                    true => rsx! {
                        div { class: "empty-state",
                            "No saved portfolios. Create your first portfolio!"
                        }
                    },
                    false => rsx! {
                        for item in portfolio_list.read().iter() {
                            PortfolioCard {
                                key: "{item.id}",
                                portfolio_item: item.clone(),
                                is_current: current_portfolio_id().as_ref() == Some(&item.id),
                                on_load: load_portfolio,
                                on_export: export_portfolio,
                                on_delete: delete_portfolio,
                            }
                        }
                    }
                }}
                }
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
struct PortfolioCardProps {
    portfolio_item: PortfolioListItem,
    is_current: bool,
    on_load: EventHandler<String>,
    on_export: EventHandler<(String, ExportFormat)>,
    on_delete: EventHandler<String>,
}

#[component]
fn PortfolioCard(props: PortfolioCardProps) -> Element {
    let mut show_export_formats = use_signal(|| false);
    let card_class = if props.is_current {
        "portfolio-card current"
    } else {
        "portfolio-card"
    };

    rsx! {
        div {
            class: "{card_class}",
            onclick: {
                let portfolio_id = props.portfolio_item.id.clone();
                let on_load = props.on_load.clone();
                move |_| on_load.call(portfolio_id.clone())
            },

            div { class: "portfolio-info",
                h4 {
                    "{props.portfolio_item.name}"
                    {if props.is_current {
                        " (Current)"
                    } else {
                        ""
                    }}
                }
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
                // Export Button with Format Dropdown
                div { class: "export-button-container",
                    button {
                        class: "action-btn export-btn primary",
                        onclick: move |e: Event<MouseData>| {
                            e.stop_propagation();
                            show_export_formats.set(!show_export_formats());
                        },
                        "📥 Export"
                    }

                    {match show_export_formats() {
                        true => rsx! {
                            div { class: "export-format-menu",
                                button {
                                    class: "format-btn",
                                    onclick: {
                                        let portfolio_id = props.portfolio_item.id.clone();
                                        let on_export = props.on_export.clone();
                                        move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            on_export.call((portfolio_id.clone(), ExportFormat::JSON));
                                            show_export_formats.set(false);
                                        }
                                    },
                                    "📄 JSON"
                                }
                                button {
                                    class: "format-btn",
                                    onclick: {
                                        let portfolio_id = props.portfolio_item.id.clone();
                                        let on_export = props.on_export.clone();
                                        move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            on_export.call((portfolio_id.clone(), ExportFormat::CSV));
                                            show_export_formats.set(false);
                                        }
                                    },
                                    "📊 CSV"
                                }
                            }
                        },
                        false => rsx! { div {} }
                    }}
                }

                // Delete Button
                button {
                    class: "action-btn delete-btn danger",
                    onclick: {
                        let portfolio_id = props.portfolio_item.id.clone();
                        let portfolio_name = props.portfolio_item.name.clone();
                        let on_delete = props.on_delete.clone();
                        let is_current = props.is_current;
                        move |e: Event<MouseData>| {
                            e.stop_propagation();
                            let message = if is_current {
                                format!("Delete current portfolio '{}'? This will close the current portfolio.", portfolio_name)
                            } else {
                                format!("Delete portfolio '{}'?", portfolio_name)
                            };
                            let confirmed = web_sys::window()
                                .map(|w| w.confirm_with_message(&message).unwrap_or(false))
                                .unwrap_or(false);
                            if confirmed {
                                on_delete.call(portfolio_id.clone());
                            }
                        }
                    },
                    "🗑️ Delete"
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

    let handle_file_select = move |evt: Event<FormData>| {
        if let Some(files) = evt.files() {
            if let Some(file) = files.files().get(0) {
                is_loading.set(true);
                let file_name = file.clone();
                let on_import = props.on_import.clone();

                spawn(async move {
                    match files.read_file(&file_name).await {
                        Some(bytes) => {
                            let content = String::from_utf8_lossy(&bytes).to_string();
                            on_import.call(content);
                        }
                        None => {
                            // Error will be handled by parent component
                        }
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
                    div { class: "loading-state",
                        div { class: "spinner" }
                        p { "Importing portfolio..." }
                    }
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
