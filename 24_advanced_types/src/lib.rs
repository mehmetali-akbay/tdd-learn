// ============================================
// Level 7, Project 2: Advanced Types
// Learn: Type aliases, newtype, never type, DSTs, fn pointers, PhantomData
// ============================================

use std::fmt;
use std::marker::PhantomData;

// ============================================
// Topic 1: Type Aliases
// Learn: type Result<T> = ..., reducing repetition
// ============================================

/// Custom error type for this module.
#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    NotFound(String),
    InvalidInput(String),
    Unauthorized,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Type alias to avoid repeating Result<T, AppError>.
pub type AppResult<T> = Result<T, AppError>;

/// Look up an item by name. Returns AppResult.
pub fn find_item(items: &[(&str, i32)], name: &str) -> AppResult<i32> {
    todo!()
}

/// Validate and parse input. Returns AppResult.
pub fn parse_age(input: &str) -> AppResult<u32> {
    todo!()
}

/// Process requires auth. Returns AppResult.
pub fn authorized_action(is_authed: bool, action: &str) -> AppResult<String> {
    todo!()
}

// ============================================
// Topic 2: Newtype for Validation
// Learn: Wrapper types enforcing invariants
// ============================================

/// A non-empty string. Cannot be created empty.
#[derive(Debug, Clone, PartialEq)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        todo!()
    }

    pub fn as_str(&self) -> &str {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}

/// A percentage value between 0.0 and 100.0 (inclusive).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(f64);

impl Percentage {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        todo!()
    }

    pub fn value(&self) -> f64 {
        todo!()
    }

    /// Convert to a 0.0-1.0 ratio.
    pub fn as_ratio(&self) -> f64 {
        todo!()
    }
}

/// A port number (1-65535).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Port(u16);

impl Port {
    pub fn new(n: u16) -> Result<Self, &'static str> {
        todo!()
    }

    pub fn value(&self) -> u16 {
        todo!()
    }

    pub fn is_privileged(&self) -> bool {
        todo!()
    }
}

// ============================================
// Topic 3: The Never Type (!)
// Learn: Diverging functions, infinite loops
// ============================================

/// Parse or panic — demonstrates a function that always returns or diverges.
pub fn parse_or_die(s: &str) -> i32 {
    todo!()
}

/// Process a Result where the error case diverges (panics).
pub fn unwrap_or_die<T: fmt::Debug>(result: Result<T, String>) -> T {
    todo!()
}

/// A function that returns a value computed using a match
/// where one arm diverges with `continue`-like behavior.
/// Filters a list, parsing strings to i32 and keeping only Ok values.
pub fn parse_all_or_skip(items: &[&str]) -> Vec<i32> {
    todo!()
}

// ============================================
// Topic 4: DSTs & Sized
// Learn: ?Sized, str vs String, trait objects
// ============================================

/// Print any Display type — works with both sized and unsized types.
pub fn print_it<T: fmt::Display + ?Sized>(item: &T) -> String {
    todo!()
}

/// Get the length of anything that can be a str reference.
pub fn str_len(s: &str) -> usize {
    todo!()
}

/// A generic wrapper that works with ?Sized types.
pub struct Ref<'a, T: ?Sized> {
    pub value: &'a T,
}

impl<'a, T: fmt::Display + ?Sized> Ref<'a, T> {
    pub fn new(value: &'a T) -> Self {
        todo!()
    }

    pub fn display(&self) -> String {
        todo!()
    }
}

// ============================================
// Topic 5: Function Pointers
// Learn: fn vs Fn, passing functions as arguments
// ============================================

/// Apply a function pointer to each element.
pub fn map_with_fn(items: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    todo!()
}

/// Apply different operations based on a string name.
pub fn get_operation(name: &str) -> Option<fn(i32, i32) -> i32> {
    todo!()
}

/// Create a list of functions and apply them in sequence.
pub fn apply_pipeline(value: i32, fns: &[fn(i32) -> i32]) -> i32 {
    todo!()
}

/// Use a closure where fn pointer is expected (closure must not capture).
pub fn transform_strings(items: &[&str], f: fn(&str) -> String) -> Vec<String> {
    todo!()
}

// ============================================
// Topic 6: Advanced — PhantomData & Type-State Pattern
// Learn: Marker types, compile-time state machines
// ============================================

/// Marker types for builder state.
pub struct Unset;
pub struct Set;

/// A type-state builder that requires all fields to be set before building.
pub struct RequestBuilder<UrlState, MethodState> {
    url: Option<String>,
    method: Option<String>,
    body: Option<String>,
    _url: PhantomData<UrlState>,
    _method: PhantomData<MethodState>,
}

impl RequestBuilder<Unset, Unset> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<M> RequestBuilder<Unset, M> {
    pub fn url(self, url: &str) -> RequestBuilder<Set, M> {
        todo!()
    }
}

impl<U> RequestBuilder<U, Unset> {
    pub fn method(self, method: &str) -> RequestBuilder<U, Set> {
        todo!()
    }
}

impl<U, M> RequestBuilder<U, M> {
    pub fn body(mut self, body: &str) -> Self {
        todo!()
    }
}

/// Only buildable when both URL and Method are Set.
impl RequestBuilder<Set, Set> {
    pub fn build(self) -> Request {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
}

// ============================================
// Tests
// ============================================
