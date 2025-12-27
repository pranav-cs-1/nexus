use crate::app::state::{AppState, Panel, ProtocolType};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
    style::{Color, Style},
};

pub struct ResponseViewer<'a> {
    state: &'a AppState,
}

impl<'a> ResponseViewer<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
    
    fn colorize_json(&self, json: &str) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        
        for line in json.lines() {
            let mut spans = Vec::new();
            let mut chars = line.chars().peekable();
            let mut current = String::new();
            
            while let Some(ch) = chars.next() {
                match ch {
                    '"' => {
                        if !current.is_empty() {
                            spans.push(Span::raw(current.clone()));
                            current.clear();
                        }
                        
                        let mut string_content = String::from("\"");
                        let mut is_key = false;
                        
                        while let Some(&next_ch) = chars.peek() {
                            chars.next();
                            string_content.push(next_ch);
                            
                            if next_ch == '"' {
                                break;
                            }
                            if next_ch == '\\' {
                                if let Some(&escaped) = chars.peek() {
                                    chars.next();
                                    string_content.push(escaped);
                                }
                            }
                        }
                        
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_whitespace() {
                                string_content.push(next_ch);
                                chars.next();
                            } else if next_ch == ':' {
                                is_key = true;
                                break;
                            } else {
                                break;
                            }
                        }
                        
                        let color = if is_key { Color::White } else { Color::Gray };
                        spans.push(Span::styled(string_content, Style::default().fg(color)));
                    }
                    't' | 'f' => {
                        if !current.is_empty() {
                            spans.push(Span::raw(current.clone()));
                            current.clear();
                        }
                        
                        let mut word = String::from(ch);
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_alphanumeric() {
                                word.push(next_ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        
                        if word == "true" || word == "false" {
                            spans.push(Span::styled(word, Style::default().fg(Color::Blue)));
                        } else {
                            spans.push(Span::raw(word));
                        }
                    }
                    'n' => {
                        if !current.is_empty() {
                            spans.push(Span::raw(current.clone()));
                            current.clear();
                        }
                        
                        let mut word = String::from(ch);
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_alphanumeric() {
                                word.push(next_ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        
                        if word == "null" {
                            spans.push(Span::styled(word, Style::default().fg(Color::DarkGray)));
                        } else {
                            spans.push(Span::raw(word));
                        }
                    }
                    '0'..='9' | '-' => {
                        if !current.is_empty() && !current.chars().all(|c| c.is_whitespace()) {
                            spans.push(Span::raw(current.clone()));
                            current.clear();
                        }
                        
                        let mut number = String::from(ch);
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_numeric() || next_ch == '.' || next_ch == 'e' || next_ch == 'E' || next_ch == '+' || next_ch == '-' {
                                number.push(next_ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        
                        if !current.is_empty() {
                            spans.push(Span::raw(current.clone()));
                            current.clear();
                        }
                        spans.push(Span::styled(number, Style::default().fg(Color::Cyan)));
                    }
                    _ => {
                        current.push(ch);
                    }
                }
            }
            
            if !current.is_empty() {
                spans.push(Span::raw(current));
            }
            
            if spans.is_empty() {
                lines.push(Line::from(""));
            } else {
                lines.push(Line::from(spans));
            }
        }
        
        lines
    }
}

impl<'a> Widget for ResponseViewer<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let is_focused = self.state.focused_panel == Panel::Response;
        
        let border_style = if is_focused {
            Theme::focused_border()
        } else {
            Theme::unfocused_border()
        };
        
        let title = if is_focused {
            format!("Response [↑/↓ scroll, c: copy | line {}]", self.state.response_scroll + 1)
        } else {
            "Response".to_string()
        };
        
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);
        
        match self.state.protocol_type {
            ProtocolType::Http => {
                if let Some(response) = &self.state.current_response {
                    let inner_area = block.inner(area);
                    block.render(area, buf);

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(1),
                            Constraint::Min(0),
                        ])
                        .split(inner_area);

                    let status_line = Line::from(vec![
                        Span::styled(
                            format!("{} ", response.status_code),
                            ratatui::style::Style::default().fg(response.status_color()),
                        ),
                        Span::raw(&response.status_text),
                        Span::raw(format!(" | {}ms | {} bytes",
                            response.duration_ms,
                            response.size_bytes
                        )),
                    ]);

                    let status_paragraph = Paragraph::new(status_line);
                    status_paragraph.render(chunks[0], buf);

                    let body = response.formatted_body();

                    let body_content = if response.is_json() {
                        self.colorize_json(&body)
                    } else {
                        vec![Line::from(body)]
                    };

                    let body_paragraph = Paragraph::new(body_content)
                        .wrap(Wrap { trim: false })
                        .scroll((self.state.response_scroll, 0));
                    body_paragraph.render(chunks[1], buf);
                } else {
                    let no_response = Paragraph::new("No response yet\n\nPress Enter to send request")
                        .block(block);
                    no_response.render(area, buf);
                }
            }
            ProtocolType::Grpc => {
                if let Some(response) = &self.state.grpc_response {
                    let inner_area = block.inner(area);
                    block.render(area, buf);

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(1),
                            Constraint::Min(0),
                        ])
                        .split(inner_area);

                    let status_color = if response.status.code == 0 {
                        Color::Green
                    } else {
                        Color::Red
                    };

                    let status_line = Line::from(vec![
                        Span::styled(
                            format!("Code {} ", response.status.code),
                            ratatui::style::Style::default().fg(status_color),
                        ),
                        Span::raw(&response.status.message),
                        Span::raw(format!(" | {}ms | {} message(s)",
                            response.duration_ms,
                            response.messages.len()
                        )),
                    ]);

                    let status_paragraph = Paragraph::new(status_line);
                    status_paragraph.render(chunks[0], buf);

                    // Display messages
                    let body = if response.messages.is_empty() {
                        if response.status.code == 0 {
                            "Success (no message body)".to_string()
                        } else {
                            format!("Error: {}", response.status.message)
                        }
                    } else {
                        response.messages
                            .iter()
                            .map(|msg| msg.message_json.as_str())
                            .collect::<Vec<_>>()
                            .join("\n\n")
                    };

                    let body_content = self.colorize_json(&body);

                    let body_paragraph = Paragraph::new(body_content)
                        .wrap(Wrap { trim: false })
                        .scroll((self.state.response_scroll, 0));
                    body_paragraph.render(chunks[1], buf);
                } else {
                    let no_response = Paragraph::new("No response yet\n\nPress Enter to send gRPC request")
                        .block(block);
                    no_response.render(area, buf);
                }
            }
        }
    }
}

