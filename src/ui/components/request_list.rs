use crate::app::state::{AppState, Panel};
use crate::ui::theme::Theme;
use crate::models::request::HttpMethod;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

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
        
        let block = Block::default()
            .title("Requests")
            .borders(Borders::ALL)
            .border_style(border_style);
        
        let items: Vec<ListItem> = self
            .state
            .requests
            .iter()
            .enumerate()
            .map(|(idx, request)| {
                let is_selected = Some(idx) == self.state.selected_request;
                let method_str = format!("{:7}", request.method.as_str());
                
                let line = if is_selected {
                    Line::from(vec![
                        Span::styled(method_str.clone(), Self::method_style(&request.method)),
                        Span::raw(" "),
                        Span::styled(request.name.clone(), Theme::selected()),
                    ])
                } else {
                    Line::from(vec![
                        Span::styled(method_str, Self::method_style(&request.method)),
                        Span::raw(" "),
                        Span::raw(request.name.clone()),
                    ])
                };
                
                ListItem::new(line)
            })
            .collect();
        
        let list = List::new(items).block(block);
        
        Widget::render(list, area, buf);
    }
}

