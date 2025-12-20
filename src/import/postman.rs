use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::models::collection::Collection;
use crate::models::request::{ApiKeyLocation, AuthType, HttpMethod, HttpRequest};

// Postman Collection v2.1 Schema Structures
#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanCollection {
    pub info: PostmanInfo,
    pub item: Vec<PostmanItem>,
    #[serde(default)]
    pub auth: Option<PostmanAuth>,
    #[serde(default)]
    pub variable: Vec<PostmanVariable>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanInfo {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "_postman_id")]
    #[serde(default)]
    pub postman_id: Option<String>,
    #[serde(default)]
    pub schema: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PostmanItem {
    Request(PostmanRequestItem),
    Folder(PostmanFolder),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanRequestItem {
    pub name: String,
    pub request: PostmanRequest,
    #[serde(default)]
    pub response: Vec<serde_json::Value>, // We don't import responses
    #[serde(default)]
    pub event: Vec<serde_json::Value>, // Scripts - we ignore these
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanFolder {
    pub name: String,
    pub item: Vec<PostmanItem>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PostmanRequest {
    Full {
        method: String,
        #[serde(default)]
        header: Vec<PostmanHeader>,
        #[serde(default)]
        body: Option<PostmanBody>,
        url: PostmanUrl,
        #[serde(default)]
        auth: Option<PostmanAuth>,
        #[serde(default)]
        description: Option<String>,
    },
    Simple(String), // Simple string URL format
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PostmanUrl {
    Object {
        raw: String,
        #[serde(default)]
        protocol: Option<String>,
        #[serde(default)]
        host: Vec<String>,
        #[serde(default)]
        path: Vec<String>,
        #[serde(default)]
        query: Vec<PostmanQueryParam>,
    },
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanHeader {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanQueryParam {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanBody {
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub raw: Option<String>,
    #[serde(default)]
    pub urlencoded: Option<Vec<PostmanKeyValue>>,
    #[serde(default)]
    pub formdata: Option<Vec<PostmanKeyValue>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanKeyValue {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanAuth {
    #[serde(rename = "type")]
    pub auth_type: String,
    #[serde(default)]
    pub bearer: Option<Vec<PostmanAuthParam>>,
    #[serde(default)]
    pub basic: Option<Vec<PostmanAuthParam>>,
    #[serde(default)]
    pub apikey: Option<Vec<PostmanAuthParam>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanAuthParam {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub param_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostmanVariable {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub var_type: Option<String>,
}

// Import functions
pub fn import_postman_collection(path: &Path) -> Result<(Collection, Vec<HttpRequest>)> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let postman_collection: PostmanCollection = serde_json::from_str(&content)
        .with_context(|| "Failed to parse Postman collection JSON")?;

    convert_postman_collection(postman_collection)
}

fn convert_postman_collection(
    postman: PostmanCollection,
) -> Result<(Collection, Vec<HttpRequest>)> {
    let mut collection = Collection::new(postman.info.name.clone());
    if let Some(desc) = postman.info.description {
        collection.description = Some(desc);
    }

    let mut requests = Vec::new();
    let collection_auth = postman.auth;

    // Process all items (flattening folders)
    process_items(&postman.item, &mut requests, &collection_auth, None)?;

    // Set collection_id for all requests
    for request in requests.iter_mut() {
        request.collection_id = Some(collection.id);
    }

    Ok((collection, requests))
}

fn process_items(
    items: &[PostmanItem],
    requests: &mut Vec<HttpRequest>,
    collection_auth: &Option<PostmanAuth>,
    folder_prefix: Option<&str>,
) -> Result<()> {
    for item in items {
        match item {
            PostmanItem::Request(req_item) => {
                let name = if let Some(prefix) = folder_prefix {
                    format!("{}/{}", prefix, req_item.name)
                } else {
                    req_item.name.clone()
                };

                let request = convert_postman_request(name, &req_item.request, collection_auth)?;
                requests.push(request);
            }
            PostmanItem::Folder(folder) => {
                let new_prefix = if let Some(prefix) = folder_prefix {
                    format!("{}/{}", prefix, folder.name)
                } else {
                    folder.name.clone()
                };

                // Recursively process folder items
                process_items(&folder.item, requests, collection_auth, Some(&new_prefix))?;
            }
        }
    }

    Ok(())
}

fn convert_postman_request(
    name: String,
    postman_req: &PostmanRequest,
    collection_auth: &Option<PostmanAuth>,
) -> Result<HttpRequest> {
    match postman_req {
        PostmanRequest::Simple(url) => {
            Ok(HttpRequest::new(name, HttpMethod::GET, url.clone()))
        }
        PostmanRequest::Full {
            method,
            header,
            body,
            url,
            auth,
            description,
        } => {
            let http_method = parse_http_method(method)?;
            let url_string = extract_url(url)?;

            let mut request = HttpRequest::new(name, http_method, url_string);

            // Set description
            if let Some(desc) = description {
                request.description = Some(desc.clone());
            }

            // Add headers
            for h in header {
                if !h.disabled {
                    request.headers.insert(h.key.clone(), h.value.clone());
                }
            }

            // Add query params from URL
            if let PostmanUrl::Object { query, .. } = url {
                for q in query {
                    if !q.disabled {
                        request
                            .query_params
                            .insert(q.key.clone(), q.value.clone());
                    }
                }
            }

            // Add body
            if let Some(b) = body {
                request.body = extract_body(b);
            }

            // Add auth (request auth takes precedence over collection auth)
            request.auth = if let Some(a) = auth {
                convert_auth(a)?
            } else if let Some(a) = collection_auth {
                convert_auth(a)?
            } else {
                AuthType::None
            };

            Ok(request)
        }
    }
}

fn parse_http_method(method: &str) -> Result<HttpMethod> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(HttpMethod::GET),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "PATCH" => Ok(HttpMethod::PATCH),
        "DELETE" => Ok(HttpMethod::DELETE),
        "HEAD" => Ok(HttpMethod::HEAD),
        "OPTIONS" => Ok(HttpMethod::OPTIONS),
        _ => Err(anyhow!("Unsupported HTTP method: {}", method)),
    }
}

fn extract_url(postman_url: &PostmanUrl) -> Result<String> {
    match postman_url {
        PostmanUrl::String(s) => Ok(clean_url(s)),
        PostmanUrl::Object { raw, .. } => Ok(clean_url(raw)),
    }
}

fn clean_url(url: &str) -> String {
    // Remove query params from raw URL as we handle them separately
    if let Some(idx) = url.find('?') {
        url[..idx].to_string()
    } else {
        url.to_string()
    }
}

fn extract_body(body: &PostmanBody) -> Option<String> {
    match body.mode.as_deref() {
        Some("raw") => body.raw.clone(),
        Some("urlencoded") => {
            if let Some(params) = &body.urlencoded {
                let encoded: Vec<String> = params
                    .iter()
                    .filter(|p| !p.disabled)
                    .map(|p| format!("{}={}", p.key, p.value))
                    .collect();
                Some(encoded.join("&"))
            } else {
                None
            }
        }
        Some("formdata") => {
            if let Some(params) = &body.formdata {
                let encoded: Vec<String> = params
                    .iter()
                    .filter(|p| !p.disabled)
                    .map(|p| format!("{}={}", p.key, p.value))
                    .collect();
                Some(encoded.join("&"))
            } else {
                None
            }
        }
        _ => body.raw.clone(),
    }
}

fn convert_auth(auth: &PostmanAuth) -> Result<AuthType> {
    match auth.auth_type.as_str() {
        "bearer" => {
            if let Some(params) = &auth.bearer {
                if let Some(token_param) = params.iter().find(|p| p.key == "token") {
                    return Ok(AuthType::Bearer {
                        token: token_param.value.clone(),
                    });
                }
            }
            Err(anyhow!("Bearer auth missing token"))
        }
        "basic" => {
            if let Some(params) = &auth.basic {
                let username = params
                    .iter()
                    .find(|p| p.key == "username")
                    .map(|p| p.value.clone())
                    .unwrap_or_default();
                let password = params
                    .iter()
                    .find(|p| p.key == "password")
                    .map(|p| p.value.clone())
                    .unwrap_or_default();
                return Ok(AuthType::Basic { username, password });
            }
            Err(anyhow!("Basic auth missing credentials"))
        }
        "apikey" => {
            if let Some(params) = &auth.apikey {
                let key = params
                    .iter()
                    .find(|p| p.key == "key")
                    .map(|p| p.value.clone())
                    .unwrap_or_default();
                let value = params
                    .iter()
                    .find(|p| p.key == "value")
                    .map(|p| p.value.clone())
                    .unwrap_or_default();
                let in_param = params
                    .iter()
                    .find(|p| p.key == "in")
                    .map(|p| p.value.as_str())
                    .unwrap_or("header");

                let location = match in_param {
                    "query" => ApiKeyLocation::QueryParam,
                    _ => ApiKeyLocation::Header,
                };

                return Ok(AuthType::ApiKey { key, value, location });
            }
            Err(anyhow!("API key auth missing parameters"))
        }
        "noauth" | "" => Ok(AuthType::None),
        other => {
            // Unsupported auth types default to None
            eprintln!("Warning: Unsupported auth type '{}', defaulting to None", other);
            Ok(AuthType::None)
        }
    }
}
