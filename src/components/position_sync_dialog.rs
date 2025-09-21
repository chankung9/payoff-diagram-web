use dioxus::prelude::*;
use crate::{
    models::{
        Position, Portfolio, SpotPosition, FuturesPosition, OptionPosition,
        position::OptionType,
        api_keys::ApiKey,
        import_data::{ImportMode, AssetSelection}
    },
    utils::LocalStorageManager
};
use serde::{Deserialize, Serialize};

// Binance API response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceBalance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceAccountInfo {
    pub balances: Vec<BinanceBalance>,
}

// Available trading pairs
#[derive(Debug, Clone, PartialEq)]
pub struct TradingPair {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub display_name: String,
}

impl TradingPair {
    pub fn new(symbol: &str, base: &str, quote: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            base_asset: base.to_string(),
            quote_asset: quote.to_string(),
            display_name: format!("{}/{}", base, quote),
        }
    }
}

// Simple sync result for now
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub positions: Vec<Position>,
    pub success: bool,
    pub message: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct PositionSyncDialogProps {
    pub api_key: ApiKey,
    pub current_portfolio: Option<Signal<Option<Portfolio>>>,
    pub on_close: EventHandler<()>,
    pub on_positions_imported: EventHandler<Vec<Position>>,
}

#[component]
pub fn PositionSyncDialog(props: PositionSyncDialogProps) -> Element {
    // UI state
    let mut step = use_signal(|| 1);
    let mut import_mode = use_signal(|| ImportMode::Append);
    let mut asset_selection = use_signal(|| AssetSelection::default());
    let mut selected_pairs = use_signal(|| vec!["SOLUSDT".to_string(), "SOLUSDC".to_string()]);
    let mut is_connecting = use_signal(|| false);
    let mut is_importing = use_signal(|| false);
    let mut connection_result = use_signal(|| None::<String>);
    let mut import_result = use_signal(|| None::<SyncResult>);

    // Available trading pairs (starting with Solana)
    let available_pairs = use_signal(|| vec![
        TradingPair::new("SOLUSDT", "SOL", "USDT"),
        TradingPair::new("SOLUSDC", "SOL", "USDC"),
        TradingPair::new("SOLBUSD", "SOL", "BUSD"),
        TradingPair::new("SOLBTC", "SOL", "BTC"),
        TradingPair::new("SOLETH", "SOL", "ETH"),
    ]);

    // Test connection using real API
    let api_key_for_test = props.api_key.clone();
    let handle_test_connection = move |_: MouseEvent| {
        is_connecting.set(true);
        connection_result.set(None);
        
        let api_key = api_key_for_test.clone();
        spawn(async move {
            match test_binance_connection(&api_key).await {
                Ok(_) => {
                    connection_result.set(Some("✅ Connection successful!".to_string()));
                }
                Err(e) => {
                    connection_result.set(Some(format!("❌ Connection failed: {}", e)));
                }
            }
            is_connecting.set(false);
        });
    };

    // Start import using real API
    let api_key_for_import = props.api_key.clone();
    let handle_start_import = move |_: MouseEvent| {
        is_importing.set(true);
        import_result.set(None);
        
        let api_key = api_key_for_import.clone();
        let pairs = selected_pairs.read().clone();
        spawn(async move {
            match import_positions_from_binance(&api_key, &pairs).await {
                Ok(positions) => {
                    let result = SyncResult {
                        positions: positions.clone(),
                        success: true,
                        message: format!("Successfully imported {} positions", positions.len()),
                    };
                    import_result.set(Some(result));
                }
                Err(e) => {
                    let result = SyncResult {
                        positions: vec![],
                        success: false,
                        message: format!("Import failed: {}", e),
                    };
                    import_result.set(Some(result));
                }
            }
            is_importing.set(false);
        });
    };

    // Complete import
    let handle_complete_import = move |_: MouseEvent| {
        if let Some(result) = import_result.read().as_ref() {
            if result.success {
                props.on_positions_imported.call(result.positions.clone());
            }
        }
    };

    let api_key_name = props.api_key.name.clone();
    let api_key_platform = props.api_key.platform.clone();

    rsx! {
        div { class: "position-sync-dialog-overlay",
            div { class: "position-sync-dialog",
                // Header
                div { class: "dialog-header",
                    h2 { "Sync Positions from {api_key_name}" }
                    button { 
                        class: "close-btn",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }

                // Progress indicator
                div { class: "sync-progress",
                    div { class: if *step.read() >= 1 { "step active" } else { "step" }, "1. Connection" }
                    div { class: if *step.read() >= 2 { "step active" } else { "step" }, "2. Configuration" }
                    div { class: if *step.read() >= 3 { "step active" } else { "step" }, "3. Import" }
                    div { class: if *step.read() >= 4 { "step active" } else { "step" }, "4. Complete" }
                }

                // Content based on step
                div { class: "dialog-content",
                    match *step.read() {
                        1 => rsx! {
                            div { class: "step-content",
                                h3 { "Test API Connection" }
                                p { "First, let's verify that we can connect to {api_key_platform:?} with your API key." }
                                
                                div { class: "connection-status",
                                    if *is_connecting.read() {
                                        div { class: "loading", "🔄 Testing connection..." }
                                    } else if let Some(result) = connection_result.read().as_ref() {
                                        div { class: "result success", "{result}" }
                                    } else {
                                        div { class: "instruction", "Click test to verify your API credentials." }
                                    }
                                }

                                div { class: "step-actions",
                                    button {
                                        class: "btn secondary",
                                        onclick: move |_| props.on_close.call(()),
                                        "Cancel"
                                    }
                                    button {
                                        class: "btn primary",
                                        disabled: *is_connecting.read(),
                                        onclick: handle_test_connection,
                                        if *is_connecting.read() { "Testing..." } else { "Test Connection" }
                                    }
                                    if connection_result.read().is_some() {
                                        button {
                                            class: "btn primary",
                                            onclick: move |_| step.set(2),
                                            "Next"
                                        }
                                    }
                                }
                            }
                        },
                        2 => rsx! {
                            div { class: "step-content",
                                h3 { "Configure Import" }
                                
                                div { class: "config-section",
                                    label { "Import Mode:" }
                                    select {
                                        value: "{import_mode.read():?}",
                                        onchange: move |e| {
                                            match e.value().as_str() {
                                                "Append" => import_mode.set(ImportMode::Append),
                                                "Replace" => import_mode.set(ImportMode::Replace),
                                                _ => {}
                                            }
                                        },
                                        option { value: "Append", "Append to current portfolio" }
                                        option { value: "Replace", "Replace current portfolio" }
                                    }
                                }

                                div { class: "config-section",
                                    label { "Trading Pairs:" }
                                    p { class: "section-description", "Select which trading pairs to import:" }
                                    div { class: "pair-selection",
                                        for (index, pair) in available_pairs.read().iter().enumerate() {
                                            div { 
                                                key: "{index}",
                                                class: "checkbox-item",
                                                input {
                                                    r#type: "checkbox",
                                                    id: "pair_{pair.symbol}",
                                                    checked: selected_pairs.read().contains(&pair.symbol),
                                                    onchange: {
                                                        let symbol = pair.symbol.clone();
                                                        move |e: Event<FormData>| {
                                                            let checked = e.value().parse::<bool>().unwrap_or(false);
                                                            let mut pairs = selected_pairs.read().clone();
                                                            if checked {
                                                                if !pairs.contains(&symbol) {
                                                                    pairs.push(symbol.clone());
                                                                }
                                                            } else {
                                                                pairs.retain(|p| p != &symbol);
                                                            }
                                                            selected_pairs.set(pairs);
                                                        }
                                                    }
                                                }
                                                label {
                                                    r#for: "pair_{pair.symbol}",
                                                    "{pair.display_name} ({pair.symbol})"
                                                }
                                            }
                                        }
                                    }
                                }

                                div { class: "config-section",
                                    label { "Asset Types:" }
                                    select {
                                        onchange: move |e| {
                                            match e.value().as_str() {
                                                "all" => asset_selection.set(AssetSelection {
                                                    spot_positions: true,
                                                    futures_positions: true,
                                                    options_positions: true,
                                                    open_orders: false,
                                                }),
                                                "spot" => asset_selection.set(AssetSelection {
                                                    spot_positions: true,
                                                    futures_positions: false,
                                                    options_positions: false,
                                                    open_orders: false,
                                                }),
                                                "futures" => asset_selection.set(AssetSelection {
                                                    spot_positions: false,
                                                    futures_positions: true,
                                                    options_positions: false,
                                                    open_orders: false,
                                                }),
                                                "options" => asset_selection.set(AssetSelection {
                                                    spot_positions: false,
                                                    futures_positions: false,
                                                    options_positions: true,
                                                    open_orders: false,
                                                }),
                                                _ => {}
                                            }
                                        },
                                        option { value: "all", "All asset types" }
                                        option { value: "spot", "Spot positions only" }
                                        option { value: "futures", "Futures positions only" }
                                        option { value: "options", "Options positions only" }
                                    }
                                }

                                div { class: "step-actions",
                                    button {
                                        class: "btn secondary",
                                        onclick: move |_| step.set(1),
                                        "Back"
                                    }
                                    button {
                                        class: "btn primary",
                                        disabled: selected_pairs.read().is_empty(),
                                        onclick: move |_| step.set(3),
                                        "Continue"
                                    }
                                }
                            }
                        },
                        3 => rsx! {
                            div { class: "step-content",
                                h3 { "Import Positions" }
                                
                                if *is_importing.read() {
                                    div { class: "import-progress",
                                        div { class: "loading", "📊 Importing positions..." }
                                        div { class: "progress-details", "Fetching data from {api_key_platform:?}..." }
                                    }
                                } else if let Some(result) = import_result.read().as_ref() {
                                    div { class: "import-result",
                                        if result.success {
                                            div { class: "result success",
                                                "✅ {result.message}"
                                            }
                                            div { class: "position-preview",
                                                h4 { "Imported Positions:" }
                                                for position in &result.positions {
                                                    div { class: "position-item",
                                                        match position {
                                                            Position::Spot(spot) => rsx! {
                                                                span { class: "position-type", "💰 " }
                                                                span { "{spot.description}: {spot.quantity}" }
                                                            },
                                                            Position::Futures(futures) => rsx! {
                                                                span { class: "position-type", "📈 " }
                                                                span { "{futures.description}: {futures.quantity} " }
                                                            },
                                                            Position::Option(option) => rsx! {
                                                                span { class: "position-type", "🎯 " }
                                                                span { "{option.description}: {option.quantity}" }
                                                            },
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            div { class: "result error",
                                                "❌ Import failed: {result.message}"
                                            }
                                        }
                                    }
                                } else {
                                    div { class: "import-instruction",
                                        p { "Ready to import positions from {props.api_key.platform:?}?" }
                                        p { "Mode: {import_mode.read():?}" }
                                        p { "Assets: Mixed configuration" }
                                    }
                                }

                                div { class: "step-actions",
                                    button {
                                        class: "btn secondary",
                                        onclick: move |_| step.set(2),
                                        "Back"
                                    }
                                    if import_result.read().is_none() {
                                        button {
                                            class: "btn primary",
                                            disabled: *is_importing.read(),
                                            onclick: handle_start_import,
                                            if *is_importing.read() { "Importing..." } else { "Start Import" }
                                        }
                                    } else if let Some(result) = import_result.read().as_ref() {
                                        if result.success {
                                            button {
                                                class: "btn primary",
                                                onclick: handle_complete_import,
                                                "Complete Import"
                                            }
                                        } else {
                                            button {
                                                class: "btn primary",
                                                onclick: move |_| {
                                                    import_result.set(None);
                                                },
                                                "Retry"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => rsx! {
                            div { class: "step-content",
                                h3 { "Import Complete" }
                                p { "Positions have been successfully imported to your portfolio!" }
                                
                                div { class: "step-actions",
                                    button {
                                        class: "btn primary",
                                        onclick: move |_| props.on_close.call(()),
                                        "Close"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        style { r"
            .position-sync-dialog-overlay {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.5);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 1000;
            }}

            .position-sync-dialog {{
                background: white;
                border-radius: 8px;
                width: 90%;
                max-width: 600px;
                max-height: 80vh;
                overflow-y: auto;
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
            }}

            .dialog-header {{
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 1rem 1.5rem;
                border-bottom: 1px solid #e5e7eb;
            }}

            .dialog-header h2 {{
                margin: 0;
                color: #1f2937;
            }}

            .close-btn {{
                background: none;
                border: none;
                font-size: 1.5rem;
                cursor: pointer;
                color: #6b7280;
            }}

            .close-btn:hover {{
                color: #ef4444;
            }}

            .sync-progress {{
                display: flex;
                justify-content: space-between;
                padding: 1rem 1.5rem;
                background: #f9fafb;
                border-bottom: 1px solid #e5e7eb;
            }}

            .sync-progress .step {{
                flex: 1;
                text-align: center;
                padding: 0.5rem;
                color: #6b7280;
                font-size: 0.875rem;
            }}

            .sync-progress .step.active {{
                color: #3b82f6;
                font-weight: 600;
            }}

            .dialog-content {{
                padding: 1.5rem;
            }}

            .step-content h3 {{
                margin: 0 0 1rem 0;
                color: #1f2937;
            }}

            .connection-status, .import-result {{
                padding: 1rem;
                border-radius: 6px;
                margin: 1rem 0;
            }}

            .result.success {{
                background: #dcfce7;
                color: #166534;
                border: 1px solid #bbf7d0;
            }}

            .result.error {{
                background: #fef2f2;
                color: #dc2626;
                border: 1px solid #fecaca;
            }}

            .loading {{
                color: #3b82f6;
                font-weight: 500;
            }}

            .config-section {{
                margin: 1rem 0;
            }}

            .config-section label {{
                display: block;
                margin-bottom: 0.5rem;
                font-weight: 500;
                color: #374151;
            }}

            .config-section select {{
                width: 100%;
                padding: 0.5rem;
                border: 1px solid #d1d5db;
                border-radius: 4px;
                background: white;
            }}

            .position-preview {{
                margin-top: 1rem;
            }}

            .position-preview h4 {{
                margin: 0 0 0.5rem 0;
                color: #1f2937;
            }}

            .position-item {{
                display: flex;
                align-items: center;
                padding: 0.5rem;
                background: #f3f4f6;
                border-radius: 4px;
                margin: 0.25rem 0;
            }}

            .position-type {{
                margin-right: 0.5rem;
            }}

            .step-actions {{
                display: flex;
                gap: 1rem;
                justify-content: flex-end;
                margin-top: 2rem;
                padding-top: 1rem;
                border-top: 1px solid #e5e7eb;
            }}

            .btn {{
                padding: 0.5rem 1rem;
                border-radius: 4px;
                border: none;
                cursor: pointer;
                font-weight: 500;
                transition: all 0.2s;
            }}

            .btn.primary {{
                background: #3b82f6;
                color: white;
            }}

            .btn.primary:hover:not(:disabled) {{
                background: #2563eb;
            }}

            .btn.secondary {{
                background: #f3f4f6;
                color: #374151;
            }}

            .btn.secondary:hover {{
                background: #e5e7eb;
            }}

            .btn:disabled {{
                opacity: 0.5;
                cursor: not-allowed;
            }}

            .pair-selection {{
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
                margin-top: 0.5rem;
            }}

            .checkbox-item {{
                display: flex;
                align-items: center;
                gap: 0.5rem;
                padding: 0.5rem;
                border: 1px solid #e5e7eb;
                border-radius: 4px;
                background: #f9fafb;
            }}

            .checkbox-item input[type="checkbox"] {{
                margin: 0;
            }}

            .section-description {{
                font-size: 0.875rem;
                color: #6b7280;
                margin: 0.25rem 0;
            }}
        " }
    }
}

// API Functions
async fn test_binance_connection(api_key: &ApiKey) -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let url = "http://127.0.0.1:3001/api/binance/account";
    
    let request_body = serde_json::json!({
        "api_key": api_key.api_key,
        "secret_key": api_key.encrypted_secret
    });
    
    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(format!("API error {}: {}", status, error_text))
    }
}

async fn import_positions_from_binance(api_key: &ApiKey, pairs: &[String]) -> Result<Vec<Position>, String> {
    let client = reqwest::Client::new();
    let mut all_positions = Vec::new();
    
    let request_body = serde_json::json!({
        "api_key": api_key.api_key,
        "secret_key": api_key.encrypted_secret
    });

    // 1. Import Spot Positions
    match import_spot_positions(&client, &request_body, pairs).await {
        Ok(mut positions) => all_positions.append(&mut positions),
        Err(e) => println!("Failed to import spot positions: {}", e),
    }

    // 2. Import Futures Positions
    match import_futures_positions(&client, &request_body, pairs).await {
        Ok(mut positions) => all_positions.append(&mut positions),
        Err(e) => println!("Failed to import futures positions: {}", e),
    }

    // 3. Import Options Positions
    match import_options_positions(&client, &request_body, pairs).await {
        Ok(mut positions) => all_positions.append(&mut positions),
        Err(e) => println!("Failed to import options positions: {}", e),
    }

    if all_positions.is_empty() {
        return Err("No positions found across any trading types".to_string());
    }

    Ok(all_positions)
}

async fn import_spot_positions(
    client: &reqwest::Client,
    request_body: &serde_json::Value,
    pairs: &[String],
) -> Result<Vec<Position>, String> {
    let account_url = "http://127.0.0.1:3001/api/binance/account";
    
    let response = client
        .post(account_url)
        .json(request_body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Spot API error {}: {}", status, error_text));
    }

    let proxy_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse spot response: {}", e))?;

    let account_data = proxy_response["data"]
        .as_object()
        .ok_or("Invalid spot response format")?;

    let balances = account_data["balances"]
        .as_array()
        .ok_or("No spot balances found")?;

    let mut positions = Vec::new();
    
    for balance_value in balances {
        if let Some(balance_obj) = balance_value.as_object() {
            let asset = balance_obj["asset"].as_str().unwrap_or_default();
            let free_str = balance_obj["free"].as_str().unwrap_or("0");
            let locked_str = balance_obj["locked"].as_str().unwrap_or("0");
            
            let free: f64 = free_str.parse().unwrap_or(0.0);
            let locked: f64 = locked_str.parse().unwrap_or(0.0);
            let total = free + locked;
            
            // Check if this asset matches any of our selected pairs
            for pair in pairs {
                if pair.starts_with(asset) && total > 0.001 {
                    // Get current price for this pair
                    if let Ok(price) = get_current_price(pair).await {
                        let position = Position::Spot(SpotPosition::new(
                            total,
                            price,
                            Some(pair.clone()),
                        ));
                        positions.push(position);
                    }
                    break;
                }
            }
        }
    }

    Ok(positions)
}

async fn import_futures_positions(
    client: &reqwest::Client,
    request_body: &serde_json::Value,
    pairs: &[String],
) -> Result<Vec<Position>, String> {
    let futures_url = "http://127.0.0.1:3001/api/binance/futures/positions";
    
    let response = client
        .post(futures_url)
        .json(request_body)
        .send()
        .await
        .map_err(|e| format!("Futures network error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Futures API error {}: {}", status, error_text));
    }

    let proxy_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse futures response: {}", e))?;

    let positions_data = proxy_response["data"]
        .as_array()
        .ok_or("Invalid futures response format")?;

    let mut positions = Vec::new();
    
    for position_value in positions_data {
        if let Some(position_obj) = position_value.as_object() {
            let symbol = position_obj["symbol"].as_str().unwrap_or_default();
            let position_amt_str = position_obj["positionAmt"].as_str().unwrap_or("0");
            let entry_price_str = position_obj["entryPrice"].as_str().unwrap_or("0");
            let position_side = position_obj["positionSide"].as_str().unwrap_or("BOTH");
            let unrealized_profit_str = position_obj["unRealizedProfit"].as_str().unwrap_or("0");
            
            let position_amt: f64 = position_amt_str.parse().unwrap_or(0.0);
            let entry_price: f64 = entry_price_str.parse().unwrap_or(0.0);
            let unrealized_profit: f64 = unrealized_profit_str.parse().unwrap_or(0.0);
            
            // Check if this symbol matches any of our selected pairs and has open position
            if position_amt.abs() > 0.000001 && pairs.iter().any(|pair| pair == symbol) {
                // Determine side from positionAmt and positionSide
                let side = if position_side == "LONG" || (position_side == "BOTH" && position_amt > 0.0) {
                    "Long"
                } else {
                    "Short"
                };
                
                let position = Position::Futures(FuturesPosition::new(
                    position_amt.abs(),
                    entry_price,
                    1.0, // contract_size
                    Some(symbol.to_string()),
                ));
                positions.push(position);
            }
        }
    }

    Ok(positions)
}

async fn import_options_positions(
    client: &reqwest::Client,
    request_body: &serde_json::Value,
    pairs: &[String],
) -> Result<Vec<Position>, String> {
    let options_url = "http://127.0.0.1:3001/api/binance/options/positions";
    
    let response = client
        .post(options_url)
        .json(request_body)
        .send()
        .await
        .map_err(|e| format!("Options network error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Options API error {}: {}", status, error_text));
    }

    let proxy_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse options response: {}", e))?;

    let positions_data = proxy_response["data"]
        .as_array()
        .ok_or("Invalid options response format")?;

    let mut positions = Vec::new();
    
    for position_value in positions_data {
        if let Some(position_obj) = position_value.as_object() {
            let symbol = position_obj["symbol"].as_str().unwrap_or_default();
            let quantity_str = position_obj["quantity"].as_str().unwrap_or("0");
            let strike_price_str = position_obj["strikePrice"].as_str().unwrap_or("0");
            let option_type = position_obj["side"].as_str().unwrap_or_default();
            
            let quantity: f64 = quantity_str.parse().unwrap_or(0.0);
            let strike_price: f64 = strike_price_str.parse().unwrap_or(0.0);
            
            // Check if this symbol matches any of our selected pairs and has open position
            if quantity > 0.0 && pairs.iter().any(|pair| symbol.contains(&pair.replace("USDT", ""))) {
                // Determine option type from side string
                let opt_type = if option_type.to_lowercase().contains("call") { 
                    OptionType::Call 
                } else { 
                    OptionType::Put 
                };
                
                let position = Position::Option(OptionPosition::new(
                    opt_type,
                    quantity,
                    strike_price,
                    0.0, // premium - default to 0 since we don't have this data
                    Some(symbol.to_string()),
                ));
                positions.push(position);
            }
        }
    }

    Ok(positions)
}

async fn get_current_price(symbol: &str) -> Result<f64, String> {
    let client = reqwest::Client::new();
    
    let url = format!("http://127.0.0.1:3001/api/binance/ticker/price?symbol={}", symbol);
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err("Failed to get price".to_string());
    }

    // Parse price from response
    let price_data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse price response: {}", e))?;

    let price_str = price_data["price"]
        .as_str()
        .ok_or("Price not found in response")?;

    price_str.parse::<f64>()
        .map_err(|e| format!("Failed to parse price: {}", e))
}
