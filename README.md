# Payoff Diagram Web Application

A web application for creating and visualizing payoff diagrams for various financial positions (Spot, Options, Futures) built with Rust, Dioxus, and WebAssembly.

## Features

- **Position Input**: Add different types of financial positions (Spot, Call/Put Options, Futures)
- **Payoff Calculation**: Calculate profit/loss across price ranges
- **Interactive Charts**: Visualize payoff diagrams with adjustable resolution
- **Data Management**: Export/Import position data as JSON
- **Responsive UI**: Clean, user-friendly interface

## Technology Stack

- **Frontend Framework**: Dioxus (Rust)
- **Compilation Target**: WebAssembly (WASM)
- **Build System**: Dioxus CLI (`dx`)
- **Charts**: Plotters + Canvas (planned)
- **Data Format**: JSON for export/import
- **No Backend Required**: Pure client-side application

## Quick Start

### Prerequisites

- Rust (latest stable)
- Dioxus CLI: `cargo install dioxus-cli`
- A modern web browser

### Installation

1. Clone the repository:
```bash
git clone https://github.com/chankung9/payoff-diagram-web.git
cd payoff-diagram-web
```

2. Install dependencies:
```bash
cargo build
```

3. Run development server:
```bash
dx serve
```

4. Open your browser and navigate to `http://localhost:8080`

### Building for Production

```bash
dx build --release
```

## Project Structure

```
payoff-diagram-web/
├── src/
│   ├── main.rs              # Application entry point
│   ├── components/          # UI components
│   │   ├── mod.rs
│   │   ├── app.rs           # Main App component
│   │   ├── position_form.rs # Position input form
│   │   ├── position_list.rs # Position list/management
│   │   ├── payoff_chart.rs  # Chart component
│   │   └── chart_controls.rs# Control panels
│   ├── models/              # Data models
│   │   ├── mod.rs
│   │   ├── position.rs      # Position structs
│   │   └── payoff.rs        # Payoff calculation
│   ├── engine/              # Business logic engine
│   │   ├── mod.rs
│   │   ├── payoff_engine.rs # Core calculations
│   │   ├── portfolio_engine.rs # Portfolio analysis
│   │   └── validation_engine.rs # Input validation
│   └── utils/               # Utility functions
│       └── mod.rs           # JSON export/import
├── assets/                  # Static assets
│   └── main.css            # Application styles
├── target/                  # Build artifacts (auto-generated)
├── Cargo.toml
├── Dioxus.toml             # Dioxus configuration
└── README.md
```

## Development Roadmap

### Epic 0: System & Directory Design ✅
- [x] Project Directory Structure
- [x] System Overview Documentation
- [x] Build/Run Workflow with Dioxus CLI

### Epic 1: Project Scaffold ✅
- [x] Create Dioxus + WASM scaffold
- [x] Setup dx build/serve workflow
- [x] Create App component foundation

### Epic 2: Position Form
- [ ] Design Position input UI
- [ ] Implement position data structures
- [ ] Input validation

### Epic 3: Payoff Logic
- [ ] Spot position calculations
- [ ] Options calculations (Call/Put)
- [ ] Futures calculations
- [ ] Unit tests

### Epic 4: Chart Rendering
- [ ] Integrate chart library
- [ ] Payoff visualization
- [ ] Resolution controls

### Epic 5: Data Management
- [ ] Export to JSON
- [ ] Import from JSON
- [ ] Data validation

### Epic 6: UI/UX Enhancement
- [ ] UI polish
- [ ] Example positions
- [ ] Error handling

### Epic 3: Payoff Logic
- [ ] Spot position calculations
- [ ] Options calculations (Call/Put)
- [ ] Futures calculations
- [ ] Unit tests

### Epic 4: Chart Rendering
- [ ] Integrate chart library
- [ ] Payoff visualization
- [ ] Resolution controls

### Epic 5: Data Management
- [ ] Export to JSON
- [ ] Import from JSON
- [ ] Data validation

### Epic 6: UI/UX Enhancement
- [ ] UI polish
- [ ] Example positions
- [ ] Error handling

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com/) - A modern Rust framework for building user interfaces
- Charts powered by [Plotters](https://plotters-rs.github.io/) - A Rust drawing library
- WebAssembly for high-performance client-side computation
