// ============================================
// Level 6, Project 1: LRU Cache
// Learn: Generics, HashMap, doubly-linked list, capacity management
// ============================================

use std::collections::HashMap;
use std::hash::Hash;

// ============================================
// Topic 1: Node & Doubly-Linked List Primitives
// Learn: Raw index-based linked list, Option-based links
// ============================================

/// A node in our doubly-linked list (arena-allocated via Vec)
#[derive(Debug)]
struct Node<K: Clone, V> {
    key: K,
    value: V,
    prev: Option<usize>,
    next: Option<usize>,
}

/// A doubly-linked list backed by a Vec (arena)
#[derive(Debug)]
struct DoublyLinkedList<K: Clone, V> {
    nodes: Vec<Node<K, V>>,
    head: Option<usize>,
    tail: Option<usize>,
    free: Vec<usize>, // recycled indices
}

impl<K: Clone, V> DoublyLinkedList<K, V> {
    fn new() -> Self {
        DoublyLinkedList {
            nodes: Vec::new(),
            head: None,
            tail: None,
            free: Vec::new(),
        }
    }

    /// Insert a new node at the front, return its index
    fn push_front(&mut self, key: K, value: V) -> usize {
        let idx = if let Some(free_idx) = self.free.pop() {
            self.nodes[free_idx] = Node {
                key,
                value,
                prev: None,
                next: self.head,
            };
            free_idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node {
                key,
                value,
                prev: None,
                next: self.head,
            });
            idx
        };

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        }
        self.head = Some(idx);
        if self.tail.is_none() {
            self.tail = Some(idx);
        }
        idx
    }

    /// Remove a node by index
    fn remove(&mut self, idx: usize) -> (K, V) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        match prev {
            Some(p) => self.nodes[p].next = next,
            None => self.head = next,
        }
        match next {
            Some(n) => self.nodes[n].prev = prev,
            None => self.tail = prev,
        }

        self.nodes[idx].prev = None;
        self.nodes[idx].next = None;
        self.free.push(idx);
        (
            self.nodes[idx].key.clone(),
            std::mem::replace(&mut self.nodes[idx].value, unsafe { std::mem::zeroed() }),
        )
    }

    /// Remove and return the tail (least recently used)
    fn pop_back(&mut self) -> Option<(K, V)> {
        self.tail.map(|idx| self.remove(idx))
    }

    /// Move an existing node to the front
    fn move_to_front(&mut self, idx: usize) {
        if self.head == Some(idx) {
            return; // already at front
        }

        // Unlink
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        if let Some(p) = prev {
            self.nodes[p].next = next;
        }
        if let Some(n) = next {
            self.nodes[n].prev = prev;
        }
        if self.tail == Some(idx) {
            self.tail = prev;
        }

        // Link at front
        self.nodes[idx].prev = None;
        self.nodes[idx].next = self.head;
        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        }
        self.head = Some(idx);
    }

    /// Get a reference to a node's value
    fn get(&self, idx: usize) -> &V {
        &self.nodes[idx].value
    }

    /// Get a mutable reference to a node's value
    fn get_mut(&mut self, idx: usize) -> &mut V {
        &mut self.nodes[idx].value
    }

    /// Iterate from head to tail (most recent to least recent)
    fn iter(&self) -> DllIter<'_, K, V> {
        DllIter {
            list: self,
            current: self.head,
        }
    }
}

struct DllIter<'a, K: Clone, V> {
    list: &'a DoublyLinkedList<K, V>,
    current: Option<usize>,
}

impl<'a, K: Clone, V> Iterator for DllIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|idx| {
            let node = &self.list.nodes[idx];
            self.current = node.next;
            (&node.key, &node.value)
        })
    }
}

// ============================================
// Topic 2: LruCache Core — get, put, capacity
// Learn: HashMap + linked list combination
// ============================================

/// An LRU (Least Recently Used) cache
#[derive(Debug)]
pub struct LruCache<K: Clone + Eq + Hash, V> {
    capacity: usize,
    map: HashMap<K, usize>, // key -> node index
    list: DoublyLinkedList<K, V>,
}

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    /// Create a new LRU cache with the given capacity
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        LruCache {
            capacity,
            map: HashMap::new(),
            list: DoublyLinkedList::new(),
        }
    }

    /// Get a reference to a value, marking it as recently used
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(&idx) = self.map.get(key) {
            self.list.move_to_front(idx);
            Some(self.list.get(idx))
        } else {
            None
        }
    }

    /// Insert a key-value pair. Returns the evicted pair if cache is full.
    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        // If key exists, update value and move to front
        if let Some(&idx) = self.map.get(&key) {
            *self.list.get_mut(idx) = value;
            self.list.move_to_front(idx);
            return None;
        }

        // Evict if at capacity
        let evicted = if self.map.len() >= self.capacity {
            let (old_key, old_val) = self.list.pop_back().unwrap();
            self.map.remove(&old_key);
            Some((old_key, old_val))
        } else {
            None
        };

        // Insert new entry
        let idx = self.list.push_front(key.clone(), value);
        self.map.insert(key, idx);
        evicted
    }

    /// Remove a key from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(idx) = self.map.remove(key) {
            let (_, val) = self.list.remove(idx);
            Some(val)
        } else {
            None
        }
    }

    /// Current number of entries
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Cache capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Check if a key exists (without updating recency)
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.map.clear();
        self.list = DoublyLinkedList::new();
    }
}

// ============================================
// Topic 3: Iteration & Inspection
// Learn: Iterating over cache entries in LRU order
// ============================================

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    /// Return keys in order from most recently used to least
    pub fn keys_mru(&self) -> Vec<&K> {
        self.list.iter().map(|(k, _)| k).collect()
    }

    /// Return values in MRU order
    pub fn values_mru(&self) -> Vec<&V> {
        self.list.iter().map(|(_, v)| v).collect()
    }

    /// Return (key, value) pairs in MRU order
    pub fn entries_mru(&self) -> Vec<(&K, &V)> {
        self.list.iter().collect()
    }

    /// Get the most recently used key
    pub fn most_recent(&self) -> Option<&K> {
        self.list.head.map(|idx| &self.list.nodes[idx].key)
    }

    /// Get the least recently used key
    pub fn least_recent(&self) -> Option<&K> {
        self.list.tail.map(|idx| &self.list.nodes[idx].key)
    }
}

// ============================================
// Topic 4: Resize & Advanced Operations
// Learn: Dynamic capacity changes, bulk operations
// ============================================

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    /// Resize the cache. If new capacity is smaller, evicts LRU entries.
    /// Returns the evicted entries.
    pub fn resize(&mut self, new_capacity: usize) -> Vec<(K, V)> {
        assert!(new_capacity > 0, "capacity must be > 0");
        let mut evicted = Vec::new();
        while self.map.len() > new_capacity {
            if let Some((k, v)) = self.list.pop_back() {
                self.map.remove(&k);
                evicted.push((k, v));
            }
        }
        self.capacity = new_capacity;
        evicted
    }

    /// Get or insert: return existing value or compute & insert
    pub fn get_or_insert(&mut self, key: K, default: impl FnOnce() -> V) -> &V {
        if self.map.contains_key(&key) {
            let idx = self.map[&key];
            self.list.move_to_front(idx);
            return self.list.get(idx);
        }
        let value = default();
        self.put(key.clone(), value);
        let idx = self.map[&key];
        self.list.get(idx)
    }
}

// ============================================
// Topic 5: Stats & Metrics
// Learn: Tracking cache performance
// ============================================

/// LRU cache with hit/miss tracking
#[derive(Debug)]
pub struct TrackedCache<K: Clone + Eq + Hash, V> {
    cache: LruCache<K, V>,
    hits: usize,
    misses: usize,
}

impl<K: Clone + Eq + Hash, V> TrackedCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        TrackedCache {
            cache: LruCache::new(capacity),
            hits: 0,
            misses: 0,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.cache.contains(key) {
            self.hits += 1;
            self.cache.get(key)
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        self.cache.put(key, value)
    }

    pub fn hits(&self) -> usize {
        self.hits
    }

    pub fn misses(&self) -> usize {
        self.misses
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

// ============================================
// Topic 6: Peek & Bulk Operations
// Learn: Non-updating reads, bulk filtering
// ============================================

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    /// Peek at a value WITHOUT updating its recency
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|&idx| self.list.get(idx))
    }

    /// Remove all entries that don't match the predicate.
    /// Returns the number of entries removed.
    pub fn retain(&mut self, mut predicate: impl FnMut(&K, &V) -> bool) -> usize {
        let keys_to_remove: Vec<K> = self
            .list
            .iter()
            .filter(|(k, v)| !predicate(k, v))
            .map(|(k, _)| k.clone())
            .collect();
        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.remove(&key);
        }
        count
    }

    /// Return all keys (unordered)
    pub fn keys(&self) -> Vec<&K> {
        self.map.keys().collect()
    }
}
