use anyhow::Result;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::path::PathBuf;

pub struct Database {
    db: Db,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        std::fs::create_dir_all(db_path.parent().unwrap())?;
        
        let db = sled::open(db_path)?;
        Ok(Self { db })
    }
    
    fn get_db_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        Ok(config_dir.join("nexus").join("data.db"))
    }
    
    pub fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(bytes) = self.db.get(key)? {
            let value: T = bincode::deserialize(&bytes)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
    
    pub fn set<T>(&self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let bytes = bincode::serialize(value)?;
        self.db.insert(key, bytes)?;
        self.db.flush()?;
        Ok(())
    }
    
    pub fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }
    
    pub fn list_keys(&self, prefix: &str) -> Result<Vec<String>> {
        let keys: Result<Vec<String>> = self
            .db
            .scan_prefix(prefix)
            .map(|result| {
                result
                    .map(|(key, _)| String::from_utf8_lossy(&key).to_string())
                    .map_err(|e| anyhow::anyhow!("Database error: {}", e))
            })
            .collect();
        keys
    }
}

