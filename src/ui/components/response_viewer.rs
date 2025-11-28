use crate::app::state::{AppState, Panel};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct ResponseViewer<'a> {
    state: &'a AppState,
}

impl<'a> ResponseViewer<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
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
            let body_paragraph = Paragraph::new(body)
                .wrap(Wrap { trim: false })
                .scroll((self.state.response_scroll, 0));
            body_paragraph.render(chunks[1], buf);
        } else {
            let no_response = Paragraph::new("No response yet\n\nPress Enter to send request")
                .block(block);
            no_response.render(area, buf);
        }
    }
}

