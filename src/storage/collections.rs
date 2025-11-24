use crate::models::{
    collection::Collection,
    request::HttpRequest,
};
use crate::storage::database::Database;
use anyhow::Result;
use uuid::Uuid;

pub struct CollectionRepository {
    db: Database,
}

impl CollectionRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    pub fn save_collection(&self, collection: &Collection) -> Result<()> {
        let key = format!("collection:{}", collection.id);
        self.db.set(&key, collection)
    }
    
    pub fn get_collection(&self, id: Uuid) -> Result<Option<Collection>> {
        let key = format!("collection:{}", id);
        self.db.get(&key)
    }
    
    pub fn list_collections(&self) -> Result<Vec<Collection>> {
        let keys = self.db.list_keys("collection:")?;
        let mut collections = Vec::new();
        
        for key in keys {
            if let Some(collection) = self.db.get::<Collection>(&key)? {
                collections.push(collection);
            }
        }
        
        collections.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(collections)
    }
    
    pub fn delete_collection(&self, id: Uuid) -> Result<()> {
        let key = format!("collection:{}", id);
        self.db.delete(&key)
    }
    
    pub fn save_request(&self, request: &HttpRequest) -> Result<()> {
        let key = format!("request:{}", request.id);
        self.db.set(&key, request)
    }
    
    pub fn get_request(&self, id: Uuid) -> Result<Option<HttpRequest>> {
        let key = format!("request:{}", id);
        self.db.get(&key)
    }
    
    pub fn list_requests(&self, collection_id: Option<Uuid>) -> Result<Vec<HttpRequest>> {
        let keys = self.db.list_keys("request:")?;
        let mut requests = Vec::new();
        
        for key in keys {
            if let Some(request) = self.db.get::<HttpRequest>(&key)? {
                if collection_id.is_none() || request.collection_id == collection_id {
                    requests.push(request);
                }
            }
        }
        
        requests.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(requests)
    }
    
    pub fn delete_request(&self, id: Uuid) -> Result<()> {
        let key = format!("request:{}", id);
        self.db.delete(&key)
    }
}

