# Nexus

A terminal-based HTTP client for API testing built in Rust.

Nexus provides a keyboard-driven interface to help you manage and execute API calls efficiently. All your collections, requests, and configurations are automatically persisted using [sled](https://github.com/spacejam/sled), an embedded database, so your work is saved between sessions.

## Demo

<div align="center">
  <video src="assets/nexus_demo.mov" width="100%" controls></video>
</div>

## Features

- **Full HTTP Method Support** - GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS
- **Request Organization** - Group requests into collections for easy management
- **Response Viewer** - View formatted responses with status codes and headers
- **Complete Request Editing** - Edit URL, method, headers, query parameters, body, and authentication
- **Persistent Storage** - All data automatically saved using sled embedded database
- **Keyboard-Driven** - Vim-like navigation and shortcuts for maximum efficiency
- **Built-in Examples** - Sample requests included to help you get started quickly
- **Export Support** - Export collections as JSON or individual requests as curl commands

## Installation

```bash
cargo install --path .
```

## Usage

```bash
nexus
```

On first launch, you'll be greeted with a welcome screen that provides an overview and quick start guide. Press any key to dismiss it and start using Nexus. Check out the **Example Collection** to see sample requests demonstrating the various features.

### Exporting

Nexus supports exporting your collections and requests:

- **Collection Export**: Press `o` to open the export menu. Use arrow keys to select a collection, then press Enter to export it as JSON. The file will be saved in the `exports/` directory with a timestamp.
- **curl Export**: Press `s` to open the curl export menu. Use arrow keys to select a collection, press Enter, then select a specific request. The curl command is saved as a shell script in the `exports/` directory and also copied to your clipboard.

### Keyboard Shortcuts

**Navigation:**
- `Tab` / `Shift+Tab` - Switch between panels
- `j` / `k` or `↓` / `↑` - Navigate lists and move cursor
- `t` - Switch editor tabs (Params, Headers, Body, Auth)

**Actions:**
- `Enter` - Send the current request
- `e` - Edit request (in Request Editor panel)
- `Esc` - Save and exit edit mode
- `n` - Create new request
- `d` - Delete current request
- `y` - Duplicate current request
- `c` - Create new collection (in Collections panel)
- `x` - Delete collection (in Collections panel)
- `o` - Open collection export menu
- `s` - Open curl export menu

**Editing (when in edit mode):**
- Arrow keys - Navigate text fields
- `Tab` - Switch between fields
- `+` / `-` - Add/remove params or headers
- `Ctrl+U` - Clear current field

**Other:**
- `?` - Toggle help screen
- `q` / `Ctrl+C` - Quit

## Building

```bash
cargo build --release
```

## License

MIT

