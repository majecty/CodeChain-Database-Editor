use serde::{Deserialize, Serialize};

pub struct FileStorage {}

impl FileStorage {
    pub fn get_by_prefix(prefix: &str) -> serde_json::Value {
        
    }
}

pub struct KVStorage {
    prefix: &'static str,
}

#[derive(Serialize, DeSerialize)]
pub struct ExampleStruct {
    n: i32,
    s: String,
}

impl KVStorage {
    pub fn new(prefix: &'static str) -> Self {
        KVStorage { prefix }
    }

    pub fn get<T: Deserialize>(fs: &mut FileStorage, key: &str) -> serde_json::Result<T> {}
    // pub fn get(key: &str) {}
}
