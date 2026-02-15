// ============================================
// Level 10, Project 6: Concurrency Primitives
// Learn: Build locks, barriers, semaphores from atomics
// ============================================

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Condvar};

// ============================================
// Topic 1: SpinLock
// Learn: Busy-wait lock using AtomicBool
// ============================================

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock { locked: AtomicBool::new(false) }
    }

    pub fn lock(&self) {
        while self.locked.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            std::hint::spin_loop();
        }
    }

    pub fn try_lock(&self) -> bool {
        self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok()
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }
}

impl Default for SpinLock {
    fn default() -> Self { Self::new() }
}

// ============================================
// Topic 2: SpinLock-Protected Value
// Learn: Combining spinlock with data
// ============================================

pub struct SpinMutex<T> {
    lock: SpinLock,
    data: std::cell::UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinMutex<T> {}
unsafe impl<T: Send> Sync for SpinMutex<T> {}

impl<T> SpinMutex<T> {
    pub fn new(value: T) -> Self {
        SpinMutex {
            lock: SpinLock::new(),
            data: std::cell::UnsafeCell::new(value),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.lock.lock();
        let result = f(unsafe { &mut *self.data.get() });
        self.lock.unlock();
        result
    }
}

// ============================================
// Topic 3: Barrier
// Learn: Synchronization point for N threads
// ============================================

pub struct Barrier {
    count: usize,
    waiting: Mutex<usize>,
    condvar: Condvar,
}

impl Barrier {
    pub fn new(count: usize) -> Self {
        Barrier {
            count,
            waiting: Mutex::new(0),
            condvar: Condvar::new(),
        }
    }

    /// Wait until all N threads reach this point.
    pub fn wait(&self) {
        let mut waiting = self.waiting.lock().unwrap();
        *waiting += 1;
        if *waiting >= self.count {
            *waiting = 0;
            self.condvar.notify_all();
        } else {
            while *waiting != 0 {
                waiting = self.condvar.wait(waiting).unwrap();
            }
        }
    }

    pub fn count(&self) -> usize { self.count }
}

// ============================================
// Topic 4: Counting Semaphore
// Learn: Limit concurrent access to N permits
// ============================================

pub struct Semaphore {
    permits: Mutex<usize>,
    condvar: Condvar,
    max_permits: usize,
}

impl Semaphore {
    pub fn new(permits: usize) -> Self {
        Semaphore {
            permits: Mutex::new(permits),
            condvar: Condvar::new(),
            max_permits: permits,
        }
    }

    pub fn acquire(&self) {
        let mut permits = self.permits.lock().unwrap();
        while *permits == 0 {
            permits = self.condvar.wait(permits).unwrap();
        }
        *permits -= 1;
    }

    pub fn try_acquire(&self) -> bool {
        let mut permits = self.permits.lock().unwrap();
        if *permits > 0 {
            *permits -= 1;
            true
        } else {
            false
        }
    }

    pub fn release(&self) {
        let mut permits = self.permits.lock().unwrap();
        *permits = (*permits + 1).min(self.max_permits);
        self.condvar.notify_one();
    }

    pub fn available(&self) -> usize {
        *self.permits.lock().unwrap()
    }
}

// ============================================
// Topic 5: Atomic Counter Patterns
// Learn: Advanced atomic patterns, fetch_add, compare_exchange
// ============================================

pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    pub fn new(initial: usize) -> Self {
        AtomicCounter { value: AtomicUsize::new(initial) }
    }

    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    pub fn decrement(&self) -> usize {
        self.value.fetch_sub(1, Ordering::SeqCst)
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    /// Atomically set to new_val if current == expected. Returns true if swapped.
    pub fn compare_and_swap(&self, expected: usize, new_val: usize) -> bool {
        self.value.compare_exchange(expected, new_val, Ordering::SeqCst, Ordering::SeqCst).is_ok()
    }

    /// Atomically update using a function.
    pub fn fetch_update(&self, f: impl Fn(usize) -> usize) -> usize {
        loop {
            let current = self.get();
            let new_val = f(current);
            if self.compare_and_swap(current, new_val) {
                return current;
            }
        }
    }

    pub fn share(&self) -> Arc<AtomicCounter> {
        Arc::new(AtomicCounter::new(self.get()))
    }
}

// ============================================
// Topic 6: Bounded Channel (from scratch)
// Learn: Build mpsc channel using Mutex + Condvar
// ============================================

pub struct BoundedChannel<T> {
    buffer: Mutex<Vec<T>>,
    capacity: usize,
    not_empty: Condvar,
    not_full: Condvar,
}

impl<T> BoundedChannel<T> {
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(BoundedChannel {
            buffer: Mutex::new(Vec::new()),
            capacity,
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        })
    }

    pub fn send(&self, item: T) {
        let mut buf = self.buffer.lock().unwrap();
        while buf.len() >= self.capacity {
            buf = self.not_full.wait(buf).unwrap();
        }
        buf.push(item);
        self.not_empty.notify_one();
    }

    pub fn try_send(&self, item: T) -> Result<(), T> {
        let mut buf = self.buffer.lock().unwrap();
        if buf.len() >= self.capacity {
            Err(item)
        } else {
            buf.push(item);
            self.not_empty.notify_one();
            Ok(())
        }
    }

    pub fn recv(&self) -> T {
        let mut buf = self.buffer.lock().unwrap();
        while buf.is_empty() {
            buf = self.not_empty.wait(buf).unwrap();
        }
        let item = buf.remove(0);
        self.not_full.notify_one();
        item
    }

    pub fn try_recv(&self) -> Option<T> {
        let mut buf = self.buffer.lock().unwrap();
        if buf.is_empty() {
            None
        } else {
            let item = buf.remove(0);
            self.not_full.notify_one();
            Some(item)
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.lock().unwrap().is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
