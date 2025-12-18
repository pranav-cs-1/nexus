use crate::app::state::AppState;
use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
    style::{Style, Color},
};

pub struct StatusBar<'a> {
    state: &'a AppState,
}

impl<'a> StatusBar<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    fn truncate_text(text: &str, max_width: usize) -> String {
        if max_width == 0 {
            return String::new();
        }
        
        if text.len() <= max_width {
            return text.to_string();
        }
        
        let truncate_at = max_width.saturating_sub(3);
        let truncated: String = text.chars().take(truncate_at).collect();
        format!("{}...", truncated)
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        if area.height == 0 {
            return;
        }

        let full_text = if self.state.is_loading {
            format!(" Loading... | {}", self.state.loading_message)
        } else {
            " q: quit | ?: help | Tab: next panel | Enter: send | n: new | i: import | o: export menu | s: export curl".to_string()
        };

        let status_text = Self::truncate_text(&full_text, area.width as usize);

        let paragraph = Paragraph::new(status_text)
            .style(Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White))
            .block(Block::default());

        Widget::render(paragraph, area, buf);
    }
}