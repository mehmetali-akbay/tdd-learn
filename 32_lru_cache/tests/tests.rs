use lru_cache::*;

// ===== Topic 1 & 2: LRU Cache Core =====

#[test]
fn test_new_cache() {
    let cache = LruCache::<String, i32>::new(3);
    assert_eq!(cache.capacity(), 3);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn test_put_and_get() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), None);
}

#[test]
fn test_put_update() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("a", 10); // update
    assert_eq!(cache.get(&"a"), Some(&10));
    assert_eq!(cache.len(), 1); // no duplicates
}

#[test]
fn test_eviction() {
    let mut cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    let evicted = cache.put("c", 3);
    assert_eq!(evicted, Some(("a", 1))); // "a" was LRU
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), Some(&3));
}

#[test]
fn test_access_updates_recency() {
    let mut cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.get(&"a"); // access "a", so "b" becomes LRU
    let evicted = cache.put("c", 3);
    assert_eq!(evicted, Some(("b", 2))); // "b" was LRU
}

#[test]
fn test_remove() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    assert_eq!(cache.remove(&"a"), Some(1));
    assert_eq!(cache.len(), 1);
    assert!(!cache.contains(&"a"));
    assert_eq!(cache.remove(&"a"), None); // already gone
}

#[test]
fn test_contains() {
    let mut cache = LruCache::new(3);
    cache.put("x", 42);
    assert!(cache.contains(&"x"));
    assert!(!cache.contains(&"y"));
}

#[test]
fn test_clear() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.clear();
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

// ===== Topic 3: Iteration & Inspection =====

#[test]
fn test_keys_mru_order() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    assert_eq!(cache.keys_mru(), vec![&"c", &"b", &"a"]);
}

#[test]
fn test_keys_after_access() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    cache.get(&"a"); // "a" becomes MRU
    assert_eq!(cache.keys_mru(), vec![&"a", &"c", &"b"]);
}

#[test]
fn test_most_least_recent() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    assert_eq!(cache.most_recent(), Some(&"c"));
    assert_eq!(cache.least_recent(), Some(&"a"));
}

#[test]
fn test_values_mru() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");
    assert_eq!(cache.values_mru(), vec![&"three", &"two", &"one"]);
}

#[test]
fn test_entries_mru() {
    let mut cache = LruCache::new(2);
    cache.put("x", 10);
    cache.put("y", 20);
    let entries = cache.entries_mru();
    assert_eq!(entries, vec![(&"y", &20), (&"x", &10)]);
}

// ===== Topic 4: Resize & Advanced =====

#[test]
fn test_resize_smaller() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    let evicted = cache.resize(1);
    assert_eq!(evicted.len(), 2);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.capacity(), 1);
    assert!(cache.contains(&"c")); // most recent survives
}

#[test]
fn test_resize_larger() {
    let mut cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    let evicted = cache.resize(5);
    assert!(evicted.is_empty());
    assert_eq!(cache.capacity(), 5);
    assert_eq!(cache.len(), 2);
}

#[test]
fn test_get_or_insert() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    let val = cache.get_or_insert("a", || 999);
    assert_eq!(*val, 1); // existing
    let val = cache.get_or_insert("b", || 42);
    assert_eq!(*val, 42); // newly inserted
    assert_eq!(cache.len(), 2);
}

// ===== Topic 5: TrackedCache =====

#[test]
fn test_tracked_hits_misses() {
    let mut cache = TrackedCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.get(&"a"); // hit
    cache.get(&"a"); // hit
    cache.get(&"c"); // miss
    assert_eq!(cache.hits(), 2);
    assert_eq!(cache.misses(), 1);
}

#[test]
fn test_tracked_hit_rate() {
    let mut cache = TrackedCache::new(3);
    cache.put("a", 1);
    cache.get(&"a"); // hit
    cache.get(&"a"); // hit
    cache.get(&"b"); // miss
    assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 0.01);
}

#[test]
fn test_tracked_hit_rate_empty() {
    let cache = TrackedCache::<String, i32>::new(3);
    assert_eq!(cache.hit_rate(), 0.0);
}

#[test]
fn test_tracked_eviction() {
    let mut cache = TrackedCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3); // evicts "a"
    assert_eq!(cache.get(&"a"), None); // miss
    assert_eq!(cache.misses(), 1);
    assert_eq!(cache.len(), 2);
}

// ===== Stress / edge cases =====

#[test]
fn test_single_capacity() {
    let mut cache = LruCache::new(1);
    cache.put("a", 1);
    let evicted = cache.put("b", 2);
    assert_eq!(evicted, Some(("a", 1)));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&"b"), Some(&2));
}

#[test]
fn test_many_entries() {
    let mut cache = LruCache::new(100);
    for i in 0..200 {
        cache.put(i, i * 10);
    }
    assert_eq!(cache.len(), 100);
    // First 100 should be evicted
    assert_eq!(cache.get(&0), None);
    assert_eq!(cache.get(&99), None);
    // Last 100 should exist
    assert_eq!(cache.get(&100), Some(&1000));
    assert_eq!(cache.get(&199), Some(&1990));
}

// ===== Topic 6: Peek & Bulk Operations =====

#[test]
fn test_peek_no_recency_update() {
    let mut cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    // Peek "a" — should NOT move it to front
    assert_eq!(cache.peek(&"a"), Some(&1));
    // "a" should still be LRU
    assert_eq!(cache.least_recent(), Some(&"a"));
    // Put "c" — should evict "a" (still LRU despite peek)
    let evicted = cache.put("c", 3);
    assert_eq!(evicted, Some(("a", 1)));
}

#[test]
fn test_peek_missing() {
    let cache = LruCache::<String, i32>::new(3);
    assert_eq!(cache.peek(&"missing".to_string()), None);
}

#[test]
fn test_retain() {
    let mut cache = LruCache::new(5);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    cache.put("d", 4);
    // Keep only even values
    let removed = cache.retain(|_, v| v % 2 == 0);
    assert_eq!(removed, 2); // "a"(1) and "c"(3) removed
    assert_eq!(cache.len(), 2);
    assert!(cache.contains(&"b"));
    assert!(cache.contains(&"d"));
    assert!(!cache.contains(&"a"));
}

#[test]
fn test_retain_all() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    let removed = cache.retain(|_, _| true); // keep all
    assert_eq!(removed, 0);
    assert_eq!(cache.len(), 2);
}

#[test]
fn test_keys_unordered() {
    let mut cache = LruCache::new(3);
    cache.put("x", 10);
    cache.put("y", 20);
    cache.put("z", 30);
    let mut keys = cache.keys();
    keys.sort();
    assert_eq!(keys, vec![&"x", &"y", &"z"]);
}

#[test]
fn test_keys_empty() {
    let cache = LruCache::<String, i32>::new(3);
    assert!(cache.keys().is_empty());
}
