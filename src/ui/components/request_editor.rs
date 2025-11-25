use crate::app::state::{AppState, EditorTab, InputMode, Panel, EditorField, KeyValueEditMode};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Widget, Wrap},
    style::Style,
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
        let is_editing = self.state.input_mode == InputMode::Editing && is_focused;
        
        let border_style = if is_focused {
            Theme::focused_border()
        } else {
            Theme::unfocused_border()
        };
        
        let title = if is_editing {
            "Request Editor [EDITING - ESC to save, Tab to switch fields]"
        } else {
            "Request Editor [Press 'e' to edit]"
        };
        
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);
        
        let inner_area = block.inner(area);
        block.render(area, buf);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Name + Method
                Constraint::Length(3), // URL
                Constraint::Length(1), // Tabs
                Constraint::Min(0),    // Content
            ])
            .split(inner_area);
        
        if let Some(request) = self.state.get_current_request() {
            // Name and Method row
            let name_method_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[0]);
            
            self.render_name_field(name_method_chunks[0], buf, request, is_editing);
            self.render_method_field(name_method_chunks[1], buf, request, is_editing);
            
            // URL field
            self.render_url_field(chunks[1], buf, request, is_editing);
            
            // Tabs
            let tabs = Tabs::new(vec!["Params", "Headers", "Body", "Auth"])
                .select(match self.state.editor_tab {
                    EditorTab::Params => 0,
                    EditorTab::Headers => 1,
                    EditorTab::Body => 2,
                    EditorTab::Auth => 3,
                })
                .style(Theme::default())
                .highlight_style(Theme::selected());
            tabs.render(chunks[2], buf);
            
            // Content area
            match self.state.editor_tab {
                EditorTab::Params => self.render_params_content(chunks[3], buf, request, is_editing),
                EditorTab::Headers => self.render_headers_content(chunks[3], buf, request, is_editing),
                EditorTab::Body => self.render_body_content(chunks[3], buf, request, is_editing),
                EditorTab::Auth => self.render_auth_content(chunks[3], buf, request, is_editing),
            }
        } else {
            let no_request = Paragraph::new("No request selected")
                .block(Block::default());
            no_request.render(inner_area, buf);
        }
    }
}

impl<'a> RequestEditor<'a> {
    fn render_name_field(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, request: &crate::models::request::HttpRequest, is_editing: bool) {
        let is_focused = is_editing && self.state.editor_focused_field == EditorField::Name;
        
        let block = Block::default()
            .title("Name")
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });
        
        let text = if is_editing {
            let display_text = &self.state.name_input;
            if is_focused {
                let cursor_pos = self.state.name_cursor;
                let before = display_text.chars().take(cursor_pos).collect::<String>();
                let cursor_char = display_text.chars().nth(cursor_pos).unwrap_or(' ');
                let after = display_text.chars().skip(cursor_pos + 1).collect::<String>();
                
                Line::from(vec![
                    Span::raw(before),
                    Span::styled(cursor_char.to_string(), Theme::selected()),
                    Span::raw(after),
                ])
            } else {
                Line::from(display_text.as_str())
            }
        } else {
            Line::from(request.name.as_str())
        };
        
        Paragraph::new(text).block(block).render(area, buf);
    }
    
    fn render_method_field(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, request: &crate::models::request::HttpRequest, is_editing: bool) {
        let is_focused = is_editing && self.state.editor_focused_field == EditorField::Method;
        
        let block = Block::default()
            .title("Method")
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });
        
        let text = if is_editing {
            let methods = crate::models::request::HttpMethod::all();
            let method_str = if let Some(method) = methods.get(self.state.method_input) {
                method.as_str().to_string()
            } else {
                "GET".to_string()
            };
            Line::from(method_str)
        } else {
            Line::from(request.method.as_str())
        };
        
        Paragraph::new(text).block(block).render(area, buf);
    }
    
    fn render_url_field(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, request: &crate::models::request::HttpRequest, is_editing: bool) {
        let is_focused = is_editing && self.state.editor_focused_field == EditorField::Url;
        
        let block = Block::default()
            .title("URL")
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });
        
        let text = if is_editing {
            let display_text = &self.state.url_input;
            if is_focused {
                let cursor_pos = self.state.url_cursor;
                let before = display_text.chars().take(cursor_pos).collect::<String>();
                let cursor_char = display_text.chars().nth(cursor_pos).unwrap_or(' ');
                let after = display_text.chars().skip(cursor_pos + 1).collect::<String>();
                
                Line::from(vec![
                    Span::raw(before),
                    Span::styled(cursor_char.to_string(), Theme::selected()),
                    Span::raw(after),
                ])
            } else {
                Line::from(display_text.as_str())
            }
        } else {
            Line::from(request.url.as_str())
        };
        
        Paragraph::new(text).block(block).render(area, buf);
    }
    
    fn render_params_content(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, request: &crate::models::request::HttpRequest, is_editing: bool) {
        let is_focused = is_editing && self.state.editor_focused_field == EditorField::Params;
        
        let title = if is_focused {
            match self.state.kv_edit_mode {
                KeyValueEditMode::None => "Query Parameters [+ add, - delete, ↑↓ navigate, Enter edit]",
                KeyValueEditMode::Key => "Query Parameters [EDITING KEY - Tab to switch, Esc to finish]",
                KeyValueEditMode::Value => "Query Parameters [EDITING VALUE - Tab to switch, Esc to finish]",
            }
        } else {
            "Query Parameters"
        };
        
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });
        
        if is_editing {
            let items: Vec<ListItem> = self.state.params_input
                .iter()
                .enumerate()
                .map(|(i, (key, value))| {
                    let is_selected = i == self.state.params_selected;
                    let style = if is_selected {
                        Theme::selected()
                    } else {
                        Style::default()
                    };
                    
                    let text = if is_selected && self.state.kv_edit_mode != KeyValueEditMode::None {
                        match self.state.kv_edit_mode {
                            KeyValueEditMode::Key => format!("[{}]: {}", key, value),
                            KeyValueEditMode::Value => format!("{}: [{}]", key, value),
                            _ => format!("{}: {}", key, value),
                        }
                    } else {
                        format!("{}: {}", key, value)
                    };
                    
                    ListItem::new(text).style(style)
                })
                .collect();
            
            if items.is_empty() {
                Paragraph::new("No parameters (press + to add)")
                    .block(block)
                    .render(area, buf);
            } else {
                List::new(items)
                    .block(block)
                    .render(area, buf);
            }
        } else {
            let content = if request.query_params.is_empty() {
                "No query parameters".to_string()
            } else {
                request.query_params
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            
            Paragraph::new(content).block(block).render(area, buf);
        }
    }
    
    fn render_headers_content(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, request: &crate::models::request::HttpRequest, is_editing: bool) {
        let is_focused = is_editing && self.state.editor_focused_field == EditorField::Headers;
        
        let title = if is_focused {
            match self.state.kv_edit_mode {
                KeyValueEditMode::None => "Headers [+ add, - delete, ↑↓ navigate, Enter edit]",
                KeyValueEditMode::Key => "Headers [EDITING KEY - Tab to switch, Esc to finish]",
                KeyValueEditMode::Value => "Headers [EDITING VALUE - Tab to switch, Esc to finish]",
            }
        } else {
            "Headers"
        };
        
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });
        
        if is_editing {
            let items: Vec<ListItem> = self.state.headers_input
                .iter()
                .enumerate()
                .map(|(i, (key, value))| {
                    let is_selected = i == self.state.headers_selected;
                    let style = if is_selected {
                        Theme::selected()
                    } else {
                        Style::default()
                    };
                    
                    let text = if is_selected && self.state.kv_edit_mode != KeyValueEditMode::None {
                        match self.state.kv_edit_mode {
                            KeyValueEditMode::Key => format!("[{}]: {}", key, value),
                            KeyValueEditMode::Value => format!("{}: [{}]", key, value),
                            _ => format!("{}: {}", key, value),
                        }
                    } else {
                        format!("{}: {}", key, value)
                    };
                    
                    ListItem::new(text).style(style)
                })
                .collect();
            
            if items.is_empty() {
                Paragraph::new("No headers (press + to add)")
                    .block(block)
                    .render(area, buf);
            } else {
                List::new(items)
                    .block(block)
                    .render(area, buf);
            }
        } else {
            let content = if request.headers.is_empty() {
                "No headers".to_string()
            } else {
                request.headers
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            
            Paragraph::new(content).block(block).render(area, buf);
        }
    }
    
    fn render_body_content(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, request: &crate::models::request::HttpRequest, is_editing: bool) {
        let is_focused = is_editing && self.state.editor_focused_field == EditorField::Body;
        
        let block = Block::default()
            .title("Body")
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });
        
        let mut display_text = if is_editing {
            self.state.body_input.clone()
        } else {
            request
                .body
                .clone()
                .unwrap_or_else(|| "No body".to_string())
        };
        
        if is_editing && is_focused {
            let cursor_pos = self.state.body_cursor.min(display_text.len());
            display_text.insert(cursor_pos, '▌');
        }
        
        Paragraph::new(display_text)
            .wrap(Wrap { trim: false })
            .block(block)
            .render(area, buf);
    }
    
    fn render_auth_content(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, request: &crate::models::request::HttpRequest, is_editing: bool) {
        let is_focused = is_editing && self.state.editor_focused_field == EditorField::Auth;
        
        let block = Block::default()
            .title("Authentication (Bearer Token)")
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });
        
        let text = if is_editing {
            let display_text = &self.state.auth_input;
            if is_focused {
                let cursor_pos = self.state.auth_cursor;
                let before = display_text.chars().take(cursor_pos).collect::<String>();
                let cursor_char = display_text.chars().nth(cursor_pos).unwrap_or(' ');
                let after = display_text.chars().skip(cursor_pos + 1).collect::<String>();
                
                Line::from(vec![
                    Span::raw(before),
                    Span::styled(cursor_char.to_string(), Theme::selected()),
                    Span::raw(after),
                ])
            } else {
                Line::from(display_text.as_str())
            }
        } else {
            let auth_text = match &request.auth {
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
            };
            Line::from(auth_text)
        };
        
        Paragraph::new(text).block(block).render(area, buf);
    }
}

