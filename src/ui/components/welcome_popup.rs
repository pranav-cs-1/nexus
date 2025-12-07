use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
    style::{Style, Color, Modifier},
};

pub struct WelcomePopup;

impl WelcomePopup {
    pub fn new() -> Self {
        Self
    }
}

impl Widget for WelcomePopup {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        Clear.render(area, buf);
        
        let block = Block::default()
            .title("Welcome to Nexus")
            .borders(Borders::ALL)
            .border_style(Theme::focused_border());
        
        let welcome_text = vec![
            "",
            "  ███╗   ██╗███████╗██╗  ██╗██╗   ██╗███████╗",
            "  ████╗  ██║██╔════╝╚██╗██╔╝██║   ██║██╔════╝",
            "  ██╔██╗ ██║█████╗   ╚███╔╝ ██║   ██║███████╗",
            "  ██║╚██╗██║██╔══╝   ██╔██╗ ██║   ██║╚════██║",
            "  ██║ ╚████║███████╗██╔╝ ██╗╚██████╔╝███████║",
            "  ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝",
            "",
            "  A terminal-based HTTP client",
            "",
            "  Nexus is a keyboard-driven API testing tool that lives",
            "  in your terminal.",
            "",
            "  Quick Start:",
            "    • Tab / Shift+Tab   - Navigate between panels",
            "    • j/k or ↑/↓        - Move through items",
            "    • e                 - Edit a request",
            "    • Enter             - Send the request",
            "    • n                 - Create a new request",
            "    • ?                 - Show help anytime",
            "",
            "  Get Started:",
            "    Check out the 'Example Collection' on the left to see",
            "    sample requests demonstrating Nexus features.",
            "",
            "  Press any key to continue...",
            "",
        ];
        
        let lines: Vec<Line> = welcome_text
            .into_iter()
            .enumerate()
            .map(|(i, s)| {
                // Color the logo lines (lines 1-7)
                if i >= 1 && i <= 7 {
                    Line::from(s).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                } else if i == 8 || i == 13 || i == 21 {
                    // Highlight section headers
                    Line::from(s).style(Style::default().add_modifier(Modifier::BOLD))
                } else {
                    Line::from(s)
                }
            })
            .collect();
        
        let paragraph = Paragraph::new(lines).block(block);
        
        Widget::render(paragraph, area, buf);
    }
}

