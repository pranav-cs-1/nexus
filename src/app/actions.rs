use crate::app::state::AppState;
use crate::models::request::HttpRequest;
use uuid::Uuid;

pub enum Action {
    Quit,
    ToggleHelp,
    NextPanel,
    PrevPanel,
    NextRequest,
    PrevRequest,
    NextCollection,
    PrevCollection,
    NextEditorTab,
    SendRequest,
    NewRequest,
    DeleteRequest,
    DuplicateRequest,
    SaveRequest,
    ToggleEnvironmentSelector,
    ExportAsCurl,
}

impl Action {
    pub fn execute(&self, state: &mut AppState) {
        match self {
            Action::Quit => state.should_quit = true,
            Action::ToggleHelp => state.show_help = !state.show_help,
            Action::NextPanel => state.next_panel(),
            Action::PrevPanel => state.prev_panel(),
            Action::NextRequest => state.next_request(),
            Action::PrevRequest => state.prev_request(),
            Action::NextCollection => state.next_collection(),
            Action::PrevCollection => state.prev_collection(),
            Action::NextEditorTab => state.next_editor_tab(),
            Action::NewRequest => {
                let request = HttpRequest::default();
                state.requests.push(request);
                state.selected_request = Some(state.requests.len() - 1);
            }
            Action::DeleteRequest => {
                if let Some(idx) = state.selected_request {
                    state.requests.remove(idx);
                    if state.requests.is_empty() {
                        state.selected_request = None;
                    } else if idx >= state.requests.len() {
                        state.selected_request = Some(state.requests.len() - 1);
                    }
                }
            }
            Action::DuplicateRequest => {
                if let Some(request) = state.get_current_request() {
                    let mut new_request = request.clone();
                    new_request.id = Uuid::new_v4();
                    new_request.name = format!("{} (copy)", new_request.name);
                    state.requests.push(new_request);
                    state.selected_request = Some(state.requests.len() - 1);
                }
            }
            Action::ToggleEnvironmentSelector => {
                state.show_environment_selector = !state.show_environment_selector;
            }
            _ => {}
        }
    }
}

