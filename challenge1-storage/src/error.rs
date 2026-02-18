use std::error::Error;
use std::fmt::{Display, Formatter};

/// Unified error type for serializer-specific failures and empty storage reads.
#[derive(Debug)]
pub enum StorageError {
    EmptyStorage,
    Borsh(String),
    Wincode(String),
    Json(String),
}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyStorage => write!(f, "storage has no data"),
            Self::Borsh(err) => write!(f, "borsh error: {err}"),
            Self::Wincode(err) => write!(f, "wincode error: {err}"),
            Self::Json(err) => write!(f, "json error: {err}"),
        }
    }
}

impl Error for StorageError {}
