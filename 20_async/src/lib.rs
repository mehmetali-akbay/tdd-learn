// ============================================
// Level 8, Project 1: Async — Async/Await, Futures & Streams
// Learn: async fn, .await, tokio runtime, join!, select!, channels, streams
// ============================================

use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time;
use tokio_stream::{Stream, StreamExt};

// ============================================
// Topic 1: Future Basics
// Learn: async fn, .await, returning values from futures
// ============================================

/// Basic async greeting
pub async fn async_greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Async sum of a slice
pub async fn async_sum(items: &[i32]) -> i32 {
    items.iter().sum()
}

/// Fetch a simulated value by ID
pub async fn fetch_value(id: u32) -> String {
    format!("value_{}", id)
}

/// Process items sequentially (double each)
pub async fn process_sequential(items: &[i32]) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}

/// Async identity — returns the value unchanged
pub async fn async_identity<T: Send>(value: T) -> T {
    value
}

/// Async string transformation: uppercase
pub async fn async_uppercase(s: &str) -> String {
    s.to_uppercase()
}

/// Async concatenation of two strings
pub async fn async_concat(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

/// Async computation: fibonacci (iterative)
pub async fn async_fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

/// Chain two async operations: fetch then transform
pub async fn fetch_and_transform(id: u32) -> String {
    let val = fetch_value(id).await;
    val.to_uppercase()
}

// ============================================
// Topic 2: Tokio Runtime & Spawning
// Learn: tokio::spawn, JoinHandle, task management
// ============================================

/// Spawn a computation on a tokio task
pub async fn spawn_computation(x: i32) -> i32 {
    let handle = tokio::spawn(async move { x * x });
    handle.await.unwrap()
}

/// Spawn multiple tasks, each computing a square
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

/// Spawn a background task that transforms a string
pub async fn background_work(input: String) -> String {
    let handle = tokio::spawn(async move { input.to_uppercase() });
    handle.await.unwrap()
}

/// Spawn N tasks, each computing f(i), collect results
pub async fn spawn_generate(n: usize) -> Vec<usize> {
    let mut handles = Vec::new();
    for i in 0..n {
        handles.push(tokio::spawn(async move { i * i }));
    }
    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Spawn a task that returns a Result
pub async fn spawn_fallible(succeed: bool) -> Result<String, String> {
    let handle = tokio::spawn(async move {
        if succeed {
            Ok("success".to_string())
        } else {
            Err("failure".to_string())
        }
    });
    handle.await.unwrap()
}

/// Spawn and collect — parallel map using spawn
pub async fn spawn_map(items: Vec<i32>, multiplier: i32) -> Vec<i32> {
    let mut handles = Vec::new();
    for item in items {
        handles.push(tokio::spawn(async move { item * multiplier }));
    }
    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

// ============================================
// Topic 3: Concurrent Futures — join! and concurrency
// Learn: tokio::join!, running multiple futures concurrently
// ============================================

/// Fetch two values concurrently
pub async fn fetch_two(id1: u32, id2: u32) -> (String, String) {
    let (a, b) = tokio::join!(fetch_value(id1), fetch_value(id2));
    (a, b)
}

/// Fetch three values concurrently
pub async fn triple_fetch(a: u32, b: u32, c: u32) -> (String, String, String) {
    let (ra, rb, rc) = tokio::join!(fetch_value(a), fetch_value(b), fetch_value(c));
    (ra, rb, rc)
}

/// Fetch all IDs concurrently using spawn
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

/// Run two async computations and return the sum of their results
pub async fn concurrent_sum(a: i32, b: i32) -> i32 {
    let (ra, rb) = tokio::join!(
        async move { a * a },
        async move { b * b }
    );
    ra + rb
}

/// Concurrently compute multiple fibonacci numbers
pub async fn concurrent_fibs(ns: Vec<u32>) -> Vec<u64> {
    let mut handles = Vec::new();
    for n in ns {
        handles.push(tokio::spawn(async move {
            async_fibonacci(n).await
        }));
    }
    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Race: run two futures, return the first one that completes
pub async fn race_values(ms1: u64, val1: i32, ms2: u64, val2: i32) -> i32 {
    tokio::select! {
        v = async { time::sleep(Duration::from_millis(ms1)).await; val1 } => v,
        v = async { time::sleep(Duration::from_millis(ms2)).await; val2 } => v,
    }
}

// ============================================
// Topic 4: Async Channels — mpsc & oneshot
// Learn: tokio::sync::mpsc, oneshot, async message passing
// ============================================

/// Send messages through an mpsc channel and collect them
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

/// Use oneshot for a single response
pub async fn oneshot_result(input: i32) -> i32 {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        tx.send(input * 2).unwrap();
    });
    rx.await.unwrap()
}

/// Producer-consumer pattern: producer doubles, consumer collects
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

/// Multi-producer: multiple tasks send to one receiver
pub async fn multi_producer(values: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(32);
    for val in values {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(val).await.unwrap();
        });
    }
    drop(tx);
    let mut results = Vec::new();
    while let Some(msg) = rx.recv().await {
        results.push(msg);
    }
    results.sort();
    results
}

/// Channel pipeline: producer -> transformer -> collector
pub async fn channel_pipeline(numbers: Vec<i32>) -> Vec<i32> {
    let (tx1, mut rx1) = mpsc::channel(32);
    let (tx2, mut rx2) = mpsc::channel(32);

    // Producer
    tokio::spawn(async move {
        for n in numbers {
            tx1.send(n).await.unwrap();
        }
    });

    // Transformer: triples each value
    tokio::spawn(async move {
        while let Some(n) = rx1.recv().await {
            tx2.send(n * 3).await.unwrap();
        }
    });

    // Collector
    let mut results = Vec::new();
    while let Some(v) = rx2.recv().await {
        results.push(v);
    }
    results
}

/// Bounded channel back-pressure: returns how many items were sent
pub async fn bounded_channel_test(capacity: usize, items: Vec<i32>) -> Vec<i32> {
    let (tx, mut rx) = mpsc::channel(capacity);

    tokio::spawn(async move {
        for item in items {
            tx.send(item).await.unwrap();
        }
    });

    let mut results = Vec::new();
    while let Some(v) = rx.recv().await {
        results.push(v);
    }
    results
}

/// Request-reply pattern using oneshot channels
pub async fn request_reply(request: String) -> String {
    let (resp_tx, resp_rx) = oneshot::channel();

    tokio::spawn(async move {
        let response = format!("reply: {}", request);
        resp_tx.send(response).unwrap();
    });

    resp_rx.await.unwrap()
}

// ============================================
// Topic 5: Async Utilities — Timing & Error Handling
// Learn: sleep, timeout, retry, interval
// ============================================

/// Return a value after a delay
pub async fn delayed_value(ms: u64, value: i32) -> i32 {
    time::sleep(Duration::from_millis(ms)).await;
    value
}

/// Apply a timeout to an operation
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

/// Retry an operation up to max_retries times
pub async fn retry_operation(max_retries: u32) -> Result<String, String> {
    for i in 0..max_retries {
        if i == max_retries - 1 {
            return Ok("success".into());
        }
    }
    Err("failed".into())
}

/// Timeout wrapper: wrap any async result with a timeout
pub async fn timeout_or_default(ms: u64, delay_ms: u64, default: i32) -> i32 {
    match time::timeout(Duration::from_millis(ms), async {
        time::sleep(Duration::from_millis(delay_ms)).await;
        42
    })
    .await
    {
        Ok(v) => v,
        Err(_) => default,
    }
}

/// Collect values at intervals
pub async fn interval_collect(count: usize, interval_ms: u64) -> Vec<usize> {
    let mut interval = time::interval(Duration::from_millis(interval_ms));
    let mut results = Vec::new();
    for i in 0..count {
        interval.tick().await;
        results.push(i);
    }
    results
}

/// Async retry with a predicate: keep retrying until predicate succeeds
pub async fn retry_until(attempts: &[bool]) -> Result<usize, String> {
    for (i, &success) in attempts.iter().enumerate() {
        if success {
            return Ok(i);
        }
    }
    Err("all attempts failed".to_string())
}

/// Async map with timeout: apply f to each item, skip if too slow
pub async fn map_with_timeout(items: Vec<i32>, timeout_ms: u64) -> Vec<Option<i32>> {
    let mut results = Vec::new();
    for item in items {
        let result = time::timeout(Duration::from_millis(timeout_ms), async move {
            item * 2
        })
        .await;
        results.push(result.ok());
    }
    results
}

// ============================================
// Topic 6: Streams
// Learn: Stream trait, tokio_stream, stream combinators
// ============================================

/// A countdown stream: yields values from start down to 1
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

/// A range stream: yields values from start to end (exclusive)
pub struct RangeStream {
    current: i32,
    end: i32,
}

impl RangeStream {
    pub fn new(start: i32, end: i32) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Stream for RangeStream {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current >= self.end {
            Poll::Ready(None)
        } else {
            let val = self.current;
            self.current += 1;
            Poll::Ready(Some(val))
        }
    }
}

/// Consume a stream into a Vec
pub async fn consume_stream(mut stream: impl Stream<Item = u32> + Unpin) -> Vec<u32> {
    let mut results = Vec::new();
    while let Some(val) = stream.next().await {
        results.push(val);
    }
    results
}

/// Filter even numbers and multiply by 10
pub async fn stream_operations(stream: impl Stream<Item = u32> + Unpin) -> Vec<u32> {
    stream
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 10)
        .collect()
        .await
}

/// Sum all values from a stream
pub async fn stream_sum(stream: impl Stream<Item = i32> + Unpin) -> i32 {
    stream.fold(0, |acc, x| acc + x).await
}

/// Count items in a stream
pub async fn stream_count(stream: impl Stream<Item = i32> + Unpin) -> usize {
    stream.fold(0usize, |acc, _| acc + 1).await
}

/// Collect first N items from a stream
pub async fn stream_take(stream: impl Stream<Item = i32> + Unpin, n: usize) -> Vec<i32> {
    stream.take(n).collect().await
}

/// Skip first N items, then collect the rest
pub async fn stream_skip(stream: impl Stream<Item = i32> + Unpin, n: usize) -> Vec<i32> {
    stream.skip(n).collect().await
}

/// Chain two streams together
pub async fn stream_chain(
    s1: impl Stream<Item = i32> + Unpin,
    s2: impl Stream<Item = i32> + Unpin,
) -> Vec<i32> {
    s1.chain(s2).collect().await
}

// ============================================
// Topic 7: Shared Async State — Mutex & Patterns
// Learn: tokio::sync::Mutex, async-safe shared state, patterns
// ============================================

/// An async-safe counter using tokio::sync::Mutex
pub struct AsyncCounter {
    value: Mutex<i64>,
}

impl AsyncCounter {
    pub fn new(initial: i64) -> Self {
        AsyncCounter {
            value: Mutex::new(initial),
        }
    }

    pub async fn increment(&self) {
        let mut guard = self.value.lock().await;
        *guard += 1;
    }

    pub async fn decrement(&self) {
        let mut guard = self.value.lock().await;
        *guard -= 1;
    }

    pub async fn get(&self) -> i64 {
        *self.value.lock().await
    }

    pub async fn add(&self, amount: i64) {
        let mut guard = self.value.lock().await;
        *guard += amount;
    }

    pub async fn reset(&self, value: i64) {
        let mut guard = self.value.lock().await;
        *guard = value;
    }

    pub async fn swap(&self, new_value: i64) -> i64 {
        let mut guard = self.value.lock().await;
        let old = *guard;
        *guard = new_value;
        old
    }
}

/// An async key-value cache
pub struct AsyncCache {
    data: Mutex<HashMap<String, String>>,
}

impl AsyncCache {
    pub fn new() -> Self {
        AsyncCache {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub async fn insert(&self, key: &str, value: &str) {
        self.data
            .lock()
            .await
            .insert(key.to_string(), value.to_string());
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.data.lock().await.get(key).cloned()
    }

    pub async fn remove(&self, key: &str) -> Option<String> {
        self.data.lock().await.remove(key)
    }

    pub async fn len(&self) -> usize {
        self.data.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.data.lock().await.is_empty()
    }

    pub async fn contains_key(&self, key: &str) -> bool {
        self.data.lock().await.contains_key(key)
    }

    pub async fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.data.lock().await.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub async fn clear(&self) {
        self.data.lock().await.clear();
    }
}

impl Default for AsyncCache {
    fn default() -> Self {
        Self::new()
    }
}

/// An async task queue that processes items in order
pub struct AsyncTaskQueue {
    tx: mpsc::Sender<String>,
    results: Mutex<Vec<String>>,
}

impl AsyncTaskQueue {
    pub fn new() -> (Self, mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel(64);
        (
            AsyncTaskQueue {
                tx,
                results: Mutex::new(Vec::new()),
            },
            rx,
        )
    }

    pub async fn submit(&self, task: &str) {
        self.tx.send(task.to_string()).await.unwrap();
    }

    pub async fn record_result(&self, result: &str) {
        self.results.lock().await.push(result.to_string());
    }

    pub async fn get_results(&self) -> Vec<String> {
        self.results.lock().await.clone()
    }

    pub async fn result_count(&self) -> usize {
        self.results.lock().await.len()
    }
}

/// Process a queue: read from receiver, transform, and return results
pub async fn process_queue(mut rx: mpsc::Receiver<String>) -> Vec<String> {
    let mut results = Vec::new();
    while let Some(task) = rx.recv().await {
        results.push(format!("done: {}", task));
    }
    results
}

/// Async semaphore-like pattern: limit concurrent operations
pub async fn limited_concurrency(items: Vec<i32>, max_concurrent: usize) -> Vec<i32> {
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_concurrent));
    let mut handles = Vec::new();

    for item in items {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        handles.push(tokio::spawn(async move {
            let result = item * 2;
            drop(permit);
            result
        }));
    }

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}
