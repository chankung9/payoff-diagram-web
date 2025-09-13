# Engine Architecture - WASM Core Logic

## 🏗️ โครงสร้างใหม่

```
src/
├── engine/                    # 🔧 Core calculation engine (WASM-compatible)
│   ├── mod.rs                # Engine module exports
│   ├── payoff_engine.rs      # Payoff calculation engine
│   ├── portfolio_engine.rs   # Portfolio analysis engine
│   └── validation_engine.rs  # Input validation engine
├── models/                   # 📊 Pure data structures
│   ├── mod.rs               # Model exports
│   └── position.rs          # Position data types
├── components/              # 🎨 UI components (Dioxus)
├── utils/                   # 🛠️ Utility functions
└── lib.rs                   # WASM entry point

```

## 🎯 Design Principles

### Engine Layer (Pure Rust - WASM Compatible)
- **ไม่มี UI dependencies** - ไม่ import Dioxus
- **Pure functions** - เป็น deterministic calculations
- **Self-contained** - ไม่ depend on external state
- **Testable** - มี comprehensive unit tests
- **WASM-ready** - export functions สำหรับใช้ใน browser

### Frontend Layer (Dioxus Components)
- **เรียกใช้ Engine functions** - ผ่าน clean interfaces
- **Handle UI state** - positions, user inputs, chart settings
- **Display results** - แสดงผล calculations จาก engine
- **User interactions** - forms, buttons, chart controls

## 🔧 Engine Modules

### 1. PayoffEngine
```rust
// Core payoff calculations
PayoffEngine::calculate_single_payoff(position, price) -> f64
PayoffEngine::calculate_portfolio_payoff(positions, price) -> f64
PayoffEngine::generate_payoff_curve(positions, start, end, step) -> Vec<PayoffPoint>
PayoffEngine::find_breakeven_points(positions, start, end, precision) -> Vec<f64>
PayoffEngine::calculate_max_profit(positions, start, end, step) -> Option<f64>
PayoffEngine::calculate_max_loss(positions, start, end, step) -> Option<f64>
```

### 2. PortfolioEngine
```rust
// Portfolio analysis
PortfolioEngine::analyze_portfolio(positions, start, end, step) -> PortfolioMetrics
PortfolioEngine::has_unlimited_profit(positions) -> bool
PortfolioEngine::has_unlimited_loss(positions) -> bool
PortfolioEngine::get_risk_level(positions) -> RiskLevel
```

### 3. ValidationEngine
```rust
// Input validation
ValidationEngine::validate_position(position) -> ValidationResult
ValidationEngine::validate_portfolio(positions) -> ValidationResult
ValidationEngine::validate_chart_parameters(start, end, step) -> ValidationResult
```

## 🔄 Data Flow

```
User Input (UI) 
    ↓
Validation Engine ← Frontend Components
    ↓
Position Models
    ↓  
Payoff Engine ← Chart Components
    ↓
PayoffPoints/Metrics
    ↓
Display (Charts/Stats)
```

## ✅ Benefits

1. **Clear Separation** - Engine logic แยกจาก UI code
2. **Reusable** - Engine สามารถใช้ใน contexts อื่นได้
3. **Testable** - Engine functions ทดสอบได้อย่างง่าย
4. **WASM Performance** - Core calculations รันใน native speed
5. **Maintainable** - แก้ไข business logic ไม่กระทบ UI

## 🚀 Next Steps

1. **Chart Integration** - เชื่อมต่อ engine กับ chart library
2. **Error Handling** - implement error boundaries ใน UI
3. **Performance** - optimize calculations สำหรับ large datasets
4. **Export Functions** - expose engine functions เป็น WASM exports

---

ตอนนี้เรามี **Clean Architecture** ที่แยก business logic (engine) ออกจาก presentation layer (components) อย่างชัดเจน! 🎉
