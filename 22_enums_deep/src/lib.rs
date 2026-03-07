// ============================================
// Level 4, Project 1: Enums Deep — Slices, Option & Cow
// Learn: Slices, Option combinators, Cow, enum dispatch
// ============================================

use std::borrow::Cow;

// ============================================
// Topic 1: Slice Basics
// Learn: &[T], &str, indexing, windows, chunks, sub-slices
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
pub fn windows_of(items: &[i32], n: usize) -> Vec<Vec<i32>> {
    if n == 0 || n > items.len() {
        return vec![];
    }
    items.windows(n).map(|w| w.to_vec()).collect()
}

/// Split a string slice into the first word and the rest.
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

/// Divide a slice into chunks of a given size.
pub fn divide_into_chunks(items: &[i32], chunk_size: usize) -> Vec<Vec<i32>> {
    if chunk_size == 0 {
        return vec![];
    }
    items.chunks(chunk_size).map(|c| c.to_vec()).collect()
}

/// Return the middle element(s) of a slice.
/// For odd length, return a single element; for even, return the two middle elements.
pub fn middle_elements(items: &[i32]) -> Option<Vec<i32>> {
    if items.is_empty() {
        return None;
    }
    let mid = items.len() / 2;
    if items.len() % 2 == 1 {
        Some(vec![items[mid]])
    } else {
        Some(vec![items[mid - 1], items[mid]])
    }
}

/// Rotate a slice left by `n` positions, returning a new Vec.
pub fn rotate_left(items: &[i32], n: usize) -> Vec<i32> {
    if items.is_empty() {
        return vec![];
    }
    let n = n % items.len();
    let mut result = items[n..].to_vec();
    result.extend_from_slice(&items[..n]);
    result
}

/// Return true if the slice is sorted in ascending order.
pub fn is_sorted(items: &[i32]) -> bool {
    items.windows(2).all(|w| w[0] <= w[1])
}

/// Deduplicate consecutive equal elements.
pub fn dedup_consecutive(items: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &item in items {
        if result.last() != Some(&item) {
            result.push(item);
        }
    }
    result
}

// ============================================
// Topic 2: Option Combinators
// Learn: map, and_then, unwrap_or, filter, zip, flatten
// ============================================

/// Parse a string as i32 and double it. Return None if parsing fails.
pub fn parse_and_double(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().map(|n| n * 2)
}

/// Parse or return a default.
pub fn parse_or_default(s: &str, default: i32) -> i32 {
    s.parse::<i32>().unwrap_or(default)
}

/// Filter: keep only even values.
pub fn filter_even(opt: Option<i32>) -> Option<i32> {
    opt.filter(|&n| n % 2 == 0)
}

/// Given two optional values, return their sum, or None if either is None.
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.zip(b).map(|(x, y)| x + y)
}

/// Find the first even number in the list, then square it.
pub fn first_even_squared(items: &[i32]) -> Option<i32> {
    items.iter().find(|&&x| x % 2 == 0).map(|&x| x * x)
}

/// Look up a key in a list of (key, value) pairs.
pub fn lookup(pairs: &[(&str, i32)], key: &str) -> Option<i32> {
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

/// Multiply option value by a factor, return None if None or result is zero.
pub fn option_multiply_nonzero(opt: Option<i32>, factor: i32) -> Option<i32> {
    opt.map(|v| v * factor).filter(|&v| v != 0)
}

/// Get the max of two options, where None is treated as less than any value.
pub fn option_max(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.max(y)),
        (Some(x), None) | (None, Some(x)) => Some(x),
        (None, None) => None,
    }
}

/// Convert Option<&str> to Option<String> by mapping.
pub fn option_to_owned(opt: Option<&str>) -> Option<String> {
    opt.map(|s| s.to_string())
}

// ============================================
// Topic 3: if let / let...else / matches!
// Learn: Concise control flow beyond basic match
// ============================================

/// Extract the value from Some, or return a default message.
pub fn describe_option(opt: Option<i32>) -> String {
    if let Some(v) = opt {
        format!("Value: {}", v)
    } else {
        "No value".to_string()
    }
}

/// Return true if the value is Some and the inner value is positive.
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

#[derive(Debug, Clone, PartialEq)]
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

/// Process user information using let...else.
pub fn process_user(user: &User) -> String {
    let User::LoggedIn { username, role } = user else {
        return "Guest user".to_string();
    };
    format!("User {} has role {}", username, role)
}

/// Check if a value matches a specific range.
pub fn is_in_range(value: i32, min: i32, max: i32) -> bool {
    matches!(value, v if v >= min && v <= max)
}

/// Classify a character type using matches!
pub fn char_type(c: char) -> &'static str {
    if c.is_ascii_alphabetic() {
        "letter"
    } else if c.is_ascii_digit() {
        "digit"
    } else {
        "other"
    }
}

// ============================================
// Topic 4: Nested Enums & Flattening
// Learn: Option<Option<T>>, Result<Option<T>>, flatten, transpose
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

/// Sum all parseable numbers from a list of strings.
pub fn sum_parseable(items: &[&str]) -> i32 {
    items.iter().filter_map(|s| s.parse::<i32>().ok()).sum()
}

/// Partition options into (somes, none_count).
pub fn partition_options(items: &[Option<i32>]) -> (Vec<i32>, usize) {
    let mut somes = Vec::new();
    let mut none_count = 0;
    for item in items {
        match item {
            Some(v) => somes.push(*v),
            None => none_count += 1,
        }
    }
    (somes, none_count)
}

/// Chain of optional operations: parse -> double -> make positive.
pub fn parse_double_positive(s: &str) -> Option<i32> {
    s.parse::<i32>()
        .ok()
        .map(|n| n * 2)
        .filter(|&n| n > 0)
}

// ============================================
// Topic 5: Cow<T> & Borrow/ToOwned
// Learn: Zero-copy patterns, Cow::Borrowed vs Cow::Owned
// ============================================

/// If the string is already lowercase, return it borrowed; otherwise, return owned lowercase.
pub fn to_lowercase_cow(s: &str) -> Cow<'_, str> {
    if s.chars().all(|c| !c.is_uppercase()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_lowercase())
    }
}

/// Remove all spaces. Return borrowed if no spaces, owned otherwise.
pub fn remove_spaces_cow(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.chars().filter(|c| *c != ' ').collect())
    } else {
        Cow::Borrowed(s)
    }
}

/// Ensure the string ends with a period. Borrow if it already does.
pub fn ensure_period(s: &str) -> Cow<'_, str> {
    if s.ends_with('.') {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{}.", s))
    }
}

/// Trim whitespace from both ends. Use Cow to avoid allocation when not needed.
pub fn trim_cow(s: &str) -> Cow<'_, str> {
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

/// Ensure a suffix is present on the string.
pub fn ensure_suffix<'a>(s: &'a str, suffix: &str) -> Cow<'a, str> {
    if s.ends_with(suffix) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{}{}", s, suffix))
    }
}

/// Replace a substring only if it exists (avoiding allocation when not needed).
pub fn replace_cow<'a>(s: &'a str, from: &str, to: &str) -> Cow<'a, str> {
    if s.contains(from) {
        Cow::Owned(s.replace(from, to))
    } else {
        Cow::Borrowed(s)
    }
}

/// Truncate string to max_len characters, borrowing if already short enough.
pub fn truncate_cow(s: &str, max_len: usize) -> Cow<'_, str> {
    if s.len() <= max_len {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.chars().take(max_len).collect())
    }
}

// ============================================
// Topic 6: Enum Dispatch
// Learn: Enum vs trait object dispatch, pattern matching, methods
// ============================================

#[derive(Debug, Clone, PartialEq)]
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

    /// Calculate the perimeter.
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(w, h) => 2.0 * (w + h),
            Shape::Triangle(b, h) => {
                // Isoceles triangle with given base and height
                let side = ((b / 2.0).powi(2) + h.powi(2)).sqrt();
                b + 2.0 * side
            }
        }
    }

    /// Check if this shape is a circle.
    pub fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle(_))
    }

    /// Scale the shape by a factor.
    pub fn scale(&self, factor: f64) -> Shape {
        match self {
            Shape::Circle(r) => Shape::Circle(r * factor),
            Shape::Rectangle(w, h) => Shape::Rectangle(w * factor, h * factor),
            Shape::Triangle(b, h) => Shape::Triangle(b * factor, h * factor),
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

/// Count shapes by variant.
pub fn count_circles(shapes: &[Shape]) -> usize {
    shapes.iter().filter(|s| s.is_circle()).count()
}

/// Sort shapes by area ascending, return as new Vec.
pub fn sort_by_area(shapes: &[Shape]) -> Vec<Shape> {
    let mut sorted = shapes.to_vec();
    sorted.sort_by(|a, b| a.area().partial_cmp(&b.area()).unwrap());
    sorted
}

// ============================================
// Topic 7: Rich Enums with Data
// Learn: Enums carrying different data in each variant, methods
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

#[allow(clippy::should_implement_trait)]
impl Expr {
    /// Evaluate the expression.
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
            Expr::Neg(e) => -e.eval(),
        }
    }

    /// Count the number of nodes in the expression tree.
    pub fn node_count(&self) -> usize {
        match self {
            Expr::Num(_) => 1,
            Expr::Add(a, b) | Expr::Mul(a, b) => 1 + a.node_count() + b.node_count(),
            Expr::Neg(e) => 1 + e.node_count(),
        }
    }

    /// Return the depth of the expression tree.
    pub fn depth(&self) -> usize {
        match self {
            Expr::Num(_) => 1,
            Expr::Add(a, b) | Expr::Mul(a, b) => 1 + a.depth().max(b.depth()),
            Expr::Neg(e) => 1 + e.depth(),
        }
    }

    /// Check if the expression contains a negation.
    pub fn has_negation(&self) -> bool {
        match self {
            Expr::Num(_) => false,
            Expr::Add(a, b) | Expr::Mul(a, b) => a.has_negation() || b.has_negation(),
            Expr::Neg(_) => true,
        }
    }

    /// Convenience constructors.
    pub fn num(n: f64) -> Self {
        Expr::Num(n)
    }

    pub fn add(a: Expr, b: Expr) -> Self {
        Expr::Add(Box::new(a), Box::new(b))
    }

    pub fn mul(a: Expr, b: Expr) -> Self {
        Expr::Mul(Box::new(a), Box::new(b))
    }

    pub fn neg(e: Expr) -> Self {
        Expr::Neg(Box::new(e))
    }
}

// ============================================
// Topic 8: State Machines with Enums
// Learn: Modeling state transitions without trait objects
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// Advance to the next state.
    pub fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    /// Return the duration in seconds for this state.
    pub fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    /// Can vehicles go?
    pub fn can_go(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }

    /// Return the color name.
    pub fn color(&self) -> &str {
        match self {
            TrafficLight::Red => "red",
            TrafficLight::Yellow => "yellow",
            TrafficLight::Green => "green",
        }
    }
}

/// Advance a traffic light `n` steps.
pub fn advance_n(light: &TrafficLight, n: usize) -> TrafficLight {
    let mut current = light.clone();
    for _ in 0..n {
        current = current.next();
    }
    current
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped { tracking: String },
    Delivered,
    Cancelled { reason: String },
}

impl OrderStatus {
    /// Return a human-readable status string.
    pub fn display(&self) -> String {
        match self {
            OrderStatus::Pending => "Pending".to_string(),
            OrderStatus::Confirmed => "Confirmed".to_string(),
            OrderStatus::Shipped { tracking } => format!("Shipped ({})", tracking),
            OrderStatus::Delivered => "Delivered".to_string(),
            OrderStatus::Cancelled { reason } => format!("Cancelled: {}", reason),
        }
    }

    /// Can the order be cancelled?
    pub fn can_cancel(&self) -> bool {
        matches!(self, OrderStatus::Pending | OrderStatus::Confirmed)
    }

    /// Is the order in a final state?
    pub fn is_final(&self) -> bool {
        matches!(self, OrderStatus::Delivered | OrderStatus::Cancelled { .. })
    }

    /// Attempt to confirm: only works from Pending.
    pub fn confirm(self) -> OrderStatus {
        match self {
            OrderStatus::Pending => OrderStatus::Confirmed,
            other => other,
        }
    }

    /// Attempt to ship: only works from Confirmed.
    pub fn ship(self, tracking: &str) -> OrderStatus {
        match self {
            OrderStatus::Confirmed => OrderStatus::Shipped {
                tracking: tracking.to_string(),
            },
            other => other,
        }
    }

    /// Attempt to deliver: only works from Shipped.
    pub fn deliver(self) -> OrderStatus {
        match self {
            OrderStatus::Shipped { .. } => OrderStatus::Delivered,
            other => other,
        }
    }

    /// Attempt to cancel: only works from Pending or Confirmed.
    pub fn cancel(self, reason: &str) -> OrderStatus {
        match self {
            OrderStatus::Pending | OrderStatus::Confirmed => OrderStatus::Cancelled {
                reason: reason.to_string(),
            },
            other => other,
        }
    }
}

// ============================================
// Topic 9: Result Combinators
// Learn: map, map_err, and_then, or_else, unwrap_or_else
// ============================================

/// Parse two strings and add them.
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, String> {
    let x = a.parse::<i32>().map_err(|e| format!("a: {}", e))?;
    let y = b.parse::<i32>().map_err(|e| format!("b: {}", e))?;
    Ok(x + y)
}

/// Divide two numbers, returning an error string on division by zero.
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Chain: parse -> validate positive -> double.
pub fn parse_validate_double(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| {
            if n > 0 {
                Ok(n * 2)
            } else {
                Err("not positive".to_string())
            }
        })
}

/// Convert a vector of strings to i32s, collecting all errors.
pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, Vec<String>> {
    let mut values = Vec::new();
    let mut errors = Vec::new();
    for &s in items {
        match s.parse::<i32>() {
            Ok(n) => values.push(n),
            Err(e) => errors.push(format!("{}: {}", s, e)),
        }
    }
    if errors.is_empty() {
        Ok(values)
    } else {
        Err(errors)
    }
}

/// Try primary parse, fallback to secondary on error.
pub fn parse_with_fallback(primary: &str, fallback: &str) -> Result<i32, String> {
    primary
        .parse::<i32>()
        .or_else(|_| fallback.parse::<i32>())
        .map_err(|e| e.to_string())
}

/// Map a Result value and error type simultaneously.
pub fn transform_result(r: Result<i32, i32>) -> Result<String, String> {
    r.map(|v| format!("ok:{}", v))
        .map_err(|e| format!("err:{}", e))
}

// ============================================
// Topic 10: Enum-based Error Types
// Learn: Custom error enums, From impl, error propagation
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    ParseError(String),
    ValidationError(String),
    NotFound(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::ParseError(e.to_string())
    }
}

/// Parse a string to i32, returning AppError on failure.
pub fn app_parse(s: &str) -> Result<i32, AppError> {
    Ok(s.parse::<i32>()?)
}

/// Parse and validate that the number is in range [min, max].
pub fn app_parse_in_range(s: &str, min: i32, max: i32) -> Result<i32, AppError> {
    let n = app_parse(s)?;
    if n < min || n > max {
        Err(AppError::ValidationError(format!(
            "{} not in range [{}, {}]",
            n, min, max
        )))
    } else {
        Ok(n)
    }
}

/// Look up a value, returning NotFound if missing.
pub fn app_lookup(items: &[(&str, i32)], key: &str) -> Result<i32, AppError> {
    items
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .ok_or_else(|| AppError::NotFound(key.to_string()))
}

/// Classify an AppError.
pub fn error_kind(err: &AppError) -> &str {
    match err {
        AppError::ParseError(_) => "parse",
        AppError::ValidationError(_) => "validation",
        AppError::NotFound(_) => "not_found",
    }
}

/// Check if an error is recoverable (parse and validation are, not_found isn't).
pub fn is_recoverable(err: &AppError) -> bool {
    matches!(
        err,
        AppError::ParseError(_) | AppError::ValidationError(_)
    )
}
