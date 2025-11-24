use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{request::HttpRequest, response::HttpResponse};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub request: HttpRequest,
    pub response: HttpResponse,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HistoryEntry {
    pub fn new(request: HttpRequest, response: HttpResponse) -> Self {
        Self {
            id: Uuid::new_v4(),
            request,
            response,
            timestamp: chrono::Utc::now(),
        }
    }
}

