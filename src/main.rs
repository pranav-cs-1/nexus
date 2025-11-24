mod app;
mod http;
mod models;
mod storage;
mod ui;
mod utils;
mod export;
mod import;

use app::state::{AppState, InputMode, Panel};
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
    
    let mut state = AppState::new();
    
    state.requests.push(models::request::HttpRequest::new(
        "Get JSONPlaceholder Post".to_string(),
        models::request::HttpMethod::GET,
        "https://jsonplaceholder.typicode.com/posts/1".to_string(),
    ));
    
    state.requests.push(models::request::HttpRequest::new(
        "List JSONPlaceholder Posts".to_string(),
        models::request::HttpMethod::GET,
        "https://jsonplaceholder.typicode.com/posts".to_string(),
    ));
    
    state.requests.push(
        models::request::HttpRequest::new(
            "Create Post".to_string(),
            models::request::HttpMethod::POST,
            "https://jsonplaceholder.typicode.com/posts".to_string(),
        )
        .with_header("Content-Type".to_string(), "application/json".to_string())
        .with_body(r#"{"title": "foo", "body": "bar", "userId": 1}"#.to_string())
    );
    
    state.requests.push(
        models::request::HttpRequest::new(
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
        .with_body(r#"{"filter": {"userId": 1}, "fields": ["id", "title", "body"]}"#.to_string())
    );
    
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
                handle_edit_mode(&mut state, key);
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
                        _ => {}
                    }
                }
                (KeyCode::Up | KeyCode::Char('k'), KeyModifiers::NONE) => {
                    match state.focused_panel {
                        app::state::Panel::Collections => Action::PrevCollection.execute(&mut state),
                        app::state::Panel::Requests => Action::PrevRequest.execute(&mut state),
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
                }
                (KeyCode::Char('d'), KeyModifiers::NONE) => {
                    Action::DeleteRequest.execute(&mut state);
                }
                (KeyCode::Char('y'), KeyModifiers::NONE) => {
                    Action::DuplicateRequest.execute(&mut state);
                }
                (KeyCode::Char('e'), KeyModifiers::NONE) => {
                    if state.focused_panel == Panel::RequestEditor {
                        state.load_current_request_to_input();
                        state.input_mode = InputMode::Editing;
                    }
                }
                (KeyCode::Char('c'), KeyModifiers::NONE) => {
                    if state.focused_panel == Panel::Collections {
                        Action::NewCollection.execute(&mut state);
                    }
                }
                (KeyCode::Char('x'), KeyModifiers::NONE) => {
                    if state.focused_panel == Panel::Collections {
                        Action::DeleteCollection.execute(&mut state);
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

fn handle_edit_mode(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.save_input_to_request();
            state.input_mode = InputMode::Normal;
        }
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


