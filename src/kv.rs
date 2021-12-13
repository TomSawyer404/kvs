use crate::error::Result;
use crate::KvsError;
use std::collections::HashMap;
use std::path::PathBuf;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stores in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Open a file
    pub fn open<P: Into<PathBuf>>(_path: P) -> Result<KvStore> {
        Ok(KvStore {
            map: HashMap::new(),
        })
    }

    /// Sets the value of a string key to a string.
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        self.map.insert(key, val);
        Ok(())
    }

    /// Gets the string value of a given string key.
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        if let Some(x) = self.map.get(&key) {
            Ok(Some(x.clone()))
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let None = self.map.remove(&key) {
            Err(KvsError::KeyNotFound)
        } else {
            Ok(())
        }
    }
}
