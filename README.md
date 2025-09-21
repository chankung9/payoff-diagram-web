# Payoff Diagram Web Application

A web application for creating and visualizing payoff diagrams for various financial positions (Spot, Options, Futures) built with Rust, Dioxus, and WebAssembly. Now with **real Binance API integration** for importing live trading positions.

## 🚀 Latest Features

### Real API Integration
- **Live Binance Integration**: Import real positions from Binance accounts
- **Multi-Market Support**: Spot, Futures, and Options trading
- **Trading Pair Selection**: Focus on Solana ecosystem (SOL/USDT, SOL/USDC, SOL/BUSD, SOL/BTC, SOL/ETH)
- **Real-time Data**: Current market prices and position P&L
- **Secure API Handling**: Proxy server for safe API key management

### Enhanced Position Management
- **One-Click Import**: Import positions directly from exchange APIs
- **Position Filtering**: Select specific trading pairs to import
- **Real Position Data**: Actual entry prices, quantities, and unrealized P&L
- **Mixed Trading**: Combine imported and manual positions

## Features

- **Position Input**: Add different types of financial positions (Spot, Call/Put Options, Futures)
- **API Integration**: Import real positions from Binance with secure API handling
- **Position Management**: Toggle positions on/off without deletion, inline editing capabilities
- **Payoff Calculation**: Calculate profit/loss across price ranges with active position filtering
- **Interactive Charts**: SVG-based interactive charts with hover effects and draggable legend
- **Chart Interactivity**: 
  - Hover data display in interactive legend (no popup flickering)
  - Draggable legend with live/last data indicators
  - Responsive SVG scaling and smooth animations
  - Real-time price, P&L, and percentage change display
- **Long/Short Support**: Visual indicators for position direction with color-coded styling
- **Auto-Range Calculation**: Dynamic price range adjustment based on position strike prices
- **Enhanced Layout**: Chart prominently displayed at top, controls organized below for better workflow
- **Responsive Design**: Optimized for wide screens with adaptive chart sizing (450px-650px)
- **Data Management**: Export/Import position data as JSON
- **Professional UI**: Clean, user-friendly interface with smooth transitions and enhanced spacing

## Technology Stack

- **Frontend Framework**: Dioxus 0.6.3 (Rust)
- **Compilation Target**: WebAssembly (WASM)
- **Build System**: Dioxus CLI (`dx`)
- **API Integration**: Binance REST APIs (Spot, Futures, Options)
- **Proxy Server**: Axum-based Rust server for secure API handling
- **HTTP Client**: reqwest for API communication
- **Charts**: Interactive SVG charts with hover effects, draggable legends, and real-time data display
- **State Management**: Reactive signals with use_signal
- **Styling**: CSS3 with smooth transitions and responsive design
- **Data Format**: JSON for export/import
- **Security**: HMAC-SHA256 authentication for Binance APIs

## Architecture

### Frontend (Dioxus WASM)
- Reactive UI components
- Real-time chart rendering
- API key management
- Position import workflow

### Proxy Server (Rust/Axum)
- Secure API key handling
- Binance API integration
- CORS support
- Request/response proxying

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

2. Start the proxy server:
```bash
cd proxy-server
cargo run
```

3. In a new terminal, start the frontend:
```bash
dx serve
```

4. Open your browser to `http://localhost:8080`

### Using with Binance API

1. **Create API Keys**: Generate Binance API keys with trading permissions
2. **Add API Keys**: Use the API Key Manager in the app
3. **Import Positions**: Click "Import Positions" in the position form
4. **Select Pairs**: Choose Solana trading pairs to import
5. **Analyze**: View imported positions in payoff diagrams
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
- [x] Interactive hover effects with legend display
- [x] Draggable legend with live/last data indicators
- [x] Smooth animations without popup flickering

### Epic 5: Enhanced UI/UX ✅
- [x] Position toggle checkbox controls
- [x] Visual feedback for disabled positions
- [x] Smooth CSS transitions
- [x] Responsive design improvements
- [x] Position direction indicators
- [x] Chart engine selection (hidden for simplicity)
- [x] Interactive chart tooltips integrated into legend

### Epic 6: Data Management
- [ ] Export to JSON
- [ ] Import from JSON
- [ ] Data validation

### Epic 7: UI/UX Enhancement
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

### Interactive Chart Features

- **Binance-Style Legend**: Click and hold on any data point to display detailed analysis
- **Real-time Data**: View price, P&L, and percentage change for each point
- **Draggable Legend**: Move the legend around the chart for optimal viewing
- **Professional Theme**: Dark theme with trading platform colors (Binance-inspired)
- **Responsive Design**: Automatically adapts to mobile devices with vertical layout
- **Chart Statistics**: View max profit, max loss, and breakeven points above the chart
- **Clear All**: Use "Clear All" button to remove all positions at once

### Position Toggle Feature

The position toggle feature allows you to:
- ✅ **Enable/Disable positions** without losing data
- ✅ **See visual feedback** - disabled positions appear grayed out with reduced opacity
- ✅ **Update charts automatically** - only active positions are included in calculations
- ✅ **Preserve position data** - disabled positions retain all their information
- ✅ **Smooth transitions** - CSS animations provide smooth state changes

### Chart Layout and Navigation

The application features an **enhanced chart-first layout** for better analysis workflow:

- **📊 Full-Width Chart Display**: Charts are prominently displayed at the top in a dedicated full-width section
- **🎯 Enhanced Visibility**: Larger chart size (450px-650px) adapts to screen size for better readability
- **⚙️ Organized Controls**: Position management and chart controls are neatly organized below the chart
- **📱 Responsive Design**: Layout adapts from desktop (side-by-side) to mobile (stacked) seamlessly

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
