# Position Toggle Feature Documentation

## Overview

The Position Toggle Feature allows users to temporarily enable/disable positions without deleting them, providing better portfolio analysis flexibility and user experience.

**Implementation Date:** September 14, 2025  
**Version:** v1.2.0  
**Commit:** 9c3bbb8

## Features

### ✅ Checkbox Toggle Controls
- Checkbox input in each position card header
- Toggle between active/inactive states
- Preserves all position data when disabled
- Clear visual feedback with labels ("Active" / "Disabled")

### ✅ Visual Feedback System
- **Disabled positions** appear with:
  - Reduced opacity (0.6)
  - Grayed out background (#f8f9fa)
  - Muted text colors (#6c757d)
  - Subdued position direction indicators
- **Smooth CSS transitions** for state changes (0.3s ease)
- **Hover effects** maintained for interactive elements

### ✅ Smart Chart Integration
- **Active position filtering**: Only active positions included in calculations
- **Auto-range calculation**: Price ranges calculated from active positions only
- **Real-time updates**: Charts update immediately when positions are toggled
- **Breakeven recalculation**: Breakeven points recalculated for active positions

### ✅ State Management
- **Reactive UI**: Uses Dioxus `use_signal` for reactive updates
- **State preservation**: Position data retained during toggle operations
- **Edit mode compatibility**: Toggle state preserved during inline editing
- **Persistent state**: Active state maintained across all operations

## Technical Implementation

### Data Model Changes

```rust
// Added to all position structs
pub struct SpotPosition {
    pub quantity: f64,
    pub entry_price: f64,
    pub description: String,
    pub active: bool,  // New field
}

pub struct OptionPosition {
    pub option_type: OptionType,
    pub quantity: f64,
    pub strike_price: f64,
    pub premium: f64,
    pub expiry_price: f64,
    pub description: String,
    pub active: bool,  // New field
}

pub struct FuturesPosition {
    pub quantity: f64,
    pub entry_price: f64,
    pub contract_size: f64,
    pub description: String,
    pub active: bool,  // New field
}
```

### Position Methods

```rust
impl Position {
    /// Check if position is active
    pub fn is_active(&self) -> bool { /* ... */ }
    
    /// Toggle position active state
    pub fn toggle_active(&mut self) { /* ... */ }
    
    /// Set position active state
    pub fn set_active(&mut self, active: bool) { /* ... */ }
}
```

### PayoffEngine Integration

```rust
impl PayoffEngine {
    pub fn generate_payoff_curve(positions: &[Position], /* ... */) -> Vec<PayoffPoint> {
        // Filter only active positions
        let active_positions: Vec<&Position> = positions
            .iter()
            .filter(|pos| pos.is_active())
            .collect();
        
        // Calculate payoff for active positions only
        // ...
    }
}
```

### UI Components

#### PositionCard Toggle Control
```rust
// Checkbox input in position card header
input {
    r#type: "checkbox",
    class: "position-checkbox",
    checked: "{position.is_active()}",
    onchange: move |_| {
        props.on_toggle.call(props.index);
    }
}
```

#### CSS Styling
```css
/* Position Toggle Controls */
.position-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-right: auto;
}

.position-checkbox {
    width: 18px;
    height: 18px;
    cursor: pointer;
    accent-color: #28a745;
}

/* Disabled Position Styling */
.position-card[data-active="false"] {
    opacity: 0.6;
    background-color: #f8f9fa !important;
    border-color: #dee2e6 !important;
}

.position-card:not(.editing) {
    transition: all 0.3s ease;
}
```

## Usage Examples

### Basic Toggle Operations
1. **Enable Position**: Check the checkbox → Position becomes active → Included in calculations
2. **Disable Position**: Uncheck the checkbox → Position becomes inactive → Excluded from calculations
3. **Edit Disabled Position**: Click edit → Make changes → Position remains disabled after saving

### Portfolio Analysis Workflow
1. Add multiple positions to portfolio
2. Toggle positions on/off to analyze different scenarios
3. View how individual positions affect overall payoff
4. Compare strategies by enabling/disabling position groups

### Benefits for Users
- **Non-destructive analysis**: Test scenarios without losing position data
- **Scenario modeling**: Quickly compare different portfolio compositions
- **Position validation**: Isolate individual positions to verify calculations
- **Strategy development**: Build complex strategies incrementally

## Performance Considerations

- **Reactive Updates**: Uses Dioxus reactive signals for efficient re-rendering
- **Filtered Calculations**: PayoffEngine processes only active positions
- **CSS Optimization**: Hardware-accelerated transitions for smooth animations
- **Memory Efficiency**: Inactive positions remain in memory but skip calculations

## Future Enhancements

- [ ] **Bulk Toggle Operations**: Select multiple positions and toggle all at once
- [ ] **Position Groups**: Create groups of positions that can be toggled together
- [ ] **Toggle History**: Undo/redo toggle operations
- [ ] **Conditional Toggles**: Auto-toggle based on market conditions or rules
- [ ] **Toggle Presets**: Save and load toggle state configurations

## Testing Checklist

- [x] ✅ Checkbox toggles position active state
- [x] ✅ Visual feedback shows correctly for disabled positions
- [x] ✅ Charts update when positions are toggled
- [x] ✅ Position data preserved when disabled
- [x] ✅ Toggle state maintained during editing
- [x] ✅ Auto-range calculation respects active positions
- [x] ✅ Breakeven points recalculated for active positions only
- [x] ✅ Smooth CSS transitions work properly
- [x] ✅ All position types support toggle functionality

## Known Issues

None at this time.

## Related Files

- `src/models/position.rs` - Position data structures and methods
- `src/engine/payoff_engine.rs` - Payoff calculations with active filtering
- `src/components/position_list.rs` - UI components for position toggle
- `src/components/app.rs` - Main app with toggle event handlers
- `assets/main.css` - CSS styling for toggle controls and disabled states

---

**Last Updated:** September 14, 2025  
**Author:** GitHub Copilot & Development Team
