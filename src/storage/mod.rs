use anyhow::{Context, Result};
use sled::Db;
use std::path::PathBuf;

use crate::models::{collection::Collection, request::HttpRequest};

const COLLECTIONS_TREE: &str = "collections";
const REQUESTS_TREE: &str = "requests";

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let data_dir = Self::get_data_dir()?;
        std::fs::create_dir_all(&data_dir)
            .context("Failed to create data directory")?;
        
        let db_path = data_dir.join("nexus.db");
        let db = sled::open(db_path)
            .context("Failed to open database")?;
        
        Ok(Self { db })
    }
    
    fn get_data_dir() -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .context("Failed to get data directory")?
            .join("nexus");
        Ok(data_dir)
    }
    
    pub fn save_collection(&self, collection: &Collection) -> Result<()> {
        let tree = self.db.open_tree(COLLECTIONS_TREE)
            .context("Failed to open collections tree")?;
        
        let key = collection.id.as_bytes();
        let value = bincode::serialize(collection)
            .context("Failed to serialize collection")?;
        
        tree.insert(key, value)
            .context("Failed to save collection")?;
        
        self.db.flush()
            .context("Failed to flush database")?;
        
        Ok(())
    }
    
    pub fn save_request(&self, request: &HttpRequest) -> Result<()> {
        let tree = self.db.open_tree(REQUESTS_TREE)
            .context("Failed to open requests tree")?;
        
        let key = request.id.as_bytes();
        let value = bincode::serialize(request)
            .context("Failed to serialize request")?;
        
        tree.insert(key, value)
            .context("Failed to save request")?;
        
        self.db.flush()
            .context("Failed to flush database")?;
        
        Ok(())
    }
    
    pub fn load_collections(&self) -> Result<Vec<Collection>> {
        let tree = self.db.open_tree(COLLECTIONS_TREE)
            .context("Failed to open collections tree")?;
        
        let mut collections = Vec::new();
        
        for result in tree.iter() {
            let (_, value) = result.context("Failed to iterate collections")?;
            let collection: Collection = bincode::deserialize(&value)
                .context("Failed to deserialize collection")?;
            collections.push(collection);
        }
        
        collections.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        Ok(collections)
    }
    
    pub fn load_requests(&self) -> Result<Vec<HttpRequest>> {
        let tree = self.db.open_tree(REQUESTS_TREE)
            .context("Failed to open requests tree")?;
        
        let mut requests = Vec::new();
        
        for result in tree.iter() {
            let (_, value) = result.context("Failed to iterate requests")?;
            let request: HttpRequest = bincode::deserialize(&value)
                .context("Failed to deserialize request")?;
            requests.push(request);
        }
        
        requests.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        Ok(requests)
    }
    
    pub fn delete_collection(&self, id: &uuid::Uuid) -> Result<()> {
        let tree = self.db.open_tree(COLLECTIONS_TREE)
            .context("Failed to open collections tree")?;
        
        tree.remove(id.as_bytes())
            .context("Failed to delete collection")?;
        
        self.db.flush()
            .context("Failed to flush database")?;
        
        Ok(())
    }
    
    pub fn delete_request(&self, id: &uuid::Uuid) -> Result<()> {
        let tree = self.db.open_tree(REQUESTS_TREE)
            .context("Failed to open requests tree")?;
        
        tree.remove(id.as_bytes())
            .context("Failed to delete request")?;
        
        self.db.flush()
            .context("Failed to flush database")?;
        
        Ok(())
    }
    
    pub fn delete_requests_by_collection(&self, collection_id: &uuid::Uuid) -> Result<()> {
        let tree = self.db.open_tree(REQUESTS_TREE)
            .context("Failed to open requests tree")?;
        
        let mut keys_to_delete = Vec::new();
        
        for result in tree.iter() {
            let (key, value) = result.context("Failed to iterate requests")?;
            let request: HttpRequest = bincode::deserialize(&value)
                .context("Failed to deserialize request")?;
            
            if request.collection_id == Some(*collection_id) {
                keys_to_delete.push(key.to_vec());
            }
        }
        
        for key in keys_to_delete {
            tree.remove(key)
                .context("Failed to delete request")?;
        }
        
        self.db.flush()
            .context("Failed to flush database")?;
        
        Ok(())
    }
}

