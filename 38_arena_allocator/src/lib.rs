// ============================================
// Level 10, Project 4: Arena Allocator
// Learn: Custom memory allocation, bump allocation, pools
// ============================================

// ============================================
// Topic 1: Bump Allocator
// Learn: Simple linear allocator, fast allocation, no individual free
// ============================================

pub struct BumpAllocator {
    memory: Vec<u8>,
    offset: usize,
}

impl BumpAllocator {
    pub fn new(capacity: usize) -> Self {
        BumpAllocator { memory: vec![0; capacity], offset: 0 }
    }

    pub fn alloc(&mut self, size: usize) -> Option<usize> {
        if self.offset + size > self.memory.len() { return None; }
        let ptr = self.offset;
        self.offset += size;
        Some(ptr)
    }

    pub fn used(&self) -> usize { self.offset }
    pub fn remaining(&self) -> usize { self.memory.len() - self.offset }
    pub fn capacity(&self) -> usize { self.memory.len() }

    pub fn reset(&mut self) { self.offset = 0; }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> bool {
        if offset + data.len() > self.memory.len() { return false; }
        self.memory[offset..offset+data.len()].copy_from_slice(data);
        true
    }

    pub fn read(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset + len > self.memory.len() { return None; }
        Some(&self.memory[offset..offset+len])
    }
}

// ============================================
// Topic 2: Typed Arena
// Learn: Allocating typed values, alignment
// ============================================

pub struct TypedArena<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> TypedArena<T> {
    pub fn new(capacity: usize) -> Self {
        TypedArena { items: Vec::with_capacity(capacity), capacity }
    }

    pub fn alloc(&mut self, value: T) -> Option<usize> {
        if self.items.len() >= self.capacity { return None; }
        let idx = self.items.len();
        self.items.push(value);
        Some(idx)
    }

    pub fn get(&self, index: usize) -> Option<&T> { self.items.get(index) }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> { self.items.get_mut(index) }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn remaining(&self) -> usize { self.capacity - self.items.len() }
}

// ============================================
// Topic 3: Pool Allocator
// Learn: Fixed-size block allocation with reuse
// ============================================

pub struct PoolAllocator<T: Default + Clone> {
    slots: Vec<Option<T>>,
    free_list: Vec<usize>,
}

impl<T: Default + Clone> PoolAllocator<T> {
    pub fn new(capacity: usize) -> Self {
        let slots = vec![None; capacity];
        let free_list: Vec<usize> = (0..capacity).rev().collect();
        PoolAllocator { slots, free_list }
    }

    pub fn alloc(&mut self, value: T) -> Option<usize> {
        let idx = self.free_list.pop()?;
        self.slots[idx] = Some(value);
        Some(idx)
    }

    pub fn dealloc(&mut self, index: usize) -> Option<T> {
        if index >= self.slots.len() { return None; }
        let item = self.slots[index].take()?;
        self.free_list.push(index);
        Some(item)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.slots.get(index)?.as_ref()
    }

    pub fn available(&self) -> usize { self.free_list.len() }
    pub fn in_use(&self) -> usize { self.slots.len() - self.free_list.len() }
    pub fn capacity(&self) -> usize { self.slots.len() }
}

// ============================================
// Topic 4: Slab Allocator
// Learn: Size-class based allocation
// ============================================

pub struct SlabAllocator {
    small: PoolAllocator<Vec<u8>>,  // <= 64 bytes
    medium: PoolAllocator<Vec<u8>>, // <= 256 bytes
    large: PoolAllocator<Vec<u8>>,  // <= 1024 bytes
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlabClass { Small, Medium, Large }

impl SlabAllocator {
    pub fn new(small_slots: usize, medium_slots: usize, large_slots: usize) -> Self {
        SlabAllocator {
            small: PoolAllocator::new(small_slots),
            medium: PoolAllocator::new(medium_slots),
            large: PoolAllocator::new(large_slots),
        }
    }

    pub fn classify(size: usize) -> Option<SlabClass> {
        if size <= 64 { Some(SlabClass::Small) }
        else if size <= 256 { Some(SlabClass::Medium) }
        else if size <= 1024 { Some(SlabClass::Large) }
        else { None }
    }

    pub fn alloc(&mut self, size: usize) -> Option<(SlabClass, usize)> {
        let class = Self::classify(size)?;
        let data = vec![0u8; size];
        let idx = match class {
            SlabClass::Small => self.small.alloc(data)?,
            SlabClass::Medium => self.medium.alloc(data)?,
            SlabClass::Large => self.large.alloc(data)?,
        };
        Some((class, idx))
    }

    pub fn dealloc(&mut self, class: SlabClass, index: usize) -> bool {
        match class {
            SlabClass::Small => self.small.dealloc(index).is_some(),
            SlabClass::Medium => self.medium.dealloc(index).is_some(),
            SlabClass::Large => self.large.dealloc(index).is_some(),
        }
    }

    pub fn available(&self, class: SlabClass) -> usize {
        match class {
            SlabClass::Small => self.small.available(),
            SlabClass::Medium => self.medium.available(),
            SlabClass::Large => self.large.available(),
        }
    }
}

// ============================================
// Topic 5: Generational Index
// Learn: Use-after-free prevention with generation counters
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenIndex {
    pub index: usize,
    pub generation: u64,
}

struct GenEntry<T> {
    value: Option<T>,
    generation: u64,
}

pub struct GenerationalArena<T> {
    entries: Vec<GenEntry<T>>,
    free_list: Vec<usize>,
    len: usize,
}

impl<T> GenerationalArena<T> {
    pub fn new() -> Self {
        GenerationalArena { entries: Vec::new(), free_list: Vec::new(), len: 0 }
    }

    pub fn insert(&mut self, value: T) -> GenIndex {
        self.len += 1;
        if let Some(idx) = self.free_list.pop() {
            let next_gen = self.entries[idx].generation + 1;
            self.entries[idx] = GenEntry { value: Some(value), generation: next_gen };
            GenIndex { index: idx, generation: next_gen }
        } else {
            let idx = self.entries.len();
            self.entries.push(GenEntry { value: Some(value), generation: 0 });
            GenIndex { index: idx, generation: 0 }
        }
    }

    pub fn remove(&mut self, idx: GenIndex) -> Option<T> {
        let entry = self.entries.get_mut(idx.index)?;
        if entry.generation != idx.generation || entry.value.is_none() { return None; }
        self.len -= 1;
        self.free_list.push(idx.index);
        entry.value.take()
    }

    pub fn get(&self, idx: GenIndex) -> Option<&T> {
        let entry = self.entries.get(idx.index)?;
        if entry.generation != idx.generation { return None; }
        entry.value.as_ref()
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
}

impl<T> Default for GenerationalArena<T> {
    fn default() -> Self { Self::new() }
}

// ============================================
// Topic 6: Arena Stats & Diagnostics
// Learn: Memory usage reporting, fragmentation analysis
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub struct AllocStats {
    pub total_capacity: usize,
    pub used: usize,
    pub free: usize,
    pub allocation_count: usize,
    pub utilization: f64,
}

pub struct TrackedAllocator {
    bump: BumpAllocator,
    allocation_count: usize,
}

impl TrackedAllocator {
    pub fn new(capacity: usize) -> Self {
        TrackedAllocator { bump: BumpAllocator::new(capacity), allocation_count: 0 }
    }

    pub fn alloc(&mut self, size: usize) -> Option<usize> {
        let result = self.bump.alloc(size);
        if result.is_some() { self.allocation_count += 1; }
        result
    }

    pub fn stats(&self) -> AllocStats {
        let cap = self.bump.capacity();
        let used = self.bump.used();
        AllocStats {
            total_capacity: cap,
            used,
            free: cap - used,
            allocation_count: self.allocation_count,
            utilization: if cap == 0 { 0.0 } else { used as f64 / cap as f64 },
        }
    }

    pub fn reset(&mut self) {
        self.bump.reset();
        self.allocation_count = 0;
    }
}
