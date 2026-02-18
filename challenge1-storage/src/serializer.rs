use crate::{StorageCompatible, StorageError};

/// Behavior every serializer format must support for this challenge.
pub trait Serializer {
    /// Convert a strongly typed value into raw bytes.
    ///
    /// The concrete format (Borsh/Wincode/JSON) is chosen by the implementer.
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, StorageError>
    where
        T: StorageCompatible;

    /// Convert raw bytes back into a strongly typed value.
    ///
    /// The same format used for serialization must be used for deserialization.
    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, StorageError>
    where
        T: StorageCompatible;
}
