use crate::{StorageCompatible, StorageError};

/// Behavior every serializer format must support for this challenge.
pub trait Serializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, StorageError>
    where
        T: StorageCompatible;

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, StorageError>
    where
        T: StorageCompatible;
}
