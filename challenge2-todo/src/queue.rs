/// Optimized generic FIFO queue.
///
/// This queue avoids shifting elements on every `dequeue` by keeping a moving
/// `head` index, then compacting occasionally to reclaim memory.
pub struct Queue<T> {
    items: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            head: 0,
            len: 0,
        }
    }

    /// Adds an item to the back of the queue.
    pub fn enqueue(&mut self, value: T) {
        self.items.push(Some(value));
        self.len += 1;
    }

    /// Removes and returns the front item.
    pub fn dequeue(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let value = self.items[self.head].take();
        self.head += 1;
        self.len -= 1;
        self.compact_if_needed();
        value
    }

    /// Returns the front item without removing it.
    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }

        self.items.get(self.head).and_then(Option::as_ref)
    }

    /// Number of pending items.
    pub fn len(&self) -> usize {
        self.len
    }

    /// True when the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Iterates from oldest to newest item.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items[self.head..].iter().filter_map(Option::as_ref)
    }

    fn compact_if_needed(&mut self) {
        if self.head == 0 {
            return;
        }

        if self.len == 0 {
            self.items.clear();
            self.head = 0;
            return;
        }

        if self.head >= 64 && self.head * 2 >= self.items.len() {
            self.items = self.items.split_off(self.head);
            self.head = 0;
        }
    }
}
