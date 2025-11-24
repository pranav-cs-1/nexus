use crate::models::{
    collection::Collection,
    environment::Environment,
    history::HistoryEntry,
    request::HttpRequest,
    response::HttpResponse,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Collections,
    Requests,
    RequestEditor,
    Response,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorTab {
    Params,
    Headers,
    Body,
    Auth,
}

#[derive(Debug)]
pub struct AppState {
    pub collections: Vec<Collection>,
    pub requests: Vec<HttpRequest>,
    pub current_response: Option<HttpResponse>,
    pub history: Vec<HistoryEntry>,
    pub environments: Vec<Environment>,
    
    pub selected_collection: Option<usize>,
    pub selected_request: Option<usize>,
    pub selected_history: Option<usize>,
    
    pub focused_panel: Panel,
    pub editor_tab: EditorTab,
    pub show_help: bool,
    pub show_environment_selector: bool,
    
    pub is_loading: bool,
    pub loading_message: String,
    
    pub url_input: String,
    pub url_cursor: usize,
    pub body_input: String,
    pub body_cursor: usize,
    
    pub should_quit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            collections: Vec::new(),
            requests: Vec::new(),
            current_response: None,
            history: Vec::new(),
            environments: Vec::new(),
            
            selected_collection: None,
            selected_request: None,
            selected_history: None,
            
            focused_panel: Panel::Collections,
            editor_tab: EditorTab::Params,
            show_help: false,
            show_environment_selector: false,
            
            is_loading: false,
            loading_message: String::new(),
            
            url_input: String::new(),
            url_cursor: 0,
            body_input: String::new(),
            body_cursor: 0,
            
            should_quit: false,
        }
    }
    
    pub fn get_current_request(&self) -> Option<&HttpRequest> {
        self.selected_request.and_then(|idx| self.requests.get(idx))
    }
    
    pub fn get_current_request_mut(&mut self) -> Option<&mut HttpRequest> {
        self.selected_request.and_then(|idx| self.requests.get_mut(idx))
    }
    
    pub fn active_environment(&self) -> Option<&Environment> {
        self.environments.iter().find(|e| e.is_active)
    }
    
    pub fn next_panel(&mut self) {
        use Panel::*;
        self.focused_panel = match self.focused_panel {
            Collections => Requests,
            Requests => RequestEditor,
            RequestEditor => Response,
            Response => Collections,
        };
    }
    
    pub fn prev_panel(&mut self) {
        use Panel::*;
        self.focused_panel = match self.focused_panel {
            Collections => Response,
            Requests => Collections,
            RequestEditor => Requests,
            Response => RequestEditor,
        };
    }
    
    pub fn next_editor_tab(&mut self) {
        use EditorTab::*;
        self.editor_tab = match self.editor_tab {
            Params => Headers,
            Headers => Body,
            Body => Auth,
            Auth => Params,
        };
    }
    
    pub fn next_request(&mut self) {
        if let Some(idx) = self.selected_request {
            if idx < self.requests.len().saturating_sub(1) {
                self.selected_request = Some(idx + 1);
            }
        } else if !self.requests.is_empty() {
            self.selected_request = Some(0);
        }
    }
    
    pub fn prev_request(&mut self) {
        if let Some(idx) = self.selected_request {
            if idx > 0 {
                self.selected_request = Some(idx - 1);
            }
        }
    }
    
    pub fn next_collection(&mut self) {
        if let Some(idx) = self.selected_collection {
            if idx < self.collections.len().saturating_sub(1) {
                self.selected_collection = Some(idx + 1);
            }
        } else if !self.collections.is_empty() {
            self.selected_collection = Some(0);
        }
    }
    
    pub fn prev_collection(&mut self) {
        if let Some(idx) = self.selected_collection {
            if idx > 0 {
                self.selected_collection = Some(idx - 1);
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

