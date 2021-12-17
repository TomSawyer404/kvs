use std::io;

/// Some error type defined here
#[derive(Debug)]
pub enum KvsError {
    /// Getting/Removing non-existent key error
    KeyNotFound,

    /// IO error
    IoError(io::Error),

    /// Serialize or Deserialize error
    SerdeError(serde_json::Error),

    /// Unexpected command type error
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::IoError(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> Self {
        KvsError::SerdeError(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
