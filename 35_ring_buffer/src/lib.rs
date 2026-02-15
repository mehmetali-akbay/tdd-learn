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
        assert!(capacity > 0);
        let mut buf = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buf.push(None);
        }
        RingBuffer { buf, head: 0, tail: 0, len: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Push an item. Returns false if full.
    pub fn push(&mut self, item: T) -> bool {
        if self.is_full() {
            return false;
        }
        self.buf[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity();
        self.len += 1;
        true
    }

    /// Pop the oldest item.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.buf[self.head].take();
        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;
        item
    }
}

// ============================================
// Topic 2: Peek & Access
// Learn: Non-consuming reads, index-based access
// ============================================

impl<T> RingBuffer<T> {
    /// Peek at the front (oldest) without removing.
    pub fn peek_front(&self) -> Option<&T> {
        if self.is_empty() { None } else { self.buf[self.head].as_ref() }
    }

    /// Peek at the back (newest) without removing.
    pub fn peek_back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let idx = if self.tail == 0 { self.capacity() - 1 } else { self.tail - 1 };
            self.buf[idx].as_ref()
        }
    }

    /// Get item at logical index (0 = front/oldest).
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let real = (self.head + index) % self.capacity();
        self.buf[real].as_ref()
    }
}

// ============================================
// Topic 3: Overwriting Mode
// Learn: Circular overwrite, wrapping behavior
// ============================================

impl<T> RingBuffer<T> {
    /// Push an item, overwriting the oldest if full. Returns the evicted item.
    pub fn push_overwrite(&mut self, item: T) -> Option<T> {
        if self.is_full() {
            let evicted = self.buf[self.head].take();
            self.buf[self.tail] = Some(item);
            self.head = (self.head + 1) % self.capacity();
            self.tail = (self.tail + 1) % self.capacity();
            evicted
        } else {
            self.push(item);
            None
        }
    }
}

// ============================================
// Topic 4: Conversion & Collection
// Learn: Converting buffer to linear collections
// ============================================

impl<T: Clone> RingBuffer<T> {
    /// Collect all items into a Vec in order (front to back).
    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        for i in 0..self.len {
            let idx = (self.head + i) % self.capacity();
            if let Some(ref item) = self.buf[idx] {
                result.push(item.clone());
            }
        }
        result
    }
}

impl<T> RingBuffer<T> {
    /// Drain all items, returning them as a Vec.
    pub fn drain(&mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        while let Some(item) = self.pop() {
            result.push(item);
        }
        result
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

// ============================================
// Topic 5: Search & Query
// Learn: Iterating internal structure for lookups
// ============================================

impl<T: PartialEq> RingBuffer<T> {
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            let idx = (self.head + i) % self.capacity();
            if self.buf[idx].as_ref() == Some(item) {
                return true;
            }
        }
        false
    }

    /// Find the logical index of the first occurrence.
    pub fn position(&self, item: &T) -> Option<usize> {
        for i in 0..self.len {
            let idx = (self.head + i) % self.capacity();
            if self.buf[idx].as_ref() == Some(item) {
                return Some(i);
            }
        }
        None
    }
}

// ============================================
// Topic 6: Advanced — Sliding Window
// Learn: Using ring buffer for sliding window algorithms
// ============================================

impl<T: Clone + std::ops::Add<Output = T> + Default> RingBuffer<T> {
    /// Compute a sliding window sum: push value, maintain window of `capacity`, return current sum.
    pub fn window_sum(&self) -> T {
        let mut sum = T::default();
        for i in 0..self.len {
            let idx = (self.head + i) % self.capacity();
            if let Some(ref v) = self.buf[idx] {
                sum = sum + v.clone();
            }
        }
        sum
    }
}

/// Compute moving averages over a stream using a ring buffer.
pub fn moving_average(values: &[f64], window_size: usize) -> Vec<f64> {
    let mut buf = RingBuffer::new(window_size);
    let mut result = Vec::new();
    for &v in values {
        buf.push_overwrite(v);
        let sum: f64 = buf.to_vec().iter().sum();
        result.push(sum / buf.len() as f64);
    }
    result
}

/// Find the maximum in each sliding window position.
pub fn sliding_window_max(values: &[i32], window_size: usize) -> Vec<i32> {
    if values.is_empty() || window_size == 0 {
        return vec![];
    }
    let mut result = Vec::new();
    let mut buf = RingBuffer::new(window_size);
    for &v in values {
        buf.push_overwrite(v);
        if let Some(&max) = buf.to_vec().iter().max() {
            result.push(max);
        }
    }
    result
}
