use crate::app::state::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

pub struct ImportPopup<'a> {
    state: &'a AppState,
}

impl<'a> ImportPopup<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for ImportPopup<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        Clear.render(area, buf);

        if let Some(result) = &self.state.import_result_message {
            self.render_result(area, buf, result);
        } else {
            self.render_file_input(area, buf);
        }
    }
}

impl<'a> ImportPopup<'a> {
    fn render_file_input(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = Block::default()
            .title("Import Postman Collection")
            .borders(Borders::ALL)
            .border_style(Theme::focused_border())
            .border_type(BorderType::Rounded);

        let cursor_pos = self.state.import_file_cursor;
        let input_value = &self.state.import_file_input;

        // Get current directory
        let current_dir = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        // Create cursor indicator
        let display_text = if input_value.is_empty() {
            "│".to_string()
        } else {
            let before = &input_value[..cursor_pos.min(input_value.len())];
            let after = &input_value[cursor_pos.min(input_value.len())..];
            format!("{}│{}", before, after)
        };

        let lines = vec![
            Line::from(""),
            Line::from(format!("Working directory: {}", current_dir)),
            Line::from(""),
            Line::from("File path: (use ~/ for home directory)"),
            Line::from(""),
            Line::from(display_text),
            Line::from(""),
            Line::from("Tab: autocomplete | Enter: import | Ctrl+U: clear | Esc: cancel"),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(lines).block(block);
        Widget::render(paragraph, area, buf);
    }

    fn render_result(&self, area: Rect, buf: &mut ratatui::buffer::Buffer, result: &str) {
        let is_error = result.starts_with("Error:");
        let title = if is_error {
            "Import Failed"
        } else {
            "Import Complete"
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Theme::focused_border())
            .border_type(BorderType::Rounded);

        let lines = vec![
            Line::from(""),
            Line::from(result),
            Line::from(""),
            Line::from("Press any key to close..."),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(lines).block(block);
        Widget::render(paragraph, area, buf);
    }
}
