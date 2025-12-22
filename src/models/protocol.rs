use serde::{Deserialize, Serialize};
use super::{request::HttpRequest, response::HttpResponse, GrpcRequest, GrpcResponse};

/// Unified request type supporting multiple protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    Http(HttpRequest),
    Grpc(GrpcRequest),
}

/// Unified response type supporting multiple protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    Http(HttpResponse),
    Grpc(GrpcResponse),
}

impl RequestType {
    pub fn name(&self) -> &str {
        match self {
            RequestType::Http(req) => &req.name,
            RequestType::Grpc(req) => &req.name,
        }
    }

    pub fn is_http(&self) -> bool {
        matches!(self, RequestType::Http(_))
    }

    pub fn is_grpc(&self) -> bool {
        matches!(self, RequestType::Grpc(_))
    }

    pub fn as_http(&self) -> Option<&HttpRequest> {
        match self {
            RequestType::Http(req) => Some(req),
            _ => None,
        }
    }

    pub fn as_grpc(&self) -> Option<&GrpcRequest> {
        match self {
            RequestType::Grpc(req) => Some(req),
            _ => None,
        }
    }
}

impl ResponseType {
    pub fn is_http(&self) -> bool {
        matches!(self, ResponseType::Http(_))
    }

    pub fn is_grpc(&self) -> bool {
        matches!(self, ResponseType::Grpc(_))
    }

    pub fn as_http(&self) -> Option<&HttpResponse> {
        match self {
            ResponseType::Http(resp) => Some(resp),
            _ => None,
        }
    }

    pub fn as_grpc(&self) -> Option<&GrpcResponse> {
        match self {
            ResponseType::Grpc(resp) => Some(resp),
            _ => None,
        }
    }
}
