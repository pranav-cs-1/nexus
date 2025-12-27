use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcRequest {
    pub id: Uuid,
    pub name: String,
    pub server_url: String, // e.g., "localhost:50051"

    // Service definition
    pub service_name: String, // e.g., "user.UserService"
    pub method_name: String,  // e.g., "GetUser"
    pub rpc_type: RpcType,

    // Proto source
    pub proto_source: ProtoSource,

    // Request data
    pub message_json: String,                 // JSON representation
    pub metadata: HashMap<String, String>,    // gRPC headers

    // Options
    pub use_tls: bool,
    pub timeout_seconds: Option<u64>,

    // Organization (same as HttpRequest)
    pub collection_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcType {
    Unary,
    ServerStreaming,
    ClientStreaming,
    BidirectionalStreaming,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtoSource {
    File {
        proto_path: String,
        file_descriptor_set: Vec<u8>, // Cached descriptor
    },
    Reflection {
        cached_descriptor: Option<Vec<u8>>,
        last_fetched: DateTime<Utc>,
    },
}

#[allow(dead_code)]
impl GrpcRequest {
    pub fn new(name: String, server_url: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            server_url,
            service_name: String::new(),
            method_name: String::new(),
            rpc_type: RpcType::Unary,
            proto_source: ProtoSource::Reflection {
                cached_descriptor: None,
                last_fetched: now,
            },
            message_json: "{}".to_string(),
            metadata: HashMap::new(),
            use_tls: false,
            timeout_seconds: Some(30),
            collection_id: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            description: None,
        }
    }

    /// Generate grpcurl command equivalent for this request
    pub fn to_grpcurl(&self) -> String {
        let mut cmd = String::from("grpcurl");

        // Add TLS/plaintext flag
        if !self.use_tls {
            cmd.push_str(" -plaintext");
        }

        // Add metadata (headers)
        for (key, value) in &self.metadata {
            cmd.push_str(&format!(" \\\n  -H '{}: {}'", key, value));
        }

        // Add proto source if it's from a file
        match &self.proto_source {
            ProtoSource::File { proto_path, .. } => {
                // Use -protoset for .pb files (FileDescriptorSet), -proto for .proto files
                if proto_path.ends_with(".pb") || proto_path.ends_with(".bin") {
                    cmd.push_str(&format!(" \\\n  -protoset '{}'", proto_path));
                } else {
                    cmd.push_str(&format!(" \\\n  -proto '{}'", proto_path));
                }
            }
            ProtoSource::Reflection { .. } => {
                // When using reflection, grpcurl automatically uses it
                // No additional flag needed
            }
        }

        // Add request data
        if !self.message_json.is_empty() && self.message_json != "{}" {
            let escaped_json = self.message_json.replace('\'', "'\\''");
            cmd.push_str(&format!(" \\\n  -d '{}'", escaped_json));
        }

        // Add server address and full method name
        let full_method = if !self.service_name.is_empty() && !self.method_name.is_empty() {
            format!("{}/{}", self.service_name, self.method_name)
        } else {
            "SERVICE/METHOD".to_string()
        };

        cmd.push_str(&format!(" \\\n  {} \\", self.server_url));
        cmd.push_str(&format!("\n  {}", full_method));

        cmd
    }
}
