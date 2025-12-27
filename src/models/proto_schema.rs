use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a cached proto schema descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoSchema {
    pub id: Uuid,
    pub name: String,
    pub collection_id: Option<Uuid>,

    // Compiled proto descriptor
    pub file_descriptor_set: Vec<u8>,

    // Source information
    pub source_type: ProtoSourceType,
    pub source_path: Option<String>,

    // Metadata
    pub services: Vec<ServiceInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtoSourceType {
    LocalFile,
    Reflection,
    Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub methods: Vec<MethodInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodInfo {
    pub name: String,
    pub rpc_type: String, // "unary", "server_streaming", etc.
    pub input_type: String,
    pub output_type: String,
}

#[allow(dead_code)]
impl ProtoSchema {
    pub fn new(name: String, file_descriptor_set: Vec<u8>, source_type: ProtoSourceType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            collection_id: None,
            file_descriptor_set,
            source_type,
            source_path: None,
            services: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
