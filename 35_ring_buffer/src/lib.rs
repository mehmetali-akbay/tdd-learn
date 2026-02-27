// ============================================
// Level 10, Project 1: Ring Buffer
// Learn: Circular buffer, index wrapping, fixed-size data structures
// ============================================

// ============================================
// Topic 1: Core Operations
// Learn: Fixed-size buffer, head/tail pointers, push/pop
// ============================================

pub struct RingBuffer<T> {
    buf: Vec<Option<T>>,
    head: usize, // next read position
    tail: usize, // next write position
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn is_full(&self) -> bool {
        todo!()
    }

    /// Push an item. Returns false if full.
    pub fn push(&mut self, item: T) -> bool {
        todo!()
    }

    /// Pop the oldest item.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
}

// ============================================
// Topic 2: Peek & Access
// Learn: Non-consuming reads, index-based access
// ============================================

impl<T> RingBuffer<T> {
    /// Peek at the front (oldest) without removing.
    pub fn peek_front(&self) -> Option<&T> {
        todo!()
    }

    /// Peek at the back (newest) without removing.
    pub fn peek_back(&self) -> Option<&T> {
        todo!()
    }

    /// Get item at logical index (0 = front/oldest).
    pub fn get(&self, index: usize) -> Option<&T> {
        todo!()
    }
}

// ============================================
// Topic 3: Overwriting Mode
// Learn: Circular overwrite, wrapping behavior
// ============================================

impl<T> RingBuffer<T> {
    /// Push an item, overwriting the oldest if full. Returns the evicted item.
    pub fn push_overwrite(&mut self, item: T) -> Option<T> {
        todo!()
    }
}

// ============================================
// Topic 4: Conversion & Collection
// Learn: Converting buffer to linear collections
// ============================================

impl<T: Clone> RingBuffer<T> {
    /// Collect all items into a Vec in order (front to back).
    pub fn to_vec(&self) -> Vec<T> {
        todo!()
    }
}

impl<T> RingBuffer<T> {
    /// Drain all items, returning them as a Vec.
    pub fn drain(&mut self) -> Vec<T> {
        todo!()
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        todo!()
    }
}

// ============================================
// Topic 5: Search & Query
// Learn: Iterating internal structure for lookups
// ============================================

impl<T: PartialEq> RingBuffer<T> {
    pub fn contains(&self, item: &T) -> bool {
        todo!()
    }

    /// Find the logical index of the first occurrence.
    pub fn position(&self, item: &T) -> Option<usize> {
        todo!()
    }
}

// ============================================
// Topic 6: Advanced — Sliding Window
// Learn: Using ring buffer for sliding window algorithms
// ============================================

impl<T: Clone + std::ops::Add<Output = T> + Default> RingBuffer<T> {
    /// Compute a sliding window sum: push value, maintain window of `capacity`, return current sum.
    pub fn window_sum(&self) -> T {
        todo!()
    }
}

/// Compute moving averages over a stream using a ring buffer.
pub fn moving_average(values: &[f64], window_size: usize) -> Vec<f64> {
        todo!()
}

/// Find the maximum in each sliding window position.
pub fn sliding_window_max(values: &[i32], window_size: usize) -> Vec<i32> {
        todo!()
}
