use std::collections::HashMap;
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        if let Some(x) = self.map.get(&key) {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
