use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    pub id: Uuid,
    pub request_id: Uuid,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub body_text: Option<String>,
    pub duration_ms: u64,
    pub size_bytes: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
}

impl HttpResponse {
    
    pub fn get_content_type(&self) -> Option<&String> {
        self.headers.get("content-type")
            .or_else(|| self.headers.get("Content-Type"))
    }
    
    pub fn is_json(&self) -> bool {
        self.get_content_type()
            .map(|ct| ct.contains("application/json"))
            .unwrap_or(false)
    }
    
    pub fn formatted_body(&self) -> String {
        if let Some(text) = &self.body_text {
            if self.is_json() {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
                    return serde_json::to_string_pretty(&value)
                        .unwrap_or_else(|_| text.clone());
                }
            }
            text.clone()
        } else {
            format!("<binary data, {} bytes>", self.size_bytes)
        }
    }
    
    pub fn status_color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self.status_code {
            200..=299 => Color::Green,
            300..=399 => Color::Blue,
            400..=499 => Color::Yellow,
            500..=599 => Color::Red,
            _ => Color::White,
        }
    }
}

