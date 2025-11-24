mod app;
mod http;
mod models;
mod storage;
mod ui;
mod utils;
mod export;
mod import;

use app::state::AppState;
use app::actions::Action;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
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
                    Action::NextEditorTab.execute(&mut state);
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

