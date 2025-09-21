use crate::components::{ChartControls, PayoffChart, PortfolioManager, PositionForm, PositionList, ApiTester};
use crate::models::{Portfolio, Position};
use crate::utils::{AppSettings, LocalStorageManager};
use dioxus::prelude::*;

pub fn App() -> Element {
    // Portfolio management state
    let mut current_portfolio = use_signal(|| None::<Portfolio>);
    let mut show_portfolio_manager = use_signal(|| false);
    let mut app_settings = use_signal(|| AppSettings::default());

    // Chart state (derived from current portfolio)
    let mut price_start = use_signal(|| 0.0);
    let mut price_end = use_signal(|| 300.0);

    // Load app on startup
    use_effect(move || {
        // Load app settings
        match LocalStorageManager::get_app_settings() {
            Ok(settings) => app_settings.set(settings),
            Err(_) => {
                // Save default settings if none exist
                let default_settings = AppSettings::default();
                let _ = LocalStorageManager::save_app_settings(&default_settings);
                app_settings.set(default_settings);
            }
        }

        // Try to migrate legacy data first
        if let Ok(Some(migrated_portfolio)) = LocalStorageManager::migrate_legacy_data() {
            current_portfolio.set(Some(migrated_portfolio));
            return;
        }

        // Try to load current portfolio
        match LocalStorageManager::load_current_portfolio() {
            Ok(portfolio) => current_portfolio.set(Some(portfolio)),
            Err(_) => {
                // Check if there are any portfolios available
                if let Ok(list) = LocalStorageManager::get_portfolio_list() {
                    if !list.is_empty() {
                        // Load the most recently updated portfolio
                        if let Ok(portfolio) = LocalStorageManager::load_portfolio(&list[0].id) {
                            current_portfolio.set(Some(portfolio.clone()));
                            let _ = LocalStorageManager::set_current_portfolio_id(&portfolio.id);
                        }
                    } else {
                        // No portfolios exist, show portfolio manager
                        show_portfolio_manager.set(true);
                    }
                }
            }
        }
    });

    // Auto-save current portfolio
    let auto_save_effect = use_resource(move || {
        let settings = app_settings();
        let portfolio = current_portfolio();

        async move {
            if settings.auto_save_enabled {
                if let Some(portfolio) = portfolio {
                    // Auto-save every 30 seconds
                    gloo_timers::future::sleep(std::time::Duration::from_secs(
                        settings.auto_save_interval,
                    ))
                    .await;
                    let _ = LocalStorageManager::auto_save_portfolio(&portfolio);
                }
            }
        }
    });

    // Get current positions from portfolio
    let positions = use_memo(move || {
        current_portfolio()
            .map(|p| {
                p.positions
                    .iter()
                    .map(|ep| ep.position.clone())
                    .collect::<Vec<Position>>()
            })
            .unwrap_or_default()
    });

    // Auto-adjust price range based on positions
    let auto_range = use_memo(move || {
        let pos = positions();
        if pos.is_empty() {
            return (0.0, 300.0);
        }

        let mut min_relevant = f64::INFINITY;
        let mut max_relevant = f64::NEG_INFINITY;

        for position in pos {
            match position {
                Position::Option(ref option) => {
                    // Extend range around strike price
                    let range_padding = option.strike_price * 0.5; // 50% padding
                    min_relevant = min_relevant.min(option.strike_price - range_padding);
                    max_relevant = max_relevant.max(option.strike_price + range_padding);
                }
                Position::Spot(ref spot) => {
                    let range_padding = spot.entry_price * 0.3; // 30% padding
                    min_relevant = min_relevant.min(spot.entry_price - range_padding);
                    max_relevant = max_relevant.max(spot.entry_price + range_padding);
                }
                Position::Futures(ref futures) => {
                    let range_padding = futures.entry_price * 0.3; // 30% padding
                    min_relevant = min_relevant.min(futures.entry_price - range_padding);
                    max_relevant = max_relevant.max(futures.entry_price + range_padding);
                }
            }
        }

        // Ensure minimum range and floor at 0
        let start = (min_relevant.max(0.0)).max(0.0);
        let end = max_relevant.max(start + 100.0);

        (start, end)
    });

    // Update price range when positions change
    use_effect(move || {
        let (start, end) = auto_range();
        price_start.set(start);
        price_end.set(end);
    });

    // Portfolio handlers
    let handle_portfolio_selected = {
        let mut current_portfolio = current_portfolio;
        let mut show_portfolio_manager = show_portfolio_manager;
        move |portfolio: Portfolio| {
            spawn(async move {
                // Save as current portfolio
                let _ = LocalStorageManager::set_current_portfolio_id(&portfolio.id);
                current_portfolio.set(Some(portfolio));
                show_portfolio_manager.set(false);
            });
        }
    };

    let handle_portfolio_manager_close = {
        let mut show_portfolio_manager = show_portfolio_manager;
        move || {
            show_portfolio_manager.set(false);
        }
    };

    // Position handlers (modify current portfolio)
    let add_position = {
        let mut current_portfolio = current_portfolio;
        move |position: Position| {
            if let Some(mut portfolio) = current_portfolio() {
                portfolio.add_position(position);
                let _ = LocalStorageManager::save_portfolio(&portfolio);
                current_portfolio.set(Some(portfolio));
            }
        }
    };

    let remove_position = {
        let mut current_portfolio = current_portfolio;
        move |index: usize| {
            if let Some(mut portfolio) = current_portfolio() {
                if index < portfolio.positions.len() {
                    portfolio.positions.remove(index);
                    portfolio.update_timestamp();
                    let _ = LocalStorageManager::save_portfolio(&portfolio);
                    current_portfolio.set(Some(portfolio));
                }
            }
        }
    };

    let mut step_size = use_signal(|| 1.0);

    rsx! {
        div {
            class: "app-container",

            // Portfolio Manager Dialog
            if show_portfolio_manager() {
                PortfolioManager {
                    current_portfolio: current_portfolio,
                    on_portfolio_change: handle_portfolio_selected,
                    on_delete_portfolio: {
                        let mut current_portfolio = current_portfolio;
                        move |_| {
                            spawn(async move {
                                current_portfolio.set(None);
                            });
                        }
                    },
                    on_close: handle_portfolio_manager_close
                }
            }

            header {
                class: "app-header",
                div {
                    class: "header-content",
                    h1 { "Payoff Diagram Web Application" }
                    p { "Create and visualize payoff diagrams for financial positions" }
                }

                div {
                    class: "header-actions",
                    if let Some(portfolio) = current_portfolio() {
                        div {
                            class: "current-portfolio-info",
                            span { class: "portfolio-name", "{portfolio.name}" }
                            button {
                                class: "btn btn-outline",
                                onclick: move |_| show_portfolio_manager.set(true),
                                "Switch Portfolio"
                            }
                        }
                    } else {
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| show_portfolio_manager.set(true),
                            "Create Portfolio"
                        }
                    }
                }
            }

            // Only show main content if a portfolio is selected
            if current_portfolio().is_some() {
                main {
                    class: "app-main",

                // Chart and Position Form - Side by Side on Desktop, Stacked on Mobile
                div {
                    class: "chart-and-form-section responsive-layout",

                    // Chart Section - 9/12 width on desktop, full width on mobile (order: 1)
                    div {
                        class: "section chart-section-side mobile-order-1",
                        PayoffChart {
                            positions: positions.read().clone(),
                            price_start: price_start(),
                            price_end: price_end(),
                            step_size: step_size()
                        }
                    }

                    // Position Form - 3/12 width on desktop, full width on mobile (order: 2)
                    div {
                        class: "section position-form-side mobile-order-2",
                        PositionForm {
                            on_add_position: add_position
                        }
                    }
                }

                // Position List and Chart Controls - Grid Layout Below (order: 3 on mobile)
                div {
                    class: "app-grid-bottom mobile-order-3",

                    // Left Column: Position List Only
                    div {
                        class: "left-column",

                        div {
                            class: "section position-list-section",
                            PositionList {
                                positions: positions.read().clone(),
                                on_remove_position: remove_position,
                                on_update_position: move |(index, updated_position): (usize, Position)| {
                                    if let Some(mut portfolio) = current_portfolio() {
                                        if index < portfolio.positions.len() {
                                            portfolio.positions[index].position = updated_position;
                                            portfolio.update_timestamp();
                                            let _ = LocalStorageManager::save_portfolio(&portfolio);
                                            current_portfolio.set(Some(portfolio));
                                        }
                                    }
                                },
                                on_toggle_position: move |index: usize| {
                                    if let Some(mut portfolio) = current_portfolio() {
                                        if index < portfolio.positions.len() {
                                            portfolio.positions[index].position.toggle_active();
                                            portfolio.update_timestamp();
                                            let _ = LocalStorageManager::save_portfolio(&portfolio);
                                            current_portfolio.set(Some(portfolio));
                                        }
                                    }
                                },
                                on_clear_all: move |_| {
                                    if let Some(mut portfolio) = current_portfolio() {
                                        portfolio.positions.clear();
                                        portfolio.update_timestamp();
                                        let _ = LocalStorageManager::save_portfolio(&portfolio);
                                        current_portfolio.set(Some(portfolio));
                                    }
                                }
                            }
                        }
                    }

                    // Right Column: Chart Controls Only
                    div {
                        class: "right-column",

                        div {
                            class: "section chart-controls-section",
                            ChartControls {
                                price_start: price_start(),
                                price_end: price_end(),
                                step_size: step_size(),
                                on_price_range_change: move |(start, end): (f64, f64)| {
                                    price_start.set(start);
                                    price_end.set(end);
                                },
                                on_step_size_change: move |step: f64| {
                                    step_size.set(step);
                                },
                                on_calculate: move |_| {
                                    // Force re-render of chart
                                    // The chart will automatically update due to reactive signals
                                }
                            }
                        }
                    }
                }

                // Footer with helpful information
                div {
                    class: "app-info",
                    details {
                        class: "help-section",
                        summary { "ℹ️ How to use this application" }
                        div {
                            class: "help-content",
                            h4 { "Getting Started:" }
                            ol {
                                li { "Choose a position type (Spot, Option, or Futures)" }
                                li { "Fill in the required fields" }
                                li { "Click 'Add Position' to add it to your portfolio" }
                                li { "Adjust the price range and resolution as needed" }
                                li { "Click 'Calculate Payoff Diagram' to see the results" }
                            }

                            h4 { "Position Types:" }
                            ul {
                                li { strong { "Spot: " } "Direct ownership of an asset (stocks, commodities, etc.)" }
                                li { strong { "Option: " } "Call or Put options with strike price and premium" }
                                li { strong { "Futures: " } "Futures contracts with contract size multiplier" }
                            }

                            h4 { "Tips:" }
                            ul {
                                li { "Use negative quantities for short positions" }
                                li { "Start with simple positions to understand the payoff patterns" }
                                li { "Adjust the step size for smoother or more detailed charts" }
                                li { "Breakeven points show where profit/loss crosses zero" }
                            }
                        }
                    }
                }
                }
            } else {
                // No portfolio selected - show welcome message
                div {
                    class: "no-portfolio-message",
                    div {
                        class: "welcome-content",
                        h2 { "Welcome to Payoff Diagram Web" }
                        p { "Create or import a portfolio to start building payoff diagrams." }
                        button {
                            class: "btn btn-primary btn-lg",
                            onclick: move |_| show_portfolio_manager.set(true),
                            "Get Started"
                        }
                    }
                }

                // Temporary API Tester (for development)
                div {
                    style: "margin-top: 2rem; padding: 1rem; border: 2px dashed #e74c3c; border-radius: 8px;",
                    h3 { style: "color: #e74c3c; margin-bottom: 1rem;", "🧪 Development: API Tester" }
                    ApiTester {}
                }
            }

            footer {
                class: "app-footer",
                p { "Built with Rust + Dioxus + WebAssembly | © 2025 Payoff Diagram Web" }
            }
        }
    }
}
