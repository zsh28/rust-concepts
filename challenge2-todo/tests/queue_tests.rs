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
