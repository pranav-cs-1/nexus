use crate::app::state::{AppState, EditorTab, InputMode, Panel};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Widget},
};

pub struct RequestEditor<'a> {
    state: &'a AppState,
}

impl<'a> RequestEditor<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for RequestEditor<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let is_focused = self.state.focused_panel == Panel::RequestEditor;
        
        let border_style = if is_focused {
            Theme::focused_border()
        } else {
            Theme::unfocused_border()
        };
        
        
        let block = Block::default()
            .title("Request Editor")
            .borders(Borders::ALL)
            .border_style(border_style);
        
        let inner_area = block.inner(area);
        block.render(area, buf);
        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(inner_area);
        
        if let Some(request) = self.state.get_current_request() {
            let is_editing_url = self.state.input_mode == InputMode::Editing 
                && self.state.focused_panel == Panel::RequestEditor;
            
            let url_title = if is_editing_url {
                "URL [EDITING - ESC to finish]"
            } else {
                "URL [Press 'e' to edit]"
            };
            
            let url_block = Block::default()
                .title(url_title)
                .borders(Borders::ALL)
                .border_style(if is_editing_url {
                    Theme::selected()
                } else {
                    Theme::unfocused_border()
                });
            
            let url_display = if is_editing_url {
                &self.state.url_input
            } else {
                &request.url
            };
            
            let url_line = if is_editing_url {
                let cursor_pos = self.state.url_cursor;
                let before = url_display.chars().take(cursor_pos).collect::<String>();
                let cursor_char = url_display.chars().nth(cursor_pos).unwrap_or(' ');
                let after = url_display.chars().skip(cursor_pos + 1).collect::<String>();
                
                Line::from(vec![
                    Span::raw(before),
                    Span::styled(cursor_char.to_string(), Theme::selected()),
                    Span::raw(after),
                ])
            } else {
                Line::from(url_display.as_str())
            };
            
            let url_text = Paragraph::new(url_line)
                .block(url_block);
            url_text.render(inner_chunks[0], buf);
            
            let tabs = Tabs::new(vec!["Params", "Headers", "Body", "Auth"])
                .select(match self.state.editor_tab {
                    EditorTab::Params => 0,
                    EditorTab::Headers => 1,
                    EditorTab::Body => 2,
                    EditorTab::Auth => 3,
                })
                .style(Theme::default())
                .highlight_style(Theme::selected());
            tabs.render(inner_chunks[1], buf);
            
            let content = match self.state.editor_tab {
                EditorTab::Params => {
                    if request.query_params.is_empty() {
                        "No query parameters".to_string()
                    } else {
                        request.query_params
                            .iter()
                            .map(|(k, v)| format!("{}: {}", k, v))
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                }
                EditorTab::Headers => {
                    if request.headers.is_empty() {
                        "No headers".to_string()
                    } else {
                        request.headers
                            .iter()
                            .map(|(k, v)| format!("{}: {}", k, v))
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                }
                EditorTab::Body => {
                    request.body.as_deref().unwrap_or("No body").to_string()
                }
                EditorTab::Auth => {
                    match &request.auth {
                        crate::models::request::AuthType::None => "No authentication".to_string(),
                        crate::models::request::AuthType::Bearer { token } => {
                            format!("Bearer: {}", token)
                        }
                        crate::models::request::AuthType::Basic { username, password } => {
                            format!("Basic: {} / {}", username, password)
                        }
                        crate::models::request::AuthType::ApiKey { key, value, location } => {
                            format!("API Key: {} = {} ({:?})", key, value, location)
                        }
                    }
                }
            };
            
            let content_paragraph = Paragraph::new(content);
            content_paragraph.render(inner_chunks[2], buf);
        } else {
            let no_request = Paragraph::new("No request selected")
                .block(Block::default());
            no_request.render(inner_area, buf);
        }
    }
}

