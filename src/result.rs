//! Module related to Result's and Error's

/// std::result::Result alias, where Err is Error
pub type Result<T> = std::result::Result<T, Error>;

/// Error type
#[derive(Debug)]
pub struct Error {
    /// The type of error
    kind:       ErrorKind,
    /// A message describing the error
    message:    String
}

impl Error {
    /// Create a new Error
    pub fn new<S: AsRef<str>>(kind: ErrorKind, message: S) -> Self {
        let message = message.as_ref().to_string();
        Self { kind, message }
    }
}

/// Describes different kinds of Errors
#[derive(Debug)]
pub enum ErrorKind {
    /// Occurs when a database operation fails
    Database,
    /// Occurs when loading a Plugin fails
    PluginError,
    /// Occurs when an IO operation fails
    IoError(std::io::Error),
    /// Occurs when locking e.g a Mutex fails
    LockError,
}

