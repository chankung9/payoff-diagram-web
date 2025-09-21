use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct BinanceRequest {
    api_key: String,
    secret_key: String,
    symbol: Option<String>,      // Client สามารถระบุ symbol ได้
}

#[derive(Debug, Serialize)]
struct ProxyResponse {
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/binance/account", post(get_account_info))
        .route("/api/binance/orders", post(get_orders))
        .route("/api/binance/options/positions", post(get_options_positions))
        .route("/api/binance/futures/positions", post(get_futures_positions))
        .route("/api/binance/ticker/price", get(get_ticker_price))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("🚀 Binance Proxy Server running on http://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "Binance Proxy Server is running"
    }))
}

async fn get_account_info(Json(payload): Json<BinanceRequest>) -> Result<Json<ProxyResponse>, StatusCode> {
    match fetch_binance_data(&payload.api_key, &payload.secret_key, "account", None).await {
        Ok(data) => Ok(Json(ProxyResponse {
            success: true,
            data: Some(data),
            error: None,
        })),
        Err(e) => Ok(Json(ProxyResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

async fn get_orders(Json(payload): Json<BinanceRequest>) -> Result<Json<ProxyResponse>, StatusCode> {
    // ใช้ symbol ที่ client ส่งมา หรือ default เป็น SOLUSDT
    let symbol = payload.symbol.as_deref().unwrap_or("SOLUSDT");
    match fetch_binance_data(&payload.api_key, &payload.secret_key, "allOrders", Some(symbol)).await {
        Ok(data) => Ok(Json(ProxyResponse {
            success: true,
            data: Some(data),
            error: None,
        })),
        Err(e) => Ok(Json(ProxyResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

async fn get_options_positions(Json(payload): Json<BinanceRequest>) -> Result<Json<ProxyResponse>, StatusCode> {
    match fetch_options_data(&payload.api_key, &payload.secret_key, "position").await {
        Ok(data) => Ok(Json(ProxyResponse {
            success: true,
            data: Some(data),
            error: None,
        })),
        Err(e) => Ok(Json(ProxyResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

async fn get_futures_positions(Json(payload): Json<BinanceRequest>) -> Result<Json<ProxyResponse>, StatusCode> {
    match fetch_futures_data(&payload.api_key, &payload.secret_key, "positionRisk").await {
        Ok(data) => Ok(Json(ProxyResponse {
            success: true,
            data: Some(data),
            error: None,
        })),
        Err(e) => Ok(Json(ProxyResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

async fn get_ticker_price(Query(params): Query<std::collections::HashMap<String, String>>) -> Result<Json<serde_json::Value>, StatusCode> {
    let symbol = params.get("symbol").ok_or(StatusCode::BAD_REQUEST)?;
    
    match fetch_ticker_price(symbol).await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn fetch_binance_data(
    api_key: &str,
    secret_key: &str,
    endpoint: &str,
    symbol: Option<&str>,
) -> Result<serde_json::Value, String> {
    let timestamp = chrono::Utc::now().timestamp_millis();
    let mut query_params = format!("timestamp={}", timestamp);

    // สำหรับ orders endpoint ต้องมี symbol
    if endpoint == "allOrders" {
        let symbol_param = symbol.unwrap_or("SOLUSDT");
        query_params = format!("symbol={}&{}", symbol_param, query_params);
    }

    let signature = create_signature(secret_key, &query_params)
        .map_err(|e| format!("Failed to create signature: {}", e))?;

    let url = format!(
        "https://api.binance.com/api/v3/{}?{}&signature={}",
        endpoint, query_params, signature
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("X-MBX-APIKEY", api_key)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Binance API Error ({}): {}", status, error_text));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data)
}

fn create_signature(secret: &str, query_string: &str) -> Result<String, String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| format!("Invalid secret key: {}", e))?;

    mac.update(query_string.as_bytes());
    let result = mac.finalize();

    Ok(hex::encode(result.into_bytes()))
}

// ฟังก์ชั่นสำหรับ Binance Options API
async fn fetch_options_data(
    api_key: &str,
    secret_key: &str,
    endpoint: &str,
) -> Result<serde_json::Value, String> {
    let timestamp = chrono::Utc::now().timestamp_millis();
    let query_params = format!("timestamp={}", timestamp);

    let signature = create_signature(secret_key, &query_params)
        .map_err(|e| format!("Failed to create signature: {}", e))?;

    // Binance Options API base URL
    let url = format!(
        "https://eapi.binance.com/eapi/v1/{}?{}&signature={}",
        endpoint, query_params, signature
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("X-MBX-APIKEY", api_key)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Binance Options API Error ({}): {}", status, error_text));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data)
}

// ฟังก์ชั่นสำหรับ Binance Futures API
async fn fetch_futures_data(
    api_key: &str,
    secret_key: &str,
    endpoint: &str,
) -> Result<serde_json::Value, String> {
    let timestamp = chrono::Utc::now().timestamp_millis();
    let query_params = format!("timestamp={}", timestamp);

    let signature = create_signature(secret_key, &query_params)
        .map_err(|e| format!("Failed to create signature: {}", e))?;

    // Binance Futures API base URL
    let url = format!(
        "https://fapi.binance.com/fapi/v3/{}?{}&signature={}",
        endpoint, query_params, signature
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("X-MBX-APIKEY", api_key)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Binance Futures API Error ({}): {}", status, error_text));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data)
}

// ฟังก์ชั่นสำหรับ Ticker Price (Public API - ไม่ต้องใช้ signature)
async fn fetch_ticker_price(symbol: &str) -> Result<serde_json::Value, String> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Binance API Error ({}): {}", status, error_text));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data)
}
