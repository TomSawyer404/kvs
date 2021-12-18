use crate::{error::Result, KvsError};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::{
    collections::HashMap,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    ops::Range,
    path::{Path, PathBuf},
};

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stores in a `HashMap` in memory and persisted to disk
/// using a Write-Ahead Log. A `HashMap` in memory stores the keys and the value
/// location for fast query.
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
    path: PathBuf,

    // write-head log
    log_writer: BufWriterWithPos<File>,
    log_reader: BufReaderWithPos<File>,
    index: HashMap<String, CommandPos>,
}

impl KvStore {
    /// Opens a `KvStore` with a given path.
    /// This will create a new directory if the given one doesn't exist.
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<KvStore> {
        let path = path.into();
        create_dir_all(&path)?;

        let mut wal_path = path.clone();
        wal_path.push("dblog.txt");

        let mut log_writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(&wal_path)?,
        )?;

        // Set pos to end of the file
        log_writer.seek(SeekFrom::End(0))?;

        let mut log_reader = BufReaderWithPos::new(File::open(wal_path)?)?;

        let mut store = KvStore {
            path,
            log_reader,
            log_writer,
            index: HashMap::new(),
        };
        store.load_from_wal();

        return Ok(store);
    }

    /// Sets the value of a string key to a string.
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = SetCommand::new(key, value);
        let pos = self.log_writer.pos;
        serde_json::to_writer(&mut self.log_writer.writer, &cmd)?;
        self.log_writer.flush()?;
        self.index
            .insert(cmd.key, (pos..self.log_writer.pos).into());
        Ok(())
    }

    /// Gets the string value of a given string key.
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, _key: String) -> Result<Option<String>> {
        unimplemented!();
        //if let Some(x) = self.map.get(&key) {
        //    Ok(Some(x.clone()))
        //} else {
        //    Err(KvsError::KeyNotFound)
        //}
    }

    /// Remove a given key.
    pub fn remove(&mut self, _key: String) -> Result<()> {
        unimplemented!();
        //if let None = self.map.remove(&key) {
        //    Err(KvsError::KeyNotFound)
        //} else {
        //    Ok(())
        //}
    }

    fn load_from_wal(&mut self) -> Result<()> {
        let mut pos = self.log_reader.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(&mut self.log_reader).into_iter::<SetCommand>();

        while let Some(set_command) = stream.next() {
            let new_pos = stream.byte_offset() as u64;
            self.index.insert(set_command?.key, (pos..new_pos).into());
            pos = new_pos;
        }

        Ok(())
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

/// Represents the position and length of a json-serialized command in wal-log
#[derive(Debug)]
struct CommandPos {
    pos: u64,
    len: u64,
}

impl From<Range<u64>> for CommandPos {
    fn from(range: Range<u64>) -> Self {
        CommandPos {
            pos: range.start,
            len: range.end - range.start,
        }
    }
}

#[derive(Debug)]
struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)
    }
}

#[derive(Debug)]
struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}
