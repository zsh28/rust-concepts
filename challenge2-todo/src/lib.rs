//! Challenge 2 library entrypoint.
//!
//! Modules are split into:
//! - `todo`: persisted data model
//! - `queue`: custom generic FIFO queue
//! - `app`: application logic + disk persistence

pub mod app;
pub mod queue;
pub mod todo;

pub use app::TodoApp;
pub use queue::Queue;
pub use todo::Todo;
