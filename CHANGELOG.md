# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2025-09-14

### Added
- **Binance-Style Legend**: Long-press activation legend with professional dark theme
- **Chart Render Area Fix**: Proper vertical expansion to accommodate chart and statistics
- **Interactive Data Points**: Larger dots (6px) with enhanced hover effects
- **Real-time Data Display**: Price, P&L, and percentage data with color coding
- **Professional Styling**: Dark theme with Binance-inspired colors (#F0B90B yellow, #02C076 green, #F6465D red)

### Changed
- **Legend Activation**: Changed from always-visible to click-and-hold activation (like Binance)
- **Legend Size**: Increased to 240x160 for better data readability
- **Chart Layout**: Fixed chart-render-area flexbox behavior for proper content expansion
- **Color Scheme**: Updated to professional trading platform colors
- **Interaction Model**: Simplified to click-to-show instead of complex hover states

### Fixed
- **Chart Vertical Expansion**: Chart now properly expands to fit content and statistics
- **Legend Positioning**: Better automatic positioning near clicked points
- **Chart Point Interaction**: Improved click detection with larger interactive areas

### Technical Improvements
- **Simplified State Management**: Removed complex timer logic for cleaner interaction
- **CSS Flexbox**: Added proper flex properties for chart-render-area
- **Event Handling**: Streamlined click events for legend activation
- **Performance**: Reduced DOM manipulation for better responsiveness

### Previous Features (Still Active)
- **Interactive Chart Legend**: Draggable legend with real-time hover data display
- **Hover Data Integration**: Live price, P&L, and percentage data shown in legend
- **LIVE/LAST Data Indicators**: Visual status indicators for current vs. last data
- **Mobile Responsive Design**: Vertical stacking on mobile devices

## [Previous] - 2025-09-13

### Added
- SVG Native chart engine with interactive elements
- Draggable chart legend functionality
- Enhanced tooltip system with price, P&L, and percentage data
- Responsive chart design with breakpoint optimization
- Chart engine selection system (hidden for UX simplicity)

### Changed
- Moved chart to top of layout for better visibility
- Enhanced chart statistics display
- Improved position management UI

### Fixed
- Chart rendering performance
- Position calculation accuracy
- UI responsiveness on various screen sizes
