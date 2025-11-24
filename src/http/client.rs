use crate::models::{request::HttpRequest, response::HttpResponse};
use anyhow::Result;
use std::time::Instant;
use uuid::Uuid;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        Ok(Self { client })
    }
    
    pub async fn execute(&self, request: &HttpRequest) -> Result<HttpResponse> {
        let start = Instant::now();
        
        let mut req_builder = match request.method {
            crate::models::request::HttpMethod::GET => {
                self.client.get(&request.full_url())
            }
            crate::models::request::HttpMethod::POST => {
                self.client.post(&request.full_url())
            }
            crate::models::request::HttpMethod::PUT => {
                self.client.put(&request.full_url())
            }
            crate::models::request::HttpMethod::PATCH => {
                self.client.patch(&request.full_url())
            }
            crate::models::request::HttpMethod::DELETE => {
                self.client.delete(&request.full_url())
            }
            crate::models::request::HttpMethod::HEAD => {
                self.client.head(&request.full_url())
            }
            crate::models::request::HttpMethod::OPTIONS => {
                self.client.request(
                    reqwest::Method::OPTIONS,
                    &request.full_url()
                )
            }
        };
        
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }
        
        req_builder = match &request.auth {
            crate::models::request::AuthType::Bearer { token } => {
                req_builder.bearer_auth(token)
            }
            crate::models::request::AuthType::Basic { username, password } => {
                req_builder.basic_auth(username, Some(password))
            }
            crate::models::request::AuthType::ApiKey { key, value, location } => {
                match location {
                    crate::models::request::ApiKeyLocation::Header => {
                        req_builder.header(key, value)
                    }
                    crate::models::request::ApiKeyLocation::QueryParam => {
                        req_builder.query(&[(key, value)])
                    }
                }
            }
            crate::models::request::AuthType::None => req_builder,
        };
        
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }
        
        let response = req_builder.send().await?;
        let duration = start.elapsed();
        
        let status_code = response.status().as_u16();
        let status_text = response.status().to_string();
        
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    v.to_str().unwrap_or("<invalid>").to_string(),
                )
            })
            .collect();
        
        let body = response.bytes().await?;
        let size_bytes = body.len();
        
        let body_text = String::from_utf8(body.to_vec()).ok();
        
        Ok(HttpResponse {
            id: Uuid::new_v4(),
            request_id: request.id,
            status_code,
            status_text,
            headers,
            body: body.to_vec(),
            body_text,
            duration_ms: duration.as_millis() as u64,
            size_bytes,
            timestamp: chrono::Utc::now(),
            error: None,
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

