use dioxus::prelude::*;

mod components;
mod models;
mod utils;
mod engine;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        components::app::App {}
    }
}
