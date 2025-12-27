use crate::models::{
    collection::Collection,
    request::HttpRequest,
    response::HttpResponse,
    GrpcRequest,
    GrpcResponse,
    ProtoSchema,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportMode {
    RequestCurl,
    GrpcRequestGrpcurl,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportMenuStage {
    SelectingCollection,
    SelectingRequest,
    ShowingResult,
}

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
pub enum GrpcEditorField {
    Name,
    ServerUrl,
    ServiceName,
    MethodName,
    Message,
    Metadata,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyValueEditMode {
    None,
    Key,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ProtocolType {
    Http,
    Grpc,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProtoLoaderFocus {
    Input,
    SchemaList,
}

#[derive(Debug)]
pub struct AppState {
    pub collections: Vec<Collection>,
    pub requests: Vec<HttpRequest>,
    pub current_response: Option<HttpResponse>,
    
    pub selected_collection: Option<usize>,
    pub selected_request: Option<usize>,
    
    pub focused_panel: Panel,
    pub editor_tab: EditorTab,
    pub show_help: bool,
    pub show_welcome: bool,
    pub show_export_menu: bool,
    pub export_mode: Option<ExportMode>,
    pub export_menu_stage: ExportMenuStage,
    pub export_selected_collection: Option<usize>,
    pub export_selected_request: Option<usize>,
    pub export_result_message: Option<String>,
    pub show_import_menu: bool,
    pub import_file_input: String,
    pub import_file_cursor: usize,
    pub import_result_message: Option<String>,
    pub input_mode: InputMode,
    
    pub is_loading: bool,
    pub loading_message: String,
    
    pub editor_focused_field: EditorField,
    pub kv_edit_mode: KeyValueEditMode,
    
    // Collection editing
    pub editing_collection: bool,
    pub collection_name_input: String,
    pub collection_name_cursor: usize,
    
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

    // gRPC support
    #[allow(dead_code)]
    pub protocol_type: ProtocolType,
    pub grpc_requests: Vec<GrpcRequest>,
    #[allow(dead_code)]
    pub grpc_response: Option<GrpcResponse>,
    pub proto_schemas: Vec<ProtoSchema>,

    // gRPC input buffers for editing
    pub grpc_editor_focused_field: GrpcEditorField,
    pub grpc_name_input: String,
    pub grpc_name_cursor: usize,
    pub grpc_server_url_input: String,
    pub grpc_server_url_cursor: usize,
    pub grpc_service_name_input: String,
    pub grpc_service_name_cursor: usize,
    pub grpc_method_name_input: String,
    pub grpc_method_name_cursor: usize,
    pub grpc_message_input: String,
    pub grpc_message_cursor: usize,
    pub grpc_metadata_input: Vec<(String, String)>,
    pub grpc_metadata_selected: usize,

    // Proto file loading
    pub show_proto_loader: bool,
    pub proto_file_input: String,
    pub proto_file_cursor: usize,
    pub proto_load_result: Option<String>,
    pub selected_proto_schema: Option<usize>, // Index into proto_schemas
    pub proto_loader_focus: ProtoLoaderFocus,

    pub should_quit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            collections: Vec::new(),
            requests: Vec::new(),
            current_response: None,
            
            selected_collection: None,
            selected_request: None,
            
            focused_panel: Panel::Collections,
            editor_tab: EditorTab::Params,
            show_help: false,
            show_welcome: true,
            show_export_menu: false,
            export_mode: None,
            export_menu_stage: ExportMenuStage::SelectingCollection,
            export_selected_collection: None,
            export_selected_request: None,
            export_result_message: None,
            show_import_menu: false,
            import_file_input: String::new(),
            import_file_cursor: 0,
            import_result_message: None,
            input_mode: InputMode::Normal,
            
            is_loading: false,
            loading_message: String::new(),
            
            editor_focused_field: EditorField::Url,
            kv_edit_mode: KeyValueEditMode::None,
            
            editing_collection: false,
            collection_name_input: String::new(),
            collection_name_cursor: 0,
            
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

            // gRPC support
            protocol_type: ProtocolType::Http,
            grpc_requests: Vec::new(),
            grpc_response: None,
            proto_schemas: Vec::new(),

            // gRPC input buffers
            grpc_editor_focused_field: GrpcEditorField::ServerUrl,
            grpc_name_input: String::new(),
            grpc_name_cursor: 0,
            grpc_server_url_input: String::new(),
            grpc_server_url_cursor: 0,
            grpc_service_name_input: String::new(),
            grpc_service_name_cursor: 0,
            grpc_method_name_input: String::new(),
            grpc_method_name_cursor: 0,
            grpc_message_input: String::new(),
            grpc_message_cursor: 0,
            grpc_metadata_input: Vec::new(),
            grpc_metadata_selected: 0,

            // Proto file loading
            show_proto_loader: false,
            proto_file_input: "./".to_string(),
            proto_file_cursor: 2,
            proto_load_result: None,
            selected_proto_schema: None,
            proto_loader_focus: ProtoLoaderFocus::Input,

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
        let name = self.name_input.trim().to_string();
        let method_idx = self.method_input;
        let url = self.url_input.clone();
        let params = self.params_input.clone();
        let headers = self.headers_input.clone();
        let body = self.body_input.clone();
        let auth = self.auth_input.clone();
        
        if let Some(request) = self.get_current_request_mut() {
            if !name.is_empty() {
                request.name = name;
            }
            
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

    pub fn load_current_grpc_request_to_input(&mut self) {
        if let Some(request) = self.get_current_grpc_request() {
            let name = request.name.clone();
            let server_url = request.server_url.clone();
            let service_name = request.service_name.clone();
            let method_name = request.method_name.clone();
            let message = request.message_json.clone();
            let metadata: Vec<(String, String)> = request.metadata.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            self.grpc_name_input = name;
            self.grpc_name_cursor = self.grpc_name_input.len();

            self.grpc_server_url_input = server_url;
            self.grpc_server_url_cursor = self.grpc_server_url_input.len();

            self.grpc_service_name_input = service_name;
            self.grpc_service_name_cursor = self.grpc_service_name_input.len();

            self.grpc_method_name_input = method_name;
            self.grpc_method_name_cursor = self.grpc_method_name_input.len();

            self.grpc_message_input = message;
            self.grpc_message_cursor = self.grpc_message_input.len();

            self.grpc_metadata_input = metadata;
            self.grpc_metadata_selected = 0;
        }
    }

    pub fn save_grpc_input_to_request(&mut self) {
        // Clone all the input values first to avoid borrow checker issues
        let name = self.grpc_name_input.trim().to_string();
        let server_url = self.grpc_server_url_input.clone();
        let service_name = self.grpc_service_name_input.clone();
        let method_name = self.grpc_method_name_input.clone();
        let message = self.grpc_message_input.clone();
        let metadata = self.grpc_metadata_input.clone();

        if let Some(request) = self.get_current_grpc_request_mut() {
            if !name.is_empty() {
                request.name = name;
            }

            request.server_url = server_url;
            request.service_name = service_name;
            request.method_name = method_name;
            request.message_json = message;

            request.metadata.clear();
            for (key, value) in &metadata {
                if !key.is_empty() {
                    request.metadata.insert(key.clone(), value.clone());
                }
            }
        }
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
        // Get the collection_id to filter by
        let collection_id = self.selected_collection
            .and_then(|idx| self.collections.get(idx))
            .map(|c| c.id);

        match self.protocol_type {
            ProtocolType::Http => {
                if let Some(current_idx) = self.selected_request {
                    // Find the next request that belongs to the same collection
                    for idx in (current_idx + 1)..self.requests.len() {
                        let request_collection_id = self.requests.get(idx).and_then(|r| r.collection_id);
                        if collection_id == request_collection_id {
                            self.selected_request = Some(idx);
                            self.clear_input_buffers();
                            return;
                        }
                    }
                } else if !self.requests.is_empty() {
                    // Find the first request that belongs to the selected collection
                    for (idx, request) in self.requests.iter().enumerate() {
                        if collection_id == request.collection_id {
                            self.selected_request = Some(idx);
                            self.clear_input_buffers();
                            return;
                        }
                    }
                }
            }
            ProtocolType::Grpc => {
                if let Some(current_idx) = self.selected_request {
                    // Find the next gRPC request that belongs to the same collection
                    for idx in (current_idx + 1)..self.grpc_requests.len() {
                        let request_collection_id = self.grpc_requests.get(idx).and_then(|r| r.collection_id);
                        if collection_id == request_collection_id {
                            self.selected_request = Some(idx);
                            self.clear_input_buffers();
                            return;
                        }
                    }
                } else if !self.grpc_requests.is_empty() {
                    // Find the first gRPC request that belongs to the selected collection
                    for (idx, request) in self.grpc_requests.iter().enumerate() {
                        if collection_id == request.collection_id {
                            self.selected_request = Some(idx);
                            self.clear_input_buffers();
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn prev_request(&mut self) {
        // Get the collection_id to filter by
        let collection_id = self.selected_collection
            .and_then(|idx| self.collections.get(idx))
            .map(|c| c.id);

        match self.protocol_type {
            ProtocolType::Http => {
                if let Some(current_idx) = self.selected_request {
                    // Find the previous request that belongs to the same collection
                    if current_idx > 0 {
                        for idx in (0..current_idx).rev() {
                            let request_collection_id = self.requests.get(idx).and_then(|r| r.collection_id);
                            if collection_id == request_collection_id {
                                self.selected_request = Some(idx);
                                self.clear_input_buffers();
                                return;
                            }
                        }
                    }
                }
            }
            ProtocolType::Grpc => {
                if let Some(current_idx) = self.selected_request {
                    // Find the previous gRPC request that belongs to the same collection
                    if current_idx > 0 {
                        for idx in (0..current_idx).rev() {
                            let request_collection_id = self.grpc_requests.get(idx).and_then(|r| r.collection_id);
                            if collection_id == request_collection_id {
                                self.selected_request = Some(idx);
                                self.clear_input_buffers();
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn next_collection(&mut self) {
        if let Some(idx) = self.selected_collection {
            if idx < self.collections.len().saturating_sub(1) {
                self.selected_collection = Some(idx + 1);
                self.update_selected_request_for_collection();
            }
        } else if !self.collections.is_empty() {
            self.selected_collection = Some(0);
            self.update_selected_request_for_collection();
        }
    }
    
    pub fn prev_collection(&mut self) {
        if let Some(idx) = self.selected_collection {
            if idx > 0 {
                self.selected_collection = Some(idx - 1);
                self.update_selected_request_for_collection();
            }
        }
    }
    
    pub fn update_selected_request_for_collection(&mut self) {
        match self.protocol_type {
            ProtocolType::Http => {
                if let Some(collection_idx) = self.selected_collection {
                    if let Some(collection) = self.collections.get(collection_idx) {
                        let collection_id = collection.id;
                        if let Some(first_request_idx) = self.requests.iter().position(|r| r.collection_id == Some(collection_id)) {
                            self.selected_request = Some(first_request_idx);
                        } else {
                            self.selected_request = None;
                        }
                    }
                } else {
                    if let Some(first_request_idx) = self.requests.iter().position(|r| r.collection_id.is_none()) {
                        self.selected_request = Some(first_request_idx);
                    } else {
                        self.selected_request = None;
                    }
                }
            }
            ProtocolType::Grpc => {
                if let Some(collection_idx) = self.selected_collection {
                    if let Some(collection) = self.collections.get(collection_idx) {
                        let collection_id = collection.id;
                        if let Some(first_request_idx) = self.grpc_requests.iter().position(|r| r.collection_id == Some(collection_id)) {
                            self.selected_request = Some(first_request_idx);
                        } else {
                            self.selected_request = None;
                        }
                    }
                } else {
                    if let Some(first_request_idx) = self.grpc_requests.iter().position(|r| r.collection_id.is_none()) {
                        self.selected_request = Some(first_request_idx);
                    } else {
                        self.selected_request = None;
                    }
                }
            }
        }
        self.clear_input_buffers();
    }
    
    pub fn clear_input_buffers(&mut self) {
        // Clear HTTP input buffers
        self.name_input.clear();
        self.name_cursor = 0;
        self.method_input = 0;
        self.url_input.clear();
        self.url_cursor = 0;
        self.params_input.clear();
        self.params_selected = 0;
        self.headers_input.clear();
        self.headers_selected = 0;
        self.body_input.clear();
        self.body_cursor = 0;
        self.auth_input.clear();
        self.auth_cursor = 0;
        self.input_mode = InputMode::Normal;
        self.kv_edit_mode = KeyValueEditMode::None;

        // Clear gRPC input buffers
        self.grpc_name_input.clear();
        self.grpc_name_cursor = 0;
        self.grpc_server_url_input.clear();
        self.grpc_server_url_cursor = 0;
        self.grpc_service_name_input.clear();
        self.grpc_service_name_cursor = 0;
        self.grpc_method_name_input.clear();
        self.grpc_method_name_cursor = 0;
        self.grpc_message_input.clear();
        self.grpc_message_cursor = 0;
        self.grpc_metadata_input.clear();
        self.grpc_metadata_selected = 0;
    }
    
    pub fn start_editing_collection(&mut self) {
        if let Some(idx) = self.selected_collection {
            if let Some(collection) = self.collections.get(idx) {
                self.editing_collection = true;
                self.collection_name_input = collection.name.clone();
                self.collection_name_cursor = self.collection_name_input.len();
            }
        }
    }
    
    pub fn save_collection_name(&mut self) {
        if let Some(idx) = self.selected_collection {
            if let Some(collection) = self.collections.get_mut(idx) {
                let trimmed_name = self.collection_name_input.trim();
                if !trimmed_name.is_empty() {
                    collection.name = trimmed_name.to_string();
                }
            }
        }
        self.editing_collection = false;
        self.collection_name_input.clear();
        self.collection_name_cursor = 0;
    }
    
    pub fn cancel_collection_editing(&mut self) {
        self.editing_collection = false;
        self.collection_name_input.clear();
        self.collection_name_cursor = 0;
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

    pub fn add_grpc_metadata(&mut self) {
        self.grpc_metadata_input.push((String::new(), String::new()));
        self.grpc_metadata_selected = self.grpc_metadata_input.len().saturating_sub(1);
    }

    pub fn delete_grpc_metadata(&mut self) {
        if !self.grpc_metadata_input.is_empty() && self.grpc_metadata_selected < self.grpc_metadata_input.len() {
            self.grpc_metadata_input.remove(self.grpc_metadata_selected);
            if self.grpc_metadata_selected >= self.grpc_metadata_input.len() && !self.grpc_metadata_input.is_empty() {
                self.grpc_metadata_selected = self.grpc_metadata_input.len() - 1;
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

    // gRPC request helpers

    #[allow(dead_code)]
    pub fn get_current_grpc_request(&self) -> Option<&GrpcRequest> {
        self.selected_request.and_then(|idx| self.grpc_requests.get(idx))
    }

    #[allow(dead_code)]
    pub fn get_current_grpc_request_mut(&mut self) -> Option<&mut GrpcRequest> {
        self.selected_request.and_then(|idx| self.grpc_requests.get_mut(idx))
    }

    #[allow(dead_code)]
    pub fn get_all_requests_count(&self) -> usize {
        match self.protocol_type {
            ProtocolType::Http => self.requests.len(),
            ProtocolType::Grpc => self.grpc_requests.len(),
        }
    }

    // Proto file management helpers

    pub fn get_selected_proto_schema(&self) -> Option<&ProtoSchema> {
        self.selected_proto_schema
            .and_then(|idx| self.proto_schemas.get(idx))
    }

    pub fn open_proto_loader(&mut self) {
        self.show_proto_loader = true;
        self.proto_file_input = "./".to_string();
        self.proto_file_cursor = 2;
        self.proto_load_result = None;
        self.proto_loader_focus = ProtoLoaderFocus::Input;
    }

    pub fn close_proto_loader(&mut self) {
        self.show_proto_loader = false;
        self.proto_file_input.clear();
        self.proto_file_cursor = 0;
        self.proto_load_result = None;
        self.proto_loader_focus = ProtoLoaderFocus::Input;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

