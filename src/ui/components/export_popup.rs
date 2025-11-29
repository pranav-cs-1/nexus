use crate::app::state::{AppState, ExportMode, ExportMenuStage};
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget},
    style::Style,
};

pub struct ExportPopup<'a> {
    state: &'a AppState,
}

impl<'a> ExportPopup<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for ExportPopup<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        Clear.render(area, buf);
        
        match self.state.export_menu_stage {
            ExportMenuStage::ShowingResult => {
                self.render_result(area, buf);
            }
            ExportMenuStage::SelectingCollection => {
                self.render_collection_selection(area, buf);
            }
            ExportMenuStage::SelectingRequest => {
                self.render_request_selection(area, buf);
            }
        }
    }
}

impl<'a> ExportPopup<'a> {
    fn render_result(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = Block::default()
            .title("Export Complete")
            .borders(Borders::ALL)
            .border_style(Theme::focused_border());
        
        if let Some(export_result) = &self.state.export_result_message {
            let lines = vec![
                Line::from(""),
                Line::from("Export successful!"),
                Line::from(""),
                Line::from(format!("Saved to: {}", export_result)),
                Line::from(""),
                Line::from("Press any key to continue..."),
                Line::from(""),
            ];
            
            let paragraph = Paragraph::new(lines).block(block);
            Widget::render(paragraph, area, buf);
        }
    }
    
    fn render_collection_selection(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let title = match self.state.export_mode {
            Some(ExportMode::CollectionJson) => "Export Collection as JSON",
            Some(ExportMode::RequestCurl) => "Export Request as curl - Select Collection",
            None => "Export",
        };
        
        let items: Vec<ListItem> = self.state.collections
            .iter()
            .enumerate()
            .map(|(i, collection)| {
                let is_selected = Some(i) == self.state.export_selected_collection;
                let style = if is_selected {
                    Theme::selected()
                } else {
                    Style::default()
                };
                
                let request_count = self.state.requests
                    .iter()
                    .filter(|r| r.collection_id == Some(collection.id))
                    .count();
                
                let text = format!("{} ({} requests)", collection.name, request_count);
                ListItem::new(text).style(style)
            })
            .collect();
        
        if items.is_empty() {
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Theme::focused_border());
            
            let paragraph = Paragraph::new(vec![
                Line::from(""),
                Line::from("No collections available"),
                Line::from(""),
                Line::from("Press any key to close..."),
                Line::from(""),
            ]).block(block);
            Widget::render(paragraph, area, buf);
        } else {
            let list = List::new(items).block(
                Block::default()
                    .title(format!("{} - Use ↑↓ to select, Enter to continue, Esc to cancel", title))
                    .borders(Borders::ALL)
                    .border_style(Theme::focused_border())
            );
            Widget::render(list, area, buf);
        }
    }
    
    fn render_request_selection(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = Block::default()
            .title("Export Request as curl - Select Request")
            .borders(Borders::ALL)
            .border_style(Theme::focused_border());
        
        if let Some(collection_idx) = self.state.export_selected_collection {
            if let Some(collection) = self.state.collections.get(collection_idx) {
                let requests_in_collection: Vec<_> = self.state.requests
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.collection_id == Some(collection.id))
                    .collect();
                
                if requests_in_collection.is_empty() {
                    let paragraph = Paragraph::new(vec![
                        Line::from(""),
                        Line::from("No requests in this collection"),
                        Line::from(""),
                        Line::from("Press Esc to go back..."),
                        Line::from(""),
                    ]).block(block);
                    Widget::render(paragraph, area, buf);
                } else {
                    let items: Vec<ListItem> = requests_in_collection
                        .iter()
                        .map(|(global_idx, request)| {
                            let is_selected = Some(*global_idx) == self.state.export_selected_request;
                            let style = if is_selected {
                                Theme::selected()
                            } else {
                                Style::default()
                            };
                            
                            let text = format!("{} - {}", request.method.as_str(), request.name);
                            ListItem::new(text).style(style)
                        })
                        .collect();
                    
                    let list = List::new(items).block(
                        Block::default()
                            .title("Export Request as curl - Use ↑↓ to select, Enter to export, Esc to go back")
                            .borders(Borders::ALL)
                            .border_style(Theme::focused_border())
                    );
                    Widget::render(list, area, buf);
                }
            }
        }
    }
}

