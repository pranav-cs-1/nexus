use crate::models::{GrpcRequest, GrpcResponse, GrpcStatus, GrpcMessage, ProtoSchema};
use crate::grpc::proto_loader::ProtoLoader;
use crate::grpc::dynamic_invoker::{DynamicInvoker, dynamic_message_to_json};
use anyhow::{Result, Context, anyhow};
use std::time::Instant;
use tonic::transport::Channel;
use uuid::Uuid;
use prost_reflect::DescriptorPool;

pub struct GrpcClient {
    proto_loader: ProtoLoader,
}

impl GrpcClient {
    pub fn new() -> Self {
        Self {
            proto_loader: ProtoLoader::new(),
        }
    }

    /// Execute a unary gRPC call
    pub async fn execute_unary(&self, request: &GrpcRequest, proto_schema: Option<&ProtoSchema>) -> Result<GrpcResponse> {
        let start = Instant::now();

        // Validate request fields
        if request.server_url.trim().is_empty() {
            return Err(anyhow!("Server URL cannot be empty"));
        }

        if request.service_name.trim().is_empty() {
            return Err(anyhow!("Service name cannot be empty (e.g., 'greet.Greeter')"));
        }

        if request.method_name.trim().is_empty() {
            return Err(anyhow!("Method name cannot be empty (e.g., 'SayHello')"));
        }

        // Validate against proto schema if available
        if let Some(schema) = proto_schema {
            self.validate_against_schema(request, schema)?;
        }

        // Attempt to create a channel
        match self.create_channel(&request.server_url, request.use_tls, request.timeout_seconds).await {
            Ok(channel) => {
                // If we have a proto schema, make the actual RPC call
                if let Some(schema) = proto_schema {
                    match self.execute_with_proto(channel, request, schema, start).await {
                        Ok(response) => return Ok(response),
                        Err(e) => {
                            // RPC execution failed, return error response
                            let duration_ms = start.elapsed().as_millis() as u64;
                            return Ok(GrpcResponse {
                                id: Uuid::new_v4(),
                                request_id: request.id,
                                messages: vec![],
                                status: GrpcStatus {
                                    code: 13, // INTERNAL
                                    message: format!("RPC execution failed: {}", e),
                                },
                                metadata: std::collections::HashMap::new(),
                                trailers: std::collections::HashMap::new(),
                                duration_ms,
                                timestamp: chrono::Utc::now(),
                            });
                        }
                    }
                }

                // No proto schema, just connectivity check
                let duration_ms = start.elapsed().as_millis() as u64;
                let message = self.create_connectivity_response(request);

                Ok(GrpcResponse {
                    id: Uuid::new_v4(),
                    request_id: request.id,
                    messages: vec![message],
                    status: GrpcStatus {
                        code: 0,
                        message: "Connection OK - Load proto file for full RPC support".to_string(),
                    },
                    metadata: std::collections::HashMap::new(),
                    trailers: std::collections::HashMap::new(),
                    duration_ms,
                    timestamp: chrono::Utc::now(),
                })
            }
            Err(e) => {
                let duration_ms = start.elapsed().as_millis() as u64;

                // Return detailed error response
                let error_message = format!("Failed to connect to gRPC server: {}", e);

                Ok(GrpcResponse {
                    id: Uuid::new_v4(),
                    request_id: request.id,
                    messages: vec![],
                    status: GrpcStatus {
                        code: 14, // UNAVAILABLE
                        message: error_message,
                    },
                    metadata: std::collections::HashMap::new(),
                    trailers: std::collections::HashMap::new(),
                    duration_ms,
                    timestamp: chrono::Utc::now(),
                })
            }
        }
    }

    /// Execute gRPC request with proto schema (actual RPC invocation)
    async fn execute_with_proto(
        &self,
        channel: Channel,
        request: &GrpcRequest,
        schema: &ProtoSchema,
        start: Instant,
    ) -> Result<GrpcResponse> {
        // Parse the descriptor from the schema
        let descriptor = DescriptorPool::decode(&schema.file_descriptor_set[..])
            .context("Failed to parse proto descriptor")?;

        // Create dynamic invoker
        let mut invoker = DynamicInvoker::new(channel, descriptor);

        // Invoke the RPC
        let response_msg = invoker.invoke_unary(
            &request.service_name,
            &request.method_name,
            &request.message_json,
            &request.metadata,
        ).await?;

        // Convert response to JSON
        let response_json = dynamic_message_to_json(&response_msg)?;

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(GrpcResponse {
            id: Uuid::new_v4(),
            request_id: request.id,
            messages: vec![GrpcMessage {
                message_json: response_json,
                received_at: chrono::Utc::now(),
            }],
            status: GrpcStatus {
                code: 0, // OK
                message: "RPC call successful".to_string(),
            },
            metadata: std::collections::HashMap::new(),
            trailers: std::collections::HashMap::new(),
            duration_ms,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Validate request against proto schema
    fn validate_against_schema(&self, request: &GrpcRequest, schema: &ProtoSchema) -> Result<()> {
        // Check if service exists
        let service_exists = schema.services.iter().any(|s| s.name == request.service_name);
        if !service_exists {
            let available_services: Vec<_> = schema.services.iter().map(|s| s.name.as_str()).collect();
            return Err(anyhow!(
                "Service '{}' not found in proto schema. Available services: {}",
                request.service_name,
                available_services.join(", ")
            ));
        }

        // Check if method exists in service
        if let Some(method_info) = self.proto_loader.get_method_info(schema, &request.service_name, &request.method_name) {
            // Validate RPC type matches
            let expected_type = match request.rpc_type {
                crate::models::grpc_request::RpcType::Unary => "unary",
                crate::models::grpc_request::RpcType::ServerStreaming => "server_streaming",
                crate::models::grpc_request::RpcType::ClientStreaming => "client_streaming",
                crate::models::grpc_request::RpcType::BidirectionalStreaming => "bidirectional_streaming",
            };

            if method_info.rpc_type != expected_type {
                return Err(anyhow!(
                    "RPC type mismatch: expected '{}', but proto defines '{}'",
                    expected_type,
                    method_info.rpc_type
                ));
            }
        } else {
            let service = schema.services.iter().find(|s| s.name == request.service_name).unwrap();
            let available_methods: Vec<_> = service.methods.iter().map(|m| m.name.as_str()).collect();
            return Err(anyhow!(
                "Method '{}' not found in service '{}'. Available methods: {}",
                request.method_name,
                request.service_name,
                available_methods.join(", ")
            ));
        }

        Ok(())
    }

    /// Create a basic connectivity response
    fn create_connectivity_response(&self, request: &GrpcRequest) -> GrpcMessage {
        GrpcMessage {
            message_json: format!(
                r#"{{
  "status": "Connection Successful",
  "message": "Successfully connected to gRPC server at {}",
  "service": "{}",
  "method": "{}",
  "note": "Load a proto file (Press 'l' in gRPC mode) to enable validation and get message templates.",
  "tip": "Generate proto descriptor: protoc --descriptor_set_out=service.pb --include_imports service.proto"
}}"#,
                request.server_url, request.service_name, request.method_name
            ),
            received_at: chrono::Utc::now(),
        }
    }

    /// Create a channel to a gRPC server
    async fn create_channel(&self, server_url: &str, use_tls: bool, timeout_seconds: Option<u64>) -> Result<Channel> {
        let url = if use_tls {
            format!("https://{}", server_url)
        } else {
            format!("http://{}", server_url)
        };

        let mut endpoint = Channel::from_shared(url)
            .context("Invalid server URL")?;

        // Set timeout if specified
        if let Some(timeout) = timeout_seconds {
            endpoint = endpoint
                .timeout(std::time::Duration::from_secs(timeout))
                .connect_timeout(std::time::Duration::from_secs(timeout.min(10)));
        }

        let channel = endpoint
            .connect()
            .await
            .context("Failed to connect to server")?;

        Ok(channel)
    }
}

impl Default for GrpcClient {
    fn default() -> Self {
        Self::new()
    }
}
