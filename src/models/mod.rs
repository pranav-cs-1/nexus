pub mod request;
pub mod response;
pub mod collection;

// gRPC models
pub mod grpc_request;
pub mod grpc_response;
pub mod proto_schema;

// Protocol abstraction
pub mod protocol;

// Re-exports for convenience
pub use grpc_request::GrpcRequest;
pub use grpc_response::{GrpcResponse, GrpcStatus, GrpcMessage};
pub use proto_schema::ProtoSchema;

