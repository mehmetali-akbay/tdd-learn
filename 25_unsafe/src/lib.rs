// ============================================
// Level 7, Project 4: Unsafe Rust
// ============================================
use std::sync::atomic::{AtomicU64, Ordering};

// Topic 1: Raw Pointers
/// Create a raw const pointer from a reference, dereference it safely.
pub fn read_via_raw_pointer(value: &i32) -> i32 {
    let ptr = value as *const i32;
    unsafe { *ptr }
}
/// Create a raw mutable pointer and modify the value.
pub fn modify_via_raw_pointer(value: &mut i32, new_value: i32) {
    let ptr = value as *mut i32;
    unsafe {
        *ptr = new_value;
    }
}
/// Get the memory address of a value as a usize.
pub fn address_of(value: &i32) -> usize {
    value as *const i32 as usize
}
/// Check if two references point to the same memory location.
pub fn same_address<T>(a: &T, b: &T) -> bool {
    std::ptr::eq(a, b)
}

// Topic 2: Unsafe Functions
pub unsafe fn read_ptr(ptr: *const i32) -> i32 {
    *ptr
}
pub unsafe fn write_ptr(ptr: *mut i32, value: i32) {
    *ptr = value;
}
pub unsafe fn swap_raw(a: *mut i32, b: *mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}
/// Safe wrapper around swap_raw.
pub fn safe_swap(a: &mut i32, b: &mut i32) {
    unsafe {
        swap_raw(a as *mut i32, b as *mut i32);
    }
}

// Topic 3: Dereferencing Raw Pointers
/// Find the index of a value in a slice using raw pointer arithmetic.
pub fn find_index(slice: &[i32], target: i32) -> Option<usize> {
    let ptr = slice.as_ptr();
    for i in 0..slice.len() {
        unsafe {
            if *ptr.add(i) == target {
                return Some(i);
            }
        }
    }
    None
}
/// Sum all elements in a slice using raw pointer iteration.
pub fn sum_with_pointers(slice: &[i32]) -> i32 {
    let mut sum = 0;
    let ptr = slice.as_ptr();
    for i in 0..slice.len() {
        unsafe {
            sum += *ptr.add(i);
        }
    }
    sum
}
/// Copy elements from one slice to another using raw pointers.
/// Returns the number of elements copied (min of both lengths).
pub fn copy_elements(src: &[i32], dst: &mut [i32]) -> usize {
    let count = src.len().min(dst.len());
    let src_ptr = src.as_ptr();
    let dst_ptr = dst.as_mut_ptr();
    for i in 0..count {
        unsafe {
            *dst_ptr.add(i) = *src_ptr.add(i);
        }
    }
    count
}

// Topic 4: Static Variables
static COUNTER: AtomicU64 = AtomicU64::new(0);
/// Increment the global counter and return new value.
pub fn increment_counter() -> u64 {
    COUNTER.fetch_add(1, Ordering::SeqCst) + 1
}
/// Get the current counter value.
pub fn get_counter() -> u64 {
    COUNTER.load(Ordering::SeqCst)
}
/// Reset the counter to zero.
pub fn reset_counter() {
    COUNTER.store(0, Ordering::SeqCst);
}
static MAX_RETRIES: AtomicU64 = AtomicU64::new(3);
pub fn get_max_retries() -> u64 {
    MAX_RETRIES.load(Ordering::SeqCst)
}
pub fn set_max_retries(n: u64) {
    MAX_RETRIES.store(n, Ordering::SeqCst);
}

// Topic 5: Send & Sync
#[derive(Debug)]
pub struct SendableData {
    pub value: String,
}
impl SendableData {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
    /// Get a reference to a value by index.
    pub fn get(&self) -> &str {
        &self.value
    }
}
fn _assert_send<T: Send>() {}
fn _assert_sync<T: Sync>() {}
/// Demonstrate that standard types implement Send + Sync.
pub fn check_send_sync() -> bool {
    _assert_send::<SendableData>();
    _assert_send::<String>();
    _assert_send::<Vec<i32>>();
    _assert_sync::<i32>();
    true
}

/// A wrapper that is explicitly NOT Sync (contains raw pointer).
/// This demonstrates why some types aren't thread-safe.
pub struct NotSyncWrapper {
    data: *mut i32,
    _owned: Box<i32>,
}
impl NotSyncWrapper {
    pub fn new(value: i32) -> Self {
        let mut b = Box::new(value);
        let ptr = &mut *b as *mut i32;
        Self {
            data: ptr,
            _owned: b,
        }
    }
    /// Get a reference to a value by index.
    pub fn get(&self) -> i32 {
        unsafe { *self.data }
    }
}

// Topic 6: Safe Abstractions
/// Split a mutable slice into two non-overlapping mutable slices.
/// This is the classic example from the Rust Book.
pub fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// A simple arena allocator that hands out references to allocated values.
pub struct Arena {
    storage: Vec<i32>,
}
impl Arena {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
    /// Allocate a value and return its index.
    pub fn alloc(&mut self, value: i32) -> usize {
        let idx = self.storage.len();
        self.storage.push(value);
        idx
    }
    /// Get a reference to a value by index.
    pub fn get(&self, index: usize) -> Option<&i32> {
        self.storage.get(index)
    }
    /// Get all allocated values.
    pub fn all(&self) -> &[i32] {
        &self.storage
    }
    pub fn len(&self) -> usize {
        self.storage.len()
    }
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}
