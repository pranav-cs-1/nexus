# Nexus - Usage Guide

## Getting Started

### Running the Application

```bash
cargo run
```

The application will start with a sample collection of requests using the JSONPlaceholder API for testing.

## UI Layout

Nexus uses a 4-panel layout:

1. **Collections Panel** (left) - Lists your request collections
2. **Requests Panel** (left-middle) - Lists requests in the selected collection
3. **Request Editor** (right-middle) - Edit request details (URL, params, headers, body, auth)
4. **Response Viewer** (right) - View HTTP response details

## Navigation

### Switching Panels

- **Tab** - Move to the next panel (Collections → Requests → Editor → Response)
- **Shift+Tab** - Move to the previous panel

### Moving Within Lists

- **j** or **↓** - Move down in the current list
- **k** or **↑** - Move up in the current list

### Editor Tabs

- **t** - Switch to the next tab in the Request Editor (Params → Headers → Body → Auth)

## Editing URLs

1. Navigate to the **Request Editor** panel using Tab
2. Press **e** to enter edit mode
   - The URL field title will change to show `[EDITING - ESC to finish]`
   - A cursor will appear in the URL field
3. Edit the URL:
   - **Type** any character to insert at cursor position
   - **Backspace** - Delete character before cursor
   - **Delete** - Delete character at cursor
   - **←/→** - Move cursor left/right
   - **Home** - Jump to beginning of URL
   - **End** - Jump to end of URL
4. Press **Esc** to save and exit edit mode

## Sending Requests

1. Select a request from the Requests panel
2. Optionally edit the URL (press `e` when focused on Request Editor)
3. Press **Enter** to send the request
4. The response will appear in the Response Viewer panel

The application shows a loading indicator while the request is being sent.

## Managing Requests

### Creating a New Request

- Press **n** to create a new empty request
- The new request will be added to the current collection

### Deleting a Request

1. Select the request you want to delete
2. Press **d** to delete it

### Duplicating a Request

1. Select the request you want to duplicate
2. Press **y** to create a copy

## Keyboard Shortcuts Reference

### Navigation
- **Tab** - Next panel
- **Shift+Tab** - Previous panel
- **j** / **↓** - Move down
- **k** / **↑** - Move up
- **t** - Next editor tab

### Actions
- **Enter** - Send request
- **e** - Edit URL (when in Request Editor)
- **Esc** - Save and exit edit mode
- **n** - New request
- **d** - Delete request
- **y** - Duplicate request

### Help & Quit
- **?** - Toggle help popup
- **q** - Quit application
- **Ctrl+C** - Quit application

## Sample Requests

The application comes with three sample requests:

1. **Get JSONPlaceholder Post** - GET request to fetch a single post
2. **List JSONPlaceholder Posts** - GET request to fetch all posts
3. **Create Post** - POST request with JSON body to create a new post

These use the [JSONPlaceholder API](https://jsonplaceholder.typicode.com/) for testing.

## Tips

- The focused panel is highlighted with a different border color
- When editing URLs, you'll see visual feedback with the cursor position
- Response details include status code, duration, size, and formatted body
- HTTP methods are color-coded in the request list (GET=blue, POST=green, etc.)

## Troubleshooting

- If the terminal looks corrupted after exiting, run `reset` in your terminal
- Make sure your terminal supports 256 colors for the best experience
- The application requires a minimum terminal size to display all panels properly

