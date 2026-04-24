# Mouse Gesture App - Development Guide

## Project Setup

This is a Tauri-based cross-platform mouse gesture application with global and per-app settings, theme system, and gesture recognition.

## Prerequisites

### Windows
- Visual Studio 2019+ (or Build Tools)
- Node.js 18+
- Rust (Install from https://rustup.rs/)

### macOS
- Xcode Command Line Tools: `xcode-select --install`
- Node.js 18+
- Rust (Install from https://rustup.rs/)

### Linux
```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev

# Fedora/RHEL
sudo dnf install webkit2-gtk3-devel appindicator-gtk3-devel librsvg2-devel
```

## Building

### Development Mode
```bash
npm install
npm run dev
```

### Production Build
```bash
npm run build
```

## Project Architecture

### Frontend (src/)
- **index.html** - Multi-page app shell with 4 main pages
- **styles.css** - Responsive design with CSS variables for theming
- **main.js** - UI logic, IPC communication, gesture recording

### Backend (src-tauri/src/)
- **lib.rs** - IPC command handlers, module definitions
- **main.rs** - Entry point
- **settings.rs** - Global + per-app config management
- **gesture_detector.rs** - Path simplification and pattern recognition
- **mouse_listener.rs** - Global mouse event tracking (stub for platform-specific implementation)
- **app_detector.rs** - Active window/application detection
- **theme.rs** - Theme configuration with validation

## Features

### Dashboard
- System status display
- Active window detection
- Recent gestures history
- Start/stop gesture listening

### Gesture Recorder
- Interactive canvas for drawing gestures
- Visual feedback (green start point, red end point)
- Gesture detection and confidence scoring
- Save gestures with custom names

### Settings
- **Global Settings**: Enable/disable, sensitivity, minimum gesture length
- **Per-App Settings**: Application-specific overrides and custom gestures

### Theme System
- **Light/Dark modes**: Manual selection
- **Sync mode**: Follow system preference (default)
- **Custom mode**: User-defined colors with JSON schema
- Optimized redraws using CSS classes (no style manipulation on change)

## IPC Commands

All communication between frontend and backend uses Tauri IPC:

```javascript
// Gesture Control
await invoke('start_listening')
await invoke('stop_listening')

// Settings Management
const settings = await invoke('get_global_settings')
await invoke('update_global_settings', { config })
const appSettings = await invoke('get_app_settings', { app_name })
await invoke('update_app_settings', { app_name, config })

// Window & Gesture Detection
const activeWindow = await invoke('get_active_window')
const gestureResult = await invoke('detect_gesture', { points })
```

## Configuration Files

### Global Config
`~/.config/tauri-gestures/config.json` (Linux/macOS) or `%APPDATA%\tauri-gestures\config.json` (Windows)

```json
{
  "enabled": true,
  "sensitivity": 0.8,
  "min_gesture_length": 50
}
```

### Per-App Config
`~/.config/tauri-gestures/apps/{app_name}.json`

```json
{
  "appName": "firefox",
  "overrides": {
    "enabled": true,
    "disabledGestures": [],
    "customGestures": {}
  }
}
```

### Theme Config
`~/.config/tauri-gestures/theme.json`

```json
{
  "mode": "sync",
  "custom_theme": null
}
```

## Gesture Recognition Algorithm

1. **Path Simplification**: Douglas-Peucker algorithm reduces point count
2. **Direction Calculation**: Analyzes angle between start and end points
3. **Straightness Check**: Calculates direct distance vs. path distance
4. **Pattern Matching**: Classifies gestures as:
   - `swipe_left`, `swipe_right`, `swipe_up`, `swipe_down`
   - `angle` (L-shaped or corner patterns)
   - `curve` (rounded paths)

## Security

- ✅ API keys excluded via `.gitignore`
- ✅ Config files stored in user-specific directories
- ✅ No hardcoded secrets
- ✅ JSON schema validation for custom themes

## Known Limitations

### Platform-Specific Implementations Needed

The following modules contain stub implementations and need proper OS-level integration:

1. **mouse_listener.rs**
   - Windows: Use Win32 API hooks (e.g., SetWindowsHookEx)
   - macOS: Use CoreGraphics event taps
   - Linux: Use X11/Wayland event listeners

2. **app_detector.rs**
   - Windows: GetForegroundWindow() + GetWindowText()
   - macOS: NSWorkspace.activeApplication
   - Linux: X11 window manager queries

### Gesture Detection
- Currently uses simple pattern matching
- Could be enhanced with machine learning models
- Path tolerance and sensitivity thresholds are configurable

## Testing

### Manual Testing Checklist
- [ ] Gesture recording works on canvas
- [ ] Settings persist after app restart
- [ ] Per-app overrides take precedence
- [ ] Theme changes apply without full redraw
- [ ] Custom theme colors save and load
- [ ] Active window detection updates
- [ ] Recent gestures list populates

### Future Tests
- Unit tests for gesture recognition algorithm
- Integration tests for settings persistence
- E2E tests for full workflow

## Development Notes

- Use `npm run tauri dev` for hot-reload development
- Check browser DevTools in the Tauri dev window for debugging
- Rust compilation errors typically relate to system dependencies on Linux
- CSS variables enable efficient theme switching without DOM manipulation

## License

MIT
