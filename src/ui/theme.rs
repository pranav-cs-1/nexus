use ratatui::style::{Color, Style};

pub struct Theme;

impl Theme {
    pub fn default() -> Style {
        Style::default().fg(Color::White)
    }
    
    pub fn selected() -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
    }
    
    pub fn focused_border() -> Style {
        Style::default().fg(Color::Cyan)
    }
    
    pub fn unfocused_border() -> Style {
        Style::default().fg(Color::DarkGray)
    }
    
    pub fn method_get() -> Style {
        Style::default().fg(Color::Green)
    }
    
    pub fn method_post() -> Style {
        Style::default().fg(Color::Blue)
    }
    
    pub fn method_put() -> Style {
        Style::default().fg(Color::Yellow)
    }
    
    pub fn method_delete() -> Style {
        Style::default().fg(Color::Red)
    }
    
    pub fn method_patch() -> Style {
        Style::default().fg(Color::Magenta)
    }
    
    pub fn method_other() -> Style {
        Style::default().fg(Color::Gray)
    }
}

