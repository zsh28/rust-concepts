use borsh::{BorshDeserialize, BorshSerialize};

/// Single todo item persisted to disk.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub created_at: u64,
}
