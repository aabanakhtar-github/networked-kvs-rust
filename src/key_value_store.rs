use std::collections::HashMap;
use std::fmt::Display;
use serde_json::{ Value};
pub enum DocType {
    JSON(String),
    Raw(String),
}

pub struct Document {
   pub data: DocType,
}

pub struct KeyValueStore {
    store: HashMap<String, Document>,
}

impl KeyValueStore {
    pub fn new() -> KeyValueStore {
        KeyValueStore {store: HashMap::new() }
    }

    pub fn ping() -> Document {
        Document{data: DocType::Raw("pong".to_string())}
    }

    pub fn get(&self, key: &str) -> Result<&Document, String> {
        self.store.get(key).ok_or_else(|| format!("Key {} not found", key))
    }

    pub fn put(&mut self, key: String, value: Document) -> Result<(), String> {
        // pattern matches based on enum type
        if let DocType::JSON(doc) = &value.data {
            // <value> is a generic type, ? moves the error up
            serde_json::from_str::<serde_json::Value>(doc)
                .map_err(|_| "Couldn't parse json document!".to_string())?;
        }
        self.store.insert(key, value);
        Ok(())
    }

    pub fn del(&mut self, key: &str) {
        self.store.remove(key);
    }
}

impl Display for DocType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let value = match self {
            DocType::JSON(v) => v,
            DocType::Raw(v) => v,
        };
        write!(fmt, "{}", value)
    }
}
