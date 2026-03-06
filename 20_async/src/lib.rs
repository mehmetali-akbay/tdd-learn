// ============================================
// Level 8, Project 1: Async — Async/Await, Futures & Streams
// ============================================
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::time;

// Topic 1: Future Basics
pub async fn async_greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
pub async fn async_sum(items: &[i32]) -> i32 {
    items.iter().sum()
}
pub async fn fetch_value(id: u32) -> String {
    format!("value_{}", id)
}
pub async fn process_sequential(items: &[i32]) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}

// Topic 2: Tokio Runtime
pub async fn spawn_computation(x: i32) -> i32 {
    let handle = tokio::spawn(async move { x * x });
    handle.await.unwrap()
}
pub async fn spawn_multiple(values: Vec<i32>) -> Vec<i32> {
    let mut handles = Vec::new();
    for v in values {
        handles.push(tokio::spawn(async move { v * v }));
    }
    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}
pub async fn background_work(input: String) -> String {
    let handle = tokio::spawn(async move { input.to_uppercase() });
    handle.await.unwrap()
}

// Topic 3: Concurrent Futures
pub async fn fetch_two(id1: u32, id2: u32) -> (String, String) {
    let (a, b) = tokio::join!(fetch_value(id1), fetch_value(id2));
    (a, b)
}
pub async fn triple_fetch(a: u32, b: u32, c: u32) -> (String, String, String) {
    let (ra, rb, rc) = tokio::join!(fetch_value(a), fetch_value(b), fetch_value(c));
    (ra, rb, rc)
}
pub async fn fetch_all(ids: Vec<u32>) -> Vec<String> {
    let mut handles = Vec::new();
    for id in ids {
        handles.push(tokio::spawn(async move { format!("value_{}", id) }));
    }
    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

// Topic 4: Async Channels
pub async fn channel_basic(messages: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(32);
    for msg in messages {
        tx.send(msg).await.unwrap();
    }
    drop(tx);
    let mut received = Vec::new();
    while let Some(msg) = rx.recv().await {
        received.push(msg);
    }
    received
}
pub async fn oneshot_result(input: i32) -> i32 {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        tx.send(input * 2).unwrap();
    });
    rx.await.unwrap()
}
pub async fn producer_consumer(items: Vec<i32>) -> Vec<i32> {
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        for item in items {
            tx.send(item * 2).await.unwrap();
        }
    });
    let mut results = Vec::new();
    while let Some(v) = rx.recv().await {
        results.push(v);
    }
    results
}

// Topic 5: Async Utilities
pub async fn delayed_value(ms: u64, value: i32) -> i32 {
    time::sleep(Duration::from_millis(ms)).await;
    value
}
pub async fn with_timeout(ms: u64) -> Result<String, String> {
    match time::timeout(Duration::from_millis(ms), async {
        time::sleep(Duration::from_millis(10)).await;
        "done".to_string()
    })
    .await
    {
        Ok(v) => Ok(v),
        Err(_) => Err("timeout".into()),
    }
}
pub async fn retry_operation(max_retries: u32) -> Result<String, String> {
    for i in 0..max_retries {
        if i == max_retries - 1 {
            return Ok("success".into());
        }
    }
    Err("failed".into())
}

// Topic 6: Streams
use tokio_stream::StreamExt;
use tokio_stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Countdown {
    current: u32,
}

impl Countdown {
    pub fn new(start: u32) -> Self {
        Self { current: start }
    }
}

impl Stream for Countdown {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current == 0 {
            Poll::Ready(None)
        } else {
            let val = self.current;
            self.current -= 1;
            Poll::Ready(Some(val))
        }
    }
}

pub async fn consume_stream(mut stream: impl Stream<Item = u32> + Unpin) -> Vec<u32> {
    let mut results = Vec::new();
    while let Some(val) = stream.next().await {
        results.push(val);
    }
    results
}

pub async fn stream_operations(stream: impl Stream<Item = u32> + Unpin) -> Vec<u32> {
    stream
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 10)
        .collect()
        .await
}
