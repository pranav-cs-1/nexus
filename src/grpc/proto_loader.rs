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
