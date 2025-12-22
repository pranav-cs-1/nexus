use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcResponse {
    pub id: Uuid,
    pub request_id: Uuid,

    pub messages: Vec<GrpcMessage>, // Support streaming
    pub status: GrpcStatus,         // gRPC status code
    pub metadata: HashMap<String, String>,
    pub trailers: HashMap<String, String>,

    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcMessage {
    pub message_json: String, // JSON representation
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcStatus {
    pub code: i32, // 0 = OK
    pub message: String,
}

impl GrpcResponse {
    pub fn new(request_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            request_id,
            messages: Vec::new(),
            status: GrpcStatus {
                code: 0,
                message: "OK".to_string(),
            },
            metadata: HashMap::new(),
            trailers: HashMap::new(),
            duration_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status.code == 0
    }
}
