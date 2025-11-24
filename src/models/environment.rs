use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    pub id: Uuid,
    pub name: String,
    pub variables: HashMap<String, String>,
    pub is_active: bool,
}

impl Environment {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            variables: HashMap::new(),
            is_active: false,
        }
    }
    
    pub fn substitute(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (key, value) in &self.variables {
            let pattern = format!("{{{{{}}}}}", key);
            result = result.replace(&pattern, value);
        }
        result
    }
}

