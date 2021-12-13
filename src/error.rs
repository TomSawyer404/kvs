use std::io;

/// Some error type defined here
#[derive(Debug)]
pub enum KvsError {
    /// Getting/Removing non-existent key error
    KeyNotFound,

    /// IO error
    IoError(io::Error),

    /// Unexpected command type error
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::IoError(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
