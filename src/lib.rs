use wasm_bindgen::prelude::*;
use dioxus::prelude::*;

mod components;
mod models;
mod utils;
mod engine; // Core calculation engine (WASM-compatible)

use components::App;

// WASM entry point
#[wasm_bindgen]
pub fn hydrate() {
    // Setup console error panics and logging
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    
    // Launch the Dioxus app
    launch(App);
}

// Export engine functions for WASM usage
pub use engine::{PayoffEngine, PortfolioEngine, ValidationEngine};
pub use engine::{PayoffPoint, PortfolioMetrics, ValidationResult};
