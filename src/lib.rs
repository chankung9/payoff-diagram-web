use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod models;
mod utils;

use components::App;

// WASM entry point
#[wasm_bindgen]
pub fn hydrate() {
    // Setup console error panics and logging
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    
    // Launch the Dioxus app
    dioxus::launch(App);
}
