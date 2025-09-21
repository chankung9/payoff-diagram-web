use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod engine;
mod models;
mod utils;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[wasm_bindgen(start)]
pub fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        components::app::App {}
    }
}
