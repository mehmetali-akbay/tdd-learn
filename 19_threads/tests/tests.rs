use std::sync::{Arc, Mutex};
use threads::*;

// ===== Topic 1: Thread Basics — Spawning & Joining =====

#[test]
fn test_threaded_sum() {
    assert_eq!(threaded_sum(10), 55);
    assert_eq!(threaded_sum(0), 0);
}

#[test]
fn test_threaded_sum_one() {
    assert_eq!(threaded_sum(1), 1);
}

#[test]
fn test_threaded_sum_large() {
    assert_eq!(threaded_sum(100), 5050);
}

#[test]
fn test_parallel_squares() {
    assert_eq!(parallel_squares(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
    assert_eq!(parallel_squares(vec![]), vec![]);
}

#[test]
fn test_parallel_squares_single() {
    assert_eq!(parallel_squares(vec![7]), vec![49]);
}

#[test]
fn test_parallel_squares_negative() {
    assert_eq!(parallel_squares(vec![-3, -2, 0, 2]), vec![9, 4, 0, 4]);
}

#[test]
fn test_thread_ids() {
    assert_eq!(thread_ids(5), vec![0, 1, 2, 3, 4]);
    assert_eq!(thread_ids(0), vec![]);
}

#[test]
fn test_thread_ids_one() {
    assert_eq!(thread_ids(1), vec![0]);
}

#[test]
fn test_spawn_compute() {
    let result = spawn_compute(|| 42);
    assert_eq!(result, 42);
}

#[test]
fn test_spawn_compute_string() {
    let result = spawn_compute(|| "hello".to_string());
    assert_eq!(result, "hello");
}

#[test]
fn test_spawn_compute_closure() {
    let x = 10;
    let result = spawn_compute(move || x * x);
    assert_eq!(result, 100);
}

#[test]
fn test_parallel_generate() {
    let result = parallel_generate(5, |i| i * 2);
    assert_eq!(result, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_parallel_generate_empty() {
    let result = parallel_generate(0, |i| i);
    assert_eq!(result, Vec::<usize>::new());
}

#[test]
fn test_parallel_generate_strings() {
    let result = parallel_generate(3, |i| format!("item-{}", i));
    assert_eq!(result, vec!["item-0", "item-1", "item-2"]);
}

#[test]
fn test_current_thread_name() {
    // Main thread should have a name
    let name = current_thread_name();
    assert!(!name.is_empty());
}

#[test]
fn test_spawn_named_thread() {
    let name = spawn_named_thread("my-worker");
    assert_eq!(name, "my-worker");
}

// ===== Topic 2: Shared State — Mutex =====

#[test]
fn test_shared_counter() {
    assert_eq!(shared_counter(4, 250), 1000);
    assert_eq!(shared_counter(1, 100), 100);
}

#[test]
fn test_shared_counter_single_thread() {
    assert_eq!(shared_counter(1, 1000), 1000);
}

#[test]
fn test_shared_counter_many_threads() {
    assert_eq!(shared_counter(10, 100), 1000);
}

#[test]
fn test_parallel_collect() {
    let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let result = parallel_collect(items);
    assert_eq!(result, vec!["a", "b", "c"]); // sorted
}

#[test]
fn test_parallel_collect_empty() {
    let result = parallel_collect(vec![]);
    assert_eq!(result, Vec::<String>::new());
}

#[test]
fn test_parallel_map() {
    let result = parallel_map(vec![1, 2, 3], |x| x * 10);
    assert_eq!(result, vec![10, 20, 30]);
}

#[test]
fn test_parallel_map_strings() {
    let result = parallel_map(vec![1, 2, 3], |x| format!("n={}", x));
    assert_eq!(result, vec!["n=1", "n=2", "n=3"]);
}

#[test]
fn test_parallel_filter() {
    let result = parallel_filter(vec![1, 2, 3, 4, 5, 6], |x| x % 2 == 0);
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn test_parallel_filter_empty() {
    let result = parallel_filter(vec![], |_x| true);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_parallel_filter_none_match() {
    let result = parallel_filter(vec![1, 3, 5], |x| x % 2 == 0);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_parallel_max() {
    assert_eq!(parallel_max(&[3, 1, 4, 1, 5, 9, 2, 6], 4), Some(9));
}

#[test]
fn test_parallel_max_empty() {
    assert_eq!(parallel_max(&[], 4), None);
}

#[test]
fn test_parallel_max_single() {
    assert_eq!(parallel_max(&[42], 2), Some(42));
}

#[test]
fn test_parallel_min() {
    assert_eq!(parallel_min(&[3, 1, 4, 1, 5, 9, 2, 6], 4), Some(1));
}

#[test]
fn test_parallel_min_empty() {
    assert_eq!(parallel_min(&[], 4), None);
}

#[test]
fn test_parallel_min_negative() {
    assert_eq!(parallel_min(&[-5, 3, -10, 7], 2), Some(-10));
}

// ===== Topic 3: Channels — Message Passing =====

#[test]
fn test_channel_basics() {
    assert_eq!(channel_basics(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_channel_empty() {
    assert_eq!(channel_basics(vec![]), Vec::<i32>::new());
}

#[test]
fn test_multi_producer() {
    let values = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let result = multi_producer(values);
    assert_eq!(result, vec!["a", "b", "c"]); // sorted
}

#[test]
fn test_multi_producer_single() {
    let result = multi_producer(vec!["only".to_string()]);
    assert_eq!(result, vec!["only"]);
}

#[test]
fn test_channel_pipeline() {
    assert_eq!(channel_pipeline(vec![1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn test_channel_pipeline_empty() {
    assert_eq!(channel_pipeline(vec![]), Vec::<i32>::new());
}

#[test]
fn test_channel_fanout() {
    let results = channel_fanout(vec![1, 2, 3], 3);
    assert_eq!(results.len(), 3);
    for recv in &results {
        assert_eq!(recv, &vec![1, 2, 3]);
    }
}

#[test]
fn test_channel_fanout_single_receiver() {
    let results = channel_fanout(vec![10, 20], 1);
    assert_eq!(results, vec![vec![10, 20]]);
}

#[test]
fn test_channel_accumulate() {
    let sums = channel_accumulate(vec![1, 2, 3, 4]);
    assert_eq!(sums, vec![1, 3, 6, 10]);
}

#[test]
fn test_channel_accumulate_empty() {
    let sums = channel_accumulate(vec![]);
    assert_eq!(sums, Vec::<i64>::new());
}

#[test]
fn test_channel_accumulate_single() {
    let sums = channel_accumulate(vec![42]);
    assert_eq!(sums, vec![42]);
}

#[test]
fn test_channel_ping_pong() {
    let log = channel_ping_pong(2);
    // Thread A logs: ping-0, pong-0 (received), ping-1, pong-1 (received) = 4
    // Thread B logs: pong-0, pong-1 = 2
    // Total = 6
    assert_eq!(log.len(), 6);
    assert!(log.contains(&"ping-0".to_string()));
    assert!(log.contains(&"pong-0".to_string()));
    assert!(log.contains(&"ping-1".to_string()));
    assert!(log.contains(&"pong-1".to_string()));
}

#[test]
fn test_channel_ping_pong_zero() {
    let log = channel_ping_pong(0);
    assert!(log.is_empty());
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
fn test_atomic_counter_decrement() {
    let counter = AtomicCounter::new(5);
    counter.decrement();
    counter.decrement();
    assert_eq!(counter.get(), 3);
}

#[test]
fn test_atomic_counter_add() {
    let counter = AtomicCounter::new(0);
    counter.add(10);
    counter.add(5);
    counter.add(-3);
    assert_eq!(counter.get(), 12);
}

#[test]
fn test_atomic_counter_reset() {
    let counter = AtomicCounter::new(42);
    assert_eq!(counter.get(), 42);
    counter.reset(0);
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_atomic_counter_swap() {
    let counter = AtomicCounter::new(10);
    let old = counter.swap(20);
    assert_eq!(old, 10);
    assert_eq!(counter.get(), 20);
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
fn test_bounded_buffer_peek() {
    let buf = BoundedBuffer::new(3);
    assert_eq!(buf.peek(), None);
    buf.try_push(42);
    assert_eq!(buf.peek(), Some(42));
    assert_eq!(buf.len(), 1); // peek doesn't remove
}

#[test]
fn test_bounded_buffer_clear() {
    let buf = BoundedBuffer::new(5);
    buf.try_push(1);
    buf.try_push(2);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_bounded_buffer_capacity_remaining() {
    let buf = BoundedBuffer::new(5);
    assert_eq!(buf.capacity(), 5);
    assert_eq!(buf.remaining(), 5);
    buf.try_push(1);
    buf.try_push(2);
    assert_eq!(buf.remaining(), 3);
}

#[test]
fn test_bounded_buffer_share_threaded() {
    let buf = BoundedBuffer::new(100);
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let b = buf.share();
            std::thread::spawn(move || {
                for j in 0..10 {
                    b.try_push(i * 10 + j);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(buf.len(), 100);
}

// ===== Topic 5: Thread Pool Pattern =====

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
fn test_pool_map() {
    let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
        Box::new(|| 1 + 1),
        Box::new(|| 2 * 3),
        Box::new(|| 10 - 5),
    ];
    let results = pool_map(2, tasks);
    assert_eq!(results, vec![2, 6, 5]);
}

#[test]
fn test_pool_map_strings() {
    let tasks: Vec<Box<dyn FnOnce() -> String + Send>> = (0..5)
        .map(|i| Box::new(move || format!("result-{}", i)) as Box<dyn FnOnce() -> String + Send>)
        .collect();
    let results = pool_map(3, tasks);
    assert_eq!(results, vec!["result-0", "result-1", "result-2", "result-3", "result-4"]);
}

#[test]
fn test_pool_execute_count() {
    let count = pool_execute_count(4, 20);
    assert_eq!(count, 20);
}

#[test]
fn test_pool_execute_count_single_worker() {
    let count = pool_execute_count(1, 10);
    assert_eq!(count, 10);
}

// ===== Topic 6: Advanced — Parallel Algorithms =====

#[test]
fn test_parallel_sum() {
    let nums: Vec<i64> = (1..=100).collect();
    assert_eq!(parallel_sum(&nums, 4), 5050);
    assert_eq!(parallel_sum(&[], 4), 0);
}

#[test]
fn test_parallel_sum_single_thread() {
    let nums: Vec<i64> = (1..=10).collect();
    assert_eq!(parallel_sum(&nums, 1), 55);
}

#[test]
fn test_parallel_find() {
    let items: Vec<i32> = (0..1000).collect();
    assert!(parallel_find(&items, 500, 4));
    assert!(!parallel_find(&items, 9999, 4));
}

#[test]
fn test_parallel_find_empty() {
    assert!(!parallel_find(&[], 1, 4));
}

#[test]
fn test_parallel_apply() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = parallel_apply(&items, |x| x * 2, 3);
    assert_eq!(result, vec![2, 4, 6, 8, 10, 12, 14, 16]);
}

#[test]
fn test_parallel_apply_empty() {
    assert_eq!(parallel_apply(&[], |x| x * 2, 4), Vec::<i32>::new());
}

#[test]
fn test_parallel_reduce_sum() {
    let items: Vec<i64> = (1..=10).collect();
    let result = parallel_reduce(&items, 0, |a, b| a + b, 4);
    assert_eq!(result, 55);
}

#[test]
fn test_parallel_reduce_product() {
    let items: Vec<i64> = vec![1, 2, 3, 4, 5];
    let result = parallel_reduce(&items, 1, |a, b| a * b, 2);
    assert_eq!(result, 120);
}

#[test]
fn test_parallel_reduce_empty() {
    let result = parallel_reduce(&[], 0, |a, b| a + b, 4);
    assert_eq!(result, 0);
}

#[test]
fn test_parallel_count() {
    let items: Vec<i32> = (1..=20).collect();
    let count = parallel_count(&items, |x| x % 2 == 0, 4);
    assert_eq!(count, 10);
}

#[test]
fn test_parallel_count_empty() {
    assert_eq!(parallel_count(&[], |_| true, 4), 0);
}

#[test]
fn test_parallel_count_none() {
    let items = vec![1, 3, 5, 7];
    assert_eq!(parallel_count(&items, |x| x % 2 == 0, 2), 0);
}

#[test]
fn test_parallel_any() {
    let items: Vec<i32> = (1..=10).collect();
    assert!(parallel_any(&items, |x| x > 5, 3));
    assert!(!parallel_any(&items, |x| x > 100, 3));
}

#[test]
fn test_parallel_any_empty() {
    assert!(!parallel_any(&[], |_| true, 4));
}

#[test]
fn test_parallel_all() {
    let items = vec![2, 4, 6, 8];
    assert!(parallel_all(&items, |x| x % 2 == 0, 2));
    assert!(!parallel_all(&items, |x| x > 5, 2));
}

#[test]
fn test_parallel_all_empty() {
    assert!(parallel_all(&[], |_| false, 4)); // vacuously true
}

// ===== Topic 7: Thread-Safe Data Structures =====

#[test]
fn test_concurrent_map_basic() {
    let map = ConcurrentMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    assert_eq!(map.get("a"), Some(1));
    assert_eq!(map.get("b"), Some(2));
    assert_eq!(map.get("c"), None);
}

#[test]
fn test_concurrent_map_remove() {
    let map = ConcurrentMap::new();
    map.insert("x", 10);
    assert_eq!(map.remove("x"), Some(10));
    assert!(map.is_empty());
}

#[test]
fn test_concurrent_map_len_keys() {
    let map = ConcurrentMap::new();
    map.insert("b", 2);
    map.insert("a", 1);
    map.insert("c", 3);
    assert_eq!(map.len(), 3);
    assert_eq!(map.keys(), vec!["a", "b", "c"]); // sorted
}

#[test]
fn test_concurrent_map_contains_clear() {
    let map = ConcurrentMap::new();
    map.insert("key", 42);
    assert!(map.contains_key("key"));
    assert!(!map.contains_key("other"));
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_concurrent_map_threaded() {
    let map = ConcurrentMap::new();
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let m = map.share();
            std::thread::spawn(move || {
                m.insert(&format!("key-{}", i), i);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(map.len(), 10);
}

#[test]
fn test_concurrent_map_default() {
    let map: ConcurrentMap<i32> = ConcurrentMap::default();
    assert!(map.is_empty());
}

#[test]
fn test_shared_log_basic() {
    let log = SharedLog::new();
    assert!(log.is_empty());
    log.append("first");
    log.append("second");
    assert_eq!(log.len(), 2);
    assert_eq!(log.entries(), vec!["first", "second"]);
}

#[test]
fn test_shared_log_last() {
    let log = SharedLog::new();
    assert_eq!(log.last(), None);
    log.append("msg1");
    log.append("msg2");
    assert_eq!(log.last(), Some("msg2".to_string()));
}

#[test]
fn test_shared_log_contains() {
    let log = SharedLog::new();
    log.append("error: file not found");
    log.append("info: startup complete");
    assert!(log.contains("error"));
    assert!(log.contains("startup"));
    assert!(!log.contains("warning"));
}

#[test]
fn test_shared_log_clear() {
    let log = SharedLog::new();
    log.append("msg");
    log.clear();
    assert!(log.is_empty());
}

#[test]
fn test_shared_log_threaded() {
    let log = SharedLog::new();
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let l = log.share();
            std::thread::spawn(move || {
                l.append(&format!("thread-{}", i));
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(log.len(), 10);
}

#[test]
fn test_shared_log_default() {
    let log = SharedLog::default();
    assert!(log.is_empty());
}

#[test]
fn test_shared_stats_basic() {
    let stats = SharedStats::new();
    stats.add(10.0);
    stats.add(20.0);
    stats.add(30.0);
    assert_eq!(stats.count(), 3);
    assert_eq!(stats.sum(), 60.0);
    assert_eq!(stats.mean(), Some(20.0));
}

#[test]
fn test_shared_stats_min_max() {
    let stats = SharedStats::new();
    stats.add(5.0);
    stats.add(1.0);
    stats.add(9.0);
    stats.add(3.0);
    assert_eq!(stats.min(), Some(1.0));
    assert_eq!(stats.max(), Some(9.0));
}

#[test]
fn test_shared_stats_empty() {
    let stats = SharedStats::new();
    assert_eq!(stats.count(), 0);
    assert_eq!(stats.sum(), 0.0);
    assert_eq!(stats.mean(), None);
    assert_eq!(stats.min(), None);
    assert_eq!(stats.max(), None);
}

#[test]
fn test_shared_stats_threaded() {
    let stats = SharedStats::new();
    let handles: Vec<_> = (1..=10)
        .map(|i| {
            let s = stats.share();
            std::thread::spawn(move || {
                s.add(i as f64);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(stats.count(), 10);
    assert_eq!(stats.sum(), 55.0);
}

#[test]
fn test_shared_stats_default() {
    let stats = SharedStats::default();
    assert_eq!(stats.count(), 0);
}
