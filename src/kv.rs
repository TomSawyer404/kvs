use crate::{error::Result, KvsError};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

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
#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
    log: File,
}

impl KvStore {
    /// Open a file
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<KvStore> {
        let path = path.into();
        let log = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&path)?;

        Ok(KvStore {
            map: HashMap::new(),
            log,
        })
    }

    /// Sets the value of a string key to a string.
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        //self.map.insert(key, val);
        let command = Command::set(key, val);
        serde_json::to_writer(&mut self.log, &command)?;
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

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Self::Set { key, value }
    }
}
