// ============================================
// Level 4, Project 1: Enums Deep — Slices, Option & Cow
// Learn: Slices, Option combinators, Cow, enum dispatch
// ============================================

use std::borrow::Cow;

// ============================================
// Topic 1: Slice Basics
// ============================================

/// Return the first and last elements of a slice as a tuple.
/// Returns None if the slice is empty.
pub fn first_and_last(items: &[i32]) -> Option<(i32, i32)> {
    if items.is_empty() {
        None
    } else {
        Some((items[0], items[items.len() - 1]))
    }
}

/// Return the sum of a sub-slice from index `start` to `end` (exclusive).
/// If indices are out of bounds, return 0.
pub fn sum_range(items: &[i32], start: usize, end: usize) -> i32 {
    if start >= items.len() || end > items.len() || start >= end {
        return 0;
    }
    items[start..end].iter().sum()
}

/// Return all contiguous windows of size `n` from the slice.
/// e.g., windows_of(&[1,2,3,4], 2) => [[1,2],[2,3],[3,4]]
pub fn windows_of(items: &[i32], n: usize) -> Vec<Vec<i32>> {
    if n == 0 || n > items.len() {
        return vec![];
    }
    items.windows(n).map(|w| w.to_vec()).collect()
}

/// Split a string slice into the first word and the rest.
/// e.g., "hello world foo" => Some(("hello", "world foo"))
/// Returns None if the string is empty.
pub fn split_first_word(s: &str) -> Option<(&str, &str)> {
    if s.is_empty() {
        return None;
    }
    match s.find(' ') {
        Some(idx) => Some((&s[..idx], &s[idx + 1..])),
        None => Some((s, "")),
    }
}

/// Check if a slice contains a given sub-slice.
pub fn contains_subslice(haystack: &[i32], needle: &[i32]) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}

pub fn divide_into_chunks(items: &[i32], chunk_size: usize) -> Vec<Vec<i32>> {
    if chunk_size == 0 {
        return vec![];
    }
    items.chunks(chunk_size).map(|c| c.to_vec()).collect()
}

// ============================================
// Topic 2: Option Combinators
// ============================================

/// Parse a string as i32 and double it. Return None if parsing fails.
pub fn parse_and_double(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().map(|n| n * 2)
}

pub fn parse_or_default(s: &str, default: i32) -> i32 {
    s.parse::<i32>().unwrap_or_else(|_| default)
}

pub fn filter_even(opt: Option<i32>) -> Option<i32> {
    opt.filter(|&n| n % 2 == 0)
}

/// Given two optional values, return their sum, or None if either is None.
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.zip(b).map(|(x, y)| x + y)
}

/// Find the first even number in the list, then square it.
/// Return None if no even number exists.
pub fn first_even_squared(items: &[i32]) -> Option<i32> {
    items.iter().find(|&&x| x % 2 == 0).map(|&x| x * x)
}

/// Look up a key in a list of (key, value) pairs, return the value.
pub fn lookup<'a>(pairs: &'a [(&str, i32)], key: &str) -> Option<i32> {
    pairs.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// Chain two lookups: find a name by id, then find an age by name.
pub fn lookup_age(id_to_name: &[(i32, &str)], name_to_age: &[(&str, i32)], id: i32) -> Option<i32> {
    id_to_name
        .iter()
        .find(|(i, _)| *i == id)
        .map(|(_, name)| *name)
        .and_then(|name| {
            name_to_age
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, age)| *age)
        })
}

// ============================================
// Topic 3: if let / let...else / matches!
// ============================================

/// Extract the value from Some, or return a default message.
/// Use if-let style.
pub fn describe_option(opt: Option<i32>) -> String {
    if let Some(v) = opt {
        format!("Value: {}", v)
    } else {
        "No value".to_string()
    }
}

/// Return true if the value is Some and the inner value is positive.
/// Use matches! macro.
pub fn is_some_positive(opt: Option<i32>) -> bool {
    matches!(opt, Some(v) if v > 0)
}

/// Classify a Result: "ok:<value>", "err:<message>"
pub fn classify_result(r: Result<i32, String>) -> String {
    match r {
        Ok(v) => format!("ok:{}", v),
        Err(e) => format!("err:{}", e),
    }
}

#[derive(Debug, Clone)]
pub enum User {
    Anonymous,
    LoggedIn { username: String, role: String },
}

pub fn get_username(user: &User) -> &str {
    match user {
        User::LoggedIn { username, .. } => username,
        User::Anonymous => "anonymous",
    }
}

/// Return true only if the user is a LoggedIn admin.
pub fn is_admin(user: &User) -> bool {
    matches!(user, User::LoggedIn { role, .. } if role == "admin")
}

pub fn process_user(user: &User) -> String {
    let User::LoggedIn { username, role } = user else {
        return "Guest user".to_string();
    };
    format!("User {} has role {}", username, role)
}

// ============================================
// Topic 4: Nested Enums & Flattening
// ============================================

/// Flatten Option<Option<T>> into Option<T>.
pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    opt.flatten()
}

/// Given a Vec of Option<i32>, collect only the Some values.
pub fn collect_somes(items: &[Option<i32>]) -> Vec<i32> {
    items.iter().filter_map(|x| *x).collect()
}

/// Transpose: Option<Result<T, E>> => Result<Option<T>, E>
pub fn transpose_option_result<T, E>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    opt.transpose()
}

/// Find the first parseable number in a list of strings.
pub fn first_parseable(items: &[&str]) -> Option<i32> {
    items.iter().find_map(|s| s.parse::<i32>().ok())
}

/// Try to parse, then check if positive. Combines Result and Option.
pub fn parse_positive(s: &str) -> Result<Option<i32>, String> {
    let n = s.parse::<i32>().map_err(|e| e.to_string())?;
    if n > 0 {
        Ok(Some(n))
    } else {
        Ok(None)
    }
}

// ============================================
// Topic 5: Cow<T> & Borrow/ToOwned
// ============================================

/// If the string is already lowercase, return it borrowed; otherwise, return owned lowercase.
pub fn to_lowercase_cow(s: &str) -> Cow<str> {
    if s.chars().all(|c| !c.is_uppercase()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_lowercase())
    }
}

/// Remove all spaces. Return borrowed if no spaces, owned otherwise.
pub fn remove_spaces_cow(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.chars().filter(|c| *c != ' ').collect())
    } else {
        Cow::Borrowed(s)
    }
}

/// Ensure the string ends with a period. Borrow if it already does.
pub fn ensure_period(s: &str) -> Cow<str> {
    if s.ends_with('.') {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{}.", s))
    }
}

/// Trim whitespace from both ends. Use Cow to avoid allocation when not needed.
pub fn trim_cow(s: &str) -> Cow<str> {
    let trimmed = s.trim();
    if trimmed.len() == s.len() {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(trimmed.to_string())
    }
}

/// Prepend a prefix only if the string doesn't start with it.
pub fn ensure_prefix<'a>(s: &'a str, prefix: &str) -> Cow<'a, str> {
    if s.starts_with(prefix) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{}{}", prefix, s))
    }
}

// ============================================
// Topic 6: Advanced — Enum Dispatch
// ============================================

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    /// Calculate the area of the shape.
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(b, h) => 0.5 * b * h,
        }
    }

    /// Return a human-readable description.
    pub fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("Circle with radius {}", r),
            Shape::Rectangle(w, h) => format!("Rectangle {}x{}", w, h),
            Shape::Triangle(b, h) => format!("Triangle base={} height={}", b, h),
        }
    }
}

/// Sum areas of all shapes (uses enum dispatch, not trait objects).
pub fn total_area(shapes: &[Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

/// Filter shapes larger than a minimum area.
pub fn filter_by_min_area(shapes: &[Shape], min_area: f64) -> Vec<&Shape> {
    shapes.iter().filter(|s| s.area() >= min_area).collect()
}

/// Find the shape with the largest area. Returns None if empty.
pub fn largest_shape(shapes: &[Shape]) -> Option<&Shape> {
    shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
}

// ============================================
// Tests
// ============================================
