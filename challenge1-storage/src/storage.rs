use std::marker::PhantomData;

use crate::{Serializer, StorageCompatible, StorageError};

/// Generic in-memory storage that keeps serialized bytes and type information.
pub struct Storage<T, S>
where
    T: StorageCompatible,
    S: Serializer,
{
    serializer: S,
    bytes: Option<Vec<u8>>,
    marker: PhantomData<T>,
}

impl<T, S> Storage<T, S>
where
    T: StorageCompatible,
    S: Serializer,
{
    /// Creates empty storage for `T` using the chosen serializer implementation.
    pub fn new(serializer: S) -> Self {
        Self {
            serializer,
            bytes: None,
            marker: PhantomData,
        }
    }

    /// Serializes and stores the value bytes.
    pub fn save(&mut self, value: &T) -> Result<(), StorageError> {
        self.bytes = Some(self.serializer.to_bytes(value)?);
        Ok(())
    }

    /// Loads and deserializes the currently stored value.
    pub fn load(&self) -> Result<T, StorageError> {
        let bytes = self.bytes.as_deref().ok_or(StorageError::EmptyStorage)?;
        self.serializer.from_bytes(bytes)
    }

    /// Returns true when serialized data exists.
    pub fn has_data(&self) -> bool {
        self.bytes.is_some()
    }
}
