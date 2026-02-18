use crate::{Serializer, StorageCompatible, StorageError};

/// Borsh format adapter.
pub struct Borsh;

impl Serializer for Borsh {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, StorageError>
    where
        T: StorageCompatible,
    {
        borsh::to_vec(value).map_err(|err| StorageError::Borsh(err.to_string()))
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, StorageError>
    where
        T: StorageCompatible,
    {
        T::try_from_slice(bytes).map_err(|err| StorageError::Borsh(err.to_string()))
    }
}

/// Wincode format adapter.
pub struct Wincode;

impl Serializer for Wincode {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, StorageError>
    where
        T: StorageCompatible,
    {
        wincode::serialize(value).map_err(|err| StorageError::Wincode(err.to_string()))
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, StorageError>
    where
        T: StorageCompatible,
    {
        wincode::deserialize(bytes).map_err(|err| StorageError::Wincode(err.to_string()))
    }
}

/// JSON format adapter.
pub struct Json;

impl Serializer for Json {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, StorageError>
    where
        T: StorageCompatible,
    {
        serde_json::to_vec(value).map_err(|err| StorageError::Json(err.to_string()))
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, StorageError>
    where
        T: StorageCompatible,
    {
        serde_json::from_slice(bytes).map_err(|err| StorageError::Json(err.to_string()))
    }
}
