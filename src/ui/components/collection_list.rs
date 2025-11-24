use crate::app::state::{AppState, Panel};
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

pub struct CollectionList<'a> {
    state: &'a AppState,
}

impl<'a> CollectionList<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for CollectionList<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let is_focused = self.state.focused_panel == Panel::Collections;
        
        let border_style = if is_focused {
            Theme::focused_border()
        } else {
            Theme::unfocused_border()
        };
        
        let block = Block::default()
            .title("Collections")
            .borders(Borders::ALL)
            .border_style(border_style);
        
        let items: Vec<ListItem> = self
            .state
            .collections
            .iter()
            .enumerate()
            .map(|(idx, collection)| {
                let style = if Some(idx) == self.state.selected_collection {
                    Theme::selected()
                } else {
                    Theme::default()
                };
                
                ListItem::new(Line::from(vec![
                    Span::styled(&collection.name, style),
                ]))
            })
            .collect();
        
        let list = List::new(items).block(block);
        
        Widget::render(list, area, buf);
    }
}

