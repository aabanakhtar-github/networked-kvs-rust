mod key_value_store;
mod kvs_types;

use key_value_store::*;
use kvs_types::*;

fn main() {
    let mut store = KeyValueStore::new();
    let json = Document {data: DocType::JSON("{\"name\": \"aaban}".to_string())};
    store.put("hello".to_string(), json).unwrap();
    println!("{}", store.get("hello").unwrap().data);
    store.del("hello");
}

