use std::sync::{Arc, Mutex};
use threads::*;

// ===== Topic 1: Thread Basics =====

#[test]
fn test_threaded_sum() {
    assert_eq!(threaded_sum(10), 55);
    assert_eq!(threaded_sum(0), 0);
}

#[test]
fn test_parallel_squares() {
    assert_eq!(parallel_squares(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
    assert_eq!(parallel_squares(vec![]), vec![]);
}

#[test]
fn test_thread_ids() {
    assert_eq!(thread_ids(5), vec![0, 1, 2, 3, 4]);
    assert_eq!(thread_ids(0), vec![]);
}

// ===== Topic 2: Mutex =====

#[test]
fn test_shared_counter() {
    assert_eq!(shared_counter(4, 250), 1000);
    assert_eq!(shared_counter(1, 100), 100);
}

#[test]
fn test_parallel_collect() {
    let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut result = parallel_collect(items);
    result.sort();
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_parallel_map() {
    let result = parallel_map(vec![1, 2, 3], |x| x * 10);
    assert_eq!(result, vec![10, 20, 30]);
}

// ===== Topic 3: Channels =====

#[test]
fn test_channel_basics() {
    assert_eq!(channel_basics(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_multi_producer() {
    let values = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let result = multi_producer(values);
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_channel_pipeline() {
    assert_eq!(channel_pipeline(vec![1, 2, 3]), vec![2, 4, 6]);
}

// ===== Topic 4: AtomicCounter & BoundedBuffer =====

#[test]
fn test_atomic_counter() {
    let counter = AtomicCounter::new(0);
    counter.increment();
    counter.increment();
    counter.decrement();
    assert_eq!(counter.get(), 1);
}

#[test]
fn test_atomic_counter_threaded() {
    let counter = AtomicCounter::new(0);
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let c = counter.share();
            std::thread::spawn(move || {
                for _ in 0..100 {
                    c.increment();
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(counter.get(), 1000);
}

#[test]
fn test_bounded_buffer() {
    let buf = BoundedBuffer::new(3);
    assert!(buf.is_empty());
    assert!(buf.try_push(1));
    assert!(buf.try_push(2));
    assert!(buf.try_push(3));
    assert!(buf.is_full());
    assert!(!buf.try_push(4)); // at capacity
    assert_eq!(buf.try_pop(), Some(1));
    assert_eq!(buf.len(), 2);
}

#[test]
fn test_bounded_buffer_empty() {
    let buf = BoundedBuffer::<i32>::new(5);
    assert_eq!(buf.try_pop(), None);
    assert!(buf.is_empty());
}

// ===== Topic 5: Thread Pool =====

#[test]
fn test_thread_pool() {
    let pool = ThreadPool::new(4);
    let results = Arc::new(Mutex::new(Vec::new()));
    for i in 0..10 {
        let results = Arc::clone(&results);
        pool.execute(move || {
            results.lock().unwrap().push(i);
        });
    }
    drop(pool); // waits for all jobs
    let mut results = results.lock().unwrap().clone();
    results.sort();
    assert_eq!(results, (0..10).collect::<Vec<_>>());
}

#[test]
fn test_thread_pool_worker_count() {
    let pool = ThreadPool::new(8);
    assert_eq!(pool.worker_count(), 8);
}

// ===== Topic 6: Parallel Reduce =====

#[test]
fn test_parallel_sum() {
    let nums: Vec<i64> = (1..=100).collect();
    assert_eq!(parallel_sum(&nums, 4), 5050);
    assert_eq!(parallel_sum(&[], 4), 0);
}

#[test]
fn test_parallel_find() {
    let items: Vec<i32> = (0..1000).collect();
    assert!(parallel_find(&items, 500, 4));
    assert!(!parallel_find(&items, 9999, 4));
}

#[test]
fn test_parallel_apply() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = parallel_apply(&items, |x| x * 2, 3);
    assert_eq!(result, vec![2, 4, 6, 8, 10, 12, 14, 16]);
}

// ===== Edge Cases =====

#[test]
fn test_threaded_sum_one() {
    assert_eq!(threaded_sum(1), 1);
}

#[test]
fn test_parallel_squares_single() {
    assert_eq!(parallel_squares(vec![7]), vec![49]);
}

#[test]
fn test_shared_counter_single_thread() {
    assert_eq!(shared_counter(1, 1000), 1000);
}

#[test]
fn test_channel_empty() {
    assert_eq!(channel_basics(vec![]), Vec::<i32>::new());
}

#[test]
fn test_atomic_counter_decrement() {
    let counter = AtomicCounter::new(5);
    counter.decrement();
    counter.decrement();
    assert_eq!(counter.get(), 3);
}

#[test]
fn test_bounded_buffer_fifo() {
    let buf = BoundedBuffer::new(5);
    buf.try_push(10);
    buf.try_push(20);
    buf.try_push(30);
    assert_eq!(buf.try_pop(), Some(10));
    assert_eq!(buf.try_pop(), Some(20));
    assert_eq!(buf.try_pop(), Some(30));
    assert_eq!(buf.try_pop(), None);
}

#[test]
fn test_thread_pool_single_worker() {
    let pool = ThreadPool::new(1);
    let result = Arc::new(Mutex::new(0));
    let r = Arc::clone(&result);
    pool.execute(move || {
        *r.lock().unwrap() = 42;
    });
    drop(pool);
    assert_eq!(*result.lock().unwrap(), 42);
}

#[test]
fn test_parallel_sum_single_thread() {
    let nums: Vec<i64> = (1..=10).collect();
    assert_eq!(parallel_sum(&nums, 1), 55);
}

#[test]
fn test_parallel_find_empty() {
    let items: Vec<i32> = vec![];
    assert!(!parallel_find(&items, 1, 4));
}

#[test]
fn test_parallel_apply_empty() {
    let items: Vec<i32> = vec![];
    assert_eq!(parallel_apply(&items, |x| x * 2, 4), Vec::<i32>::new());
}
