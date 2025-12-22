mod app;
mod grpc;
mod http;
mod import;
mod models;
mod storage;
mod ui;
mod utils;

use app::state::{AppState, InputMode, Panel, EditorField};
use app::actions::Action;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, poll},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio::sync::mpsc;
use uuid::Uuid;

enum HttpResult {
    Success(models::response::HttpResponse),
    Error(String),
}

#[allow(dead_code)]
enum GrpcResult {
    Success(models::GrpcResponse),
    Error(String),
}

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
    state.grpc_requests = storage.load_grpc_requests()?;
    state.proto_schemas = storage.load_proto_schemas()?;
    
    // If no data exists, create sample data
    if state.collections.is_empty() {
        let default_collection = models::collection::Collection::new("Example Collection".to_string());
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
    let (response_tx, mut response_rx) = mpsc::channel::<HttpResult>(32);
    
    loop {
        while let Ok(result) = response_rx.try_recv() {
            match result {
                HttpResult::Success(response) => {
                    state.current_response = Some(response);
                    state.loading_message.clear();
                }
                HttpResult::Error(error_msg) => {
                    let error_response = models::response::HttpResponse {
                        id: Uuid::new_v4(),
                        request_id: Uuid::new_v4(),
                        status_code: 0,
                        status_text: "Request Failed".to_string(),
                        headers: std::collections::HashMap::new(),
                        body: Vec::new(),
                        body_text: Some(error_msg),
                        duration_ms: 0,
                        size_bytes: 0,
                        timestamp: chrono::Utc::now(),
                        error: None,
                    };
                    state.current_response = Some(error_response);
                    state.loading_message.clear();
                }
            }
            state.is_loading = false;
        }
        
        terminal.draw(|frame| {
            ui::app::UI::draw(frame, &state);
        })?;
        
        if poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
            if state.show_welcome {
                // Any key dismisses the welcome screen
                state.show_welcome = false;
                continue;
            }
            
            if state.show_export_menu {
                handle_export_menu(&mut state, key);
                continue;
            }

            if state.show_import_menu {
                handle_import_menu(&mut state, key, &storage);
                continue;
            }

            if state.show_help {
                match key.code {
                    KeyCode::Char('?') | KeyCode::Esc => {
                        Action::ToggleHelp.execute(&mut state);
                    }
                    _ => {}
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
                        if !state.is_loading {
                            state.is_loading = true;
                            state.loading_message = format!("Sending {} request...", request.method.as_str());
                            state.reset_response_scroll();
                            state.current_response = None;
                            
                            let client = http_client.clone();
                            let tx = response_tx.clone();
                            
                            tokio::spawn(async move {
                                let result = match client.execute(&request).await {
                                    Ok(response) => HttpResult::Success(response),
                                    Err(e) => HttpResult::Error(e.to_string()),
                                };
                                let _ = tx.send(result).await;
                            });
                        }
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
                (KeyCode::Char('o'), KeyModifiers::NONE) => {
                    Action::OpenExportMenu.execute(&mut state);
                }
                (KeyCode::Char('s'), KeyModifiers::NONE) => {
                    Action::OpenCurlExportMenu.execute(&mut state);
                }
                (KeyCode::Char('i'), KeyModifiers::NONE) => {
                    Action::OpenImportMenu.execute(&mut state);
                }
                _ => {}
            }
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
                // Switch between fields in edit mode (forward)
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
        KeyCode::BackTab => {
            // Check if we're in key-value editing mode for params or headers
            if (state.editor_focused_field == EditorField::Params || 
                state.editor_focused_field == EditorField::Headers) && 
               state.kv_edit_mode != app::state::KeyValueEditMode::None {
                // Let the specific field handler deal with Shift+Tab in key-value editing mode
                match state.editor_focused_field {
                    EditorField::Params => handle_params_edit(state, key),
                    EditorField::Headers => handle_headers_edit(state, key),
                    _ => {}
                }
            } else {
                // Switch between fields in edit mode (backward)
                state.kv_edit_mode = app::state::KeyValueEditMode::None; // Reset KV edit mode when switching fields
                state.editor_focused_field = match state.editor_focused_field {
                    EditorField::Name => EditorField::Auth,
                    EditorField::Method => EditorField::Name,
                    EditorField::Url => EditorField::Method,
                    EditorField::Params => EditorField::Url,
                    EditorField::Headers => EditorField::Params,
                    EditorField::Body => EditorField::Headers,
                    EditorField::Auth => EditorField::Body,
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
    match (key.code, key.modifiers) {
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            state.name_input.clear();
            state.name_cursor = 0;
        }
        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
            state.name_input.insert(state.name_cursor, c);
            state.name_cursor += 1;
        }
        (KeyCode::Backspace, _) => {
            if state.name_cursor > 0 {
                state.name_cursor -= 1;
                state.name_input.remove(state.name_cursor);
            }
        }
        (KeyCode::Delete, _) => {
            if state.name_cursor < state.name_input.len() {
                state.name_input.remove(state.name_cursor);
            }
        }
        (KeyCode::Left, _) => {
            if state.name_cursor > 0 {
                state.name_cursor -= 1;
            }
        }
        (KeyCode::Right, _) => {
            if state.name_cursor < state.name_input.len() {
                state.name_cursor += 1;
            }
        }
        (KeyCode::Home, _) => {
            state.name_cursor = 0;
        }
        (KeyCode::End, _) => {
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
    match (key.code, key.modifiers) {
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            state.url_input.clear();
            state.url_cursor = 0;
        }
        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
            state.url_input.insert(state.url_cursor, c);
            state.url_cursor += 1;
        }
        (KeyCode::Backspace, _) => {
            if state.url_cursor > 0 {
                state.url_cursor -= 1;
                state.url_input.remove(state.url_cursor);
            }
        }
        (KeyCode::Delete, _) => {
            if state.url_cursor < state.url_input.len() {
                state.url_input.remove(state.url_cursor);
            }
        }
        (KeyCode::Left, _) => {
            if state.url_cursor > 0 {
                state.url_cursor -= 1;
            }
        }
        (KeyCode::Right, _) => {
            if state.url_cursor < state.url_input.len() {
                state.url_cursor += 1;
            }
        }
        (KeyCode::Home, _) => {
            state.url_cursor = 0;
        }
        (KeyCode::End, _) => {
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
            match (key.code, key.modifiers) {
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                    if let Some((key, _)) = state.params_input.get_mut(state.params_selected) {
                        key.clear();
                    }
                }
                (KeyCode::Esc, _) => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                (KeyCode::Tab, _) => {
                    state.kv_edit_mode = KeyValueEditMode::Value;
                }
                (KeyCode::Char(c), _) => {
                    if let Some((key, _)) = state.params_input.get_mut(state.params_selected) {
                        key.push(c);
                    }
                }
                (KeyCode::Backspace, _) => {
                    if let Some((key, _)) = state.params_input.get_mut(state.params_selected) {
                        key.pop();
                    }
                }
                _ => {}
            }
        }
        KeyValueEditMode::Value => {
            match (key.code, key.modifiers) {
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                    if let Some((_, value)) = state.params_input.get_mut(state.params_selected) {
                        value.clear();
                    }
                }
                (KeyCode::Esc, _) => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                (KeyCode::Tab, _) => {
                    state.kv_edit_mode = KeyValueEditMode::Key;
                }
                (KeyCode::Char(c), _) => {
                    if let Some((_, value)) = state.params_input.get_mut(state.params_selected) {
                        value.push(c);
                    }
                }
                (KeyCode::Backspace, _) => {
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
            match (key.code, key.modifiers) {
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                    if let Some((key, _)) = state.headers_input.get_mut(state.headers_selected) {
                        key.clear();
                    }
                }
                (KeyCode::Esc, _) => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                (KeyCode::Tab, _) => {
                    state.kv_edit_mode = KeyValueEditMode::Value;
                }
                (KeyCode::Char(c), _) => {
                    if let Some((key, _)) = state.headers_input.get_mut(state.headers_selected) {
                        key.push(c);
                    }
                }
                (KeyCode::Backspace, _) => {
                    if let Some((key, _)) = state.headers_input.get_mut(state.headers_selected) {
                        key.pop();
                    }
                }
                _ => {}
            }
        }
        KeyValueEditMode::Value => {
            match (key.code, key.modifiers) {
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                    if let Some((_, value)) = state.headers_input.get_mut(state.headers_selected) {
                        value.clear();
                    }
                }
                (KeyCode::Esc, _) => {
                    state.kv_edit_mode = KeyValueEditMode::None;
                }
                (KeyCode::Tab, _) => {
                    state.kv_edit_mode = KeyValueEditMode::Key;
                }
                (KeyCode::Char(c), _) => {
                    if let Some((_, value)) = state.headers_input.get_mut(state.headers_selected) {
                        value.push(c);
                    }
                }
                (KeyCode::Backspace, _) => {
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
    match (key.code, key.modifiers) {
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            state.body_input.clear();
            state.body_cursor = 0;
        }
        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
            state.body_input.insert(state.body_cursor, c);
            state.body_cursor += 1;
        }
        (KeyCode::Backspace, _) => {
            if state.body_cursor > 0 {
                state.body_cursor -= 1;
                state.body_input.remove(state.body_cursor);
            }
        }
        (KeyCode::Delete, _) => {
            if state.body_cursor < state.body_input.len() {
                state.body_input.remove(state.body_cursor);
            }
        }
        (KeyCode::Left, _) => {
            if state.body_cursor > 0 {
                state.body_cursor -= 1;
            }
        }
        (KeyCode::Right, _) => {
            if state.body_cursor < state.body_input.len() {
                state.body_cursor += 1;
            }
        }
        (KeyCode::Up, _) => {
            state.body_cursor = move_cursor_up(&state.body_input, state.body_cursor);
        }
        (KeyCode::Down, _) => {
            state.body_cursor = move_cursor_down(&state.body_input, state.body_cursor);
        }
        (KeyCode::Home, _) => {
            state.body_cursor = move_cursor_to_line_start(&state.body_input, state.body_cursor);
        }
        (KeyCode::End, _) => {
            state.body_cursor = move_cursor_to_line_end(&state.body_input, state.body_cursor);
        }
        (KeyCode::Enter, _) => {
            state.body_input.insert(state.body_cursor, '\n');
            state.body_cursor += 1;
        }
        _ => {}
    }
}

fn handle_auth_edit(state: &mut AppState, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            state.auth_input.clear();
            state.auth_cursor = 0;
        }
        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
            state.auth_input.insert(state.auth_cursor, c);
            state.auth_cursor += 1;
        }
        (KeyCode::Backspace, _) => {
            if state.auth_cursor > 0 {
                state.auth_cursor -= 1;
                state.auth_input.remove(state.auth_cursor);
            }
        }
        (KeyCode::Delete, _) => {
            if state.auth_cursor < state.auth_input.len() {
                state.auth_input.remove(state.auth_cursor);
            }
        }
        (KeyCode::Left, _) => {
            if state.auth_cursor > 0 {
                state.auth_cursor -= 1;
            }
        }
        (KeyCode::Right, _) => {
            if state.auth_cursor < state.auth_input.len() {
                state.auth_cursor += 1;
            }
        }
        (KeyCode::Up, _) => {
            state.auth_cursor = move_cursor_up(&state.auth_input, state.auth_cursor);
        }
        (KeyCode::Down, _) => {
            state.auth_cursor = move_cursor_down(&state.auth_input, state.auth_cursor);
        }
        (KeyCode::Home, _) => {
            state.auth_cursor = move_cursor_to_line_start(&state.auth_input, state.auth_cursor);
        }
        (KeyCode::End, _) => {
            state.auth_cursor = move_cursor_to_line_end(&state.auth_input, state.auth_cursor);
        }
        (KeyCode::Enter, _) => {
            state.auth_input.insert(state.auth_cursor, '\n');
            state.auth_cursor += 1;
        }
        _ => {}
    }
}

fn handle_export_menu(state: &mut AppState, key: KeyEvent) {
    use app::state::{ExportMode, ExportMenuStage};
    
    match state.export_menu_stage {
        ExportMenuStage::ShowingResult => {
            // Check if user wants to copy the filename
            if key.code == KeyCode::Char('c') || key.code == KeyCode::Char('y') {
                // Copy the exported filename to clipboard
                if let Some(filepath) = &state.export_result_message {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        // Extract just the filename from the full path
                        let filename = std::path::Path::new(filepath)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(filepath);
                        let _ = clipboard.set_text(filename.to_string());
                    }
                }
            }
            
            // Any key closes the menu after showing result
            state.show_export_menu = false;
            state.export_result_message = None;
            state.export_selected_collection = None;
            state.export_selected_request = None;
            state.export_mode = None;
        }
        ExportMenuStage::SelectingCollection => {
            match key.code {
                KeyCode::Esc => {
                    state.show_export_menu = false;
                    state.export_selected_collection = None;
                    state.export_mode = None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let Some(idx) = state.export_selected_collection {
                        if idx > 0 {
                            state.export_selected_collection = Some(idx - 1);
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(idx) = state.export_selected_collection {
                        if idx < state.collections.len().saturating_sub(1) {
                            state.export_selected_collection = Some(idx + 1);
                        }
                    }
                }
                KeyCode::Enter => {
                    match state.export_mode {
                        Some(ExportMode::CollectionJson) => {
                            Action::ExportCollectionJson.execute(state);
                        }
                        Some(ExportMode::RequestCurl) => {
                            // Move to request selection stage
                            if let Some(collection_idx) = state.export_selected_collection {
                                if let Some(collection) = state.collections.get(collection_idx) {
                                    // Find first request in this collection
                                    let first_request_idx = state.requests
                                        .iter()
                                        .enumerate()
                                        .find(|(_, r)| r.collection_id == Some(collection.id))
                                        .map(|(idx, _)| idx);
                                    
                                    state.export_selected_request = first_request_idx;
                                    state.export_menu_stage = ExportMenuStage::SelectingRequest;
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
        ExportMenuStage::SelectingRequest => {
            match key.code {
                KeyCode::Esc => {
                    // Go back to collection selection
                    state.export_menu_stage = ExportMenuStage::SelectingCollection;
                    state.export_selected_request = None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let Some(current_idx) = state.export_selected_request {
                        if let Some(collection_idx) = state.export_selected_collection {
                            if let Some(collection) = state.collections.get(collection_idx) {
                                // Find previous request in this collection
                                let requests_in_collection: Vec<usize> = state.requests
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, r)| r.collection_id == Some(collection.id))
                                    .map(|(idx, _)| idx)
                                    .collect();
                                
                                if let Some(pos) = requests_in_collection.iter().position(|&idx| idx == current_idx) {
                                    if pos > 0 {
                                        state.export_selected_request = Some(requests_in_collection[pos - 1]);
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(current_idx) = state.export_selected_request {
                        if let Some(collection_idx) = state.export_selected_collection {
                            if let Some(collection) = state.collections.get(collection_idx) {
                                // Find next request in this collection
                                let requests_in_collection: Vec<usize> = state.requests
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, r)| r.collection_id == Some(collection.id))
                                    .map(|(idx, _)| idx)
                                    .collect();
                                
                                if let Some(pos) = requests_in_collection.iter().position(|&idx| idx == current_idx) {
                                    if pos < requests_in_collection.len() - 1 {
                                        state.export_selected_request = Some(requests_in_collection[pos + 1]);
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    Action::ExportRequestCurl.execute(state);
                }
                _ => {}
            }
        }
    }
}

fn handle_import_menu(state: &mut AppState, key: KeyEvent, storage: &storage::Storage) {
    // If showing result, any key closes the menu
    if state.import_result_message.is_some() {
        state.show_import_menu = false;
        state.import_result_message = None;
        state.import_file_input.clear();
        state.import_file_cursor = 0;
        return;
    }

    // Otherwise, handle file input
    match (key.code, key.modifiers) {
        (KeyCode::Esc, _) => {
            state.show_import_menu = false;
            state.import_file_input.clear();
            state.import_file_cursor = 0;
        }
        (KeyCode::Enter, _) => {
            // Store the counts before import to identify what was imported
            let collections_before = state.collections.len();
            let requests_before = state.requests.len();

            Action::ImportPostmanCollection.execute(state);

            // If import was successful, save the new collection and requests to storage
            if state.import_result_message.is_some() &&
               !state.import_result_message.as_ref().unwrap().starts_with("Error") {
                // Save the newly imported collection
                if state.collections.len() > collections_before {
                    if let Some(collection) = state.collections.last() {
                        let _ = storage.save_collection(collection);
                    }
                }

                // Save all newly imported requests
                for request in state.requests.iter().skip(requests_before) {
                    let _ = storage.save_request(request);
                }
            }
        }
        (KeyCode::Tab, _) => {
            // Tab autocomplete for file paths
            autocomplete_file_path(state);
        }
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            state.import_file_input.clear();
            state.import_file_cursor = 0;
        }
        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
            state.import_file_input.insert(state.import_file_cursor, c);
            state.import_file_cursor += 1;
        }
        (KeyCode::Backspace, _) => {
            if state.import_file_cursor > 0 {
                state.import_file_cursor -= 1;
                state.import_file_input.remove(state.import_file_cursor);
            }
        }
        (KeyCode::Delete, _) => {
            if state.import_file_cursor < state.import_file_input.len() {
                state.import_file_input.remove(state.import_file_cursor);
            }
        }
        (KeyCode::Left, _) => {
            if state.import_file_cursor > 0 {
                state.import_file_cursor -= 1;
            }
        }
        (KeyCode::Right, _) => {
            if state.import_file_cursor < state.import_file_input.len() {
                state.import_file_cursor += 1;
            }
        }
        (KeyCode::Home, _) => {
            state.import_file_cursor = 0;
        }
        (KeyCode::End, _) => {
            state.import_file_cursor = state.import_file_input.len();
        }
        _ => {}
    }
}

fn autocomplete_file_path(state: &mut AppState) {
    use std::path::{Path, PathBuf};

    let input = state.import_file_input.trim();
    if input.is_empty() {
        return;
    }

    // Expand ~ to home directory
    let expanded = if input.starts_with("~/") {
        if let Some(home) = std::env::var("HOME").ok() {
            input.replacen("~", &home, 1)
        } else {
            input.to_string()
        }
    } else {
        input.to_string()
    };

    let path = Path::new(&expanded);

    // Determine the directory to search and the prefix to match
    let (search_dir, prefix) = if expanded.ends_with('/') || expanded.ends_with('\\') {
        // User typed a trailing slash, search in that directory
        (path.to_path_buf(), String::new())
    } else if let Some(parent) = path.parent() {
        // Search in parent directory for files matching the filename
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        (parent.to_path_buf(), file_name)
    } else {
        // No parent, search in current directory
        (PathBuf::from("."), expanded.clone())
    };

    // Read directory and find matches
    if let Ok(entries) = std::fs::read_dir(&search_dir) {
        let mut matches: Vec<String> = entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.starts_with(&prefix) && !file_name.starts_with('.') {
                    Some(file_name)
                } else {
                    None
                }
            })
            .collect();

        matches.sort();

        if matches.is_empty() {
            return;
        }

        if matches.len() == 1 {
            // Single match - complete it
            let mut full_path = search_dir.join(&matches[0]);

            // Add trailing slash for directories
            if full_path.is_dir() {
                full_path.push("");
            }

            // Convert back to string, preserve the original prefix
            let mut full_str = full_path.to_string_lossy().to_string();

            // Preserve ~ if it was used
            if input.starts_with("~/") {
                if let Some(home) = std::env::var("HOME").ok() {
                    full_str = full_str.replace(&home, "~");
                }
            }
            // Preserve ./ if it was used and we're in current directory
            else if input.starts_with("./") && search_dir == Path::new(".") {
                if !full_str.starts_with("./") {
                    full_str = format!("./{}", full_str);
                }
            }

            state.import_file_input = full_str;
            state.import_file_cursor = state.import_file_input.len();
        } else {
            // Multiple matches - complete to common prefix
            let common_prefix = find_common_prefix(&matches);
            if common_prefix.len() > prefix.len() {
                let full_path = search_dir.join(&common_prefix);
                let mut full_str = full_path.to_string_lossy().to_string();

                // Preserve ~ if it was used
                if input.starts_with("~/") {
                    if let Some(home) = std::env::var("HOME").ok() {
                        full_str = full_str.replace(&home, "~");
                    }
                }
                // Preserve ./ if it was used and we're in current directory
                else if input.starts_with("./") && search_dir == Path::new(".") {
                    if !full_str.starts_with("./") {
                        full_str = format!("./{}", full_str);
                    }
                }

                state.import_file_input = full_str;
                state.import_file_cursor = state.import_file_input.len();
            }
        }
    }
}

fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    if strings.len() == 1 {
        return strings[0].clone();
    }

    let mut prefix = String::new();
    let first = &strings[0];

    for (i, ch) in first.chars().enumerate() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(ch)) {
            prefix.push(ch);
        } else {
            break;
        }
    }

    prefix
}

fn handle_collection_edit_mode(state: &mut AppState, key: KeyEvent, storage: &storage::Storage) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            state.collection_name_input.clear();
            state.collection_name_cursor = 0;
        }
        (KeyCode::Esc, _) => {
            state.cancel_collection_editing();
        }
        (KeyCode::Enter, _) => {
            state.save_collection_name();
            if let Some(idx) = state.selected_collection {
                if let Some(collection) = state.collections.get(idx) {
                    let _ = storage.save_collection(collection);
                }
            }
        }
        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
            state.collection_name_input.insert(state.collection_name_cursor, c);
            state.collection_name_cursor += 1;
        }
        (KeyCode::Backspace, _) => {
            if state.collection_name_cursor > 0 {
                state.collection_name_cursor -= 1;
                state.collection_name_input.remove(state.collection_name_cursor);
            }
        }
        (KeyCode::Delete, _) => {
            if state.collection_name_cursor < state.collection_name_input.len() {
                state.collection_name_input.remove(state.collection_name_cursor);
            }
        }
        (KeyCode::Left, _) => {
            if state.collection_name_cursor > 0 {
                state.collection_name_cursor -= 1;
            }
        }
        (KeyCode::Right, _) => {
            if state.collection_name_cursor < state.collection_name_input.len() {
                state.collection_name_cursor += 1;
            }
        }
        (KeyCode::Home, _) => {
            state.collection_name_cursor = 0;
        }
        (KeyCode::End, _) => {
            state.collection_name_cursor = state.collection_name_input.len();
        }
        _ => {}
    }
}

// Helper functions for multiline text navigation

/// Move cursor up one line, maintaining column position when possible
fn move_cursor_up(text: &str, cursor_pos: usize) -> usize {
    if cursor_pos == 0 {
        return 0;
    }
    
    // Find the start of the current line
    let current_line_start = text[..cursor_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    
    // If we're already on the first line, move to the start
    if current_line_start == 0 {
        return 0;
    }
    
    // Calculate column position in current line
    let column = cursor_pos - current_line_start;
    
    // Find the start of the previous line
    let prev_line_start = text[..current_line_start - 1]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    
    // Find the end of the previous line (excluding the newline)
    let prev_line_end = current_line_start - 1;
    let prev_line_len = prev_line_end - prev_line_start;
    
    // Move to the same column or to the end of the previous line if it's shorter
    prev_line_start + column.min(prev_line_len)
}

/// Move cursor down one line, maintaining column position when possible
fn move_cursor_down(text: &str, cursor_pos: usize) -> usize {
    if cursor_pos >= text.len() {
        return text.len();
    }
    
    // Find the start of the current line
    let current_line_start = text[..cursor_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    
    // Find the end of the current line
    let current_line_end = text[cursor_pos..]
        .find('\n')
        .map(|pos| cursor_pos + pos)
        .unwrap_or(text.len());
    
    // If we're on the last line, move to the end
    if current_line_end >= text.len() {
        return text.len();
    }
    
    // Calculate column position in current line
    let column = cursor_pos - current_line_start;
    
    // The next line starts after the newline
    let next_line_start = current_line_end + 1;
    
    // Find the end of the next line
    let next_line_end = text[next_line_start..]
        .find('\n')
        .map(|pos| next_line_start + pos)
        .unwrap_or(text.len());
    
    let next_line_len = next_line_end - next_line_start;
    
    // Move to the same column or to the end of the next line if it's shorter
    next_line_start + column.min(next_line_len)
}

/// Move cursor to the start of the current line
fn move_cursor_to_line_start(text: &str, cursor_pos: usize) -> usize {
    text[..cursor_pos]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0)
}

/// Move cursor to the end of the current line
fn move_cursor_to_line_end(text: &str, cursor_pos: usize) -> usize {
    text[cursor_pos..]
        .find('\n')
        .map(|pos| cursor_pos + pos)
        .unwrap_or(text.len())
}


