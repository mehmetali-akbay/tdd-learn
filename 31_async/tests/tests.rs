use async_rust::*;
use std::time::Duration;

#[tokio::test]
async fn test_async_greet() {
    assert!(async_greet("Alice").await.contains("Alice"));
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
async fn test_fetch_value() {
    assert!(fetch_value(42).await.contains("42"));
}
#[tokio::test]
async fn test_process_sequential() {
    assert_eq!(process_sequential(&[1, 2, 3]).await, vec![2, 4, 6]);
}
#[tokio::test]
async fn test_spawn_computation() {
    assert_eq!(spawn_computation(5).await, 25);
}
#[tokio::test]
async fn test_spawn_multiple() {
    let mut r = spawn_multiple(vec![1, 2, 3]).await;
    r.sort();
    assert_eq!(r, vec![1, 4, 9]);
}
#[tokio::test]
async fn test_background_work() {
    assert_eq!(background_work("hello".into()).await, "HELLO");
}
#[tokio::test]
async fn test_fetch_two() {
    let (a, b) = fetch_two(1, 2).await;
    assert!(a.contains("1"));
    assert!(b.contains("2"));
}
#[tokio::test]
async fn test_triple_fetch() {
    let (a, b, c) = triple_fetch(1, 2, 3).await;
    assert!(a.contains("1"));
    assert!(b.contains("2"));
    assert!(c.contains("3"));
}
#[tokio::test]
async fn test_fetch_all() {
    assert_eq!(fetch_all(vec![1, 2, 3]).await.len(), 3);
}
#[tokio::test]
async fn test_channel_basic() {
    assert_eq!(
        channel_basic(vec!["hello".into(), "world".into()]).await,
        vec!["hello", "world"]
    );
}
#[tokio::test]
async fn test_oneshot_result() {
    assert_eq!(oneshot_result(21).await, 42);
}
#[tokio::test]
async fn test_producer_consumer() {
    let mut r = producer_consumer(vec![1, 2, 3]).await;
    r.sort();
    assert_eq!(r, vec![2, 4, 6]);
}
#[tokio::test]
async fn test_delayed_value() {
    let start = std::time::Instant::now();
    assert_eq!(delayed_value(50, 42).await, 42);
    assert!(start.elapsed() >= Duration::from_millis(40));
}
#[tokio::test]
async fn test_with_timeout_success() {
    assert!(with_timeout(500).await.is_ok());
}
// ===== Topic 6: Streams =====

#[tokio::test]
async fn test_consume_stream() {
    let stream = Countdown::new(3);
    let results = consume_stream(stream).await;
    assert_eq!(results, vec![3, 2, 1]);
}

#[tokio::test]
async fn test_stream_operations() {
    let stream = Countdown::new(5); // 5, 4, 3, 2, 1
    // Evens: 4, 2 -> * 10 -> 40, 20
    let results = stream_operations(stream).await;
    assert_eq!(results, vec![40, 20]);
}
