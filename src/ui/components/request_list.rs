use crate::app::state::{AppState, Panel, ProtocolType};
use crate::ui::theme::Theme;
use crate::models::request::{HttpMethod, HttpRequest};
use crate::models::GrpcRequest;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
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
    state: &'a AppState,
}

impl<'a> RequestList<'a> {
    pub fn new(state: &'a AppState) -> Self {
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

        let protocol_indicator = match self.state.protocol_type {
            ProtocolType::Http => "HTTP Requests",
            ProtocolType::Grpc => "gRPC Requests",
        };

        let block = Block::default()
            .title(protocol_indicator)
            .borders(Borders::ALL)
            .border_style(border_style);

        let selected_collection_id = self.state.selected_collection
            .and_then(|idx| self.state.collections.get(idx))
            .map(|c| c.id);

        // Combine HTTP and gRPC requests based on protocol type
        let mut all_requests: Vec<RequestItem> = Vec::new();

        match self.state.protocol_type {
            ProtocolType::Http => {
                // Show only HTTP requests
                for request in &self.state.requests {
                    if match (selected_collection_id, request.collection_id) {
                        (Some(selected_id), Some(request_id)) => selected_id == request_id,
                        (None, None) => true,
                        _ => false,
                    } {
                        all_requests.push(RequestItem::Http(request));
                    }
                }
            }
            ProtocolType::Grpc => {
                // Show only gRPC requests
                for request in &self.state.grpc_requests {
                    if match (selected_collection_id, request.collection_id) {
                        (Some(selected_id), Some(request_id)) => selected_id == request_id,
                        (None, None) => true,
                        _ => false,
                    } {
                        all_requests.push(RequestItem::Grpc(request));
                    }
                }
            }
        }

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

        let items: Vec<ListItem> = all_requests
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

        let list = List::new(items).block(block);

        Widget::render(list, area, buf);
    }
}

