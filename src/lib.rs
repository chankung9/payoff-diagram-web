use wasm_bindgen::prelude::*;
use dioxus::prelude::*;

mod components;
mod models;
mod utils;

use components::App;

// Entry point for WASM
#[cfg(target_arch = "wasm32")]
pub fn main() {
    // Setup console error panics and logging
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    
    // Launch the Dioxus app
    dioxus::launch(App);
}
