use crate::app::state::{AppState, ExportMode, ExportMenuStage};
use crate::import::import_postman_collection;
use crate::models::collection::Collection;
use crate::models::request::HttpRequest;
use uuid::Uuid;
use std::fs;
use std::path::{Path, PathBuf};

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
    NewRequest,
    DeleteRequest,
    DuplicateRequest,
    NewCollection,
    DeleteCollection,
    EditCollection,
    CopyResponse,
    OpenExportMenu,
    OpenCurlExportMenu,
    ExportCollectionJson,
    ExportRequestCurl,
    OpenImportMenu,
    ImportPostmanCollection,
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
                let mut request = HttpRequest::default();
                if let Some(collection_idx) = state.selected_collection {
                    if let Some(collection) = state.collections.get(collection_idx) {
                        request.collection_id = Some(collection.id);
                    }
                }
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
                    state.clear_input_buffers();
                }
            }
            Action::DuplicateRequest => {
                if let Some(request) = state.get_current_request() {
                    let mut new_request = request.clone();
                    new_request.id = Uuid::new_v4();
                    new_request.name = format!("{} (copy)", new_request.name);
                    if let Some(collection_idx) = state.selected_collection {
                        if let Some(collection) = state.collections.get(collection_idx) {
                            new_request.collection_id = Some(collection.id);
                        }
                    }
                    state.requests.push(new_request);
                    state.selected_request = Some(state.requests.len() - 1);
                }
            }
            Action::NewCollection => {
                let collection_num = state.collections.len() + 1;
                let collection = Collection::new(format!("Collection {}", collection_num));
                state.collections.push(collection);
                state.selected_collection = Some(state.collections.len() - 1);
                state.update_selected_request_for_collection();
            }
            Action::DeleteCollection => {
                if let Some(idx) = state.selected_collection {
                    state.collections.remove(idx);
                    if state.collections.is_empty() {
                        state.selected_collection = None;
                    } else if idx >= state.collections.len() {
                        state.selected_collection = Some(state.collections.len() - 1);
                    }
                }
            }
            Action::EditCollection => {
                state.start_editing_collection();
            }
            Action::CopyResponse => {
                if let Some(response) = &state.current_response {
                    let text_to_copy = response.formatted_body();
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        let _ = clipboard.set_text(text_to_copy);
                    }
                }
            }
            Action::OpenExportMenu => {
                state.show_export_menu = true;
                state.export_mode = Some(ExportMode::CollectionJson);
                state.export_menu_stage = ExportMenuStage::SelectingCollection;
                state.export_selected_collection = if !state.collections.is_empty() { Some(0) } else { None };
                state.export_selected_request = None;
                state.export_result_message = None;
            }
            Action::OpenCurlExportMenu => {
                state.show_export_menu = true;
                state.export_mode = Some(ExportMode::RequestCurl);
                state.export_menu_stage = ExportMenuStage::SelectingCollection;
                state.export_selected_collection = if !state.collections.is_empty() { Some(0) } else { None };
                state.export_selected_request = None;
                state.export_result_message = None;
            }
            Action::ExportCollectionJson => {
                if let Some(collection_idx) = state.export_selected_collection {
                    if let Some(collection) = state.collections.get(collection_idx) {
                        // Get all requests for this collection
                        let collection_requests: Vec<_> = state.requests
                            .iter()
                            .filter(|r| r.collection_id == Some(collection.id))
                            .cloned()
                            .collect();
                        
                        if let Ok(json) = collection.to_json(&collection_requests) {
                            // Create exports directory if it doesn't exist
                            let exports_dir = PathBuf::from("exports");
                            let _ = fs::create_dir_all(&exports_dir);
                            
                            // Generate filename based on collection name
                            let safe_name = collection.name
                                .chars()
                                .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
                                .collect::<String>();
                            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                            let filename = format!("{}_{}.json", safe_name, timestamp);
                            let filepath = exports_dir.join(&filename);
                            
                            match fs::write(&filepath, json) {
                                Ok(_) => {
                                    state.export_result_message = Some(filepath.to_string_lossy().to_string());
                                    state.export_menu_stage = ExportMenuStage::ShowingResult;
                                }
                                Err(_) => {
                                    state.export_result_message = Some("Failed to save export".to_string());
                                    state.export_menu_stage = ExportMenuStage::ShowingResult;
                                }
                            }
                        }
                    }
                }
            }
            Action::ExportRequestCurl => {
                if let Some(request_idx) = state.export_selected_request {
                    if let Some(request) = state.requests.get(request_idx) {
                        let curl = request.to_curl();

                        // Create exports directory if it doesn't exist
                        let exports_dir = PathBuf::from("exports");
                        let _ = fs::create_dir_all(&exports_dir);

                        // Generate filename based on request name
                        let safe_name = request.name
                            .chars()
                            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
                            .collect::<String>();
                        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                        let filename = format!("{}_{}.sh", safe_name, timestamp);
                        let filepath = exports_dir.join(&filename);

                        match fs::write(&filepath, format!("#!/bin/bash\n\n{}\n", curl)) {
                            Ok(_) => {
                                state.export_result_message = Some(filepath.to_string_lossy().to_string());
                                state.export_menu_stage = ExportMenuStage::ShowingResult;
                                // Also copy to clipboard for convenience
                                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                    let _ = clipboard.set_text(curl);
                                }
                            }
                            Err(_) => {
                                state.export_result_message = Some("Failed to save export".to_string());
                                state.export_menu_stage = ExportMenuStage::ShowingResult;
                            }
                        }
                    }
                }
            }
            Action::OpenImportMenu => {
                state.show_import_menu = true;
                state.import_file_input = "./".to_string();
                state.import_file_cursor = 2;
                state.import_result_message = None;
            }
            Action::ImportPostmanCollection => {
                let file_path = state.import_file_input.trim();

                if file_path.is_empty() {
                    state.import_result_message = Some("Error: Please enter a file path".to_string());
                    return;
                }

                // Expand ~ to home directory
                let expanded_path = if file_path.starts_with("~/") {
                    if let Ok(home) = std::env::var("HOME") {
                        file_path.replacen("~", &home, 1)
                    } else {
                        file_path.to_string()
                    }
                } else {
                    file_path.to_string()
                };

                let path = Path::new(&expanded_path);

                if !path.exists() {
                    state.import_result_message = Some(format!("Error: File not found: {}", file_path));
                    return;
                }

                match import_postman_collection(path) {
                    Ok((collection, requests)) => {
                        let num_requests = requests.len();

                        // Add the collection
                        state.collections.push(collection.clone());
                        state.selected_collection = Some(state.collections.len() - 1);

                        // Add all the requests
                        for request in requests {
                            state.requests.push(request);
                        }

                        // Select the first imported request
                        if num_requests > 0 {
                            state.selected_request = Some(state.requests.len() - num_requests);
                        }

                        state.import_result_message = Some(format!(
                            "Successfully imported collection '{}' with {} request(s)",
                            collection.name,
                            num_requests
                        ));
                    }
                    Err(e) => {
                        state.import_result_message = Some(format!("Error: {}", e));
                    }
                }
            }
        }
    }
}

