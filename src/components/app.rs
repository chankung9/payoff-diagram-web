use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! {
        div {
            class: "app-container",
            header {
                class: "app-header",
                h1 { "Payoff Diagram Web Application" }
                p { "Create and visualize payoff diagrams for financial positions" }
            }
            
            main {
                class: "app-main",
                div {
                    class: "welcome-section",
                    h2 { "Welcome!" }
                    p { "This application helps you analyze profit/loss scenarios for:" }
                    ul {
                        li { "Spot positions" }
                        li { "Options (Call/Put)" }
                        li { "Futures contracts" }
                    }
                    
                    div {
                        class: "getting-started",
                        h3 { "Getting Started:" }
                        ol {
                            li { "Add your positions using the form below" }
                            li { "Adjust the price range and resolution" }
                            li { "View the payoff diagram" }
                            li { "Export/Import your data as needed" }
                        }
                    }
                }
                
                div {
                    class: "placeholder-sections",
                    div {
                        class: "section position-input",
                        h3 { "Position Input" }
                        p { "📝 Position form will be implemented here" }
                    }
                    
                    div {
                        class: "section chart-display",
                        h3 { "Payoff Diagram" }
                        p { "📊 Interactive chart will be displayed here" }
                    }
                    
                    div {
                        class: "section data-management",
                        h3 { "Data Management" }
                        p { "💾 Export/Import controls will be here" }
                    }
                }
            }
            
            footer {
                class: "app-footer",
                p { "Built with Rust + Dioxus + WebAssembly" }
            }
        }
    }
}
