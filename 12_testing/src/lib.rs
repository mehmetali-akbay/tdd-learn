// ============================================
// Level 4, Project 4: Testing — Patterns & Organization
// Learn: assert macros, #[should_panic], Result tests, #[ignore], property tests, test helpers
// ============================================

use std::collections::HashSet;
use std::fmt;

// ============================================
// Topic 1: Pure Functions — Targets for Basic Assertions
// Learn: Writing testable pure functions, assert!, assert_eq!, assert_ne!, custom messages
// Reinforces: 01_ownership (values), 06_strings (format!)
// ============================================

/// Add two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtract b from a.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiply two numbers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Check if a number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Check if a number is odd.
pub fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

/// Return the larger of two values.
pub fn max_of(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

/// Return the smaller of two values.
pub fn min_of(a: i32, b: i32) -> i32 {
    if a <= b {
        a
    } else {
        b
    }
}

/// Return the absolute difference between two numbers.
pub fn abs_diff(a: i32, b: i32) -> u32 {
    a.abs_diff(b)
}

/// Clamp a value between min and max.
pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    value.max(min).min(max)
}

/// Greet a person by name.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// ============================================
// Topic 2: Panic & Result — Targets for Error Testing
// Learn: Functions that panic or return Result for testing error paths
// Reinforces: 08_results (Result/Error), 03_patterns (match)
// ============================================

/// Divide two numbers. Panics if divisor is zero.
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("division by zero");
    }
    a / b
}

/// Get element at index. Panics with descriptive message if out of bounds.
pub fn get_element(items: &[i32], index: usize) -> i32 {
    if index >= items.len() {
        panic!("index {} out of bounds for length {}", index, items.len());
    }
    items[index]
}

/// Parse a positive integer from string. Returns Err if invalid or negative.
pub fn parse_positive(s: &str) -> Result<u32, String> {
    let n: i32 = s
        .parse()
        .map_err(|e: std::num::ParseIntError| e.to_string())?;
    if n < 0 {
        return Err("negative number".into());
    }
    Ok(n as u32)
}

/// Validate an email address (simple: must contain @ and .).
pub fn validate_email(email: &str) -> Result<(), String> {
    if !email.contains('@') {
        return Err("missing @".into());
    }
    if !email.contains('.') {
        return Err("missing dot".into());
    }
    Ok(())
}

/// Safe division returning Result instead of panicking.
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("division by zero".into());
    }
    Ok(a / b)
}

/// Square root for non-negative numbers.
pub fn checked_sqrt(n: f64) -> Result<f64, String> {
    if n < 0.0 {
        return Err("negative input".into());
    }
    Ok(n.sqrt())
}

// ============================================
// Topic 3: Struct Testing & Helpers
// Learn: Testing struct methods, helper/factory functions for test data
// Reinforces: 02_structs (methods), 09_traits (Display, Clone, PartialEq), 11_lifetimes (refs)
// ============================================

/// A user for testing struct operations.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub active: bool,
}

impl User {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
            active: true,
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn display_name(&self) -> String {
        format!("{} (age {})", self.name, self.age)
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.active { "active" } else { "inactive" };
        write!(f, "{}, age {}, {}", self.name, self.age, status)
    }
}

/// Filter active adult users.
pub fn active_adults(users: &[User]) -> Vec<&User> {
    users.iter().filter(|u| u.active && u.is_adult()).collect()
}

/// Find user by name (case-insensitive).
pub fn find_user<'a>(users: &'a [User], name: &str) -> Option<&'a User> {
    users.iter().find(|u| u.name.eq_ignore_ascii_case(name))
}

/// Return the oldest user.
pub fn oldest_user(users: &[User]) -> Option<&User> {
    users.iter().max_by_key(|u| u.age)
}

/// Summary: count of (active, inactive) users.
pub fn user_summary(users: &[User]) -> (usize, usize) {
    let active = users.iter().filter(|u| u.active).count();
    (active, users.len() - active)
}

// ============================================
// Topic 4: Expensive Computations — #[ignore] Targets
// Learn: Marking slow tests with #[ignore], running with --include-ignored
// Reinforces: 05_vecs (building vectors), 03_patterns (loops/match)
// ============================================

/// Fibonacci (recursive, intentionally slow for large n).
pub fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

/// Check if a number is prime.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }
    true
}

/// Sieve of Eratosthenes: all primes up to n.
pub fn primes_up_to(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }
    let mut sieve = vec![true; (n + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 2;
    while i * i <= n as usize {
        if sieve[i] {
            let mut j = i * i;
            while j <= n as usize {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    sieve
        .iter()
        .enumerate()
        .filter(|&(_, &is_p)| is_p)
        .map(|(i, _)| i as u64)
        .collect()
}

/// Return the nth prime (0-indexed: nth_prime(0) == 2).
pub fn nth_prime(n: usize) -> u64 {
    let mut count = 0;
    let mut candidate = 2;
    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 1;
    }
}

/// Count Collatz steps from n to 1.
pub fn collatz_steps(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        n = if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

// ============================================
// Topic 5: Data Structures — Boundary & State Testing
// Learn: Testing state transitions, boundary values, empty/full/single cases
// Reinforces: 10_generics (Stack<T>), 05_vecs (Vec ops), 07_hashmaps (HashSet)
// ============================================

/// A capacity-bounded stack for testing state transitions.
pub struct Stack<T> {
    elements: Vec<T>,
    capacity: usize,
}

impl<T> Stack<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            elements: Vec::new(),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.elements.len() >= self.capacity {
            Err("stack overflow")
        } else {
            self.elements.push(item);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.elements.len() >= self.capacity
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }
}

/// Sort a vector (wrapper for testing sort properties).
pub fn sort_vec(mut items: Vec<i32>) -> Vec<i32> {
    items.sort();
    items
}

/// Remove duplicates while maintaining order.
pub fn unique(items: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    items.iter().filter(|x| seen.insert(**x)).copied().collect()
}

/// Reverse a string (for testing round-trip property).
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Split a slice into chunks of `size`.
pub fn chunk_vec(items: &[i32], size: usize) -> Vec<Vec<i32>> {
    if size == 0 {
        return vec![];
    }
    items.chunks(size).map(|c| c.to_vec()).collect()
}

/// Flatten nested vectors into a single vector.
pub fn flatten(nested: &[Vec<i32>]) -> Vec<i32> {
    nested.iter().flat_map(|v| v.iter()).copied().collect()
}

// ============================================
// Topic 6: Roundtrip & Property Testing
// Learn: Encode/decode roundtrips, invariant checking, idempotency
// Reinforces: 06_strings (char manipulation), 08_results (error returns)
// ============================================

/// Caesar cipher encode with given shift.
pub fn caesar_encode(s: &str, shift: u8) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (b'a' + (c as u8 - b'a' + shift) % 26) as char
            } else if c.is_ascii_uppercase() {
                (b'A' + (c as u8 - b'A' + shift) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

/// Caesar cipher decode with given shift.
pub fn caesar_decode(s: &str, shift: u8) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (b'a' + (c as u8 - b'a' + 26 - shift % 26) % 26) as char
            } else if c.is_ascii_uppercase() {
                (b'A' + (c as u8 - b'A' + 26 - shift % 26) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

/// Run-length encode: "aaabbc" → "3a2b1c".
pub fn run_length_encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut chars = s.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1u32;
    for c in chars {
        if c == current {
            count += 1;
        } else {
            result.push_str(&format!("{}{}", count, current));
            current = c;
            count = 1;
        }
    }
    result.push_str(&format!("{}{}", count, current));
    result
}

/// Run-length decode: "3a2b1c" → "aaabbc".
pub fn run_length_decode(s: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut num = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else {
            let count: usize = num
                .parse()
                .map_err(|_| "invalid encoding".to_string())?;
            for _ in 0..count {
                result.push(c);
            }
            num.clear();
        }
    }
    if !num.is_empty() {
        return Err("trailing digits".into());
    }
    Ok(result)
}

/// Convert bytes to hex string.
pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Parse hex string to bytes.
pub fn from_hex_string(hex: &str) -> Result<Vec<u8>, String> {
    if !hex.len().is_multiple_of(2) {
        return Err("odd length".into());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

// ============================================
// Topic 7: Validation & Trait Testing
// Learn: Testing trait implementations, parameterized-style tests, exhaustive cases
// Reinforces: 09_traits (trait defs/impls), 10_generics (dyn Trait), 03_patterns (match)
// ============================================

/// Classify a number as "positive", "negative", or "zero".
pub fn classify_number(n: i32) -> &'static str {
    match n.cmp(&0) {
        std::cmp::Ordering::Greater => "positive",
        std::cmp::Ordering::Less => "negative",
        std::cmp::Ordering::Equal => "zero",
    }
}

/// Classic fizzbuzz.
pub fn fizzbuzz(n: u32) -> String {
    match (n.is_multiple_of(3), n.is_multiple_of(5)) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

/// A validation trait for testing trait implementations.
pub trait Validator {
    type Item;
    fn validate(&self, item: &Self::Item) -> Result<(), String>;
}

/// Validates that a number is within [min, max].
pub struct RangeValidator {
    pub min: i32,
    pub max: i32,
}

impl Validator for RangeValidator {
    type Item = i32;

    fn validate(&self, item: &i32) -> Result<(), String> {
        if *item >= self.min && *item <= self.max {
            Ok(())
        } else {
            Err(format!(
                "{} out of range [{}, {}]",
                item, self.min, self.max
            ))
        }
    }
}

/// Validates that a string's length is within [min_len, max_len].
pub struct LengthValidator {
    pub min_len: usize,
    pub max_len: usize,
}

impl Validator for LengthValidator {
    type Item = String;

    fn validate(&self, item: &String) -> Result<(), String> {
        if item.len() < self.min_len {
            return Err(format!(
                "too short: {} < {}",
                item.len(),
                self.min_len
            ));
        }
        if item.len() > self.max_len {
            return Err(format!(
                "too long: {} > {}",
                item.len(),
                self.max_len
            ));
        }
        Ok(())
    }
}

/// Run multiple validators on an item, collecting all error messages.
pub fn validate_all<T>(item: &T, validators: &[&dyn Validator<Item = T>]) -> Vec<String> {
    validators
        .iter()
        .filter_map(|v| v.validate(item).err())
        .collect()
}