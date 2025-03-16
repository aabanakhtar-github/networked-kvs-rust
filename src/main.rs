mod key_value_store;
use key_value_store::{*};

fn main() {
    let mut store = key_value_store::KeyValueStore::new();
    let json = Document {data: DocType::JSON("{\"name\": \"aaban}".to_string())};
    let doc = Document {data: DocType::Raw("Hi".to_string())};
    store.put("hello".to_string(), json).unwrap();
    println!("{}", store.get("hello").unwrap().data);
    store.del("hello");
}

