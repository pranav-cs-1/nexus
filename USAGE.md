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

### Moving Within Lists and Scrolling

- **j** or **↓** - Move down in lists (Collections, Requests) or scroll down in Response Viewer
- **k** or **↑** - Move up in lists (Collections, Requests) or scroll up in Response Viewer

### Editor Tabs

When focused on the **Request Editor** panel:
- **t** - Switch to the next tab (Params → Headers → Body → Auth → back to Params)

Each tab shows different aspects of the request:
- **Params** - Query parameters (e.g., `?page=1&limit=10`)
- **Headers** - HTTP headers (e.g., `Content-Type`, `Authorization`)
- **Body** - Request body (for POST, PUT, PATCH requests)
- **Auth** - Authentication settings (Bearer, Basic, API Key)

## Comprehensive Request Editing

Nexus supports full editing of all request components. Here's how to use the enhanced editing system:

### Entering Edit Mode

1. Navigate to the **Request Editor** panel using Tab
2. Press **e** to enter edit mode
   - The title will change to show `[EDITING - ESC to save, Tab to switch fields]`
   - The currently focused field will be highlighted
3. Use **Tab** to cycle through editable fields:
   - **Name** - Request name/title
   - **Method** - HTTP method (GET, POST, PUT, etc.)
   - **URL** - Request URL
   - **Params** - Query parameters
   - **Headers** - HTTP headers  
   - **Body** - Request body content
   - **Auth** - Authentication (Bearer token)

### Field-Specific Editing

#### Name & URL Fields
- **Type** - Insert characters at cursor position
- **Backspace** - Delete character before cursor
- **Delete** - Delete character at cursor
- **←/→** - Move cursor left/right
- **Home** - Jump to beginning of line
- **End** - Jump to end of line

#### Method Selection
- **←/→ or ↑/↓** - Cycle through HTTP methods (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)

#### Parameters & Headers
- **↑/↓** - Navigate through existing key-value pairs
- **+** - Add a new parameter/header pair
- **-** or **Delete** - Remove the selected parameter/header pair
- **Enter** - Start editing the selected key-value pair
  - **Tab** - Switch between editing key and value
  - **Esc** - Finish editing the key-value pair

#### Body Field
- **Type** - Insert characters at cursor position
- **Backspace/Delete** - Delete characters
- **Arrow keys** - Move cursor
- **Home/End** - Jump to start/end of line
- **Enter** - Insert a new line
- Body text automatically wraps to the editor width (no horizontal scrolling needed)

#### Authentication
- **Type** - Enter Bearer token value
- **Standard text editing controls** (Backspace, Delete, Arrow keys, Home, End)

### Saving Changes

- Press **Esc** to save all changes and return to normal mode
- The currently focused field is highlighted with a different border color

## Viewing Request Details

After selecting a request, you can view its details in the Request Editor:

1. Navigate to the **Request Editor** panel using Tab
2. Press **t** to switch between tabs:
   - **Params** - Shows query parameters (displays "No query parameters" if none are set)
   - **Headers** - Shows HTTP headers (displays "No headers" if none are set)
   - **Body** - Shows request body (displays "No body" if none is set)
   - **Auth** - Shows authentication settings

## Sending Requests

1. Select a request from the Requests panel
2. Optionally edit the URL (press `e` when focused on Request Editor)
3. Press **Enter** to send the request
4. The response will appear in the Response Viewer panel

The application shows a loading indicator while the request is being sent.

## Viewing Responses

After sending a request, the response appears in the Response Viewer panel:

1. Navigate to the **Response** panel using Tab
2. Use **j** or **↓** to scroll down through the response
3. Use **k** or **↑** to scroll up through the response
4. The response body is automatically wrapped to fit within the panel width
5. Long responses can be scrolled line by line
6. The title bar shows the current scroll position

Response details include:
- Status code (color-coded: green for 2xx, blue for 3xx, yellow for 4xx, red for 5xx)
- Status text
- Response time in milliseconds
- Response size in bytes
- Formatted body (JSON responses are automatically pretty-printed)

## Managing Collections

### Creating a New Collection

1. Navigate to the **Collections** panel (leftmost panel)
2. Press **c** to create a new collection
3. The new collection will be automatically named "Collection 1", "Collection 2", etc.

### Deleting a Collection

1. Select the collection you want to delete in the Collections panel
2. Press **x** to delete it

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
- **e** - Enter edit mode (when in Request Editor)
- **Esc** - Save and exit edit mode
- **n** - New request
- **d** - Delete request
- **y** - Duplicate request
- **c** - New collection (when in Collections panel)
- **x** - Delete collection (when in Collections panel)

### Editing Mode (when in edit mode)
- **Tab** - Switch between editable fields
- **←/→ or ↑/↓** - Navigate/cycle through options (method selection)
- **+** - Add new parameter/header pair
- **-** or **Delete** - Remove selected parameter/header pair
- **Enter** - Start editing key-value pair (params/headers) or insert new line (body)
- **Home/End** - Jump to start/end of line
- **Backspace** - Delete character before cursor
- **Delete** - Delete character at cursor

### Key-Value Editing (when editing params/headers)
- **Tab** - Switch between editing key and value
- **Esc** - Finish editing and return to navigation mode
- **Type** - Edit the key or value text
- **Backspace** - Delete character from key or value

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
- Response text is automatically wrapped to fit the panel width (no horizontal scrolling needed)
- Long responses can be scrolled vertically using j/k or arrow keys when the Response panel is focused
- HTTP methods are color-coded in the request list (GET=blue, POST=green, etc.)
- Collections help you organize requests by project or API
- The scroll position is reset each time you send a new request

## Troubleshooting

- If the terminal looks corrupted after exiting, run `reset` in your terminal
- Make sure your terminal supports 256 colors for the best experience
- The application requires a minimum terminal size to display all panels properly

