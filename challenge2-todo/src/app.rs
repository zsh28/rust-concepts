use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use borsh::BorshDeserialize;

use crate::{Queue, Todo};

/// Application service that wraps queue operations and disk persistence.
pub struct TodoApp {
    queue: Queue<Todo>,
    file_path: PathBuf,
    next_id: u64,
}

impl TodoApp {
    /// Loads queue state from disk, or creates an empty queue when the file is missing.
    pub fn load_or_new(file_path: impl Into<PathBuf>) -> io::Result<Self> {
        let file_path = file_path.into();
        let mut queue = Queue::new();

        if file_path.exists() {
            let bytes = fs::read(&file_path)?;
            if !bytes.is_empty() {
                let todos = Vec::<Todo>::try_from_slice(&bytes)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;
                for todo in todos {
                    queue.enqueue(todo);
                }
            }
        }

        let next_id = queue
            .iter()
            .map(|todo| todo.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);

        Ok(Self {
            queue,
            file_path,
            next_id,
        })
    }

    /// Enqueues a new task and persists immediately.
    pub fn add_task(&mut self, description: String) -> io::Result<Todo> {
        let todo = Todo {
            id: self.next_id,
            description,
            created_at: unix_timestamp(),
        };
        self.next_id = self.next_id.saturating_add(1);
        self.queue.enqueue(todo.clone());
        self.persist()?;
        Ok(todo)
    }

    /// Returns tasks in FIFO order.
    pub fn list_tasks(&self) -> impl Iterator<Item = &Todo> {
        self.queue.iter()
    }

    /// Completes the next pending task and persists immediately.
    pub fn complete_next(&mut self) -> io::Result<Option<Todo>> {
        let completed = self.queue.dequeue();
        self.persist()?;
        Ok(completed)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    fn persist(&self) -> io::Result<()> {
        let todos: Vec<Todo> = self.queue.iter().cloned().collect();
        let bytes = borsh::to_vec(&todos)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;
        fs::write(&self.file_path, bytes)
    }
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}
