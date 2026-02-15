// ============================================
// Level 5, Project 2: Threads — Concurrency
// Learn: std::thread, Mutex, Arc, channels, thread safety
// ============================================

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
    let mut results: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    results.sort();
    results
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
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let result = *counter.lock().unwrap();
    result
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
    for handle in handles {
        handle.join().unwrap();
    }
    let mut collected = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    collected.sort(); // order is non-deterministic, so sort for testing
    collected
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
    for val in values {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(val).unwrap();
        });
    }
    drop(tx); // drop the original sender
    let mut results: Vec<String> = rx.iter().collect();
    results.sort();
    results
}

/// Pipeline: producer -> transformer -> collector
pub fn channel_pipeline(numbers: Vec<i32>) -> Vec<i32> {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Producer
    thread::spawn(move || {
        for n in numbers {
            tx1.send(n).unwrap();
        }
    });

    // Transformer: doubles each value
    thread::spawn(move || {
        for n in rx1 {
            tx2.send(n * 2).unwrap();
        }
    });

    // Collector
    rx2.iter().collect()
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
}

/// A thread-safe bounded buffer (producer-consumer)
pub struct BoundedBuffer<T> {
    data: Arc<Mutex<Vec<T>>>,
    capacity: usize,
}

impl<T: Clone> BoundedBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        BoundedBuffer {
            data: Arc::new(Mutex::new(Vec::new())),
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

    /// Try to pop an item.
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
        self.sender.take(); // drop the sender to signal workers
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

// ============================================
// Topic 6: Advanced — Parallel Reduce
// Learn: Divide-and-conquer with threads
// ============================================

/// Parallel sum: split work across N threads
pub fn parallel_sum(numbers: &[i64], num_threads: usize) -> i64 {
    if numbers.is_empty() || num_threads == 0 {
        return 0;
    }
    let chunk_size = numbers.len().div_ceil(num_threads);
    let chunks: Vec<Vec<i64>> = numbers.chunks(chunk_size).map(|c| c.to_vec()).collect();
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<i64>()))
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// Parallel find: search for a value across threads, return first found
pub fn parallel_find(items: &[i32], target: i32, num_threads: usize) -> bool {
    if items.is_empty() || num_threads == 0 {
        return false;
    }
    let chunk_size = items.len().div_ceil(num_threads);
    let chunks: Vec<Vec<i32>> = items.chunks(chunk_size).map(|c| c.to_vec()).collect();
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.contains(&target)))
        .collect();
    handles.into_iter().any(|h| h.join().unwrap())
}

/// Parallel apply: apply a function to each item across threads
pub fn parallel_apply(items: &[i32], f: fn(i32) -> i32, num_threads: usize) -> Vec<i32> {
    if items.is_empty() || num_threads == 0 {
        return vec![];
    }
    let chunk_size = items.len().div_ceil(num_threads);
    let indexed_chunks: Vec<(usize, Vec<i32>)> = items
        .chunks(chunk_size)
        .enumerate()
        .map(|(i, c)| (i, c.to_vec()))
        .collect();
    let handles: Vec<_> = indexed_chunks
        .into_iter()
        .map(|(idx, chunk)| {
            thread::spawn(move || (idx, chunk.into_iter().map(f).collect::<Vec<_>>()))
        })
        .collect();
    let mut results: Vec<(usize, Vec<i32>)> =
        handles.into_iter().map(|h| h.join().unwrap()).collect();
    results.sort_by_key(|(idx, _)| *idx);
    results.into_iter().flat_map(|(_, v)| v).collect()
}
