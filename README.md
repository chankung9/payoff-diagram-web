# Payoff Diagram Web Application

A web application for creating and visualizing payoff diagrams for various financial positions (Spot, Options, Futures) built with Rust, Dioxus, and WebAssembly.

## Features

- **Position Input**: Add different types of financial positions (Spot, Call/Put Options, Futures)
- **Position Management**: Toggle positions on/off without deletion, inline editing capabilities
- **Payoff Calculation**: Calculate profit/loss across price ranges with active position filtering
- **Interactive Charts**: Visualize payoff diagrams with adjustable resolution and modular chart engines
- **Long/Short Support**: Visual indicators for position direction with color-coded styling
- **Auto-Range Calculation**: Dynamic price range adjustment based on position strike prices
- **Data Management**: Export/Import position data as JSON
- **Responsive UI**: Clean, user-friendly interface with smooth transitions

## Technology Stack

- **Frontend Framework**: Dioxus 0.6.3 (Rust)
- **Compilation Target**: WebAssembly (WASM)
- **Build System**: Dioxus CLI (`dx`)
- **Charts**: Modular chart engine system (SVG Native, Canvas Rust planned)
- **State Management**: Reactive signals with use_signal
- **Styling**: CSS3 with smooth transitions and responsive design
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

### Epic 2: Position Management ✅
- [x] Position input UI and data structures
- [x] Position list management with inline editing
- [x] Position toggle functionality (enable/disable)
- [x] Long/Short position indicators
- [x] Input validation and error handling

### Epic 3: Payoff Logic ✅
- [x] Spot position calculations
- [x] Options calculations (Call/Put)
- [x] Futures calculations
- [x] Active position filtering
- [x] Auto-range calculation
- [x] Breakeven point detection

### Epic 4: Chart Rendering ✅
- [x] Modular chart engine system
- [x] SVG Native chart implementation
- [x] Interactive payoff visualization
- [x] Chart controls and resolution settings
- [x] Responsive chart design

### Epic 5: Enhanced UI/UX ✅
- [x] Position toggle checkbox controls
- [x] Visual feedback for disabled positions
- [x] Smooth CSS transitions
- [x] Responsive design improvements
- [x] Position direction indicators

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

### Epic 6: Future Enhancements 🔄
- [ ] Canvas-based chart engine for high performance
- [ ] Chart.js integration for rich chart features  
- [ ] TradingView widget integration
- [ ] Real-time market data integration
- [ ] Advanced portfolio analytics
- [ ] Export to multiple formats (PNG, SVG, PDF)
- [ ] Position templates and presets
- [ ] Multi-timeframe analysis

## How to Use

### Adding Positions

1. **Select Position Type**: Choose from Spot, Option, or Futures
2. **Enter Details**: Fill in quantity, price, and other required fields
3. **Set Direction**: Use positive quantity for Long, negative for Short
4. **Add Position**: Click "Add Position" to add to your portfolio

### Managing Positions

- **Toggle Active/Inactive**: Use the checkbox in each position card to enable/disable positions without deletion
- **Edit Positions**: Click the edit button to modify position details inline
- **Remove Positions**: Click the remove button to permanently delete positions
- **Clear All**: Use "Clear All" button to remove all positions at once

### Position Toggle Feature

The position toggle feature allows you to:
- ✅ **Enable/Disable positions** without losing data
- ✅ **See visual feedback** - disabled positions appear grayed out with reduced opacity
- ✅ **Update charts automatically** - only active positions are included in calculations
- ✅ **Preserve position data** - disabled positions retain all their information
- ✅ **Smooth transitions** - CSS animations provide smooth state changes

### Chart Controls

- **Price Range**: Set minimum and maximum price range for analysis
- **Step Size**: Adjust resolution for smoother or more detailed charts
- **Auto-Range**: Automatically calculates optimal price range based on your positions
- **Chart Engine**: Choose between different chart rendering engines (SVG Native available)

### Understanding the Charts

- **Green Line/Area**: Profit regions
- **Red Line/Area**: Loss regions  
- **Yellow Dashed Line**: Zero profit/loss line
- **Orange Dashed Lines**: Breakeven points
- **Interactive Elements**: Hover over chart elements for detailed information

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
