use crate::app::state::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub struct StatusBar<'a> {
    state: &'a AppState,
}

impl<'a> StatusBar<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let status_text = if self.state.is_loading {
            format!(" Loading... | {}", self.state.loading_message)
        } else {
            " q: quit | ?: help | Tab: next panel | Enter: send request | n: new request".to_string()
        };
        
        let status_line = Line::from(vec![
            Span::styled(status_text, Theme::default()),
        ]);
        
        let paragraph = Paragraph::new(status_line)
            .style(ratatui::style::Style::default()
                .bg(ratatui::style::Color::DarkGray));
        
        Widget::render(paragraph, area, buf);
    }
}

