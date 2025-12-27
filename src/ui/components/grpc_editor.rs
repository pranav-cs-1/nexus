use crate::app::state::{AppState, InputMode, Panel, GrpcEditorField, KeyValueEditMode};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget, Wrap},
    style::Style,
};

pub struct GrpcEditor<'a> {
    state: &'a AppState,
}

impl<'a> GrpcEditor<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for GrpcEditor<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let is_focused = self.state.focused_panel == Panel::RequestEditor;
        let is_editing = self.state.input_mode == InputMode::Editing && is_focused;

        let border_style = if is_focused {
            Theme::focused_border()
        } else {
            Theme::unfocused_border()
        };

        let proto_indicator = if let Some(schema) = self.state.get_selected_proto_schema() {
            format!(" | Proto: {}", schema.name)
        } else {
            " | No proto loaded (Press 'l')".to_string()
        };

        let title = if is_editing {
            format!("gRPC Request Editor [EDITING - ESC to save, Tab/Shift+Tab to switch fields]{}", proto_indicator)
        } else {
            format!("gRPC Request Editor [Press 'e' to edit]{}", proto_indicator)
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
                Constraint::Length(3), // Name
                Constraint::Length(3), // Server URL
                Constraint::Length(3), // Service Name
                Constraint::Length(3), // Method Name
                Constraint::Length(10), // Message (JSON)
                Constraint::Min(0),     // Metadata
            ])
            .split(inner_area);

        if let Some(request) = self.state.get_current_grpc_request() {
            self.render_name_field(chunks[0], buf, request, is_editing);
            self.render_server_url_field(chunks[1], buf, request, is_editing);
            self.render_service_name_field(chunks[2], buf, request, is_editing);
            self.render_method_name_field(chunks[3], buf, request, is_editing);
            self.render_message_field(chunks[4], buf, request, is_editing);
            self.render_metadata_field(chunks[5], buf, request, is_editing);
        } else {
            let no_request = Paragraph::new("No gRPC request selected")
                .block(Block::default());
            no_request.render(inner_area, buf);
        }
    }
}

impl<'a> GrpcEditor<'a> {
    fn render_text_field(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        title: &str,
        content: &str,
        edit_content: &str,
        cursor_pos: usize,
        is_editing: bool,
        is_focused: bool,
    ) {
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });

        let text = if is_editing {
            let display_text = edit_content;
            if is_focused {
                let available_width = area.width.saturating_sub(2) as usize;
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

                Line::from(vec![
                    Span::raw(before),
                    Span::styled(cursor_char.to_string(), Theme::selected()),
                    Span::raw(after),
                ])
            } else {
                Line::from(display_text)
            }
        } else {
            Line::from(content)
        };

        Paragraph::new(text).block(block).render(area, buf);
    }

    fn render_name_field(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        request: &crate::models::GrpcRequest,
        is_editing: bool,
    ) {
        let is_focused = is_editing && self.state.grpc_editor_focused_field == GrpcEditorField::Name;
        self.render_text_field(
            area,
            buf,
            "Name",
            &request.name,
            &self.state.grpc_name_input,
            self.state.grpc_name_cursor,
            is_editing,
            is_focused,
        );
    }

    fn render_server_url_field(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        request: &crate::models::GrpcRequest,
        is_editing: bool,
    ) {
        let is_focused = is_editing && self.state.grpc_editor_focused_field == GrpcEditorField::ServerUrl;
        self.render_text_field(
            area,
            buf,
            "Server URL",
            &request.server_url,
            &self.state.grpc_server_url_input,
            self.state.grpc_server_url_cursor,
            is_editing,
            is_focused,
        );
    }

    fn render_service_name_field(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        request: &crate::models::GrpcRequest,
        is_editing: bool,
    ) {
        let is_focused = is_editing && self.state.grpc_editor_focused_field == GrpcEditorField::ServiceName;
        self.render_text_field(
            area,
            buf,
            "Service Name",
            &request.service_name,
            &self.state.grpc_service_name_input,
            self.state.grpc_service_name_cursor,
            is_editing,
            is_focused,
        );
    }

    fn render_method_name_field(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        request: &crate::models::GrpcRequest,
        is_editing: bool,
    ) {
        let is_focused = is_editing && self.state.grpc_editor_focused_field == GrpcEditorField::MethodName;
        self.render_text_field(
            area,
            buf,
            "Method Name",
            &request.method_name,
            &self.state.grpc_method_name_input,
            self.state.grpc_method_name_cursor,
            is_editing,
            is_focused,
        );
    }

    fn render_message_field(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        request: &crate::models::GrpcRequest,
        is_editing: bool,
    ) {
        let is_focused = is_editing && self.state.grpc_editor_focused_field == GrpcEditorField::Message;

        let block = Block::default()
            .title("Message (JSON)")
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });

        let mut display_text = if is_editing {
            self.state.grpc_message_input.clone()
        } else {
            request.message_json.clone()
        };

        if is_editing && is_focused {
            let cursor_pos = self.state.grpc_message_cursor.min(display_text.len());
            display_text.insert(cursor_pos, '▌');
        }

        Paragraph::new(display_text)
            .wrap(Wrap { trim: false })
            .block(block)
            .render(area, buf);
    }

    fn render_metadata_field(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        request: &crate::models::GrpcRequest,
        is_editing: bool,
    ) {
        let is_focused = is_editing && self.state.grpc_editor_focused_field == GrpcEditorField::Metadata;

        let title = if is_focused {
            match self.state.kv_edit_mode {
                KeyValueEditMode::None => "Metadata [+ add, - delete, ↑↓ navigate, Enter edit]",
                KeyValueEditMode::Key => "Metadata [EDITING KEY - Tab to switch, Esc to finish]",
                KeyValueEditMode::Value => "Metadata [EDITING VALUE - Tab to switch, Esc to finish]",
            }
        } else {
            "Metadata"
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(if is_focused { Theme::selected() } else { Theme::unfocused_border() });

        if is_editing {
            let items: Vec<ListItem> = self.state.grpc_metadata_input
                .iter()
                .enumerate()
                .map(|(i, (key, value))| {
                    let is_selected = i == self.state.grpc_metadata_selected;
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
                Paragraph::new("No metadata (press + to add)")
                    .block(block)
                    .render(area, buf);
            } else {
                List::new(items)
                    .block(block)
                    .render(area, buf);
            }
        } else {
            let content = if request.metadata.is_empty() {
                "No metadata".to_string()
            } else {
                request.metadata
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            };

            Paragraph::new(content).block(block).render(area, buf);
        }
    }
}
