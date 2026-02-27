// ============================================
// Topic 1: Move Semantics
// Learn: How ownership moves between variables
// ============================================

use std::fmt::format;

/// Take ownership of a String and return its length
/// Example: take_and_measure("hello") => 5
pub fn take_and_measure(s: String) -> usize {
    s.chars().count()
}

/// Take ownership of a String, make it uppercase, and return it
/// Example: take_and_shout("hello") => "HELLO"
pub fn take_and_shout(s: String) -> String {
    s.to_uppercase()
}

/// Create and return the String "Hello from Rust!" (ownership transfers to caller)
pub fn create_greeting() -> String {
    String::from("Hello from Rust!")
}

/// Take a Vec, push an element, and return the Vec
pub fn push_and_return(mut v: Vec<i32>, elem: i32) -> Vec<i32> {
    v.push(elem);
    v
}

/// Swap two String values and return them as a tuple (b, a)
pub fn swap_values(a: String, b: String) -> (String, String) {
    (b, a)
}

// ============================================
// Topic 2: Clone and Copy
// Learn: When to clone, what Copy means
// ============================================

/// Return a clone of the input string
/// Example: clone_string("hello") => "hello"
pub fn clone_string(s: &str) -> String {
    s.to_owned()
}

/// Demonstrate Copy: take two i32s and return their sum
/// i32 implements Copy, so both values are still usable after this
pub fn add_copies(a: i32, b: i32) -> i32 {
    a + b
}

/// Clone a vec and reverse the clone, returning (original_order, reversed)
pub fn clone_and_reverse(v: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut reversed = v.to_owned();
    reversed.reverse();
    let orginal_order = v.to_owned();
    (orginal_order, reversed)
}

/// Take a string slice, create an owned copy, and append " (copy)"
/// Example: make_labeled_copy("original") => "original (copy)"
pub fn make_labeled_copy(s: &str) -> String {
    let mut cpy = s.to_string();
    cpy.push_str(" (copy)");
    cpy
}

/// Double a string: "abc" => "abcabc"
pub fn double_string(s: &str) -> String {
    let mut result = String::with_capacity(2 * s.len());
    result.push_str(s);
    result.push_str(s);
    result
}

// ============================================
// Topic 3: Immutable Borrowing (&T)
// Learn: Borrowing without taking ownership
// ============================================

/// Borrow a string and return its length (don't take ownership)
pub fn borrow_and_count(s: &str) -> usize {
    s.len()
}

/// Borrow a vec and return the sum (don't take ownership)
pub fn borrow_and_sum(v: &[i32]) -> i32 {
    v.iter().sum()
}

/// Borrow a string and check if it contains "rust" (case-insensitive)
/// Example: contains_rust("I love Rust!") => true, contains_rust("Python") => false
pub fn contains_rust(s: &str) -> bool {
    s.to_lowercase().contains("rust")
}

/// Borrow two strings and return a reference to the longer one
/// If equal length, return the first
pub fn longer_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// Borrow a vec and return a new vec with only strictly positive (> 0) elements
/// Example: filter_positive(&[-2, -1, 0, 1, 2, 3]) => [1, 2, 3]
pub fn filter_positive(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|&&e| e > 0).copied().collect()
}

// ============================================
// Topic 4: Mutable Borrowing (&mut T)
// Learn: Borrowing with permission to modify
// ============================================

/// Take a mutable reference to a vec and push a value
pub fn push_value(v: &mut Vec<i32>, value: i32) {
    v.push(value)
}

/// Take a mutable reference to a String and make it uppercase in-place
pub fn make_uppercase(s: &mut String) {
   *s = s.to_uppercase();
    
}

/// Increment all elements in a vec by 1
pub fn increment_all(v: &mut [i32]) {
    for e in v {
        *e += 1;
    }
}

/// Remove all negative numbers from a vec in-place
pub fn remove_negatives(v: &mut Vec<i32>) {
    for i in (0..v.len()).rev() {
        if v[i] < 0 {
            v.remove(i);
        }
    }
    
    // v.retain(|&x| x >=0);
    
}

/// Append " world" to a string via mutable reference
/// Example: "hello" => "hello world"
pub fn append_world(s: &mut String) {
    s.push_str(" world");
}

// ============================================
// Topic 5: Ownership in Functions
// Learn: Passing ownership, borrowing in params, lifetimes intro
// ============================================

/// Take ownership, process the value, return both original and reversed
/// Example: process_and_keep("hello") => ("hello", "olleh")
pub fn process_and_keep(s: String) -> (String, String) {
    let reversed = s.clone().chars().rev().collect();
    (s,reversed)
}

/// Apply a transformation function to each element
pub fn transform_each(v: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    todo!()
}

/// Return a reference to the longer of two string slices
/// Returns the one with more characters; if equal length, behavior is unspecified
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// Take a vec by value, sort it, and return it
pub fn sort_owned(mut v: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Create a new string by joining two slices with a dash
/// Example: join_with_dash("hello", "world") => "hello-world"
pub fn join_with_dash(a: &str, b: &str) -> String {
    todo!()
}

// ============================================
// Topic 6: Advanced Ownership Challenges
// Learn: Structs with ownership, builders, lifetimes
// ============================================

/// A simple stack that owns its elements
/// Exercises: implementing methods on owned data
pub struct Stack {
    elements: Vec<i32>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, value: i32) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!()
    }

    pub fn peek(&self) -> Option<&i32> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }
}

/// A string builder that owns its buffer
/// Exercises: mutable self, consuming self (build), method chaining
pub struct StringBuilder {
    buffer: String,
}

impl Default for StringBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StringBuilder {
    pub fn new() -> Self {
        todo!()
    }

    pub fn append(&mut self, s: &str) -> &mut Self {
        todo!()
    }

    pub fn append_line(&mut self, s: &str) -> &mut Self {
        todo!()
    }

    /// Consumes the builder and returns the final String
    pub fn build(self) -> String {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

/// Take ownership of a Vec<String>, keep only strings starting with prefix, join with ", "
/// Example: filter_and_join(["rust","ruby","python","raku"], "ru") => "rust, ruby"
pub fn filter_and_join(words: Vec<String>, prefix: &str) -> String {
    todo!()
}

/// Chain transformations on an owned String: trim -> lowercase -> replace spaces with hyphens
/// Example: slugify("  Hello World  ") => "hello-world"
pub fn slugify(s: String) -> String {
    todo!()
}

/// Consume a Vec<String> and return a numbered list
/// Example: vec!["a".into(), "b".into()] => "1. a\n2. b"
pub fn numbered_list(items: Vec<String>) -> String {
    todo!()
}

/// Return references to strings that are at least `min_len` characters long
/// Exercises: returning references with lifetime annotations
pub fn filter_refs(v: &[String], min_len: usize) -> Vec<&str> {
    todo!()
}

/// Merge two borrowed slices into a new sorted owned Vec
pub fn merge_borrowed(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

/// Split a string into owned chunks by a separator
pub fn owned_split(s: &str, separator: char) -> Vec<String> {
    todo!()
}

// ============================================
// Topic 7: Extra Practice
// Learn: More ownership and borrowing puzzles
// ============================================

/// Take ownership of two vectors and return a new one with alternating elements.
/// If one is longer, append remaining elements at the end.
/// Example: ownership_interleave([1, 2], [10, 20, 30]) => [1, 10, 2, 20, 30]
pub fn ownership_interleave(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Clone only the elements matching the predicate, demonstrating selective cloning.
pub fn clone_filtered(data: &[String], predicate: fn(&str) -> bool) -> Vec<String> {
    todo!()
}

/// Extract and return the first element, leaving the rest in a new vec.
/// Example: extract_first(["a","b","c"]) => (Some("a"), ["b","c"])
pub fn extract_first(mut items: Vec<String>) -> (Option<String>, Vec<String>) {
    todo!()
}

/// Swap two mutable references — classic borrow checker puzzle.
pub fn swap_pair(a: &mut i32, b: &mut i32) {
    todo!()
}

/// Split a vec into two owned vecs — values matching the predicate go left, rest go right.
/// Example: partition_owned(["apple","hi","banana"], |s| s.len() > 3) => (["apple","banana"], ["hi"])
pub fn partition_owned(
    items: Vec<String>,
    predicate: fn(&str) -> bool,
) -> (Vec<String>, Vec<String>) {
    todo!()
}
