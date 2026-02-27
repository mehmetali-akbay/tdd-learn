// ============================================
// Level 4, Project 1: Enums Deep — Slices, Option & Cow
// Learn: Slices, Option combinators, Cow, enum dispatch
// ============================================

use std::borrow::Cow;

// ============================================
// Topic 1: Slice Basics
// Learn: &[T], &str, indexing, windows, chunks
// ============================================

/// Return the first and last elements of a slice as a tuple.
/// Returns None if the slice is empty.
pub fn first_and_last(items: &[i32]) -> Option<(i32, i32)> {
    todo!()
}

/// Return the sum of a sub-slice from index `start` to `end` (exclusive).
/// If indices are out of bounds, return 0.
pub fn sum_range(items: &[i32], start: usize, end: usize) -> i32 {
    todo!()
}

/// Return all contiguous windows of size `n` from the slice.
/// e.g., windows_of(&[1,2,3,4], 2) => [[1,2],[2,3],[3,4]]
pub fn windows_of(items: &[i32], n: usize) -> Vec<Vec<i32>> {
    todo!()
}

/// Split a string slice into the first word and the rest.
/// e.g., "hello world foo" => Some(("hello", "world foo"))
/// Returns None if the string is empty.
pub fn split_first_word(s: &str) -> Option<(&str, &str)> {
    todo!()
}

/// Check if a slice contains a given sub-slice.
pub fn contains_subslice(haystack: &[i32], needle: &[i32]) -> bool {
    todo!()
}

/// Divide a slice into chunks of `chunk_size`.
pub fn divide_into_chunks(items: &[i32], chunk_size: usize) -> Vec<Vec<i32>> {
    todo!()
}

// ============================================
// Topic 2: Option Combinators
// Learn: map, and_then, unwrap_or_else, filter, zip
// ============================================

/// Parse a string as i32 and double it. Return None if parsing fails.
pub fn parse_and_double(s: &str) -> Option<i32> {
    todo!()
}

/// Try to parse as i32, return `default` if parsing fails.
pub fn parse_or_default(s: &str, default: i32) -> i32 {
    todo!()
}

/// Keep the number only if it is even.
pub fn filter_even(opt: Option<i32>) -> Option<i32> {
    todo!()
}

/// Given two optional values, return their sum, or None if either is None.
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!()
}

/// Find the first even number in the list, then square it.
/// Return None if no even number exists.
pub fn first_even_squared(items: &[i32]) -> Option<i32> {
    todo!()
}

/// Look up a key in a list of (key, value) pairs, return the value.
pub fn lookup<'a>(pairs: &'a [(&str, i32)], key: &str) -> Option<i32> {
    todo!()
}

/// Chain two lookups: find a name by id, then find an age by name.
pub fn lookup_age(id_to_name: &[(i32, &str)], name_to_age: &[(&str, i32)], id: i32) -> Option<i32> {
    todo!()
}

// ============================================
// Topic 3: if let / let...else / matches!
// Learn: Concise control flow beyond basic match
// ============================================

/// Extract the value from Some, or return a default message.
/// Use if-let style.
pub fn describe_option(opt: Option<i32>) -> String {
    todo!()
}

/// Return true if the value is Some and the inner value is positive.
/// Use matches! macro.
pub fn is_some_positive(opt: Option<i32>) -> bool {
    todo!()
}

/// Classify a Result: "ok:<value>", "err:<message>"
pub fn classify_result(r: Result<i32, String>) -> String {
    todo!()
}

/// Extract username from an enum. Return "anonymous" if not logged in.
#[derive(Debug, Clone)]
pub enum User {
    Anonymous,
    LoggedIn { username: String, role: String },
}

pub fn get_username(user: &User) -> &str {
    todo!()
}

/// Return true only if the user is a LoggedIn admin.
pub fn is_admin(user: &User) -> bool {
    todo!()
}

/// If user is `LoggedIn`, format: "User {username} has role {role}".
/// Else return "Guest user".
pub fn process_user(user: &User) -> String {
    todo!()
}

// ============================================
// Topic 4: Nested Enums & Flattening
// Learn: Option<Option<T>>, Result<Option<T>>, flatten
// ============================================

/// Flatten Option<Option<T>> into Option<T>.
pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    todo!()
}

/// Given a Vec of Option<i32>, collect only the Some values.
pub fn collect_somes(items: &[Option<i32>]) -> Vec<i32> {
    todo!()
}

/// Transpose: Option<Result<T, E>> => Result<Option<T>, E>
pub fn transpose_option_result<T, E>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    todo!()
}

/// Find the first parseable number in a list of strings.
pub fn first_parseable(items: &[&str]) -> Option<i32> {
    todo!()
}

/// Try to parse, then check if positive. Combines Result and Option.
pub fn parse_positive(s: &str) -> Result<Option<i32>, String> {
    todo!()
}

// ============================================
// Topic 5: Cow<T> & Borrow/ToOwned
// Learn: Zero-copy patterns, Cow::Borrowed vs Cow::Owned
// ============================================

/// If the string is already lowercase, return it borrowed; otherwise, return owned lowercase.
pub fn to_lowercase_cow(s: &str) -> Cow<str> {
    todo!()
}

/// Remove all spaces. Return borrowed if no spaces, owned otherwise.
pub fn remove_spaces_cow(s: &str) -> Cow<str> {
    todo!()
}

/// Ensure the string ends with a period. Borrow if it already does.
pub fn ensure_period(s: &str) -> Cow<str> {
    todo!()
}

/// Trim whitespace from both ends. Use Cow to avoid allocation when not needed.
pub fn trim_cow(s: &str) -> Cow<str> {
    todo!()
}

/// Prepend a prefix only if the string doesn't start with it.
pub fn ensure_prefix<'a>(s: &'a str, prefix: &str) -> Cow<'a, str> {
    todo!()
}

// ============================================
// Topic 6: Advanced — Enum Dispatch
// Learn: Enum vs trait object dispatch, performance patterns
// ============================================

/// A shape enum that can calculate area without dynamic dispatch.
#[derive(Debug, Clone)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    /// Calculate the area of the shape.
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// Return a human-readable description.
    pub fn describe(&self) -> String {
        todo!()
    }
}

/// Sum areas of all shapes (uses enum dispatch, not trait objects).
pub fn total_area(shapes: &[Shape]) -> f64 {
    todo!()
}

/// Filter shapes larger than a minimum area.
pub fn filter_by_min_area(shapes: &[Shape], min_area: f64) -> Vec<&Shape> {
    todo!()
}

/// Find the shape with the largest area. Returns None if empty.
pub fn largest_shape(shapes: &[Shape]) -> Option<&Shape> {
    todo!()
}

// ============================================
// Tests
// ============================================
