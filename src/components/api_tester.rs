use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Debug, Clone, Deserialize)]
struct BinanceAccountInfo {
    #[serde(rename = "makerCommission")]
    maker_commission: i32,
    #[serde(rename = "takerCommission")]
    taker_commission: i32,
    #[serde(rename = "buyerCommission")]
    buyer_commission: i32,
    #[serde(rename = "sellerCommission")]
    seller_commission: i32,
    #[serde(rename = "canTrade")]
    can_trade: bool,
    #[serde(rename = "canWithdraw")]
    can_withdraw: bool,
    #[serde(rename = "canDeposit")]
    can_deposit: bool,
    #[serde(rename = "accountType")]
    account_type: String,
    balances: Vec<BinanceBalance>,
}

#[derive(Debug, Clone, Deserialize)]
struct BinanceBalance {
    asset: String,
    free: String,
    locked: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ProxyResponse {
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
enum TestResult {
    NotTested,
    Testing,
    Success(String),
    Error(String),
}

#[component]
pub fn ApiTester() -> Element {
    let mut api_key = use_signal(|| String::new());
    let mut secret_key = use_signal(|| String::new());
    let mut selected_symbol = use_signal(|| "SOLUSDT".to_string());
    let mut test_result = use_signal(|| TestResult::NotTested);

    // Connection test handler
    let test_connection = move |_| {
        let api_key_val = api_key().clone();
        let secret_key_val = secret_key().clone();

        if api_key_val.is_empty() {
            test_result.set(TestResult::Error("API Key is required".to_string()));
            return;
        }

        test_result.set(TestResult::Testing);

        spawn(async move {
            match test_binance_api(&api_key_val, &secret_key_val).await {
                Ok(info) => {
                    let success_msg = format!(
                        "✅ Connection successful!\nAccount Type: {}\nCan Trade: {}\nBalances: {} assets",
                        info.account_type,
                        info.can_trade,
                        info.balances.len()
                    );
                    test_result.set(TestResult::Success(success_msg));
                }
                Err(e) => {
                    test_result.set(TestResult::Error(format!("❌ Error: {}", e)));
                }
            }
        });
    };

    rsx! {
        div { class: "api-tester",
            h2 { "🧪 Binance API Tester" }

            div { class: "api-form",
                div { class: "form-group",
                    label { "API Key:" }
                    input {
                        r#type: "password",
                        value: "{api_key()}",
                        placeholder: "Your Binance API Key",
                        onchange: move |event| api_key.set(event.value().clone())
                    }
                }

                div { class: "form-group",
                    label { "Secret Key:" }
                    input {
                        r#type: "password",
                        value: "{secret_key()}",
                        placeholder: "Your Binance Secret Key",
                        onchange: move |event| secret_key.set(event.value().clone())
                    }
                }

                div { class: "form-group",
                    label { "Symbol:" }
                    select {
                        value: "{selected_symbol()}",
                        onchange: move |event| selected_symbol.set(event.value().clone()),
                        option { value: "SOLUSDT", "SOL/USDT" }
                        option { value: "BTCUSDT", "BTC/USDT" }
                        option { value: "ETHUSDT", "ETH/USDT" }
                    }
                }

                div { class: "test-buttons",
                    button {
                        class: "test-btn",
                        onclick: test_connection,
                        disabled: matches!(test_result(), TestResult::Testing),
                        if matches!(test_result(), TestResult::Testing) {
                            "🔄 Testing Connection..."
                        } else {
                            "🔗 Test Connection"
                        }
                    }

                    button {
                        class: "test-btn orders-btn",
                        onclick: move |_| {
                            let api_key_val = api_key().clone();
                            let secret_key_val = secret_key().clone();
                            let symbol_val = selected_symbol().clone();

                            if api_key_val.is_empty() {
                                test_result.set(TestResult::Error("API Key is required".to_string()));
                                return;
                            }

                            test_result.set(TestResult::Testing);

                            spawn(async move {
                                match get_orders(&api_key_val, &secret_key_val, &symbol_val).await {
                                    Ok(orders) => {
                                        let success_msg = format!(
                                            "✅ Orders retrieved!\nSymbol: {}\nTotal Orders: {}\nRecent orders:\n{}",
                                            symbol_val,
                                            orders.len(),
                                            orders.iter().take(3)
                                                .map(|o| format!("- {} {} qty: {}",
                                                    o.get("side").unwrap_or(&serde_json::Value::String("Unknown".to_string())),
                                                    o.get("symbol").unwrap_or(&serde_json::Value::String("Unknown".to_string())),
                                                    o.get("origQty").unwrap_or(&serde_json::Value::String("0".to_string()))
                                                ))
                                                .collect::<Vec<_>>()
                                                .join("\n")
                                        );
                                        test_result.set(TestResult::Success(success_msg));
                                    }
                                    Err(e) => {
                                        test_result.set(TestResult::Error(format!("❌ Orders Error: {}", e)));
                                    }
                                }
                            });
                        },
                        disabled: matches!(test_result(), TestResult::Testing),
                        if matches!(test_result(), TestResult::Testing) {
                            "🔄 Testing Orders..."
                        } else {
                            "🌟 Test Orders"
                        }
                    }

                    button {
                        class: "test-btn options-btn",
                        onclick: move |_| {
                            let api_key_val = api_key().clone();
                            let secret_key_val = secret_key().clone();

                            if api_key_val.is_empty() {
                                test_result.set(TestResult::Error("API Key is required".to_string()));
                                return;
                            }

                            test_result.set(TestResult::Testing);

                            spawn(async move {
                                match get_options_positions(&api_key_val, &secret_key_val).await {
                                    Ok(positions) => {
                                        let success_msg = format!(
                                            "✅ Options Positions retrieved!\nTotal Positions: {}\nActive positions:\n{}",
                                            positions.len(),
                                            positions.iter().take(5)
                                                .filter(|p| p.get("quantity").and_then(|q| q.as_str()).unwrap_or("0") != "0")
                                                .map(|p| format!("- {} qty: {}",
                                                    p.get("symbol").unwrap_or(&serde_json::Value::String("Unknown".to_string())),
                                                    p.get("quantity").unwrap_or(&serde_json::Value::String("0".to_string()))
                                                ))
                                                .collect::<Vec<_>>()
                                                .join("\n")
                                        );
                                        test_result.set(TestResult::Success(success_msg));
                                    }
                                    Err(e) => {
                                        test_result.set(TestResult::Error(format!("❌ Options Error: {}", e)));
                                    }
                                }
                            });
                        },
                        disabled: matches!(test_result(), TestResult::Testing),
                        if matches!(test_result(), TestResult::Testing) {
                            "🔄 Testing Options..."
                        } else {
                            "🎯 Test Options Positions"
                        }
                    }

                    button {
                        class: "test-btn futures-btn",
                        onclick: move |_| {
                            let api_key_val = api_key().clone();
                            let secret_key_val = secret_key().clone();

                            if api_key_val.is_empty() {
                                test_result.set(TestResult::Error("API Key is required".to_string()));
                                return;
                            }

                            test_result.set(TestResult::Testing);

                            spawn(async move {
                                match get_futures_positions(&api_key_val, &secret_key_val).await {
                                    Ok(positions) => {
                                        let success_msg = format!(
                                            "✅ Futures Positions retrieved!\nTotal Positions: {}\nActive positions:\n{}",
                                            positions.len(),
                                            positions.iter().take(5)
                                                .filter(|p| p.get("positionAmt").and_then(|q| q.as_str()).unwrap_or("0") != "0")
                                                .map(|p| format!("- {} amt: {}",
                                                    p.get("symbol").unwrap_or(&serde_json::Value::String("Unknown".to_string())),
                                                    p.get("positionAmt").unwrap_or(&serde_json::Value::String("0".to_string()))
                                                ))
                                                .collect::<Vec<_>>()
                                                .join("\n")
                                        );
                                        test_result.set(TestResult::Success(success_msg));
                                    }
                                    Err(e) => {
                                        test_result.set(TestResult::Error(format!("❌ Futures Error: {}", e)));
                                    }
                                }
                            });
                        },
                        disabled: matches!(test_result(), TestResult::Testing),
                        if matches!(test_result(), TestResult::Testing) {
                            "🔄 Testing Futures..."
                        } else {
                            "⚡ Test Futures Positions"
                        }
                    }
                }
            }

            div { class: "test-result",
                match test_result() {
                    TestResult::NotTested => rsx! {
                        div { class: "result-placeholder",
                            "Enter your API credentials and test the connection"
                        }
                    },
                    TestResult::Testing => rsx! {
                        div { class: "result-testing",
                            "🔄 Testing connection..."
                        }
                    },
                    TestResult::Success(ref msg) => rsx! {
                        div { class: "result-success",
                            pre { "{msg}" }
                        }
                    },
                    TestResult::Error(ref msg) => rsx! {
                        div { class: "result-error",
                            "{msg}"
                        }
                    }
                }
            }
        }
    }
}

async fn test_binance_api(api_key: &str, secret_key: &str) -> Result<BinanceAccountInfo, String> {
    let proxy_url = "http://127.0.0.1:3001/api/binance/account";

    let request_body = serde_json::json!({
        "api_key": api_key,
        "secret_key": secret_key
    });

    let response = Request::post(proxy_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        return Err(format!("HTTP Error: {}", response.status()));
    }

    let proxy_response: ProxyResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse proxy response: {}", e))?;

    if !proxy_response.success {
        return Err(proxy_response.error.unwrap_or("Unknown error".to_string()));
    }

    let account_info: BinanceAccountInfo = serde_json::from_value(
        proxy_response.data.ok_or("No data in proxy response")?
    ).map_err(|e| format!("Failed to parse account info: {}", e))?;

    Ok(account_info)
}

async fn get_orders(api_key: &str, secret_key: &str, symbol: &str) -> Result<Vec<serde_json::Value>, String> {
    let proxy_url = "http://127.0.0.1:3001/api/binance/orders";

    let request_body = serde_json::json!({
        "api_key": api_key,
        "secret_key": secret_key,
        "symbol": symbol
    });

    let response = Request::post(proxy_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        return Err(format!("HTTP Error: {}", response.status()));
    }

    let proxy_response: ProxyResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse proxy response: {}", e))?;

    if !proxy_response.success {
        return Err(proxy_response.error.unwrap_or("Unknown error".to_string()));
    }

    let orders: Vec<serde_json::Value> = serde_json::from_value(
        proxy_response.data.ok_or("No data in proxy response")?
    ).map_err(|e| format!("Failed to parse orders: {}", e))?;

    Ok(orders)
}

async fn get_options_positions(api_key: &str, secret_key: &str) -> Result<Vec<serde_json::Value>, String> {
    let proxy_url = "http://127.0.0.1:3001/api/binance/options/positions";

    let request_body = serde_json::json!({
        "api_key": api_key,
        "secret_key": secret_key
    });

    let response = Request::post(proxy_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        return Err(format!("HTTP Error: {}", response.status()));
    }

    let proxy_response: ProxyResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse proxy response: {}", e))?;

    if !proxy_response.success {
        return Err(proxy_response.error.unwrap_or("Unknown error".to_string()));
    }

    let positions: Vec<serde_json::Value> = serde_json::from_value(
        proxy_response.data.ok_or("No data in proxy response")?
    ).map_err(|e| format!("Failed to parse positions: {}", e))?;

    Ok(positions)
}

async fn get_futures_positions(api_key: &str, secret_key: &str) -> Result<Vec<serde_json::Value>, String> {
    let proxy_url = "http://127.0.0.1:3001/api/binance/futures/positions";

    let request_body = serde_json::json!({
        "api_key": api_key,
        "secret_key": secret_key
    });

    let response = Request::post(proxy_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        return Err(format!("HTTP Error: {}", response.status()));
    }

    let proxy_response: ProxyResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse proxy response: {}", e))?;

    if !proxy_response.success {
        return Err(proxy_response.error.unwrap_or("Unknown error".to_string()));
    }

    let positions: Vec<serde_json::Value> = serde_json::from_value(
        proxy_response.data.ok_or("No data in proxy response")?
    ).map_err(|e| format!("Failed to parse positions: {}", e))?;

    Ok(positions)
}
