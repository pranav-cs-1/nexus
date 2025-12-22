pub mod request;
pub mod response;
pub mod collection;

// gRPC models
pub mod grpc_request;
pub mod grpc_response;
pub mod proto_schema;

// Re-exports for convenience
pub use grpc_request::{GrpcRequest, RpcType, ProtoSource};
pub use grpc_response::{GrpcResponse, GrpcMessage, GrpcStatus};
pub use proto_schema::{ProtoSchema, ProtoSourceType, ServiceInfo, MethodInfo};

