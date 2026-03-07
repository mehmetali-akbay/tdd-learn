use async_rust::*;
use std::sync::Arc;
use std::time::Duration;

// ===== Topic 1: Future Basics =====

#[tokio::test]
async fn test_async_greet() {
    assert_eq!(async_greet("Alice").await, "Hello, Alice!");
}

#[tokio::test]
async fn test_async_greet_empty() {
    assert_eq!(async_greet("").await, "Hello, !");
}

#[tokio::test]
async fn test_async_sum() {
    assert_eq!(async_sum(&[1, 2, 3, 4, 5]).await, 15);
}

#[tokio::test]
async fn test_async_sum_empty() {
    assert_eq!(async_sum(&[]).await, 0);
}

#[tokio::test]
async fn test_async_sum_negative() {
    assert_eq!(async_sum(&[-1, -2, 3]).await, 0);
}

#[tokio::test]
async fn test_fetch_value() {
    assert_eq!(fetch_value(42).await, "value_42");
}

#[tokio::test]
async fn test_fetch_value_zero() {
    assert_eq!(fetch_value(0).await, "value_0");
}

#[tokio::test]
async fn test_process_sequential() {
    assert_eq!(process_sequential(&[1, 2, 3]).await, vec![2, 4, 6]);
}

#[tokio::test]
async fn test_process_sequential_empty() {
    assert_eq!(process_sequential(&[]).await, Vec::<i32>::new());
}

#[tokio::test]
async fn test_async_identity() {
    assert_eq!(async_identity(42).await, 42);
    assert_eq!(async_identity("hello".to_string()).await, "hello");
}

#[tokio::test]
async fn test_async_uppercase() {
    assert_eq!(async_uppercase("hello").await, "HELLO");
    assert_eq!(async_uppercase("").await, "");
}

#[tokio::test]
async fn test_async_concat() {
    assert_eq!(async_concat("hello", " world").await, "hello world");
    assert_eq!(async_concat("", "b").await, "b");
}

#[tokio::test]
async fn test_async_fibonacci() {
    assert_eq!(async_fibonacci(0).await, 0);
    assert_eq!(async_fibonacci(1).await, 1);
    assert_eq!(async_fibonacci(10).await, 55);
    assert_eq!(async_fibonacci(20).await, 6765);
}

#[tokio::test]
async fn test_fetch_and_transform() {
    assert_eq!(fetch_and_transform(5).await, "VALUE_5");
}

// ===== Topic 2: Tokio Runtime & Spawning =====

#[tokio::test]
async fn test_spawn_computation() {
    assert_eq!(spawn_computation(5).await, 25);
}

#[tokio::test]
async fn test_spawn_computation_zero() {
    assert_eq!(spawn_computation(0).await, 0);
}

#[tokio::test]
async fn test_spawn_multiple() {
    let mut r = spawn_multiple(vec![1, 2, 3]).await;
    r.sort();
    assert_eq!(r, vec![1, 4, 9]);
}

#[tokio::test]
async fn test_spawn_multiple_empty() {
    assert_eq!(spawn_multiple(vec![]).await, Vec::<i32>::new());
}

#[tokio::test]
async fn test_background_work() {
    assert_eq!(background_work("hello".into()).await, "HELLO");
}

#[tokio::test]
async fn test_spawn_generate() {
    assert_eq!(spawn_generate(5).await, vec![0, 1, 4, 9, 16]);
}

#[tokio::test]
async fn test_spawn_generate_empty() {
    assert_eq!(spawn_generate(0).await, Vec::<usize>::new());
}

#[tokio::test]
async fn test_spawn_fallible_success() {
    assert_eq!(spawn_fallible(true).await, Ok("success".to_string()));
}

#[tokio::test]
async fn test_spawn_fallible_failure() {
    assert_eq!(spawn_fallible(false).await, Err("failure".to_string()));
}

#[tokio::test]
async fn test_spawn_map() {
    let result = spawn_map(vec![1, 2, 3, 4], 10).await;
    assert_eq!(result, vec![10, 20, 30, 40]);
}

#[tokio::test]
async fn test_spawn_map_empty() {
    assert_eq!(spawn_map(vec![], 5).await, Vec::<i32>::new());
}

// ===== Topic 3: Concurrent Futures =====

#[tokio::test]
async fn test_fetch_two() {
    let (a, b) = fetch_two(1, 2).await;
    assert_eq!(a, "value_1");
    assert_eq!(b, "value_2");
}

#[tokio::test]
async fn test_triple_fetch() {
    let (a, b, c) = triple_fetch(1, 2, 3).await;
    assert_eq!(a, "value_1");
    assert_eq!(b, "value_2");
    assert_eq!(c, "value_3");
}

#[tokio::test]
async fn test_fetch_all() {
    let results = fetch_all(vec![1, 2, 3]).await;
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "value_1");
}

#[tokio::test]
async fn test_fetch_all_empty() {
    assert_eq!(fetch_all(vec![]).await, Vec::<String>::new());
}

#[tokio::test]
async fn test_concurrent_sum() {
    // 3^2 + 4^2 = 9 + 16 = 25
    assert_eq!(concurrent_sum(3, 4).await, 25);
}

#[tokio::test]
async fn test_concurrent_sum_zero() {
    assert_eq!(concurrent_sum(0, 0).await, 0);
}

#[tokio::test]
async fn test_concurrent_fibs() {
    let results = concurrent_fibs(vec![0, 1, 5, 10]).await;
    assert_eq!(results, vec![0, 1, 5, 55]);
}

#[tokio::test]
async fn test_concurrent_fibs_empty() {
    assert_eq!(concurrent_fibs(vec![]).await, Vec::<u64>::new());
}

#[tokio::test]
async fn test_race_values() {
    // The faster one (10ms) should win over the slower one (200ms)
    let result = race_values(10, 1, 200, 2).await;
    assert_eq!(result, 1);
}

// ===== Topic 4: Async Channels =====

#[tokio::test]
async fn test_channel_basic() {
    assert_eq!(
        channel_basic(vec!["hello".into(), "world".into()]).await,
        vec!["hello", "world"]
    );
}

#[tokio::test]
async fn test_channel_basic_empty() {
    assert_eq!(channel_basic(vec![]).await, Vec::<String>::new());
}

#[tokio::test]
async fn test_oneshot_result() {
    assert_eq!(oneshot_result(21).await, 42);
}

#[tokio::test]
async fn test_oneshot_result_zero() {
    assert_eq!(oneshot_result(0).await, 0);
}

#[tokio::test]
async fn test_producer_consumer() {
    let r = producer_consumer(vec![1, 2, 3]).await;
    assert_eq!(r, vec![2, 4, 6]);
}

#[tokio::test]
async fn test_producer_consumer_empty() {
    assert_eq!(producer_consumer(vec![]).await, Vec::<i32>::new());
}

#[tokio::test]
async fn test_multi_producer() {
    let mut result = multi_producer(vec!["a".into(), "b".into(), "c".into()]).await;
    result.sort();
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[tokio::test]
async fn test_channel_pipeline() {
    assert_eq!(channel_pipeline(vec![1, 2, 3]).await, vec![3, 6, 9]);
}

#[tokio::test]
async fn test_channel_pipeline_empty() {
    assert_eq!(channel_pipeline(vec![]).await, Vec::<i32>::new());
}

#[tokio::test]
async fn test_bounded_channel_test() {
    let result = bounded_channel_test(2, vec![10, 20, 30]).await;
    assert_eq!(result, vec![10, 20, 30]);
}

#[tokio::test]
async fn test_request_reply() {
    let reply = request_reply("hello".to_string()).await;
    assert_eq!(reply, "reply: hello");
}

#[tokio::test]
async fn test_request_reply_empty() {
    let reply = request_reply("".to_string()).await;
    assert_eq!(reply, "reply: ");
}

// ===== Topic 5: Async Utilities =====

#[tokio::test]
async fn test_delayed_value() {
    let start = std::time::Instant::now();
    assert_eq!(delayed_value(50, 42).await, 42);
    assert!(start.elapsed() >= Duration::from_millis(40));
}

#[tokio::test]
async fn test_delayed_value_zero() {
    assert_eq!(delayed_value(0, 99).await, 99);
}

#[tokio::test]
async fn test_with_timeout_success() {
    assert_eq!(with_timeout(500).await, Ok("done".to_string()));
}

#[tokio::test]
async fn test_with_timeout_fail() {
    assert_eq!(with_timeout(1).await, Err("timeout".to_string()));
}

#[tokio::test]
async fn test_retry_operation() {
    assert_eq!(retry_operation(3).await, Ok("success".to_string()));
    assert_eq!(retry_operation(1).await, Ok("success".to_string()));
}

#[tokio::test]
async fn test_timeout_or_default_success() {
    let result = timeout_or_default(500, 10, -1).await;
    assert_eq!(result, 42);
}

#[tokio::test]
async fn test_timeout_or_default_timeout() {
    let result = timeout_or_default(1, 500, -1).await;
    assert_eq!(result, -1);
}

#[tokio::test]
async fn test_interval_collect() {
    let results = interval_collect(5, 1).await;
    assert_eq!(results, vec![0, 1, 2, 3, 4]);
}

#[tokio::test]
async fn test_interval_collect_zero() {
    let results = interval_collect(0, 10).await;
    assert_eq!(results, Vec::<usize>::new());
}

#[tokio::test]
async fn test_retry_until_success() {
    let attempts = vec![false, false, true, false];
    assert_eq!(retry_until(&attempts).await, Ok(2));
}

#[tokio::test]
async fn test_retry_until_all_fail() {
    let attempts = vec![false, false, false];
    assert_eq!(
        retry_until(&attempts).await,
        Err("all attempts failed".to_string())
    );
}

#[tokio::test]
async fn test_retry_until_first() {
    let attempts = vec![true];
    assert_eq!(retry_until(&attempts).await, Ok(0));
}

#[tokio::test]
async fn test_map_with_timeout() {
    let results = map_with_timeout(vec![1, 2, 3], 1000).await;
    assert_eq!(results, vec![Some(2), Some(4), Some(6)]);
}

// ===== Topic 6: Streams =====

#[tokio::test]
async fn test_countdown() {
    let stream = Countdown::new(3);
    let results = consume_stream(stream).await;
    assert_eq!(results, vec![3, 2, 1]);
}

#[tokio::test]
async fn test_countdown_zero() {
    let stream = Countdown::new(0);
    let results = consume_stream(stream).await;
    assert_eq!(results, Vec::<u32>::new());
}

#[tokio::test]
async fn test_range_stream() {
    let stream = RangeStream::new(1, 5);
    let results: Vec<i32> = tokio_stream::StreamExt::collect(stream).await;
    assert_eq!(results, vec![1, 2, 3, 4]);
}

#[tokio::test]
async fn test_range_stream_empty() {
    let stream = RangeStream::new(5, 5);
    let results: Vec<i32> = tokio_stream::StreamExt::collect(stream).await;
    assert_eq!(results, Vec::<i32>::new());
}

#[tokio::test]
async fn test_stream_operations() {
    let stream = Countdown::new(5); // 5, 4, 3, 2, 1
    // Evens: 4, 2 -> * 10 -> 40, 20
    let results = stream_operations(stream).await;
    assert_eq!(results, vec![40, 20]);
}

#[tokio::test]
async fn test_stream_sum() {
    let stream = RangeStream::new(1, 6); // 1..=5
    assert_eq!(stream_sum(stream).await, 15);
}

#[tokio::test]
async fn test_stream_sum_empty() {
    let stream = RangeStream::new(0, 0);
    assert_eq!(stream_sum(stream).await, 0);
}

#[tokio::test]
async fn test_stream_count() {
    let stream = RangeStream::new(0, 10);
    assert_eq!(stream_count(stream).await, 10);
}

#[tokio::test]
async fn test_stream_count_empty() {
    let stream = RangeStream::new(0, 0);
    assert_eq!(stream_count(stream).await, 0);
}

#[tokio::test]
async fn test_stream_take() {
    let stream = RangeStream::new(0, 100);
    let results = stream_take(stream, 3).await;
    assert_eq!(results, vec![0, 1, 2]);
}

#[tokio::test]
async fn test_stream_take_more_than_available() {
    let stream = RangeStream::new(0, 3);
    let results = stream_take(stream, 10).await;
    assert_eq!(results, vec![0, 1, 2]);
}

#[tokio::test]
async fn test_stream_skip() {
    let stream = RangeStream::new(0, 5);
    let results = stream_skip(stream, 2).await;
    assert_eq!(results, vec![2, 3, 4]);
}

#[tokio::test]
async fn test_stream_skip_all() {
    let stream = RangeStream::new(0, 3);
    let results = stream_skip(stream, 10).await;
    assert_eq!(results, Vec::<i32>::new());
}

#[tokio::test]
async fn test_stream_chain() {
    let s1 = RangeStream::new(1, 4);
    let s2 = RangeStream::new(10, 13);
    let results = stream_chain(s1, s2).await;
    assert_eq!(results, vec![1, 2, 3, 10, 11, 12]);
}

// ===== Topic 7: Shared Async State =====

#[tokio::test]
async fn test_async_counter_basic() {
    let counter = AsyncCounter::new(0);
    counter.increment().await;
    counter.increment().await;
    counter.decrement().await;
    assert_eq!(counter.get().await, 1);
}

#[tokio::test]
async fn test_async_counter_add() {
    let counter = AsyncCounter::new(10);
    counter.add(5).await;
    counter.add(-3).await;
    assert_eq!(counter.get().await, 12);
}

#[tokio::test]
async fn test_async_counter_reset() {
    let counter = AsyncCounter::new(42);
    counter.reset(0).await;
    assert_eq!(counter.get().await, 0);
}

#[tokio::test]
async fn test_async_counter_swap() {
    let counter = AsyncCounter::new(10);
    let old = counter.swap(20).await;
    assert_eq!(old, 10);
    assert_eq!(counter.get().await, 20);
}

#[tokio::test]
async fn test_async_counter_concurrent() {
    let counter = Arc::new(AsyncCounter::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            for _ in 0..100 {
                c.increment().await;
            }
        }));
    }
    for h in handles {
        h.await.unwrap();
    }
    assert_eq!(counter.get().await, 1000);
}

#[tokio::test]
async fn test_async_cache_basic() {
    let cache = AsyncCache::new();
    assert!(cache.is_empty().await);
    cache.insert("key1", "val1").await;
    cache.insert("key2", "val2").await;
    assert_eq!(cache.get("key1").await, Some("val1".to_string()));
    assert_eq!(cache.get("key2").await, Some("val2".to_string()));
    assert_eq!(cache.get("key3").await, None);
    assert_eq!(cache.len().await, 2);
}

#[tokio::test]
async fn test_async_cache_remove() {
    let cache = AsyncCache::new();
    cache.insert("k", "v").await;
    assert_eq!(cache.remove("k").await, Some("v".to_string()));
    assert!(cache.is_empty().await);
}

#[tokio::test]
async fn test_async_cache_contains_keys() {
    let cache = AsyncCache::new();
    cache.insert("b", "2").await;
    cache.insert("a", "1").await;
    assert!(cache.contains_key("a").await);
    assert!(!cache.contains_key("c").await);
    assert_eq!(cache.keys().await, vec!["a", "b"]); // sorted
}

#[tokio::test]
async fn test_async_cache_clear() {
    let cache = AsyncCache::new();
    cache.insert("k", "v").await;
    cache.clear().await;
    assert!(cache.is_empty().await);
}

#[tokio::test]
async fn test_async_cache_default() {
    let cache = AsyncCache::default();
    assert!(cache.is_empty().await);
}

#[tokio::test]
async fn test_async_task_queue() {
    let (queue, mut rx) = AsyncTaskQueue::new();
    queue.submit("task1").await;
    queue.submit("task2").await;

    let msg1 = rx.recv().await.unwrap();
    let msg2 = rx.recv().await.unwrap();
    assert_eq!(msg1, "task1");
    assert_eq!(msg2, "task2");

    queue.record_result("result1").await;
    queue.record_result("result2").await;
    assert_eq!(queue.result_count().await, 2);
    assert_eq!(queue.get_results().await, vec!["result1", "result2"]);
}

#[tokio::test]
async fn test_process_queue() {
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    tx.send("task1".to_string()).await.unwrap();
    tx.send("task2".to_string()).await.unwrap();
    drop(tx);

    let results = process_queue(rx).await;
    assert_eq!(results, vec!["done: task1", "done: task2"]);
}

#[tokio::test]
async fn test_process_queue_empty() {
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(32);
    drop(tx);
    let results = process_queue(rx).await;
    assert_eq!(results, Vec::<String>::new());
}

#[tokio::test]
async fn test_limited_concurrency() {
    let items: Vec<i32> = (1..=10).collect();
    let results = limited_concurrency(items, 3).await;
    assert_eq!(results, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
}

#[tokio::test]
async fn test_limited_concurrency_empty() {
    let results = limited_concurrency(vec![], 4).await;
    assert_eq!(results, Vec::<i32>::new());
}

#[tokio::test]
async fn test_limited_concurrency_single() {
    let results = limited_concurrency(vec![5], 1).await;
    assert_eq!(results, vec![10]);
}
