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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorField {
    Name,
    Method,
    Url,
    Params,
    Headers,
    Body,
    Auth,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyValueEditMode {
    None,
    Key,
    Value,
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
    pub input_mode: InputMode,
    
    pub is_loading: bool,
    pub loading_message: String,
    
    pub editor_focused_field: EditorField,
    pub kv_edit_mode: KeyValueEditMode,
    
    // Input buffers for editing
    pub name_input: String,
    pub name_cursor: usize,
    pub method_input: usize, // Index into HttpMethod::all()
    pub url_input: String,
    pub url_cursor: usize,
    pub params_input: Vec<(String, String)>,
    pub params_selected: usize,
    pub headers_input: Vec<(String, String)>,
    pub headers_selected: usize,
    pub body_input: String,
    pub body_cursor: usize,
    pub auth_input: String,
    pub auth_cursor: usize,
    
    // Response viewer scroll state
    pub response_scroll: u16,
    
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
            input_mode: InputMode::Normal,
            
            is_loading: false,
            loading_message: String::new(),
            
            editor_focused_field: EditorField::Url,
            kv_edit_mode: KeyValueEditMode::None,
            
            name_input: String::new(),
            name_cursor: 0,
            method_input: 0,
            url_input: String::new(),
            url_cursor: 0,
            params_input: Vec::new(),
            params_selected: 0,
            headers_input: Vec::new(),
            headers_selected: 0,
            body_input: String::new(),
            body_cursor: 0,
            auth_input: String::new(),
            auth_cursor: 0,
            
            response_scroll: 0,
            
            should_quit: false,
        }
    }
    
    pub fn get_current_request(&self) -> Option<&HttpRequest> {
        self.selected_request.and_then(|idx| self.requests.get(idx))
    }
    
    pub fn get_current_request_mut(&mut self) -> Option<&mut HttpRequest> {
        self.selected_request.and_then(|idx| self.requests.get_mut(idx))
    }
    
    pub fn load_current_request_to_input(&mut self) {
        if let Some(request) = self.get_current_request() {
            let name = request.name.clone();
            let method = request.method.clone();
            let url = request.url.clone();
            let params: Vec<(String, String)> = request.query_params.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let headers: Vec<(String, String)> = request.headers.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let body = request.body.clone().unwrap_or_default();
            let auth = match &request.auth {
                crate::models::request::AuthType::Bearer { token } => token.clone(),
                _ => String::new(),
            };
            
            self.name_input = name;
            self.name_cursor = self.name_input.len();
            
            self.method_input = crate::models::request::HttpMethod::all()
                .iter()
                .position(|m| *m == method)
                .unwrap_or(0);
            
            self.url_input = url;
            self.url_cursor = self.url_input.len();
            
            self.params_input = params;
            self.params_selected = 0;
            
            self.headers_input = headers;
            self.headers_selected = 0;
            
            self.body_input = body;
            self.body_cursor = self.body_input.len();
            
            self.auth_input = auth;
            self.auth_cursor = self.auth_input.len();
        }
    }
    
    pub fn save_input_to_request(&mut self) {
        // Clone all the input values first to avoid borrow checker issues
        let name = self.name_input.clone();
        let method_idx = self.method_input;
        let url = self.url_input.clone();
        let params = self.params_input.clone();
        let headers = self.headers_input.clone();
        let body = self.body_input.clone();
        let auth = self.auth_input.clone();
        
        if let Some(request) = self.get_current_request_mut() {
            request.name = name;
            
            if let Some(method) = crate::models::request::HttpMethod::all().get(method_idx) {
                request.method = method.clone();
            }
            
            request.url = url;
            
            request.query_params.clear();
            for (key, value) in &params {
                if !key.is_empty() {
                    request.query_params.insert(key.clone(), value.clone());
                }
            }
            
            request.headers.clear();
            for (key, value) in &headers {
                if !key.is_empty() {
                    request.headers.insert(key.clone(), value.clone());
                }
            }
            
            request.body = if !body.is_empty() {
                Some(body)
            } else {
                None
            };
            
            if !auth.is_empty() {
                request.auth = crate::models::request::AuthType::Bearer {
                    token: auth,
                };
            } else {
                request.auth = crate::models::request::AuthType::None;
            }
        }
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
    
    pub fn add_param(&mut self) {
        self.params_input.push((String::new(), String::new()));
        self.params_selected = self.params_input.len().saturating_sub(1);
    }
    
    pub fn delete_param(&mut self) {
        if !self.params_input.is_empty() && self.params_selected < self.params_input.len() {
            self.params_input.remove(self.params_selected);
            if self.params_selected >= self.params_input.len() && !self.params_input.is_empty() {
                self.params_selected = self.params_input.len() - 1;
            }
        }
    }
    
    pub fn add_header(&mut self) {
        self.headers_input.push((String::new(), String::new()));
        self.headers_selected = self.headers_input.len().saturating_sub(1);
    }
    
    pub fn delete_header(&mut self) {
        if !self.headers_input.is_empty() && self.headers_selected < self.headers_input.len() {
            self.headers_input.remove(self.headers_selected);
            if self.headers_selected >= self.headers_input.len() && !self.headers_input.is_empty() {
                self.headers_selected = self.headers_input.len() - 1;
            }
        }
    }
    
    pub fn scroll_response_down(&mut self) {
        self.response_scroll = self.response_scroll.saturating_add(1);
    }
    
    pub fn scroll_response_up(&mut self) {
        self.response_scroll = self.response_scroll.saturating_sub(1);
    }
    
    pub fn reset_response_scroll(&mut self) {
        self.response_scroll = 0;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

