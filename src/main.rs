mod components;
mod models;
mod utils;
mod engine;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This app is designed to run in the browser with WASM");
    println!("Build with: wasm-pack build --target web --out-dir pkg");
    println!("Serve with: python3 -m http.server 8080");
}
