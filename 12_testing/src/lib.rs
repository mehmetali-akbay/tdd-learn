// ============================================
// Level 4, Project 4: Testing — Patterns & Organization
// ============================================

/// Add two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
/// Check if a number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}
/// Return the larger of two values.
pub fn max_of(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}
/// Greet a person by name.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
/// Clamp a value between min and max.
pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    value.max(min).min(max)
}

/// Divide two numbers. Panics if divisor is zero.
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("division by zero")
    }
    a / b
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

/// Get element at index. Panics with descriptive message if out of bounds.
pub fn get_element(items: &[i32], index: usize) -> i32 {
    if index >= items.len() {
        panic!("index {} out of bounds for length {}", index, items.len());
    }
    items[index]
}

/// Validate an email address (simple: must contain @ and .).
pub fn validate_email(email: &str) -> Result<(), String> {
    if email.contains('@') && email.contains('.') {
        Ok(())
    } else {
        Err("invalid email".into())
    }
}

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
}

/// Filter active adult users.
pub fn active_adults(users: &[User]) -> Vec<&User> {
    users.iter().filter(|u| u.active && u.is_adult()).collect()
}

/// Find user by name (case-insensitive).
pub fn find_user<'a>(users: &'a [User], name: &str) -> Option<&'a User> {
    users.iter().find(|u| u.name.eq_ignore_ascii_case(name))
}

/// Expensive computation — fibonacci recursively (intentionally slow for large n).
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
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// Generate primes up to n using sieve of Eratosthenes.
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

/// A stack implementation for testing strategies.
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
}

/// Sort a vector (wrapper for testing sort properties).
pub fn sort_vec(mut items: Vec<i32>) -> Vec<i32> {
    items.sort();
    items
}

/// Remove duplicates while maintaining order.
pub fn unique(items: &[i32]) -> Vec<i32> {
    let mut seen = std::collections::HashSet::new();
    items.iter().filter(|x| seen.insert(**x)).copied().collect()
}

/// Reverse a string (for testing round-trip property).
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Encode/decode: simple Caesar cipher with shift.
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
