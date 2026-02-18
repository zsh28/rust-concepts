use challenge2_todo::Queue;

#[test]
fn queue_is_fifo() {
    let mut queue = Queue::new();
    queue.enqueue(1_u8);
    queue.enqueue(2_u8);
    queue.enqueue(3_u8);

    assert_eq!(queue.peek(), Some(&1_u8));
    assert_eq!(queue.dequeue(), Some(1_u8));
    assert_eq!(queue.dequeue(), Some(2_u8));
    assert_eq!(queue.dequeue(), Some(3_u8));
    assert!(queue.is_empty());
}

#[test]
fn queue_can_remove_at_logical_index() {
    let mut queue = Queue::new();
    queue.enqueue(10_u8);
    queue.enqueue(20_u8);
    queue.enqueue(30_u8);
    queue.enqueue(40_u8);

    assert_eq!(queue.remove_at(1), Some(20_u8));
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.dequeue(), Some(10_u8));
    assert_eq!(queue.dequeue(), Some(30_u8));
    assert_eq!(queue.dequeue(), Some(40_u8));
    assert!(queue.is_empty());
}
