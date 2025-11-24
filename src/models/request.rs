use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "PATCH" => Some(HttpMethod::PATCH),
            "DELETE" => Some(HttpMethod::DELETE),
            "HEAD" => Some(HttpMethod::HEAD),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            _ => None,
        }
    }
    
    pub fn all() -> Vec<Self> {
        vec![
            HttpMethod::GET,
            HttpMethod::POST,
            HttpMethod::PUT,
            HttpMethod::PATCH,
            HttpMethod::DELETE,
            HttpMethod::HEAD,
            HttpMethod::OPTIONS,
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthType {
    None,
    Bearer { token: String },
    Basic { username: String, password: String },
    ApiKey { key: String, value: String, location: ApiKeyLocation },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiKeyLocation {
    Header,
    QueryParam,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    pub id: Uuid,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<String>,
    pub auth: AuthType,
    pub timeout_seconds: Option<u64>,
    pub follow_redirects: bool,
    pub verify_ssl: bool,
    
    pub collection_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub description: Option<String>,
}

impl HttpRequest {
    pub fn new(name: String, method: HttpMethod, url: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            method,
            url,
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
            auth: AuthType::None,
            timeout_seconds: Some(30),
            follow_redirects: true,
            verify_ssl: true,
            collection_id: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            description: None,
        }
    }
    
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    pub fn with_query_param(mut self, key: String, value: String) -> Self {
        self.query_params.insert(key, value);
        self
    }
    
    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    
    pub fn full_url(&self) -> String {
        if self.query_params.is_empty() {
            self.url.clone()
        } else {
            let params: Vec<String> = self.query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!("{}?{}", self.url, params.join("&"))
        }
    }
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self::new(
            "New Request".to_string(),
            HttpMethod::GET,
            "https://api.example.com".to_string(),
        )
    }
}

