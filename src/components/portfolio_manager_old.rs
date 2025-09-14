// Portfolio management UI component
use dioxus::prelude::*;
use crate::models::{Portfolio, ExportFormat};
use crate::utils::{LocalStorageManager, PortfolioListItem, BrowserFileManager, StorageInfo};

#[derive(Props, Clone, PartialEq)]
pub struct PortfolioManagerProps {
    pub current_portfolio: Signal<Option<Portfolio>>,
    pub on_portfolio_change: EventHandler<Portfolio>,
   #[component]
fn PortfolioCardActions(props: PortfolioCardActionsProps) -> Element {
    let portfolio_id = props.portfolio_item.id.clone();
    let portfolio_name = props.portfolio_item.name.clone();
    
    if !props.show_actions {
        rsx! { div {} }
    } else {
        rsx! {
            div { class: "portfolio-actions-menu",
                button {
                    class: "action-btn export-btn",
                    onclick: {
                        let portfolio_id = portfolio_id.clone();
                        let on_export = props.on_export.clone();
                        let on_close = props.on_close.clone();
                        move |e| {
                            e.stop_propagation();
                            on_export.call((portfolio_id.clone(), ExportFormat::JSON));
                            on_close.call(());
                        }
                    },
                    "📄 Export JSON"
                }ortfolio: EventHandler<()>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn PortfolioManager(props: PortfolioManagerProps) -> Element {
    let mut portfolio_list = use_signal(|| Vec::<PortfolioListItem>::new());
    let mut show_create_form = use_signal(|| false);
    let mut show_import_dialog = use_signal(|| false);
    let mut show_export_dialog = use_signal(|| false);
    let mut storage_info = use_signal(|| None::<StorageInfo>);
    let mut error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);

    // Load portfolio list on mount
    use_effect(move || {
        match LocalStorageManager::get_portfolio_list() {
            Ok(list) => portfolio_list.set(list),
            Err(e) => error_message.set(Some(format!("Failed to load portfolios: {}", e))),
        }

        match LocalStorageManager::get_storage_info() {
            Ok(info) => storage_info.set(Some(info)),
            Err(_) => {}, // Non-critical error
        }
    });

    // Handle portfolio selection
    let mut handle_portfolio_select = move |portfolio_id: String| {
        match LocalStorageManager::load_portfolio(&portfolio_id) {
            Ok(portfolio) => {
                if let Err(e) = LocalStorageManager::set_current_portfolio_id(&portfolio_id) {
                    error_message.set(Some(format!("Failed to set current portfolio: {}", e)));
                } else {
                    props.on_portfolio_change.call(portfolio);
                    success_message.set(Some("Portfolio loaded successfully".to_string()));
                }
            },
            Err(e) => error_message.set(Some(format!("Failed to load portfolio: {}", e))),
        }
    };

    // Handle portfolio deletion
    let mut handle_portfolio_delete = move |portfolio_id: String| {
        if let Ok(current_id) = LocalStorageManager::get_current_portfolio_id() {
            if current_id == portfolio_id {
                error_message.set(Some("Cannot delete the currently active portfolio".to_string()));
                return;
            }
        }

        match LocalStorageManager::delete_portfolio(&portfolio_id) {
            Ok(_) => {
                // Refresh portfolio list
                if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                    portfolio_list.set(list);
                }
                success_message.set(Some("Portfolio deleted successfully".to_string()));
            },
            Err(e) => error_message.set(Some(format!("Failed to delete portfolio: {}", e))),
        }
    };

    // Handle export
    let mut handle_export_portfolio = move |portfolio_id: String, format: ExportFormat| {
        match LocalStorageManager::load_portfolio(&portfolio_id) {
            Ok(portfolio) => {
                match BrowserFileManager::export_portfolio_to_file(&portfolio, format) {
                    Ok(_) => success_message.set(Some("Portfolio exported successfully".to_string())),
                    Err(e) => error_message.set(Some(format!("Export failed: {}", e))),
                }
            },
            Err(e) => error_message.set(Some(format!("Failed to load portfolio for export: {}", e))),
        }
    };

    rsx! {
        div { class: "portfolio-manager",
            // Header with actions
            div { class: "portfolio-manager-header",
                h3 { "📁 Portfolio Management" }
                div { class: "portfolio-actions",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_create_form.set(true),
                        "➕ New Portfolio"
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| show_import_dialog.set(true),
                        "📥 Import"
                    }
                    button {
                        class: "btn btn-outline-secondary",
                        onclick: move |_| {
                            // Refresh portfolio list
                            if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                                portfolio_list.set(list);
                            }
                        },
                        "🔄 Refresh"
                    }
                }
            }

            // Messages
            if let Some(error) = error_message() {
                div { class: "alert alert-error",
                    span { "{error}" }
                    button {
                        class: "btn-close",
                        onclick: move |_| error_message.set(None),
                        "×"
                    }
                }
            }

            if let Some(success) = success_message() {
                div { class: "alert alert-success",
                    span { "{success}" }
                    button {
                        class: "btn-close",
                        onclick: move |_| success_message.set(None),
                        "×"
                    }
                }
            }

            // Storage info
            if let Some(info) = storage_info() {
                div { class: "storage-info",
                    span { class: "storage-stats",
                        "💾 {info.portfolio_count} portfolios • {info.total_size_mb():.1} MB used"
                    }
                }
            }

            // Portfolio list
            div { class: "portfolio-list",
                if portfolio_list().is_empty() {
                    div { class: "empty-state",
                        div { class: "empty-icon", "📁" }
                        h4 { "No Portfolios Yet" }
                        p { "Create your first portfolio to get started with payoff analysis." }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| show_create_form.set(true),
                            "Create Portfolio"
                        }
                    }
                } else {
                    for portfolio_item in portfolio_list() {
                        PortfolioCard {
                            portfolio_item: portfolio_item.clone(),
                            current_portfolio_id: props.current_portfolio.as_ref().map(|p| p.id.clone()),
                            on_select: move |id| handle_portfolio_select(id),
                            on_delete: move |id| handle_portfolio_delete(id),
                            on_export: move |(id, format)| handle_export_portfolio(id, format),
                        }
                    }
                }
            }

            // Create portfolio dialog
            if show_create_form() {
                CreatePortfolioDialog {
                    on_close: move |_| show_create_form.set(false),
                    on_create: move |portfolio| {
                        match LocalStorageManager::save_portfolio(&portfolio) {
                            Ok(_) => {
                                if let Err(e) = LocalStorageManager::set_current_portfolio_id(&portfolio.id) {
                                    error_message.set(Some(format!("Failed to set current portfolio: {}", e)));
                                } else {
                                    props.on_portfolio_change.call(portfolio);
                                    show_create_form.set(false);
                                    
                                    // Refresh list
                                    if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                                        portfolio_list.set(list);
                                    }
                                    success_message.set(Some("Portfolio created successfully".to_string()));
                                }
                            },
                            Err(e) => error_message.set(Some(format!("Failed to save portfolio: {}", e))),
                        }
                    }
                }
            }

            // Import dialog
            if show_import_dialog() {
                ImportPortfolioDialog {
                    on_close: move |_| show_import_dialog.set(false),
                    on_import: move |portfolio| {
                        match LocalStorageManager::save_portfolio(&portfolio) {
                            Ok(_) => {
                                props.on_portfolio_change.call(portfolio);
                                show_import_dialog.set(false);
                                
                                // Refresh list
                                if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                                    portfolio_list.set(list);
                                }
                                success_message.set(Some("Portfolio imported successfully".to_string()));
                            },
                            Err(e) => error_message.set(Some(format!("Failed to save imported portfolio: {}", e))),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PortfolioCardProps {
    portfolio_item: PortfolioListItem,
    current_portfolio_id: Option<String>,
    on_select: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_export: EventHandler<(String, ExportFormat)>,
}

#[component]
fn PortfolioCard(props: PortfolioCardProps) -> Element {
    let mut show_actions = use_signal(|| false);
    let is_current = props.current_portfolio_id.as_ref() == Some(&props.portfolio_item.id);
    let card_class = if is_current { "portfolio-card current" } else { "portfolio-card" };

    rsx! {
        div {
            class: "{card_class}",
            onclick: move |_| props.on_select.call(props.portfolio_item.id.clone()),
            
            div { class: "portfolio-card-header",
                div { class: "portfolio-info",
                    h4 { class: "portfolio-name",
                        "{props.portfolio_item.name}"
                    }
                }
                button {
                    class: "btn btn-sm btn-outline-secondary",
                    onclick: move |e| {
                        e.stop_propagation();
                        show_actions.set(!show_actions());
                    },
                    "⋮"
                }
            }

            div { class: "portfolio-meta",
                span { class: "position-count", "📊 {props.portfolio_item.position_count} positions" }
                span { class: "update-time", 
                    "Updated {props.portfolio_item.updated_at.format(\"%m/%d %H:%M\")}"
                }
            }

            div { class: "portfolio-actions",
                button {
                    class: "action-btn export-btn",
                    onclick: move |e| {
                        e.stop_propagation();
                        props.on_export.call((props.portfolio_item.id.clone(), ExportFormat::JSON));
                    },
                    "📤 Export JSON"
                }
                button {
                    class: "action-btn export-btn",
                    onclick: move |e| {
                        e.stop_propagation();
                        props.on_export.call((props.portfolio_item.id.clone(), ExportFormat::CSV));
                    },
                    "📊 Export CSV"
                }
                button {
                    class: "action-btn delete-btn",
                    onclick: move |e| {
                        e.stop_propagation();
                        let confirmed = web_sys::window()
                            .map(|w| w.confirm_with_message(&format!("Delete portfolio '{}'?", props.portfolio_item.name)).unwrap_or(false))
                            .unwrap_or(false);
                        if confirmed {
                            props.on_delete.call(props.portfolio_item.id.clone());
                        }
                    },
                    "🗑️ Delete"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PortfolioCardHeaderProps {
    portfolio_item: PortfolioListItem,
    is_current: bool,
    on_toggle_actions: EventHandler<()>,
}

#[component]
fn PortfolioCardHeader(props: PortfolioCardHeaderProps) -> Element {
    rsx! {
        div { class: "portfolio-card-header",
            div { class: "portfolio-info",
                h4 { class: "portfolio-name",
                    if props.is_current {
                        span { class: "current-indicator", "📌 " }
                    }
                    "{props.portfolio_item.name}"
                }
                if let Some(description) = &props.portfolio_item.description {
                    p { class: "portfolio-description", "{description}" }
                }
            }
            button {
                class: "btn btn-sm btn-outline-secondary",
                onclick: move |e| {
                    e.stop_propagation();
                    props.on_toggle_actions.call(());
                },
                "⋮"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PortfolioCardMetaProps {
    portfolio_item: PortfolioListItem,
}

#[component]
fn PortfolioCardMeta(props: PortfolioCardMetaProps) -> Element {
    rsx! {
        div { class: "portfolio-meta",
            span { class: "position-count", "{props.portfolio_item.position_count} positions" }
            span { class: "update-time", 
                {format!("Updated {}", props.portfolio_item.updated_at.format("%m/%d %H:%M"))}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PortfolioCardTagsProps {
    tags: Vec<String>,
}

#[component]
fn PortfolioCardTags(props: PortfolioCardTagsProps) -> Element {
    if props.tags.is_empty() {
        rsx! { div {} }
    } else {
        rsx! {
            div { class: "portfolio-tags",
                for tag in &props.tags {
                    span { class: "tag", "{tag}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PortfolioCardActionsProps {
    show_actions: bool,
    is_current: bool,
    portfolio_item: PortfolioListItem,
    on_export: EventHandler<(String, ExportFormat)>,
    on_delete: EventHandler<String>,
    on_close: EventHandler<()>,
}

#[component]
fn PortfolioCardActions(props: PortfolioCardActionsProps) -> Element {
    if !props.show_actions {
        rsx! { div {} }
    } else {
        rsx! {
            div { class: "portfolio-actions-menu",
                button {
                    class: "action-btn export-btn",
                    onclick: move |e| {
                        e.stop_propagation();
                        props.on_export.call((props.portfolio_item.id.clone(), ExportFormat::JSON));
                        props.on_close.call(());
                    },
                    "� Export JSON"
                }
                button {
                    class: "action-btn export-btn",
                    onclick: move |e| {
                        e.stop_propagation();
                        props.on_export.call((props.portfolio_item.id.clone(), ExportFormat::CSV));
                        props.on_close.call(());
                    },
                    "📊 Export CSV"
                }
                if !props.is_current {
                    button {
                        class: "action-btn delete-btn",
                        onclick: move |e| {
                            e.stop_propagation();
                            let confirmed = web_sys::window()
                                .map(|w| w.confirm_with_message(&format!("Delete portfolio '{}'?", props.portfolio_item.name)).unwrap_or(false))
                                .unwrap_or(false);
                            if confirmed {
                                props.on_delete.call(props.portfolio_item.id.clone());
                            }
                            props.on_close.call(());
                        },
                        "🗑️ Delete"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CreatePortfolioDialogProps {
    on_close: EventHandler<()>,
    on_create: EventHandler<Portfolio>,
}

#[component]
fn CreatePortfolioDialog(props: CreatePortfolioDialogProps) -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut tags = use_signal(|| String::new());

    let handle_create = move |_| {
        if name().trim().is_empty() {
            return;
        }

        let tag_list: Vec<String> = tags()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut portfolio = Portfolio::default();
        portfolio.name = name().trim().to_string();
        portfolio.description = if description().trim().is_empty() {
            None
        } else {
            Some(description().trim().to_string())
        };
        portfolio.tags = tag_list;

        props.on_create.call(portfolio);
    };

    rsx! {
        div { class: "dialog-overlay",
            div { class: "dialog create-portfolio-dialog",
                div { class: "dialog-header",
                    h3 { "Create New Portfolio" }
                    button {
                        class: "btn-close",
                        onclick: move |_| props.on_close.call(()),
                        "×"
                    }
                }

                div { class: "dialog-body",
                    div { class: "form-group",
                        label { "Portfolio Name *" }
                        input {
                            class: "form-control",
                            r#type: "text",
                            placeholder: "e.g., Options Strategy Portfolio",
                            value: "{name()}",
                            oninput: move |e| name.set(e.value()),
                            autofocus: true,
                        }
                    }

                    div { class: "form-group",
                        label { "Description" }
                        textarea {
                            class: "form-control",
                            placeholder: "Optional description...",
                            value: "{description()}",
                            oninput: move |e| description.set(e.value()),
                            rows: 3,
                        }
                    }

                    div { class: "form-group",
                        label { "Tags" }
                        input {
                            class: "form-control",
                            r#type: "text",
                            placeholder: "options, conservative, income (comma-separated)",
                            value: "{tags()}",
                            oninput: move |e| tags.set(e.value()),
                        }
                        small { class: "form-text", "Separate tags with commas" }
                    }
                }

                div { class: "dialog-footer",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| props.on_close.call(()),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: name().trim().is_empty(),
                        onclick: handle_create,
                        "Create Portfolio"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ImportPortfolioDialogProps {
    on_close: EventHandler<()>,
    on_import: EventHandler<Portfolio>,
}

#[component]
fn ImportPortfolioDialog(props: ImportPortfolioDialogProps) -> Element {
    let mut is_importing = use_signal(|| false);
    let mut import_error = use_signal(|| None::<String>);

    let handle_file_input = move |e: FormEvent| {
        if let Some(files) = e.files() {
            let file_list = files.files();
            if !file_list.is_empty() {
                let file_name = file_list[0].clone();
                is_importing.set(true);
                import_error.set(None);

                // Read and import file  
                spawn(async move {
                    match files.read_file(&file_name).await {
                        Some(bytes) => {
                            let content = String::from_utf8_lossy(&bytes).to_string();
                            match BrowserFileManager::import_portfolio_from_content(&content) {
                                Ok(portfolio) => props.on_import.call(portfolio),
                                Err(e) => import_error.set(Some(format!("Import failed: {}", e))),
                            }
                        },
                        None => import_error.set(Some("Failed to read file".to_string())),
                    }
                    is_importing.set(false);
                });
            }
        }
    };

    rsx! {
        div { class: "dialog-overlay",
            div { class: "dialog import-portfolio-dialog",
                div { class: "dialog-header",
                    h3 { "Import Portfolio" }
                    button {
                        class: "btn-close",
                        onclick: move |_| props.on_close.call(()),
                        "×"
                    }
                }

                div { class: "dialog-body",
                    if let Some(error) = import_error() {
                        div { class: "alert alert-error",
                            "{error}"
                        }
                    }

                    div { class: "import-area",
                        if is_importing() {
                            div { class: "importing-state",
                                div { class: "spinner" }
                                p { "Importing portfolio..." }
                            }
                        } else {
                            div { class: "file-input-area",
                                input {
                                    r#type: "file",
                                    accept: ".json,.csv",
                                    onchange: handle_file_input,
                                    id: "portfolio-file-input",
                                    style: "display: none",
                                }
                                label {
                                    r#for: "portfolio-file-input",
                                    class: "file-input-label",
                                    "📁 Choose File"
                                }
                                p { class: "file-help",
                                    "Supported formats: JSON, CSV"
                                }
                            }
                        }
                    }
                }

                div { class: "dialog-footer",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| props.on_close.call(()),
                        "Cancel"
                    }
                }
            }
        }
    }
}
