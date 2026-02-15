use ring_buffer::*;

// ===== Topic 1: Core Operations =====

#[test]
fn test_new_buffer() {
    let buf = RingBuffer::<i32>::new(5);
    assert_eq!(buf.capacity(), 5);
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
    assert!(!buf.is_full());
}

#[test]
fn test_push_pop() {
    let mut buf = RingBuffer::new(3);
    assert!(buf.push(1));
    assert!(buf.push(2));
    assert!(buf.push(3));
    assert!(!buf.push(4)); // full
    assert_eq!(buf.pop(), Some(1));
    assert_eq!(buf.pop(), Some(2));
    assert_eq!(buf.pop(), Some(3));
    assert_eq!(buf.pop(), None);
}

#[test]
fn test_fifo_order() {
    let mut buf = RingBuffer::new(4);
    buf.push(10);
    buf.push(20);
    buf.push(30);
    assert_eq!(buf.pop(), Some(10));
    buf.push(40);
    assert_eq!(buf.pop(), Some(20));
    assert_eq!(buf.pop(), Some(30));
    assert_eq!(buf.pop(), Some(40));
}

// ===== Topic 2: Peek & Access =====

#[test]
fn test_peek_front_back() {
    let mut buf = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert_eq!(buf.peek_front(), Some(&1));
    assert_eq!(buf.peek_back(), Some(&3));
}

#[test]
fn test_get() {
    let mut buf = RingBuffer::new(5);
    buf.push(10);
    buf.push(20);
    buf.push(30);
    assert_eq!(buf.get(0), Some(&10));
    assert_eq!(buf.get(1), Some(&20));
    assert_eq!(buf.get(2), Some(&30));
    assert_eq!(buf.get(3), None);
}

#[test]
fn test_peek_empty() {
    let buf = RingBuffer::<i32>::new(3);
    assert_eq!(buf.peek_front(), None);
    assert_eq!(buf.peek_back(), None);
}

// ===== Topic 3: Overwriting Mode =====

#[test]
fn test_push_overwrite() {
    let mut buf = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    let evicted = buf.push_overwrite(4);
    assert_eq!(evicted, Some(1));
    assert_eq!(buf.pop(), Some(2));
}

#[test]
fn test_push_overwrite_not_full() {
    let mut buf = RingBuffer::new(3);
    let evicted = buf.push_overwrite(1);
    assert_eq!(evicted, None);
    assert_eq!(buf.len(), 1);
}

// ===== Topic 4: Conversion & Collection =====

#[test]
fn test_to_vec() {
    let mut buf = RingBuffer::new(4);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert_eq!(buf.to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_drain() {
    let mut buf = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    let drained = buf.drain();
    assert_eq!(drained, vec![1, 2]);
    assert!(buf.is_empty());
}

#[test]
fn test_clear() {
    let mut buf = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    buf.clear();
    assert!(buf.is_empty());
}

// ===== Topic 5: Search & Query =====

#[test]
fn test_contains() {
    let mut buf = RingBuffer::new(5);
    buf.push(10);
    buf.push(20);
    assert!(buf.contains(&10));
    assert!(!buf.contains(&30));
}

#[test]
fn test_position() {
    let mut buf = RingBuffer::new(5);
    buf.push(10);
    buf.push(20);
    buf.push(30);
    assert_eq!(buf.position(&20), Some(1));
    assert_eq!(buf.position(&99), None);
}

// ===== Topic 6: Sliding Window =====

#[test]
fn test_window_sum() {
    let mut buf = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert_eq!(buf.window_sum(), 6);
}

#[test]
fn test_moving_average() {
    let avgs = moving_average(&[1.0, 2.0, 3.0, 4.0], 3);
    assert_eq!(avgs.len(), 4);
    assert!((avgs[2] - 2.0).abs() < 0.01);
    assert!((avgs[3] - 3.0).abs() < 0.01);
}

#[test]
fn test_sliding_window_max() {
    let maxes = sliding_window_max(&[1, 3, 2, 5, 4], 3);
    assert_eq!(maxes.len(), 5);
    assert_eq!(maxes[2], 3); // window [1,3,2]
    assert_eq!(maxes[3], 5); // window [3,2,5]
}

// ===== Edge Cases =====

#[test]
fn test_wrap_around() {
    let mut buf = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    buf.pop();
    buf.pop();
    buf.push(4);
    buf.push(5);
    assert_eq!(buf.to_vec(), vec![3, 4, 5]);
}
