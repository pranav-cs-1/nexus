use crate::app::state::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
    style::{Color, Style},
};

pub struct ProtoLoaderPopup<'a> {
    state: &'a AppState,
}

impl<'a> ProtoLoaderPopup<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for ProtoLoaderPopup<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Clear the area
        Clear.render(area, buf);

        let block = Block::default()
            .title("Load Proto File")
            .borders(Borders::ALL)
            .border_style(Theme::focused_border())
            .border_type(BorderType::Rounded);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // Instructions
                Constraint::Length(3),  // File path input
                Constraint::Length(2),  // Status message (if any)
                Constraint::Min(0),     // Loaded schemas
            ])
            .split(inner_area);

        // Instructions
        let instructions = vec![
            Line::from(""),
            Line::from(Span::styled("Enter the path to your proto descriptor file (.pb)", Style::default().fg(Color::Cyan))),
            Line::from(""),
            Line::from(Span::styled("Generate with:", Style::default().fg(Color::Cyan))),
            Line::from("  protoc --descriptor_set_out=service.pb --include_imports service.proto"),
        ];

        Paragraph::new(instructions)
            .wrap(Wrap { trim: false })
            .render(chunks[0], buf);

        // File path input
        let input_focused = self.state.proto_loader_focus == crate::app::state::ProtoLoaderFocus::Input;
        let input_block = Block::default()
            .title("File Path")
            .borders(Borders::ALL)
            .border_style(if input_focused { Theme::selected() } else { Theme::unfocused_border() })
            .border_type(BorderType::Rounded);

        let display_text = &self.state.proto_file_input;
        let cursor_pos = self.state.proto_file_cursor;

        let available_width = chunks[1].width.saturating_sub(2) as usize;
        let scroll_offset = if cursor_pos >= available_width {
            cursor_pos.saturating_sub(available_width.saturating_sub(1))
        } else {
            0
        };

        let visible_text: String = display_text
            .chars()
            .skip(scroll_offset)
            .take(available_width)
            .collect();

        let visible_cursor_pos = cursor_pos.saturating_sub(scroll_offset);

        let before = visible_text.chars().take(visible_cursor_pos).collect::<String>();
        let cursor_char = visible_text.chars().nth(visible_cursor_pos).unwrap_or(' ');
        let after = visible_text.chars().skip(visible_cursor_pos + 1).collect::<String>();

        let input_line = Line::from(vec![
            Span::raw(before),
            Span::styled(cursor_char.to_string(), Theme::selected()),
            Span::raw(after),
        ]);

        Paragraph::new(input_line)
            .block(input_block)
            .render(chunks[1], buf);

        // Status message area (non-blocking)
        if let Some(result) = &self.state.proto_load_result {
            let result_style = if result.contains("Error") || result.contains("Failed") {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };

            Paragraph::new(result.clone())
                .style(result_style)
                .wrap(Wrap { trim: false })
                .render(chunks[2], buf);
        }

        // Loaded schemas (always show if they exist)
        if !self.state.proto_schemas.is_empty() {
            // Show loaded proto schemas
            let list_focused = self.state.proto_loader_focus == crate::app::state::ProtoLoaderFocus::SchemaList;
            let schemas_block = Block::default()
                .title(format!("Loaded Proto Schemas ({})", self.state.proto_schemas.len()))
                .borders(Borders::ALL)
                .border_style(if list_focused { Theme::selected() } else { Theme::unfocused_border() })
                .border_type(BorderType::Rounded);

            let schema_lines: Vec<Line> = self.state.proto_schemas
                .iter()
                .enumerate()
                .map(|(idx, schema)| {
                    let is_selected = self.state.selected_proto_schema == Some(idx);
                    let prefix = if is_selected { "▶ " } else { "  " };
                    let service_count = schema.services.len();

                    Line::from(vec![
                        Span::styled(
                            format!("{}{} ({} service{})",
                                prefix,
                                schema.name,
                                service_count,
                                if service_count == 1 { "" } else { "s" }
                            ),
                            if is_selected {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }
                        )
                    ])
                })
                .collect();

            Paragraph::new(schema_lines)
                .block(schemas_block)
                .render(chunks[3], buf);
        }

        // Help text at bottom
        let help_text = if self.state.proto_load_result.is_some() {
            "Press Esc to close | Enter to try again"
        } else if self.state.proto_schemas.is_empty() {
            "Enter: Load | Tab: Autocomplete | Esc: Cancel | Reload same file = Update"
        } else {
            match self.state.proto_loader_focus {
                crate::app::state::ProtoLoaderFocus::Input => {
                    "Enter: Load/Update | Tab: Autocomplete | Shift+Tab: Switch to list | Esc: Close"
                }
                crate::app::state::ProtoLoaderFocus::SchemaList => {
                    "↑↓/j/k: Navigate | d/Del/Backspace: Delete | Shift+Tab: Switch to input | Esc: Close"
                }
            }
        };

        let help_line = Line::from(vec![
            Span::styled(help_text, Style::default().fg(Color::DarkGray)),
        ]);

        let help_area = Rect {
            x: area.x + 2,
            y: area.y + area.height.saturating_sub(1),
            width: area.width.saturating_sub(4),
            height: 1,
        };

        Paragraph::new(help_line).render(help_area, buf);
    }
}
