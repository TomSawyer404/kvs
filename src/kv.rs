use crate::{error::Result, KvsError};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::{
    collections::HashMap,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
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
    path: PathBuf,
    wal_writer: BufWriter<File>,
}

impl KvStore {
    /// Opens a `KvStore` with a given path.
    /// This will create a new directory if the given one doesn't exist.
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<KvStore> {
        let path = path.into();
        create_dir_all(&path)?;

        let mut wal_path = path.clone();
        wal_path.push("dblog.txt");
        let log = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&wal_path)?;

        Ok(KvStore {
            map: HashMap::new(),
            path,
            wal_writer: BufWriter::<File>::new(log),
        })
    }

    /// Sets the value of a string key to a string.
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.clone(), value.clone());
        let set_command = SetCommand::new(key, value);
        serde_json::to_writer(&mut self.wal_writer, &set_command)?;
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

fn load_from_wal(wal_path: impl AsRef<Path>) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    let reader = BufReader::new(File::open(wal_path)?);
    let stream = Deserializer::from_reader(reader).into_iter::<SetCommand>();
    for set_cmd in stream {
        let set_cmd = set_cmd?;
        map.insert(set_cmd.key, set_cmd.value);
    }
    Ok(map)
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

#[derive(Debug, Serialize, Deserialize)]
struct SetCommand {
    key: String,
    value: String,
}

impl SetCommand {
    fn new(key: String, value: String) -> SetCommand {
        SetCommand { key, value }
    }
}
