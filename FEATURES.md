# RustCast Features

## Search & Launch

- Application autoloading and search (installed macOS apps)
- Website/URL opening (raw URLs, auto-https prefix)
- Google search via `?` suffix at end of query
- File search (via `mdfind`, batched results, configurable search dirs)
- Emoji searching (by name, grid layout, copies to clipboard)
- Calculator (`evalexpr`-based, evaluates math expressions inline)
- Unit conversions (length, mass, temperature, etc.)
- Custom shell command execution (with variable passing, hotkey bindable)
- Quit all apps / quit specific app by name
- Favourites (♥️ toggle per result) and frequently used ranking
- Search aliases (map a shortcut to a longer query)
- User-defined modes (custom context filters that swap app list)
- Open settings/preferences from search results

## Clipboard

- Clipboard history (persistent, SQLite-backed, up to 300 items)
- Image rendering from clipboard history
- Paste-on-select for clipboard items (auto-pastes to frontmost app)
- Text, image, and URL clipboard content types
- Clear clipboard history

## Window Management

- Window tiling (12 positions via Accessibility API):
  - Left/Right/Top/Bottom Half
  - Top-Left, Top-Right, Bottom-Left, Bottom-Right Quarter
  - Left/Center/Right Third
  - Maximize
- Custom launcher window positioning (9 screen positions: corners, edges, center)
- Floating window level (appears above fullscreen apps)
- Mouse-following screen placement (opens on screen with cursor)

## UI & Themes

- Fully customizable theme via config:
  - Text, background, and secondary background colors
  - Dark / Light / Follow-system theme modes
  - Blur/transparent background
  - Toggle icons on/off
  - Toggle scroll bar on/off
  - Custom font selection
- Menubar/tray icon with context menu (show/hide, settings, quit, etc.)
- Scrollable results (max 5 visible, dynamic window resize)
- Custom placeholder text in search bar
- Settings panel (separate window, GUI for common config options)
- Keyboard navigation:
  - Arrow keys (Up/Down/Left/Right in emoji grid)
  - Vim-style Ctrl+N / Ctrl+P
  - Enter to open focused result
  - Esc to clear search / go back / hide window
  - Tab-based settings panel navigation
- App version display in search results

## Configuration

- TOML-based config file at `~/.config/rustcast/config.toml`
- Configurable hotkeys: toggle window (default `ALT+SPACE`), clipboard (default `SUPER+SHIFT+C`), shell command shortcuts
- User-defined modes (name → shell script mapping)
- Custom shell commands with icon, alias, and per-command hotkey
- Search aliases (shortcut → expanded query)
- Buffer rules: clear on hide, clear on enter
- Start at login (via `SMAppService`)
- Auto-update (checks GitHub releases, downloads & applies updates)
- Runtime config reload (search "refresh")
- URL scheme handler (`rustcast://` for deep linking)
- Configurable search URL template (default DuckDuckGo)
- Debounce delay for file/emoji search
- Event calendar fetch duration

## Platform (macOS)

- Global hotkey registration (via `global-hotkey` crate)
- Haptic feedback (trackpad force touch, on search/errors)
- Input source switching on open (restore on close)
- Calendar event display (via EventKit, upcoming events as search results)
- Accessibility permissions for window tiling
- Hidden dock icon (transforms to UI element process)

## Easter Eggs

- `randomvar` — generates random number (0–100), copies to clipboard
- `67` — copies "67" to clipboard
- `lemon` — displays a lemon image
- `f` — shows "Ferris Plushies" link (ferris.rs)
- `zombo` — opens zombo.com
- Sponsor easter eggs (personalized for GitHub sponsors)
