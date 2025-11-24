use crate::app::state::AppState;
use crate::ui::{
    components::{
        collection_list::CollectionList,
        request_list::RequestList,
        request_editor::RequestEditor,
        response_viewer::ResponseViewer,
        statusbar::StatusBar,
        help_popup::HelpPopup,
    },
    layout::Layout,
};
use ratatui::{Frame, layout::{Constraint, Direction, Layout as RatatuiLayout, Rect}};

pub struct UI;

impl UI {
    pub fn draw(frame: &mut Frame, state: &AppState) {
        let layout = Layout::new(frame.area());
        
        Self::draw_collections(frame, layout.collections, state);
        Self::draw_requests(frame, layout.requests, state);
        Self::draw_editor(frame, layout.editor, state);
        Self::draw_response(frame, layout.response, state);
        Self::draw_statusbar(frame, layout.statusbar, state);
        
        if state.show_help {
            Self::draw_help(frame, state);
        }
    }
    
    fn draw_collections(frame: &mut Frame, area: Rect, state: &AppState) {
        let component = CollectionList::new(state);
        frame.render_widget(component, area);
    }
    
    fn draw_requests(frame: &mut Frame, area: Rect, state: &AppState) {
        let component = RequestList::new(state);
        frame.render_widget(component, area);
    }
    
    fn draw_editor(frame: &mut Frame, area: Rect, state: &AppState) {
        let component = RequestEditor::new(state);
        frame.render_widget(component, area);
    }
    
    fn draw_response(frame: &mut Frame, area: Rect, state: &AppState) {
        let component = ResponseViewer::new(state);
        frame.render_widget(component, area);
    }
    
    fn draw_statusbar(frame: &mut Frame, area: Rect, state: &AppState) {
        let component = StatusBar::new(state);
        frame.render_widget(component, area);
    }
    
    fn draw_help(frame: &mut Frame, _state: &AppState) {
        let component = HelpPopup::new();
        let area = Self::centered_rect(frame.area(), 60, 80);
        frame.render_widget(component, area);
    }
    
    fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let popup_layout = RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        RatatuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

