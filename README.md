# Nexus

API testing TUI built in Rust.

## Features

- Send HTTP requests (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)
- View formatted responses with syntax highlighting
- Organize requests in collections
- Request history
- Environment variables
- Export to curl
- Keyboard-driven interface

## Installation

```bash
cargo install --path .
```

## Usage

```bash
nexus
```

### Keyboard Shortcuts

- `Tab` / `Shift+Tab` - Switch between panels
- `j` / `k` or `↓` / `↑` - Navigate lists
- `Enter` - Send request
- `n` - New request
- `d` - Delete request
- `y` - Duplicate request
- `t` - Switch editor tabs
- `?` - Toggle help
- `q` - Quit

## Building

```bash
cargo build --release
```

## License

MIT

