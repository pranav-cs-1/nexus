use ratatui::layout::{Constraint, Direction, Layout as RatatuiLayout, Rect};

pub struct Layout {
    pub collections: Rect,
    pub requests: Rect,
    pub editor: Rect,
    pub response: Rect,
    pub statusbar: Rect,
}

impl Layout {
    pub fn new(area: Rect) -> Self {
        let main_chunks = RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);
        
        let content_chunks = RatatuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(20),
                Constraint::Percentage(35),
                Constraint::Percentage(30),
            ])
            .split(main_chunks[0]);
        
        Self {
            collections: content_chunks[0],
            requests: content_chunks[1],
            editor: content_chunks[2],
            response: content_chunks[3],
            statusbar: main_chunks[1],
        }
    }
}

