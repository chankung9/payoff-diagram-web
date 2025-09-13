use wasm_bindgen::prelude::*;
use dioxus::prelude::*;

mod components;
mod models;
mod utils;

use components::App;

#[wasm_bindgen(start)]
pub fn main() {
    // Set panic hook for better debugging
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    // Initialize console log for debugging
    wasm_logger::init(wasm_logger::Config::default());
    
    // Launch the Dioxus app
    launch(App);
}
