mod app;
mod http;
mod models;
mod storage;
mod ui;
mod utils;

use app::state::{AppState, InputMode, Panel, EditorField};
use app::actions::Action;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::logger::init()?;
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let storage = storage::Storage::new()?;
    
    let mut state = AppState::new();
    
    // Load collections and requests from storage
    state.collections = storage.load_collections()?;
    state.requests = storage.load_requests()?;
    
    // If no data exists, create sample data
    if state.collections.is_empty() {
        let default_collection = models::collection::Collection::new("Collection 1".to_string());
        let collection_id = default_collection.id;
        storage.save_collection(&default_collection)?;
        state.collections.push(default_collection);
        state.selected_collection = Some(0);
        
        let mut request1 = models::request::HttpRequest::new(
            "Get JSONPlaceholder Post".to_string(),
            models::request::HttpMethod::GET,
            "https://jsonplaceholder.typicode.com/posts/1".to_string(),
        );
        request1.collection_id = Some(collection_id);
        storage.save_request(&request1)?;
        state.requests.push(request1);
        
        let mut request2 = models::request::HttpRequest::new(
            "List JSONPlaceholder Posts".to_string(),
            models::request::HttpMethod::GET,
            "https://jsonplaceholder.typicode.com/posts".to_string(),
        );
        request2.collection_id = Some(collection_id);
        storage.save_request(&request2)?;
        state.requests.push(request2);
        
        let mut request3 = models::request::HttpRequest::new(
            "Create Post".to_string(),
            models::request::HttpMethod::POST,
            "https://jsonplaceholder.typicode.com/posts".to_string(),
        )
        .with_header("Content-Type".to_string(), "application/json".to_string())
        .with_body(r#"{"title": "foo", "body": "bar", "userId": 1}"#.to_string());
        request3.collection_id = Some(collection_id);
        storage.save_request(&request3)?;
        state.requests.push(request3);
        
        let mut request4 = models::request::HttpRequest::new(
            "Search with Params, Headers & Body".to_string(),
            models::request::HttpMethod::POST,
            "https://jsonplaceholder.typicode.com/posts".to_string(),
        )
        .with_query_param("_page".to_string(), "1".to_string())
        .with_query_param("_limit".to_string(), "10".to_string())
        .with_query_param("_sort".to_string(), "id".to_string())
        .with_header("Content-Type".to_string(), "application/json".to_string())
        .with_header("Accept".to_string(), "application/json".to_string())
        .with_header("X-Request-ID".to_string(), "sample-123".to_string())
        .with_body(r#"{"filter": {"userId": 1}, "fields": ["id", "title", "body"]}"#.to_string());
        request4.collection_id = Some(collection_id);
        storage.save_request(&request4)?;
        state.requests.push(request4);
        
        let mut request5 = models::request::HttpRequest::new(
            "TEST: Full Editor Test (No Auth)".to_string(),
            models::request::HttpMethod::PUT,
            "https://httpbin.org/anything".to_string(),
        )
        .with_query_param("test_param_1".to_string(), "value_one".to_string())
        .with_query_param("test_param_2".to_string(), "value_two".to_string())
        .with_query_param("number".to_string(), "42".to_string())
        .with_header("Content-Type".to_string(), "application/json".to_string())
        .with_header("Accept".to_string(), "application/json".to_string())
        .with_header("X-Test-Header".to_string(), "test-value-123".to_string())
        .with_header("User-Agent".to_string(), "Nexus-TUI-Tester/1.0".to_string())
        .with_body(r#"{
  "test": true,
  "message": "This is a comprehensive test",
  "data": {
    "name": "Nexus Test",
    "version": "1.0",
    "features": ["params", "headers", "body", "method"]
  },
  "numbers": [1, 2, 3, 42],
  "nested": {
    "level1": {
      "level2": {
        "deep": "value"
      }
    }
  }
}"#.to_string());
        request5.collection_id = Some(collection_id);
        storage.save_request(&request5)?;
        state.requests.push(request5);
    } else {
        // Set initial selections if data exists
        if !state.collections.is_empty() {
            state.selected_collection = Some(0);
        }
    }
    
    if !state.requests.is_empty() {
        state.selected_request = Some(0);
    }
    
    let http_client = http::client::HttpClient::new()?;
    
    loop {
        terminal.draw(|frame| {
            ui::app::UI::draw(frame, &state);
        })?;
        
        if let Event::Key(key) = event::read()? {
            if state.show_help {
                if let KeyCode::Char('?') = key.code {
                    Action::ToggleHelp.execute(&mut state);
                }
                continue;
            }
            
            if state.input_mode == InputMode::Editing {
                handle_edit_mode(&mut state, key, &storage);
                continue;
            }
            
            if state.editing_collection {
                handle_collection_edit_mode(&mut state, key, &storage);
                continue;
            }
            
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::NONE) => {
                    Action::Quit.execute(&mut state);
                }
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    Action::Quit.execute(&mut state);
                }
                (KeyCode::Char('?'), KeyModifiers::NONE) => {
                    Action::ToggleHelp.execute(&mut state);
                }
                (KeyCode::Tab, KeyModifiers::NONE) => {
                    Action::NextPanel.execute(&mut state);
                }
                (KeyCode::BackTab, KeyModifiers::SHIFT) => {
                    Action::PrevPanel.execute(&mut state);
                }
                (KeyCode::Down | KeyCode::Char('j'), KeyModifiers::NONE) => {
                    match state.focused_panel {
                        app::state::Panel::Collections => Action::NextCollection.execute(&mut state),
                        app::state::Panel::Requests => Action::NextRequest.execute(&mut state),
                        app::state::Panel::Response => state.scroll_response_down(),
                        _ => {}
                    }
                }
                (KeyCode::Up | KeyCode::Char('k'), KeyModifiers::NONE) => {
                    match state.focused_panel {
                        app::state::Panel::Collections => Action::PrevCollection.execute(&mut state),
                        app::state::Panel::Requests => Action::PrevRequest.execute(&mut state),
                        app::state::Panel::Response => state.scroll_response_up(),
                        _ => {}
                    }
                }
                (KeyCode::Char('t'), KeyModifiers::NONE) => {
                    if state.focused_panel == Panel::RequestEditor {
                        Action::NextEditorTab.execute(&mut state);
                    }
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    if let Some(request) = state.get_current_request().cloned() {
                        state.is_loading = true;
                        state.loading_message = format!("Sending {} request...", request.method.as_str());
                        state.reset_response_scroll();
                        
                        terminal.draw(|frame| {
                            ui::app::UI::draw(frame, &state);
                        })?;
                        
                        match http_client.execute(&request).await {
                            Ok(response) => {
                                state.current_response = Some(response);
                            }
                            Err(e) => {
                                state.loading_message = format!("Error: {}", e);
                            }
                        }
                        
                        state.is_loading = false;
                        state.loading_message.clear();
                    }
                }
                (KeyCode::Char('n'), KeyModifiers::NONE) => {
                    Action::NewRequest.execute(&mut state);
                    if let Some(request) = state.requests.last() {
                        let _ = storage.save_request(request);
                    }
                }
                (KeyCode::Char('d'), KeyModifiers::NONE) => {
                    if let Some(idx) = state.selected_request {
                        if let Some(request) = state.requests.get(idx) {
                            let request_id = request.id;
                            Action::DeleteRequest.execute(&mut state);
                            let _ = storage.delete_request(&request_id);
                        }
                    }
                }
                (KeyCode::Char('y'), KeyModifiers::NONE) => {
                    Action::DuplicateRequest.execute(&mut state);
                    if let Some(request) = state.requests.last() {
                        let _ = storage.save_request(request);
                    }
                }
                (KeyCode::Char('e'), KeyModifiers::NONE) => {
                    if state.focused_panel == Panel::RequestEditor {
                        state.load_current_request_to_input();
                        state.input_mode = InputMode::Editing;
                        // Start with Name field
                        state.editor_focused_field = EditorField::Name;
                    } else if state.focused_panel == Panel::Collections {
                        Action::EditCollection.execute(&mut state);
                    }
                }
                (KeyCode::Char('c'), KeyModifiers::NONE) => {
                    if state.focused_panel == Panel::Collections {
                        Action::NewCollection.execute(&mut state);
                        if let Some(collection) = state.collections.last() {
                            let _ = storage.save_collection(collection);
                        }
                    } else if state.focused_panel == Panel::Response {
                        Action::CopyResponse.execute(&mut state);
                    }
                }
                (KeyCode::Char('x'), KeyModifiers::NONE) => {
                    if state.focused_panel == Panel::Collections {
                        if let Some(idx) = state.selected_collection {
                            if let Some(collection) = state.collections.get(idx) {
                                let collection_id = collection.id;
                                Action::DeleteCollection.execute(&mut state);
                                let _ = storage.delete_collection(&collection_id);
                                let _ = storage.delete_requests_by_collection(&collection_id);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        if state.should_quit {
            break;
        }
    }
    
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    
    Ok(())
}

fn handle_edit_mode(state: &mut AppState, key: KeyEvent, storage: &storage::Storage) {
    match key.code {
        KeyCode::Esc => {
            // If we're in key-value editing mode (editing a param/header key or value),
            // let the specific handler deal with Esc to exit key-value editing mode
            // Otherwise, exit the entire edit mode
            if (state.editor_focused_field == EditorField::Params || 
                state.editor_focused_field == EditorField::Headers) && 
               state.kv_edit_mode != app::state::KeyValueEditMode::None {
                // Let the specific field handler deal with Esc in key-value editing mode
                match state.editor_focused_field {
                    EditorField::Params => handle_params_edit(state, key),
                    EditorField::Headers => handle_headers_edit(state, key),
                    _ => {}
                }
            } else {
                // Exit the entire edit mode
                state.save_input_to_request();
                if let Some(request) = state.get_current_request() {
                    let _ = storage.save_request(request);
                }
                state.input_mode = InputMode::Normal;
                state.kv_edit_mode = app::state::KeyValueEditMode::None;
            }
        }
        KeyCode::Tab => {
            // Check if we're in key-value editing mode for params or headers
            if (state.editor_focused_field == EditorField::Params || 
                state.editor_focused_field == EditorField::Headers) && 
               state.kv_edit_mode != app::state::KeyValueEditMode::None {
                // Let the specific field handler deal with Tab in key-value editing mode
                match state.editor_focused_field {
                    EditorField::Params => handle_params_edit(state, key),
                    EditorField::Headers => handle_headers_edit(state, key),
                    _ => {}
                }
            } else {
                // Switch between fields in edit mode
                state.kv_edit_mode = app::state::KeyValueEditMode::None; // Reset KV edit mode when switching fields
                state.editor_focused_field = match state.editor_focused_field {
                    EditorField::Name => EditorField::Method,
                    EditorField::Method => EditorField::Url,
                    EditorField::Url => EditorField::Params,
                    EditorField::Params => EditorField::Headers,
                    EditorField::Headers => EditorField::Body,
                    EditorField::Body => EditorField::Auth,
                    EditorField::Auth => EditorField::Name,
                };
                
                // Update the UI tab to match the focused field
                state.editor_tab = match state.editor_focused_field {
                    EditorField::Params => app::state::EditorTab::Params,
                    EditorField::Headers => app::state::EditorTab::Headers,
                    EditorField::Body => app::state::EditorTab::Body,
                    EditorField::Auth => app::state::EditorTab::Auth,
                    _ => state.editor_tab, // Keep current tab for Name, Method, URL
                };
            }
        }
        _ => {
            match state.editor_focused_field {
                EditorField::Name => handle_name_edit(state, key),
                EditorField::Method => handle_method_edit(state, key),
                EditorField::Url => handle_url_edit(state, key),
                EditorField::Params => handle_params_edit(state, key),
                EditorField::Headers => handle_headers_edit(state, key),
                EditorField::Body => handle_body_edit(state, key),
                EditorField::Auth => handle_auth_edit(state, key),
            }
        }
    }
}

fn handle_name_edit(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            state.name_input.insert(state.name_cursor, c);
            state.name_cursor += 1;
        }
        KeyCode::Backspace => {
            if state.name_cursor > 0 {
                state.name_cursor -= 1;
                state.name_input.remove(state.name_cursor);
            }
        }
        KeyCode::Delete => {
            if state.name_cursor < state.name_input.len() {
                state.name_input.remove(state.name_cursor);
            }
        }
        KeyCode::Left => {
            if state.name_cursor > 0 {
                state.name_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.name_cursor < state.name_input.len() {
                state.name_cursor += 1;
            }
        }
        KeyCode::Home => {
            state.name_cursor = 0;
        }
        KeyCode::End => {
            state.name_cursor = state.name_input.len();
        }
        _ => {}
    }
}

fn handle_method_edit(state: &mut AppState, key: KeyEvent) {
    let methods = models::request::HttpMethod::all();
    match key.code {
        KeyCode::Up | KeyCode::Left => {
            if state.method_input > 0 {
                state.method_input -= 1;
            } else {
                state.method_input = methods.len() - 1;
            }
        }
        KeyCode::Down | KeyCode::Right => {
            state.method_input = (state.method_input + 1) % methods.len();
        }
        _ => {}
    }
}

fn handle_url_edit(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            state.url_input.insert(state.url_cursor, c);
            state.url_cursor += 1;
        }
        KeyCode::Backspace => {
            if state.url_cursor > 0 {
                state.url_cursor -= 1;
                state.url_input.remove(state.url_cursor);
            }
        }
        KeyCode::Delete => {
            if state.url_cursor < state.url_input.len() {
                state.url_input.remove(state.url_cursor);
            }
        }
        KeyCode::Left => {
            if state.url_cursor > 0 {
                state.url_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.url_cursor < state.url_input.len() {
                state.url_cursor += 1;
            }
        }
        KeyCode::Home => {
            state.url_cursor = 0;
        }
        KeyCode::End => {
            state.url_cursor = state.url_input.len();
        }
        _ => {}
    }
}

fn handle_params_edit(state: &mut AppState, key: KeyEvent) {
    use app::state::KeyValueEditMode;
    
    match state.kv_edit_mode {
        KeyValueEditMode::None => {
            match key.code {
                KeyCode::Up => {
                    if state.params_selected > 0 {
                        state.params_selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if state.params_selected < state.params_input.len().saturating_sub(1) {
                        state.params_selected += 1;
                    }
                }
                KeyCode::Char('+') => {
                    state.add_param();
                    // Automatically start editing the new parameter's key
                    state.kv_edit_mode = KeyValueEditMode::Key;
                }
                KeyCode::Char('-') | KeyCode::Delete => {
                    state.delete_param();
                }
                KeyCode::Enter => {
                    if !state.params_input.is_empty() && state.params_selected < state.params_input.len() {
                        state.kv_edit_mode = KeyValueEditMode::Key;
                    }
                }
                _ => {}
            }
        }
        KeyValueEditMode::Key => {
            match key.code {
                KeyCode::Esc => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                KeyCode::Tab => {
                    state.kv_edit_mode = KeyValueEditMode::Value;
                }
                KeyCode::Char(c) => {
                    if let Some((key, _)) = state.params_input.get_mut(state.params_selected) {
                        key.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some((key, _)) = state.params_input.get_mut(state.params_selected) {
                        key.pop();
                    }
                }
                _ => {}
            }
        }
        KeyValueEditMode::Value => {
            match key.code {
                KeyCode::Esc => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                KeyCode::Tab => {
                    state.kv_edit_mode = KeyValueEditMode::Key;
                }
                KeyCode::Char(c) => {
                    if let Some((_, value)) = state.params_input.get_mut(state.params_selected) {
                        value.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some((_, value)) = state.params_input.get_mut(state.params_selected) {
                        value.pop();
                    }
                }
                _ => {}
            }
        }
    }
}

fn handle_headers_edit(state: &mut AppState, key: KeyEvent) {
    use app::state::KeyValueEditMode;
    
    match state.kv_edit_mode {
        KeyValueEditMode::None => {
            match key.code {
                KeyCode::Up => {
                    if state.headers_selected > 0 {
                        state.headers_selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if state.headers_selected < state.headers_input.len().saturating_sub(1) {
                        state.headers_selected += 1;
                    }
                }
                KeyCode::Char('+') => {
                    state.add_header();
                    // Automatically start editing the new header's key
                    state.kv_edit_mode = KeyValueEditMode::Key;
                }
                KeyCode::Char('-') | KeyCode::Delete => {
                    state.delete_header();
                }
                KeyCode::Enter => {
                    if !state.headers_input.is_empty() && state.headers_selected < state.headers_input.len() {
                        state.kv_edit_mode = KeyValueEditMode::Key;
                    }
                }
                _ => {}
            }
        }
        KeyValueEditMode::Key => {
            match key.code {
                KeyCode::Esc => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                KeyCode::Tab => {
                    state.kv_edit_mode = KeyValueEditMode::Value;
                }
                KeyCode::Char(c) => {
                    if let Some((key, _)) = state.headers_input.get_mut(state.headers_selected) {
                        key.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some((key, _)) = state.headers_input.get_mut(state.headers_selected) {
                        key.pop();
                    }
                }
                _ => {}
            }
        }
        KeyValueEditMode::Value => {
            match key.code {
                KeyCode::Esc => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                KeyCode::Tab => {
                    state.kv_edit_mode = KeyValueEditMode::Key;
                }
                KeyCode::Char(c) => {
                    if let Some((_, value)) = state.headers_input.get_mut(state.headers_selected) {
                        value.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some((_, value)) = state.headers_input.get_mut(state.headers_selected) {
                        value.pop();
                    }
                }
                _ => {}
            }
        }
    }
}

fn handle_body_edit(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            state.body_input.insert(state.body_cursor, c);
            state.body_cursor += 1;
        }
        KeyCode::Backspace => {
            if state.body_cursor > 0 {
                state.body_cursor -= 1;
                state.body_input.remove(state.body_cursor);
            }
        }
        KeyCode::Delete => {
            if state.body_cursor < state.body_input.len() {
                state.body_input.remove(state.body_cursor);
            }
        }
        KeyCode::Left => {
            if state.body_cursor > 0 {
                state.body_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.body_cursor < state.body_input.len() {
                state.body_cursor += 1;
            }
        }
        KeyCode::Home => {
            state.body_cursor = 0;
        }
        KeyCode::End => {
            state.body_cursor = state.body_input.len();
        }
        KeyCode::Enter => {
            state.body_input.insert(state.body_cursor, '\n');
            state.body_cursor += 1;
        }
        _ => {}
    }
}

fn handle_auth_edit(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            state.auth_input.insert(state.auth_cursor, c);
            state.auth_cursor += 1;
        }
        KeyCode::Backspace => {
            if state.auth_cursor > 0 {
                state.auth_cursor -= 1;
                state.auth_input.remove(state.auth_cursor);
            }
        }
        KeyCode::Delete => {
            if state.auth_cursor < state.auth_input.len() {
                state.auth_input.remove(state.auth_cursor);
            }
        }
        KeyCode::Left => {
            if state.auth_cursor > 0 {
                state.auth_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.auth_cursor < state.auth_input.len() {
                state.auth_cursor += 1;
            }
        }
        KeyCode::Home => {
            state.auth_cursor = 0;
        }
        KeyCode::End => {
            state.auth_cursor = state.auth_input.len();
        }
        _ => {}
    }
}

fn handle_collection_edit_mode(state: &mut AppState, key: KeyEvent, storage: &storage::Storage) {
    match key.code {
        KeyCode::Esc => {
            state.cancel_collection_editing();
        }
        KeyCode::Enter => {
            state.save_collection_name();
            if let Some(idx) = state.selected_collection {
                if let Some(collection) = state.collections.get(idx) {
                    let _ = storage.save_collection(collection);
                }
            }
        }
        KeyCode::Char(c) => {
            state.collection_name_input.insert(state.collection_name_cursor, c);
            state.collection_name_cursor += 1;
        }
        KeyCode::Backspace => {
            if state.collection_name_cursor > 0 {
                state.collection_name_cursor -= 1;
                state.collection_name_input.remove(state.collection_name_cursor);
            }
        }
        KeyCode::Delete => {
            if state.collection_name_cursor < state.collection_name_input.len() {
                state.collection_name_input.remove(state.collection_name_cursor);
            }
        }
        KeyCode::Left => {
            if state.collection_name_cursor > 0 {
                state.collection_name_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.collection_name_cursor < state.collection_name_input.len() {
                state.collection_name_cursor += 1;
            }
        }
        KeyCode::Home => {
            state.collection_name_cursor = 0;
        }
        KeyCode::End => {
            state.collection_name_cursor = state.collection_name_input.len();
        }
        _ => {}
    }
}


