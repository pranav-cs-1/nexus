use anyhow::{Result, Context, anyhow};
use prost_reflect::{DescriptorPool, ServiceDescriptor};
use std::path::Path;
use std::fs;
use crate::models::proto_schema::{ProtoSchema, ProtoSourceType, ServiceInfo, MethodInfo};

/// Handles loading and parsing of .proto files and FileDescriptorSets
pub struct ProtoLoader {}

impl ProtoLoader {
    pub fn new() -> Self {
        Self {}
    }

    /// Load a FileDescriptorSet file (.pb or .bin) and create a ProtoSchema
    ///
    /// Users can generate these files using:
    /// protoc --descriptor_set_out=service.pb --include_imports service.proto
    pub fn load_descriptor_file<P: AsRef<Path>>(&self, path: P) -> Result<ProtoSchema> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(anyhow!("File not found: {}", path.display()));
        }

        // Read the file
        let bytes = fs::read(path)
            .context("Failed to read descriptor file")?;

        // Parse the descriptor
        let pool = DescriptorPool::decode(&bytes[..])
            .context("Failed to parse descriptor file. Make sure it's a valid FileDescriptorSet")?;

        // Extract service and method information
        let services = self.extract_services(&pool)?;

        // Get file name for schema name
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let mut schema = ProtoSchema::new(name, bytes, ProtoSourceType::LocalFile);
        schema.source_path = Some(path.to_string_lossy().to_string());
        schema.services = services;

        Ok(schema)
    }

    /// Load from raw descriptor bytes
    pub fn load_from_bytes(&self, name: String, bytes: Vec<u8>, source_type: ProtoSourceType) -> Result<ProtoSchema> {
        // Parse the descriptor
        let pool = DescriptorPool::decode(&bytes[..])
            .context("Failed to parse descriptor bytes")?;

        // Extract service and method information
        let services = self.extract_services(&pool)?;

        let mut schema = ProtoSchema::new(name, bytes, source_type);
        schema.services = services;

        Ok(schema)
    }

    /// Parse proto descriptor bytes and return the pool
    pub fn parse_descriptor(&self, bytes: &[u8]) -> Result<DescriptorPool> {
        let pool = DescriptorPool::decode(bytes)
            .context("Failed to parse descriptor")?;
        Ok(pool)
    }

    /// Extract all services and methods from a descriptor pool
    fn extract_services(&self, pool: &DescriptorPool) -> Result<Vec<ServiceInfo>> {
        let mut services = Vec::new();

        for service in pool.services() {
            let service_info = self.extract_service_info(&service)?;
            services.push(service_info);
        }

        if services.is_empty() {
            return Err(anyhow!("No services found in proto file. Make sure your .proto file defines at least one service."));
        }

        Ok(services)
    }

    /// Extract service information including all methods
    fn extract_service_info(&self, service: &ServiceDescriptor) -> Result<ServiceInfo> {
        let mut methods = Vec::new();

        for method in service.methods() {
            let rpc_type = if method.is_client_streaming() && method.is_server_streaming() {
                "bidirectional_streaming"
            } else if method.is_client_streaming() {
                "client_streaming"
            } else if method.is_server_streaming() {
                "server_streaming"
            } else {
                "unary"
            };

            methods.push(MethodInfo {
                name: method.name().to_string(),
                rpc_type: rpc_type.to_string(),
                input_type: method.input().full_name().to_string(),
                output_type: method.output().full_name().to_string(),
            });
        }

        Ok(ServiceInfo {
            name: service.full_name().to_string(),
            methods,
        })
    }

    /// Generate a JSON template for a message type
    pub fn generate_message_template(&self, pool: &DescriptorPool, message_type: &str) -> Result<String> {
        let message = pool.get_message_by_name(message_type)
            .ok_or_else(|| anyhow!("Message type '{}' not found in proto schema", message_type))?;

        let mut fields = Vec::new();

        for field in message.fields() {
            let field_name = field.name();
            let default_value = match field.kind() {
                prost_reflect::Kind::Double | prost_reflect::Kind::Float => "0.0",
                prost_reflect::Kind::Int32 | prost_reflect::Kind::Int64 |
                prost_reflect::Kind::Uint32 | prost_reflect::Kind::Uint64 |
                prost_reflect::Kind::Sint32 | prost_reflect::Kind::Sint64 |
                prost_reflect::Kind::Fixed32 | prost_reflect::Kind::Fixed64 |
                prost_reflect::Kind::Sfixed32 | prost_reflect::Kind::Sfixed64 => "0",
                prost_reflect::Kind::Bool => "false",
                prost_reflect::Kind::String => "\"\"",
                prost_reflect::Kind::Bytes => "\"\"",
                prost_reflect::Kind::Message(_) => "{}",
                prost_reflect::Kind::Enum(_) => "0",
            };

            fields.push(format!("  \"{}\": {}", field_name, default_value));
        }

        Ok(format!("{{\n{}\n}}", fields.join(",\n")))
    }

    /// Get all available services from a schema
    pub fn list_services(&self, schema: &ProtoSchema) -> Vec<String> {
        schema.services.iter().map(|s| s.name.clone()).collect()
    }

    /// Get all methods for a specific service
    pub fn list_methods(&self, schema: &ProtoSchema, service_name: &str) -> Vec<String> {
        schema.services
            .iter()
            .find(|s| s.name == service_name)
            .map(|s| s.methods.iter().map(|m| m.name.clone()).collect())
            .unwrap_or_default()
    }

    /// Get method information
    pub fn get_method_info<'a>(&self, schema: &'a ProtoSchema, service_name: &str, method_name: &str) -> Option<&'a MethodInfo> {
        schema.services
            .iter()
            .find(|s| s.name == service_name)?
            .methods
            .iter()
            .find(|m| m.name == method_name)
    }
}

impl Default for ProtoLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proto_loader_creation() {
        let loader = ProtoLoader::new();
        assert!(std::mem::size_of_val(&loader) >= 0);
    }
}
