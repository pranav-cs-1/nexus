use crate::app::state::{AppState, Panel};
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Widget},
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
        
        let title = "Collections";
        
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded);
        
        let items: Vec<ListItem> = self
            .state
            .collections
            .iter()
            .enumerate()
            .map(|(idx, collection)| {
                let is_selected = Some(idx) == self.state.selected_collection;
                let is_editing = is_selected && self.state.editing_collection;
                
                let style = if is_selected {
                    Theme::selected()
                } else {
                    Theme::default()
                };
                
                let text = if is_editing {
                    let display_text = &self.state.collection_name_input;
                    let cursor_pos = self.state.collection_name_cursor;
                    let before = display_text.chars().take(cursor_pos).collect::<String>();
                    let cursor_char = display_text.chars().nth(cursor_pos).unwrap_or(' ');
                    let after = display_text.chars().skip(cursor_pos + 1).collect::<String>();
                    
                    Line::from(vec![
                        Span::raw(before),
                        Span::styled(cursor_char.to_string(), Theme::selected()),
                        Span::raw(after),
                    ])
                } else {
                    Line::from(vec![
                        Span::styled(&collection.name, style),
                    ])
                };
                
                ListItem::new(text)
            })
            .collect();
        
        let list = List::new(items).block(block);
        
        Widget::render(list, area, buf);
    }
}

