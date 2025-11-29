# Nexus

A terminal-based HTTP client for API testing built in Rust.

Nexus provides a keyboard-driven interface to help you manage and execute API calls efficiently. All your collections, requests, and configurations are automatically persisted using [sled](https://github.com/spacejam/sled), an embedded database, so your work is saved between sessions.

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
- `o` - Export collection as JSON (in Collections panel)
- `s` - Export request as curl command (in Requests/Editor panel)

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

