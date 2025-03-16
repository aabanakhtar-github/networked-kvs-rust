use std::collections::HashMap;
use std::fmt::Display;

pub enum DocType {
    JSON(String),
    Raw(String),
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

    pub fn get(&self, key: &str) -> Result<&Document, String> {
        self.store.get(key).ok_or_else(|| format!("Key {} not found", key))
    }

    pub fn put(&mut self, key: String, value: Document) {
        self.store.insert(key, value);
    }

    pub fn del(&mut self, key: &str) {
        self.store.remove(key);
    }
}