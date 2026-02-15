use concurrency_primitives::*;
use std::sync::Arc;
use std::thread;

// ===== Topic 1: SpinLock =====

#[test]
fn test_spinlock_basic() {
    let lock = SpinLock::new();
    assert!(!lock.is_locked());
    lock.lock();
    assert!(lock.is_locked());
    lock.unlock();
    assert!(!lock.is_locked());
}

#[test]
fn test_spinlock_try_lock() {
    let lock = SpinLock::new();
    assert!(lock.try_lock());
    assert!(!lock.try_lock()); // already locked
    lock.unlock();
    assert!(lock.try_lock());
    lock.unlock();
}

// ===== Topic 2: SpinMutex =====

#[test]
fn test_spin_mutex() {
    let mutex = Arc::new(SpinMutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let m = Arc::clone(&mutex);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                m.with_lock(|v| *v += 1);
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    mutex.with_lock(|v| assert_eq!(*v, 1000));
}

// ===== Topic 3: Barrier =====

#[test]
fn test_barrier() {
    let barrier = Arc::new(Barrier::new(5));
    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let b = Arc::clone(&barrier);
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            b.wait();
            // After barrier, all 5 threads have incremented
            assert_eq!(c.load(std::sync::atomic::Ordering::SeqCst), 5);
        }));
    }
    for h in handles { h.join().unwrap(); }
}

#[test]
fn test_barrier_count() {
    let b = Barrier::new(3);
    assert_eq!(b.count(), 3);
}

// ===== Topic 4: Semaphore =====

#[test]
fn test_semaphore_basic() {
    let sem = Semaphore::new(2);
    assert_eq!(sem.available(), 2);
    sem.acquire();
    assert_eq!(sem.available(), 1);
    sem.acquire();
    assert_eq!(sem.available(), 0);
    assert!(!sem.try_acquire());
    sem.release();
    assert_eq!(sem.available(), 1);
}

#[test]
fn test_semaphore_try_acquire() {
    let sem = Semaphore::new(1);
    assert!(sem.try_acquire());
    assert!(!sem.try_acquire());
    sem.release();
    assert!(sem.try_acquire());
    sem.release();
}

// ===== Topic 5: Atomic Counter =====

#[test]
fn test_atomic_counter() {
    let counter = AtomicCounter::new(0);
    counter.increment();
    counter.increment();
    counter.increment();
    assert_eq!(counter.get(), 3);
    counter.decrement();
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_compare_and_swap() {
    let counter = AtomicCounter::new(10);
    assert!(counter.compare_and_swap(10, 20));
    assert_eq!(counter.get(), 20);
    assert!(!counter.compare_and_swap(10, 30)); // fails: current is 20
}

#[test]
fn test_fetch_update() {
    let counter = AtomicCounter::new(5);
    let old = counter.fetch_update(|v| v * 2);
    assert_eq!(old, 5);
    assert_eq!(counter.get(), 10);
}

#[test]
fn test_atomic_counter_threaded() {
    let counter = Arc::new(AtomicCounter::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                c.increment();
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    assert_eq!(counter.get(), 1000);
}

// ===== Topic 6: Bounded Channel =====

#[test]
fn test_channel_send_recv() {
    let ch = BoundedChannel::new(4);
    ch.send(1);
    ch.send(2);
    ch.send(3);
    assert_eq!(ch.len(), 3);
    assert_eq!(ch.recv(), 1);
    assert_eq!(ch.recv(), 2);
    assert_eq!(ch.recv(), 3);
}

#[test]
fn test_channel_try_ops() {
    let ch = BoundedChannel::new(1);
    assert!(ch.try_send(42).is_ok());
    assert!(ch.try_send(99).is_err()); // full
    assert_eq!(ch.try_recv(), Some(42));
    assert_eq!(ch.try_recv(), None);
}

#[test]
fn test_channel_threaded() {
    let ch = BoundedChannel::new(16);
    let ch_send = Arc::clone(&ch);
    let sender = thread::spawn(move || {
        for i in 0..10 {
            ch_send.send(i);
        }
    });
    let mut received = vec![];
    sender.join().unwrap();
    while let Some(v) = ch.try_recv() {
        received.push(v);
    }
    assert_eq!(received, (0..10).collect::<Vec<_>>());
}

#[test]
fn test_channel_capacity() {
    let ch = BoundedChannel::<i32>::new(5);
    assert_eq!(ch.capacity(), 5);
    assert!(ch.is_empty());
}
