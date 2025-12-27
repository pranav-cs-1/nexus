use crate::app::state::{AppState, Panel, ProtocolType};
use crate::ui::theme::Theme;
use crate::models::request::{HttpMethod, HttpRequest};
use crate::models::GrpcRequest;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget},
};
use uuid::Uuid;

enum RequestItem<'a> {
    Http(&'a HttpRequest),
    Grpc(&'a GrpcRequest),
}

impl<'a> RequestItem<'a> {
    fn id(&self) -> Uuid {
        match self {
            RequestItem::Http(req) => req.id,
            RequestItem::Grpc(req) => req.id,
        }
    }
}

pub struct RequestList<'a> {
    state: &'a mut AppState,
}

impl<'a> RequestList<'a> {
    pub fn new(state: &'a mut AppState) -> Self {
        Self { state }
    }

    fn method_style(method: &HttpMethod) -> ratatui::style::Style {
        match method {
            HttpMethod::GET => Theme::method_get(),
            HttpMethod::POST => Theme::method_post(),
            HttpMethod::PUT => Theme::method_put(),
            HttpMethod::DELETE => Theme::method_delete(),
            HttpMethod::PATCH => Theme::method_patch(),
            _ => Theme::method_other(),
        }
    }
}

impl<'a> Widget for RequestList<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let is_focused = self.state.focused_panel == Panel::Requests;

        let border_style = if is_focused {
            Theme::focused_border()
        } else {
            Theme::unfocused_border()
        };

        // Calculate position indicator
        let position_indicator = if let Some((current, total)) = self.state.get_request_list_position() {
            format!(" [{}/{}]", current, total)
        } else {
            String::new()
        };

        let search_indicator = if self.state.request_search_mode {
            " [SEARCH]"
        } else if !self.state.request_search_input.is_empty() {
            " [FILTERED]"
        } else {
            ""
        };

        let protocol_indicator = match self.state.protocol_type {
            ProtocolType::Http => format!("HTTP Requests{}{}", position_indicator, search_indicator),
            ProtocolType::Grpc => format!("gRPC Requests{}{}", position_indicator, search_indicator),
        };

        let block = Block::default()
            .title(protocol_indicator.clone())
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded);

        let selected_collection_id = self.state.selected_collection
            .and_then(|idx| self.state.collections.get(idx))
            .map(|c| c.id);

        // Calculate visible height (area minus borders and search input if active)
        let visible_height = if self.state.request_search_mode {
            area.height.saturating_sub(5) as usize // Reserve 3 lines for search input
        } else {
            area.height.saturating_sub(2) as usize
        };

        // First pass: collect indices and IDs of visible requests
        let visible_request_indices: Vec<usize> = if self.state.request_search_mode && !self.state.request_search_input.is_empty() {
            // Use filtered indices when searching
            self.state.filtered_request_indices.clone()
        } else {
            // Use all requests matching collection filter
            match self.state.protocol_type {
                ProtocolType::Http => {
                    self.state.requests.iter().enumerate()
                        .filter(|(_, request)| {
                            match (selected_collection_id, request.collection_id) {
                                (Some(selected_id), Some(request_id)) => selected_id == request_id,
                                (None, None) => true,
                                _ => false,
                            }
                        })
                        .map(|(idx, _)| idx)
                        .collect()
                }
                ProtocolType::Grpc => {
                    self.state.grpc_requests.iter().enumerate()
                        .filter(|(_, request)| {
                            match (selected_collection_id, request.collection_id) {
                                (Some(selected_id), Some(request_id)) => selected_id == request_id,
                                (None, None) => true,
                                _ => false,
                            }
                        })
                        .map(|(idx, _)| idx)
                        .collect()
                }
            }
        };

        // Find current position in the visible list by checking against selected_request
        let current_position = if let Some(selected_idx) = self.state.selected_request {
            visible_request_indices.iter().position(|&idx| idx == selected_idx)
        } else {
            None
        };

        // Auto-scroll: adjust scroll offset if cursor is out of view
        if let Some(pos) = current_position {
            let current_scroll = self.state.request_list_scroll as usize;

            if pos < current_scroll {
                // Cursor is above viewport - scroll up
                self.state.scroll_request_list_to(pos as u16);
            } else if pos >= current_scroll + visible_height && visible_height > 0 {
                // Cursor is below viewport - scroll down
                self.state.scroll_request_list_to((pos.saturating_sub(visible_height) + 1) as u16);
            }
        }

        // Now build the all_requests vector using the indices
        let all_requests: Vec<RequestItem> = match self.state.protocol_type {
            ProtocolType::Http => {
                visible_request_indices.iter()
                    .filter_map(|&idx| self.state.requests.get(idx))
                    .map(|r| RequestItem::Http(r))
                    .collect()
            }
            ProtocolType::Grpc => {
                visible_request_indices.iter()
                    .filter_map(|&idx| self.state.grpc_requests.get(idx))
                    .map(|r| RequestItem::Grpc(r))
                    .collect()
            }
        };

        // Determine selected request ID
        let selected_request_id = match self.state.protocol_type {
            ProtocolType::Http => {
                self.state.selected_request
                    .and_then(|idx| self.state.requests.get(idx))
                    .map(|r| r.id)
            }
            ProtocolType::Grpc => {
                self.state.selected_request
                    .and_then(|idx| self.state.grpc_requests.get(idx))
                    .map(|r| r.id)
            }
        };

        // Generate all list items
        let all_items: Vec<ListItem> = all_requests
            .iter()
            .map(|request_item| {
                let is_selected = Some(request_item.id()) == selected_request_id;

                match request_item {
                    RequestItem::Http(request) => {
                        let method_str = format!("{:7}", request.method.as_str());

                        let line = if is_selected {
                            Line::from(vec![
                                Span::raw("[H] "),
                                Span::styled(method_str.clone(), Self::method_style(&request.method)),
                                Span::raw(" "),
                                Span::styled(request.name.clone(), Theme::selected()),
                            ])
                        } else {
                            Line::from(vec![
                                Span::raw("[H] "),
                                Span::styled(method_str, Self::method_style(&request.method)),
                                Span::raw(" "),
                                Span::raw(request.name.clone()),
                            ])
                        };

                        ListItem::new(line)
                    }
                    RequestItem::Grpc(request) => {
                        let line = if is_selected {
                            Line::from(vec![
                                Span::styled("[G] ", Theme::method_post()), // Use a color for gRPC
                                Span::styled(request.name.clone(), Theme::selected()),
                            ])
                        } else {
                            Line::from(vec![
                                Span::styled("[G] ", Theme::method_post()),
                                Span::raw(request.name.clone()),
                            ])
                        };

                        ListItem::new(line)
                    }
                }
            })
            .collect();

        // Slice items based on scroll offset to show only visible portion
        let scroll_offset = self.state.request_list_scroll as usize;
        let visible_items: Vec<ListItem> = all_items
            .into_iter()
            .skip(scroll_offset)
            .take(visible_height)
            .collect();

        // Render with or without search input UI
        if self.state.request_search_mode {
            // Split area to show search input at bottom
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),      // List area
                    Constraint::Length(3),   // Search input
                ])
                .split(area);

            // Render list in top chunk (without bottom border)
            let list_block = Block::default()
                .title(protocol_indicator)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_style(border_style)
                .border_type(BorderType::Rounded);
            let list = List::new(visible_items).block(list_block);
            Widget::render(list, chunks[0], buf);

            // Render search input in bottom chunk
            let cursor_pos = self.state.request_search_cursor;

            // Build the search line with cursor highlight
            let mut spans = vec![Span::raw("/")];
            for (i, ch) in self.state.request_search_input.chars().enumerate() {
                if i == cursor_pos {
                    spans.push(Span::styled(ch.to_string(), Theme::selected()));
                } else {
                    spans.push(Span::raw(ch.to_string()));
                }
            }
            // Add cursor at end if cursor is past last char
            if cursor_pos >= self.state.request_search_input.len() {
                spans.push(Span::styled(" ", Theme::selected()));
            }

            let search_line = Line::from(spans);

            let search_block = Block::default()
                .title("Search (Enter: apply, Esc: cancel)")
                .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                .border_style(Theme::focused_border())
                .border_type(BorderType::Rounded);

            let search_paragraph = Paragraph::new(search_line).block(search_block);
            Widget::render(search_paragraph, chunks[1], buf);
        } else {
            // Normal rendering without search input
            let list = List::new(visible_items).block(block);
            Widget::render(list, area, buf);
        }
    }
}

