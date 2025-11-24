use ratatui::style::{Color, Modifier, Style};

pub struct Theme;

impl Theme {
    pub fn default() -> Style {
        Style::default().fg(Color::White)
    }
    
    pub fn title() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
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
    
    pub fn error() -> Style {
        Style::default().fg(Color::Red)
    }
    
    pub fn success() -> Style {
        Style::default().fg(Color::Green)
    }
    
    pub fn warning() -> Style {
        Style::default().fg(Color::Yellow)
    }
    
    pub fn info() -> Style {
        Style::default().fg(Color::Blue)
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

