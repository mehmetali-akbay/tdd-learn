// ============================================
// Level 5, Project 2: Threads — Concurrency
// Learn: std::thread, Mutex, Arc, channels, thread safety
// ============================================

use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// ============================================
// Topic 1: Thread Basics — Spawning & Joining
// Learn: thread::spawn, JoinHandle, move closures
// ============================================

/// Spawn a thread that computes the sum of numbers 1..=n
pub fn threaded_sum(n: u64) -> u64 {
    let handle = thread::spawn(move || (1..=n).sum::<u64>());
    handle.join().unwrap()
}

/// Spawn multiple threads, each computing a square, collect results
pub fn parallel_squares(numbers: Vec<i32>) -> Vec<i32> {
    let handles: Vec<_> = numbers
        .into_iter()
        .map(|n| thread::spawn(move || n * n))
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Spawn N threads, each returns its thread index
pub fn thread_ids(n: usize) -> Vec<usize> {
    let handles: Vec<_> = (0..n).map(|i| thread::spawn(move || i)).collect();
    let mut ids: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    ids.sort();
    ids
}

/// Spawn a thread that applies a function and returns the result
pub fn spawn_compute<T, F>(f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let handle = thread::spawn(f);
    handle.join().unwrap()
}

/// Spawn N threads, each computes f(i), collect results in order
pub fn parallel_generate<T, F>(n: usize, f: F) -> Vec<T>
where
    T: Send + 'static,
    F: Fn(usize) -> T + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let handles: Vec<_> = (0..n)
        .map(|i| {
            let f = Arc::clone(&f);
            thread::spawn(move || f(i))
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Get the current thread's name (or "unnamed")
pub fn current_thread_name() -> String {
    thread::current()
        .name()
        .unwrap_or("unnamed")
        .to_string()
}

/// Spawn a named thread and return its name from inside the thread
pub fn spawn_named_thread(name: &str) -> String {
    let builder = thread::Builder::new().name(name.to_string());
    let handle = builder
        .spawn(|| thread::current().name().unwrap_or("unnamed").to_string())
        .unwrap();
    handle.join().unwrap()
}

// ============================================
// Topic 2: Shared State — Mutex
// Learn: Mutex<T>, lock, poisoned mutex
// ============================================

/// Increment a shared counter from multiple threads
pub fn shared_counter(num_threads: usize, increments_per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments_per_thread {
                    *counter.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    *counter.lock().unwrap()
}

/// Collect items from multiple threads into a shared Vec
pub fn parallel_collect(items: Vec<String>) -> Vec<String> {
    let result = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = items
        .into_iter()
        .map(|item| {
            let result = Arc::clone(&result);
            thread::spawn(move || {
                result.lock().unwrap().push(item);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let mut v = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    v.sort();
    v
}

/// Parallel map: apply a function to each item in parallel
pub fn parallel_map<T, U>(items: Vec<T>, f: fn(T) -> U) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
{
    let handles: Vec<_> = items
        .into_iter()
        .map(|item| thread::spawn(move || f(item)))
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Parallel filter: keep only items matching a predicate (in parallel)
pub fn parallel_filter(items: Vec<i32>, predicate: fn(i32) -> bool) -> Vec<i32> {
    let result = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = items
        .into_iter()
        .map(|item| {
            let result = Arc::clone(&result);
            thread::spawn(move || {
                if predicate(item) {
                    result.lock().unwrap().push(item);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let mut v = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    v.sort();
    v
}

/// Shared max: find the maximum value across threads
pub fn parallel_max(items: &[i32], num_threads: usize) -> Option<i32> {
    if items.is_empty() {
        return None;
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i32>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || *chunk.iter().max().unwrap()))
        .collect();
    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    results.into_iter().max()
}

/// Shared min: find the minimum value across threads
pub fn parallel_min(items: &[i32], num_threads: usize) -> Option<i32> {
    if items.is_empty() {
        return None;
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i32>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || *chunk.iter().min().unwrap()))
        .collect();
    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    results.into_iter().min()
}

// ============================================
// Topic 3: Channels — Message Passing
// Learn: mpsc::channel, Sender, Receiver
// ============================================

/// Send numbers through a channel and collect them
pub fn channel_basics(items: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for item in items {
            tx.send(item).unwrap();
        }
    });
    rx.iter().collect()
}

/// Multiple producers: each sends its value
pub fn multi_producer(values: Vec<String>) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let handles: Vec<_> = values
        .into_iter()
        .map(|val| {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(val).unwrap();
            })
        })
        .collect();
    drop(tx);
    for h in handles {
        h.join().unwrap();
    }
    let mut results: Vec<String> = rx.iter().collect();
    results.sort();
    results
}

/// Pipeline: producer -> transformer -> collector
pub fn channel_pipeline(numbers: Vec<i32>) -> Vec<i32> {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Producer: sends numbers
    thread::spawn(move || {
        for n in numbers {
            tx1.send(n).unwrap();
        }
    });

    // Transformer: doubles each number
    thread::spawn(move || {
        for n in rx1 {
            tx2.send(n * 2).unwrap();
        }
    });

    // Collector
    rx2.iter().collect()
}

/// Fan-out: one sender broadcasts to multiple receivers
pub fn channel_fanout(items: Vec<i32>, num_receivers: usize) -> Vec<Vec<i32>> {
    let senders: Vec<_> = (0..num_receivers)
        .map(|_| mpsc::channel::<i32>())
        .collect();

    let (txs, rxs): (Vec<_>, Vec<_>) = senders.into_iter().unzip();

    // Spawn receiver threads
    let handles: Vec<_> = rxs
        .into_iter()
        .map(|rx| thread::spawn(move || rx.iter().collect::<Vec<_>>()))
        .collect();

    // Send each item to all receivers
    for item in items {
        for tx in &txs {
            tx.send(item).unwrap();
        }
    }
    drop(txs);

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Channel-based accumulator: sends values, receives running sums
pub fn channel_accumulate(values: Vec<i64>) -> Vec<i64> {
    let (tx_in, rx_in) = mpsc::channel::<i64>();
    let (tx_out, rx_out) = mpsc::channel::<i64>();

    thread::spawn(move || {
        let mut sum = 0i64;
        for val in rx_in {
            sum += val;
            tx_out.send(sum).unwrap();
        }
    });

    for val in values {
        tx_in.send(val).unwrap();
    }
    drop(tx_in);

    rx_out.iter().collect()
}

/// Bidirectional communication: ping-pong between two threads
pub fn channel_ping_pong(rounds: usize) -> Vec<String> {
    let (tx_a, rx_a) = mpsc::channel::<String>();
    let (tx_b, rx_b) = mpsc::channel::<String>();

    let log = Arc::new(Mutex::new(Vec::new()));
    let log_a = Arc::clone(&log);
    let log_b = Arc::clone(&log);

    // Thread A sends "ping", receives "pong"
    let handle_a = thread::spawn(move || {
        for i in 0..rounds {
            let msg = format!("ping-{}", i);
            log_a.lock().unwrap().push(msg.clone());
            tx_a.send(msg).unwrap();
            if let Ok(reply) = rx_b.recv() {
                log_a.lock().unwrap().push(reply);
            }
        }
    });

    // Thread B receives "ping", sends "pong"
    let handle_b = thread::spawn(move || {
        for i in 0..rounds {
            if let Ok(_msg) = rx_a.recv() {
                let reply = format!("pong-{}", i);
                log_b.lock().unwrap().push(reply.clone());
                tx_b.send(reply).unwrap();
            }
        }
    });

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    Arc::try_unwrap(log).unwrap().into_inner().unwrap()
}

// ============================================
// Topic 4: Arc<Mutex<T>> — Shared Mutable State
// Learn: Combining Arc with Mutex for thread-safe shared state
// ============================================

/// A thread-safe counter
#[derive(Debug)]
pub struct AtomicCounter {
    value: Arc<Mutex<i64>>,
}

impl AtomicCounter {
    pub fn new(initial: i64) -> Self {
        AtomicCounter {
            value: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn increment(&self) {
        *self.value.lock().unwrap() += 1;
    }

    pub fn decrement(&self) {
        *self.value.lock().unwrap() -= 1;
    }

    pub fn get(&self) -> i64 {
        *self.value.lock().unwrap()
    }

    /// Clone the inner Arc for sharing across threads
    pub fn share(&self) -> Self {
        AtomicCounter {
            value: Arc::clone(&self.value),
        }
    }

    /// Add a specific amount
    pub fn add(&self, amount: i64) {
        *self.value.lock().unwrap() += amount;
    }

    /// Reset the counter to a specific value
    pub fn reset(&self, value: i64) {
        *self.value.lock().unwrap() = value;
    }

    /// Swap the value and return the old one
    pub fn swap(&self, new_value: i64) -> i64 {
        let mut guard = self.value.lock().unwrap();
        let old = *guard;
        *guard = new_value;
        old
    }
}

/// A thread-safe bounded buffer (producer-consumer)
pub struct BoundedBuffer<T> {
    data: Arc<Mutex<Vec<T>>>,
    capacity: usize,
}

impl<T: Clone> BoundedBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        BoundedBuffer {
            data: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
            capacity,
        }
    }

    /// Try to push an item. Returns false if at capacity.
    pub fn try_push(&self, item: T) -> bool {
        let mut data = self.data.lock().unwrap();
        if data.len() >= self.capacity {
            false
        } else {
            data.push(item);
            true
        }
    }

    /// Try to pop an item (FIFO).
    pub fn try_pop(&self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        if data.is_empty() {
            None
        } else {
            Some(data.remove(0))
        }
    }

    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.data.lock().unwrap().len() >= self.capacity
    }

    /// Peek at the next item without removing it
    pub fn peek(&self) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.first().cloned()
    }

    /// Clear all items from the buffer
    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }

    /// Get the capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get remaining space
    pub fn remaining(&self) -> usize {
        self.capacity - self.data.lock().unwrap().len()
    }

    /// Clone the inner Arc for sharing across threads
    pub fn share(&self) -> Self {
        BoundedBuffer {
            data: Arc::clone(&self.data),
            capacity: self.capacity,
        }
    }
}

// ============================================
// Topic 5: Thread Pool Pattern
// Learn: Fixed thread pool, work distribution
// ============================================

/// Simple thread pool that executes jobs
pub struct ThreadPool {
    sender: Option<mpsc::Sender<Box<dyn FnOnce() + Send>>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            let handle = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            });
            workers.push(handle);
        }

        ThreadPool {
            sender: Some(sender),
            workers,
        }
    }

    pub fn execute(&self, job: impl FnOnce() + Send + 'static) {
        if let Some(sender) = &self.sender {
            sender.send(Box::new(job)).unwrap();
        }
    }

    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender to signal workers to stop
        self.sender.take();
        // Wait for all workers to finish
        for handle in self.workers.drain(..) {
            handle.join().unwrap();
        }
    }
}

/// Execute a batch of closures using a thread pool and collect results
pub fn pool_map<T, F>(pool_size: usize, tasks: Vec<F>) -> Vec<T>
where
    T: Send + std::fmt::Debug + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let pool = ThreadPool::new(pool_size);
    let results: Vec<Arc<Mutex<Option<T>>>> =
        (0..tasks.len()).map(|_| Arc::new(Mutex::new(None))).collect();

    for (i, task) in tasks.into_iter().enumerate() {
        let slot = Arc::clone(&results[i]);
        pool.execute(move || {
            let result = task();
            *slot.lock().unwrap() = Some(result);
        });
    }
    drop(pool);

    results
        .into_iter()
        .map(|slot| Arc::try_unwrap(slot).unwrap().into_inner().unwrap().unwrap())
        .collect()
}

/// Execute tasks with a pool and return how many completed successfully
pub fn pool_execute_count(pool_size: usize, num_tasks: usize) -> usize {
    let pool = ThreadPool::new(pool_size);
    let count = Arc::new(Mutex::new(0usize));

    for _ in 0..num_tasks {
        let count = Arc::clone(&count);
        pool.execute(move || {
            *count.lock().unwrap() += 1;
        });
    }

    drop(pool);
    *count.lock().unwrap()
}

// ============================================
// Topic 6: Advanced — Parallel Algorithms
// Learn: Divide-and-conquer with threads, parallel reduction
// ============================================

/// Parallel sum: split work across N threads
pub fn parallel_sum(numbers: &[i64], num_threads: usize) -> i64 {
    if numbers.is_empty() {
        return 0;
    }
    let num_threads = num_threads.max(1).min(numbers.len());
    let chunks: Vec<Vec<i64>> = numbers
        .chunks(numbers.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<i64>()))
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// Parallel find: search for a value across threads, return first found
pub fn parallel_find(items: &[i32], target: i32, num_threads: usize) -> bool {
    if items.is_empty() {
        return false;
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i32>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.contains(&target)))
        .collect();

    handles.into_iter().any(|h| h.join().unwrap())
}

/// Parallel apply: apply a function to each item across threads
pub fn parallel_apply(items: &[i32], f: fn(i32) -> i32, num_threads: usize) -> Vec<i32> {
    if items.is_empty() {
        return vec![];
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i32>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.into_iter().map(f).collect::<Vec<_>>()))
        .collect();

    handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

/// Parallel reduce: combine elements using an associative operation
pub fn parallel_reduce(
    items: &[i64],
    identity: i64,
    op: fn(i64, i64) -> i64,
    num_threads: usize,
) -> i64 {
    if items.is_empty() {
        return identity;
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i64>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.into_iter().fold(identity, op)))
        .collect();

    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .fold(identity, op)
}

/// Parallel count: count items matching a predicate
pub fn parallel_count(items: &[i32], predicate: fn(i32) -> bool, num_threads: usize) -> usize {
    if items.is_empty() {
        return 0;
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i32>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.into_iter().filter(|&x| predicate(x)).count()))
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// Parallel any: check if any element satisfies a predicate
pub fn parallel_any(items: &[i32], predicate: fn(i32) -> bool, num_threads: usize) -> bool {
    if items.is_empty() {
        return false;
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i32>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.into_iter().any(predicate)))
        .collect();

    handles.into_iter().any(|h| h.join().unwrap())
}

/// Parallel all: check if all elements satisfy a predicate
pub fn parallel_all(items: &[i32], predicate: fn(i32) -> bool, num_threads: usize) -> bool {
    if items.is_empty() {
        return true;
    }
    let num_threads = num_threads.max(1).min(items.len());
    let chunks: Vec<Vec<i32>> = items
        .chunks(items.len().div_ceil(num_threads))
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.into_iter().all(predicate)))
        .collect();

    handles.into_iter().all(|h| h.join().unwrap())
}

// ============================================
// Topic 7: Thread-Safe Data Structures
// Learn: Building concurrent collections with Arc<Mutex<T>>
// ============================================

/// A thread-safe key-value store
#[derive(Debug)]
pub struct ConcurrentMap<V: Clone> {
    data: Arc<Mutex<HashMap<String, V>>>,
}

impl<V: Clone + Send + 'static> ConcurrentMap<V> {
    pub fn new() -> Self {
        ConcurrentMap {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&self, key: &str, value: V) {
        self.data.lock().unwrap().insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<V> {
        self.data.lock().unwrap().get(key).cloned()
    }

    pub fn remove(&self, key: &str) -> Option<V> {
        self.data.lock().unwrap().remove(key)
    }

    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.data.lock().unwrap().keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.lock().unwrap().contains_key(key)
    }

    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }

    /// Clone the inner Arc for sharing across threads
    pub fn share(&self) -> Self {
        ConcurrentMap {
            data: Arc::clone(&self.data),
        }
    }
}

impl<V: Clone + Send + 'static> Default for ConcurrentMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// A thread-safe log/journal
#[derive(Debug)]
pub struct SharedLog {
    entries: Arc<Mutex<Vec<String>>>,
}

impl SharedLog {
    pub fn new() -> Self {
        SharedLog {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn append(&self, msg: &str) {
        self.entries.lock().unwrap().push(msg.to_string());
    }

    pub fn entries(&self) -> Vec<String> {
        self.entries.lock().unwrap().clone()
    }

    pub fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.lock().unwrap().is_empty()
    }

    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }

    pub fn last(&self) -> Option<String> {
        self.entries.lock().unwrap().last().cloned()
    }

    pub fn contains(&self, substr: &str) -> bool {
        self.entries
            .lock()
            .unwrap()
            .iter()
            .any(|e| e.contains(substr))
    }

    /// Clone the inner Arc for sharing across threads
    pub fn share(&self) -> Self {
        SharedLog {
            entries: Arc::clone(&self.entries),
        }
    }
}

impl Default for SharedLog {
    fn default() -> Self {
        Self::new()
    }
}

/// A thread-safe statistics accumulator
#[derive(Debug)]
pub struct SharedStats {
    data: Arc<Mutex<Vec<f64>>>,
}

impl SharedStats {
    pub fn new() -> Self {
        SharedStats {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, value: f64) {
        self.data.lock().unwrap().push(value);
    }

    pub fn count(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    pub fn sum(&self) -> f64 {
        self.data.lock().unwrap().iter().sum()
    }

    pub fn mean(&self) -> Option<f64> {
        let data = self.data.lock().unwrap();
        if data.is_empty() {
            None
        } else {
            Some(data.iter().sum::<f64>() / data.len() as f64)
        }
    }

    pub fn min(&self) -> Option<f64> {
        let data = self.data.lock().unwrap();
        data.iter().copied().reduce(f64::min)
    }

    pub fn max(&self) -> Option<f64> {
        let data = self.data.lock().unwrap();
        data.iter().copied().reduce(f64::max)
    }

    /// Clone the inner Arc for sharing across threads
    pub fn share(&self) -> Self {
        SharedStats {
            data: Arc::clone(&self.data),
        }
    }
}

impl Default for SharedStats {
    fn default() -> Self {
        Self::new()
    }
}
