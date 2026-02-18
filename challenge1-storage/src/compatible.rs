use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use wincode::config::DefaultConfig;

/// Shared trait bound for data that can be serialized in all required formats.
pub trait StorageCompatible:
    BorshSerialize
    + BorshDeserialize
    + Serialize
    + for<'de> Deserialize<'de>
    + wincode::SchemaWrite<DefaultConfig, Src = Self>
    + for<'de> wincode::SchemaRead<'de, DefaultConfig, Dst = Self>
{
}

impl<T> StorageCompatible for T where
    T: BorshSerialize
        + BorshDeserialize
        + Serialize
        + for<'de> Deserialize<'de>
        + wincode::SchemaWrite<DefaultConfig, Src = Self>
        + for<'de> wincode::SchemaRead<'de, DefaultConfig, Dst = Self>
{
}
