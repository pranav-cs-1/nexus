use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::request::HttpRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Structure for exporting a collection with all its requests
#[derive(Serialize, Deserialize)]
pub struct CollectionExport {
    pub collection: Collection,
    pub requests: Vec<HttpRequest>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    // Export collection with its requests to JSON
    pub fn to_json(&self, requests: &[HttpRequest]) -> Result<String, serde_json::Error> {
        let export = CollectionExport {
            collection: self.clone(),
            requests: requests.to_vec(),
        };
        serde_json::to_string_pretty(&export)
    }
}


