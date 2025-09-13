use dioxus::prelude::*;

mod components;
mod models;
mod utils;

use components::App;

fn main() {
    // For web target
    #[cfg(target_arch = "wasm32")]
    {
        // Set panic hook for better debugging
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        
        // Initialize console log for debugging
        wasm_logger::init(wasm_logger::Config::default());
        
        // Launch the Dioxus app
        launch(App);
    }

    // For desktop/native target (for testing)
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("This app is designed to run in the browser with WASM");
        println!("Run 'dx serve' to start the web server");
    }
}
