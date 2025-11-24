use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct HelpPopup;

impl HelpPopup {
    pub fn new() -> Self {
        Self
    }
}

impl Widget for HelpPopup {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        Clear.render(area, buf);
        
        let block = Block::default()
            .title("Help")
            .borders(Borders::ALL)
            .border_style(Theme::focused_border());
        
        let help_text = vec![
            "",
            "  Navigation:",
            "    Tab / Shift+Tab   - Switch between panels",
            "    j / Down          - Move down",
            "    k / Up            - Move up",
            "    t                 - Next tab (in editor)",
            "",
            "  Actions:",
            "    Enter             - Send request",
            "    e                 - Enter edit mode (in editor)",
            "    Esc               - Save & exit edit mode",
            "    Tab               - Switch fields (in edit mode)",
            "    n                 - New request",
            "    d                 - Delete request",
            "    y                 - Duplicate request",
            "    c                 - New collection (in collections)",
            "    x                 - Delete collection (in collections)",
            "",
            "  Editing (when in edit mode):",
            "    Name/URL/Body/Auth: Type to edit, arrows to move cursor",
            "    Method: ←→ or ↑↓ to cycle through methods",
            "    Params/Headers: ↑↓ to navigate, + to add, - to delete",
            "",
            "  Other:",
            "    ?                 - Toggle this help",
            "    q / Ctrl+C        - Quit",
            "",
            "  Press ? to close",
            "",
        ];
        
        let lines: Vec<Line> = help_text
            .into_iter()
            .map(|s| Line::from(s))
            .collect();
        
        let paragraph = Paragraph::new(lines).block(block);
        
        Widget::render(paragraph, area, buf);
    }
}

