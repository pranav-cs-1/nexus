use anyhow::{Result, Context, anyhow};
use prost_reflect::{DynamicMessage, DescriptorPool, MessageDescriptor, MethodDescriptor};
use prost::Message;
use tonic::transport::Channel;
use std::collections::HashMap;

/// Handles dynamic invocation of gRPC methods without compile-time generated code
pub struct DynamicInvoker {
    channel: Channel,
    descriptor: DescriptorPool,
}

impl DynamicInvoker {
    pub fn new(channel: Channel, descriptor: DescriptorPool) -> Self {
        Self {
            channel,
            descriptor,
        }
    }

    /// Invoke a unary RPC method dynamically
    pub async fn invoke_unary(
        &mut self,
        service_name: &str,
        method_name: &str,
        message_json: &str,
        metadata: &HashMap<String, String>,
    ) -> Result<DynamicMessage> {
        // Get the method descriptor
        let method_desc = self.get_method_descriptor(service_name, method_name)?;

        // Get input and output message descriptors
        let input_desc = method_desc.input();
        let output_desc = method_desc.output();

        // Convert JSON to DynamicMessage
        let request_msg = if message_json.trim().is_empty() {
            // Empty message
            DynamicMessage::new(input_desc.clone())
        } else {
            // Use DeserializeSeed to deserialize JSON into DynamicMessage
            // MessageDescriptor implements DeserializeSeed when serde feature is enabled
            use serde::de::DeserializeSeed;

            let mut deserializer = serde_json::Deserializer::from_str(message_json);

            // MessageDescriptor itself implements DeserializeSeed
            input_desc.deserialize(&mut deserializer)
                .context("Failed to parse JSON into protobuf message. Ensure JSON matches the proto schema.")?
        };

        // Encode the request message to bytes
        let request_bytes = request_msg.encode_to_vec();

        // Build the gRPC path
        let path = format!("/{}/{}", service_name, method_name);

        // Create a tonic client with the channel
        let mut client = tonic::client::Grpc::new(self.channel.clone());

        // Create a codec for raw bytes
        let codec = tonic::codec::ProstCodec::default();

        // Prepare the request
        let mut request = tonic::Request::new(request_bytes);

        // Add metadata as headers
        for (key, value) in metadata {
            if let Ok(header_name) = key.parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>() {
                if let Ok(header_value) = value.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>() {
                    request.metadata_mut().insert(header_name, header_value);
                }
            }
        }

        // Ensure the client is ready before making the call
        use tower::ServiceExt;
        client.ready().await.context("Client not ready")?;

        // Make the unary call
        let response: tonic::Response<Vec<u8>> = client
            .unary(request, path.try_into().context("Invalid gRPC path")?, codec)
            .await
            .context("gRPC call failed")?;

        // Get the response bytes
        let response_bytes: Vec<u8> = response.into_inner();

        // Decode the response
        let response_msg = DynamicMessage::decode(output_desc, &response_bytes[..])
            .context("Failed to decode response message")?;

        Ok(response_msg)
    }

    /// Get method descriptor from service and method name
    fn get_method_descriptor(&self, service_name: &str, method_name: &str) -> Result<MethodDescriptor> {
        // Try to find the service in the descriptor pool
        for service in self.descriptor.services() {
            if service.full_name() == service_name || service.name() == service_name {
                // Found the service, now find the method
                for method in service.methods() {
                    if method.name() == method_name {
                        return Ok(method);
                    }
                }

                // Service found but method not found
                let available_methods: Vec<String> = service.methods()
                    .map(|m| m.name().to_string())
                    .collect();
                return Err(anyhow!(
                    "Method '{}' not found in service '{}'. Available methods: {}",
                    method_name,
                    service_name,
                    available_methods.join(", ")
                ));
            }
        }

        // Service not found
        let available_services: Vec<String> = self.descriptor.services()
            .map(|s| s.full_name().to_string())
            .collect();
        Err(anyhow!(
            "Service '{}' not found in proto descriptor. Available services: {}",
            service_name,
            available_services.join(", ")
        ))
    }

    /// Create a message template (JSON) for a given message type
    pub fn create_message_template(&self, message_desc: &MessageDescriptor) -> String {
        let msg = DynamicMessage::new(message_desc.clone());

        // Serialize to JSON using serde (with serde feature enabled)
        serde_json::to_string_pretty(&msg).unwrap_or_else(|_| "{}".to_string())
    }

    /// Get the descriptor pool reference
    pub fn descriptor(&self) -> &DescriptorPool {
        &self.descriptor
    }
}

/// Helper function to convert DynamicMessage to JSON
pub fn dynamic_message_to_json(msg: &DynamicMessage) -> Result<String> {
    // With serde feature enabled, DynamicMessage implements Serialize
    serde_json::to_string_pretty(msg)
        .context("Failed to convert response message to JSON")
}
