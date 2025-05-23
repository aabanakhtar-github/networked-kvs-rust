use crate::common::kvs_types::{DocType, KVSError};
use std::collections::HashMap;

pub struct Document {
    pub data: DocType,
}

pub struct KeyValueStore {
    store: HashMap<String, Document>,
}

impl KeyValueStore {
    pub fn new() -> KeyValueStore {
        KeyValueStore {
            store: HashMap::new(),
        }
    }

    pub fn ping() -> Document {
        Document {
            data: DocType::Raw("pong".to_string()),
        }
    }

    pub fn get(&self, key: &str) -> Result<&Document, KVSError> {
        self.store
            .get(key)
            .ok_or_else(|| KVSError::InvalidKey(format!("Couldn't find key! {}", key.to_string())))
    }

    pub fn put(&mut self, key: String, value: Document) -> Result<(), KVSError> {
        // pattern matches based on enum type
        if let DocType::JSON(doc) = &value.data {
            // <value> is a generic type, ? moves the error up
            serde_json::from_str::<serde_json::Value>(doc).map_err(|_| {
                KVSError::InvalidJSON("Couldn't parse JSON string for storage!".to_string())
            })?;
        }
        self.store.insert(key, value);
        Ok(())
    }

    pub fn del(&mut self, key: &str) -> Result<(), KVSError> {
        match self.store.remove(key) {
            Some(_) => Ok(()),
            None => Err(KVSError::InvalidKey(format!(
                "Couldn't find key {}",
                key.to_string()
            ))),
        }
    }
}
