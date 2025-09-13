# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2025-09-14

### Added
- **Interactive Chart Legend**: Draggable legend with real-time hover data display
- **Hover Data Integration**: Live price, P&L, and percentage data shown in legend instead of popup tooltips
- **LIVE/LAST Data Indicators**: Visual status indicators for current vs. last hovered data
- **Improved Chart Responsiveness**: Better SVG scaling and positioning
- **Smooth Animations**: Optimized CSS animations for chart interactions (0.08s timing)

### Changed
- **Removed Popup Tooltips**: Eliminated floating tooltip popups that caused flickering
- **Enhanced Legend Size**: Expanded from 140x80 to 180x120 to accommodate data display
- **Legend Position**: Adjusted default position for better chart visibility
- **Data Display Logic**: Hover data now persists as "LAST DATA" when not actively hovering

### Fixed
- **Tooltip Flickering**: Completely resolved popup flickering issues
- **Mouse Event Handling**: Simplified hover interactions to prevent state conflicts
- **Legend Dragging**: Improved drag behavior and boundary constraints

### Technical Improvements
- **State Management**: Replaced tooltip state with hover data state for better performance
- **CSS Optimization**: Added `will-change` property for better rendering performance
- **Event Handling**: Streamlined mouse events to reduce re-renders

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
