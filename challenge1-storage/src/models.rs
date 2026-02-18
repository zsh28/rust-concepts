use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// Test data model used by all serializer format tests.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    BorshSerialize,
    BorshDeserialize,
    wincode::SchemaWrite,
    wincode::SchemaRead,
)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
