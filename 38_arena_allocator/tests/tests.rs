use arena_alloc::*;

// ===== Topic 1: Bump Allocator =====

#[test]
fn test_bump_alloc() {
    let mut alloc = BumpAllocator::new(1024);
    let ptr1 = alloc.alloc(100).unwrap();
    let ptr2 = alloc.alloc(200).unwrap();
    assert_eq!(ptr1, 0);
    assert_eq!(ptr2, 100);
    assert_eq!(alloc.used(), 300);
    assert_eq!(alloc.remaining(), 724);
}

#[test]
fn test_bump_full() {
    let mut alloc = BumpAllocator::new(10);
    assert!(alloc.alloc(5).is_some());
    assert!(alloc.alloc(5).is_some());
    assert!(alloc.alloc(1).is_none());
}

#[test]
fn test_bump_reset() {
    let mut alloc = BumpAllocator::new(100);
    alloc.alloc(50);
    alloc.reset();
    assert_eq!(alloc.used(), 0);
    assert_eq!(alloc.remaining(), 100);
}

#[test]
fn test_bump_read_write() {
    let mut alloc = BumpAllocator::new(100);
    let offset = alloc.alloc(4).unwrap();
    alloc.write(offset, &[1, 2, 3, 4]);
    assert_eq!(alloc.read(offset, 4), Some(&[1, 2, 3, 4][..]));
}

// ===== Topic 2: Typed Arena =====

#[test]
fn test_typed_arena() {
    let mut arena = TypedArena::new(10);
    let idx = arena.alloc("hello".to_string()).unwrap();
    assert_eq!(arena.get(idx), Some(&"hello".to_string()));
    assert_eq!(arena.len(), 1);
}

#[test]
fn test_typed_arena_full() {
    let mut arena = TypedArena::new(2);
    arena.alloc(1).unwrap();
    arena.alloc(2).unwrap();
    assert!(arena.alloc(3).is_none());
}

// ===== Topic 3: Pool Allocator =====

#[test]
fn test_pool_alloc_dealloc() {
    let mut pool = PoolAllocator::new(3);
    let idx = pool.alloc("hello".to_string()).unwrap();
    assert_eq!(pool.get(idx), Some(&"hello".to_string()));
    assert_eq!(pool.in_use(), 1);
    pool.dealloc(idx);
    assert_eq!(pool.available(), 3);
}

#[test]
fn test_pool_reuse() {
    let mut pool = PoolAllocator::new(2);
    let a = pool.alloc(1).unwrap();
    pool.dealloc(a);
    let b = pool.alloc(2).unwrap();
    assert_eq!(pool.get(b), Some(&2));
}

// ===== Topic 4: Slab Allocator =====

#[test]
fn test_slab_classify() {
    assert_eq!(SlabAllocator::classify(32), Some(SlabClass::Small));
    assert_eq!(SlabAllocator::classify(128), Some(SlabClass::Medium));
    assert_eq!(SlabAllocator::classify(512), Some(SlabClass::Large));
    assert_eq!(SlabAllocator::classify(2048), None);
}

#[test]
fn test_slab_alloc_dealloc() {
    let mut slab = SlabAllocator::new(4, 4, 4);
    let (class, idx) = slab.alloc(32).unwrap();
    assert_eq!(class, SlabClass::Small);
    assert_eq!(slab.available(SlabClass::Small), 3);
    assert!(slab.dealloc(class, idx));
    assert_eq!(slab.available(SlabClass::Small), 4);
}

// ===== Topic 5: Generational Arena =====

#[test]
fn test_gen_arena_insert_remove() {
    let mut arena = GenerationalArena::new();
    let idx = arena.insert("hello");
    assert_eq!(arena.get(idx), Some(&"hello"));
    assert_eq!(arena.remove(idx), Some("hello"));
    assert_eq!(arena.get(idx), None);
}

#[test]
fn test_gen_arena_stale_index() {
    let mut arena = GenerationalArena::new();
    let idx1 = arena.insert("first");
    arena.remove(idx1);
    let _idx2 = arena.insert("second");
    // idx1 is stale — generation mismatch
    assert_eq!(arena.get(idx1), None);
}

#[test]
fn test_gen_arena_len() {
    let mut arena = GenerationalArena::new();
    assert!(arena.is_empty());
    let a = arena.insert(1);
    let _b = arena.insert(2);
    assert_eq!(arena.len(), 2);
    arena.remove(a);
    assert_eq!(arena.len(), 1);
}

// ===== Topic 6: Tracked Allocator =====

#[test]
fn test_tracked_stats() {
    let mut alloc = TrackedAllocator::new(100);
    alloc.alloc(10);
    alloc.alloc(20);
    let stats = alloc.stats();
    assert_eq!(stats.used, 30);
    assert_eq!(stats.free, 70);
    assert_eq!(stats.allocation_count, 2);
    assert!((stats.utilization - 0.3).abs() < 0.01);
}

#[test]
fn test_tracked_reset() {
    let mut alloc = TrackedAllocator::new(100);
    alloc.alloc(50);
    alloc.reset();
    let stats = alloc.stats();
    assert_eq!(stats.used, 0);
    assert_eq!(stats.allocation_count, 0);
}
