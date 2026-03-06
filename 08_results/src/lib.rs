// ============================================
// Module 8: Results — Error Handling
// Learn: Result<T,E>, ? operator, custom errors, error conversion,
//        custom validation types (Guess pattern), collecting results
// Reinforces: ownership & borrowing, structs, enums, pattern matching,
//             Vec operations, String parsing, HashMap
// ============================================

use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

// ============================================
// Topic 1: Result Basics
// Learn: Ok/Err, unwrap_or, map, and_then
// Reinforces: slices (&[i32]), match on ranges, iteration
// ============================================

/// Divide two integers, returning Err on division by zero
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Parse a string to i32, mapping the error to a String
pub fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("parse error: {}", e))
}

/// Get element at index, returning Err if out of bounds
/// Reinforces: borrowing slices
pub fn get_at(v: &[i32], index: usize) -> Result<i32, String> {
    v.get(index)
        .copied()
        .ok_or_else(|| format!("index {} out of bounds", index))
}

/// Double a Result value if Ok, pass through Err
pub fn double_result(r: Result<i32, String>) -> Result<i32, String> {
    r.map(|v| v * 2)
}

/// Chain: parse then double
pub fn parse_and_double(s: &str) -> Result<i32, String> {
    parse_number(s).map(|n| n * 2)
}

/// Return the default value if the Result is Err
pub fn unwrap_or_default(r: Result<i32, String>, default: i32) -> i32 {
    r.unwrap_or(default)
}

/// Find the first even number in a slice, or error if none found
/// Reinforces: slice borrowing, iteration, pattern matching
pub fn find_first_even(slice: &[i32]) -> Result<i32, String> {
    for &n in slice {
        if n % 2 == 0 {
            return Ok(n);
        }
    }
    Err("no even number found".to_string())
}

/// Look up a letter grade based on a numeric score (0-100)
/// Reinforces: match on ranges, pattern matching
pub fn lookup_grade(score: u32) -> Result<char, String> {
    match score {
        90..=100 => Ok('A'),
        80..=89 => Ok('B'),
        70..=79 => Ok('C'),
        60..=69 => Ok('D'),
        0..=59 => Ok('F'),
        _ => Err(format!("score {} is out of range 0-100", score)),
    }
}

// ============================================
// Topic 2: The ? Operator
// Learn: Early return with ?, propagating errors
// Reinforces: String splitting, Vec collection, tuples
// ============================================

/// Parse two strings and add them (using ? operator)
pub fn add_strings(a: &str, b: &str) -> Result<i32, String> {
    let x = parse_number(a)?;
    let y = parse_number(b)?;
    Ok(x + y)
}

/// Parse a string, divide by another parsed string
pub fn parse_and_divide(a: &str, b: &str) -> Result<i32, String> {
    let x = parse_number(a)?;
    let y = parse_number(b)?;
    divide(x, y)
}

/// Calculate the average of string numbers: "1,2,3" => 2.0
pub fn average_of_csv(csv: &str) -> Result<f64, String> {
    if csv.is_empty() {
        return Err("empty input".to_string());
    }
    let parts: Vec<&str> = csv.split(',').collect();
    let mut sum = 0i64;
    let count = parts.len();
    for part in parts {
        let n = part
            .trim()
            .parse::<i64>()
            .map_err(|e| format!("parse error: {}", e))?;
        sum += n;
    }
    Ok(sum as f64 / count as f64)
}

/// Find the first positive number in a comma-separated string
pub fn first_positive_from_csv(csv: &str) -> Result<i32, String> {
    if csv.is_empty() {
        return Err("empty input".to_string());
    }
    for part in csv.split(',') {
        let n = part
            .trim()
            .parse::<i32>()
            .map_err(|e| format!("parse error: {}", e))?;
        if n > 0 {
            return Ok(n);
        }
    }
    Err("no positive number found".to_string())
}

/// Parse a point string "x,y" into a tuple of f64
/// Reinforces: String splitting, tuples, destructuring
pub fn parse_point(s: &str) -> Result<(f64, f64), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("expected 'x,y' format, got '{}'", s));
    }
    let x = parts[0]
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("invalid x: {}", e))?;
    let y = parts[1]
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("invalid y: {}", e))?;
    Ok((x, y))
}

/// Sum only the positive numbers from a CSV string
/// Reinforces: Vec iteration, conditional accumulation
pub fn sum_positive_from_csv(csv: &str) -> Result<i32, String> {
    if csv.is_empty() {
        return Err("empty input".to_string());
    }
    let mut sum = 0;
    for part in csv.split(',') {
        let n = part
            .trim()
            .parse::<i32>()
            .map_err(|e| format!("parse error: {}", e))?;
        if n > 0 {
            sum += n;
        }
    }
    Ok(sum)
}

/// Parse a range string "start..end" into a Vec of integers (exclusive end)
/// Reinforces: Vec creation, ranges, String parsing
pub fn parse_int_range(s: &str) -> Result<Vec<i32>, String> {
    let parts: Vec<&str> = s.split("..").collect();
    if parts.len() != 2 {
        return Err(format!("expected 'start..end' format, got '{}'", s));
    }
    let start = parts[0]
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("invalid start: {}", e))?;
    let end = parts[1]
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("invalid end: {}", e))?;
    if start > end {
        return Err(format!("start ({}) must be <= end ({})", start, end));
    }
    Ok((start..end).collect())
}

// ============================================
// Topic 3: Custom Error Types
// Learn: Define error enums, implement Display, std::error::Error
// Reinforces: enums with data, pattern matching, struct encapsulation
// ============================================

/// Validation error type with rich variants
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    Empty,
    TooShort(usize),
    TooLong(usize),
    InvalidChar(char),
    MissingUppercase,
    MissingLowercase,
    MissingDigit,
    InvalidFormat(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::Empty => write!(f, "cannot be empty"),
            ValidationError::TooShort(len) => write!(f, "too short: {} chars", len),
            ValidationError::TooLong(len) => write!(f, "too long: {} chars", len),
            ValidationError::InvalidChar(c) => write!(f, "invalid character: '{}'", c),
            ValidationError::MissingUppercase => write!(f, "missing uppercase letter"),
            ValidationError::MissingLowercase => write!(f, "missing lowercase letter"),
            ValidationError::MissingDigit => write!(f, "missing digit"),
            ValidationError::InvalidFormat(msg) => write!(f, "invalid format: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate a username: 3-20 chars, only alphanumeric and underscore
pub fn validate_username(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::Empty);
    }
    if name.len() < 3 {
        return Err(ValidationError::TooShort(name.len()));
    }
    if name.len() > 20 {
        return Err(ValidationError::TooLong(name.len()));
    }
    for c in name.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return Err(ValidationError::InvalidChar(c));
        }
    }
    Ok(())
}

/// Validate a password: at least 8 chars, must contain uppercase, lowercase, and digit
/// Reinforces: character iteration, pattern matching on properties
pub fn validate_password(pw: &str) -> Result<(), ValidationError> {
    if pw.is_empty() {
        return Err(ValidationError::Empty);
    }
    if pw.len() < 8 {
        return Err(ValidationError::TooShort(pw.len()));
    }
    if !pw.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::MissingUppercase);
    }
    if !pw.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::MissingLowercase);
    }
    if !pw.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::MissingDigit);
    }
    Ok(())
}

/// Validate an email: must contain exactly one '@' with text before and after,
/// and at least one '.' after the '@'
/// Reinforces: String operations, character counting, pattern matching
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::Empty);
    }
    let at_count = email.chars().filter(|&c| c == '@').count();
    if at_count != 1 {
        return Err(ValidationError::InvalidFormat(
            "must contain exactly one '@'".to_string(),
        ));
    }
    let parts: Vec<&str> = email.splitn(2, '@').collect();
    let local = parts[0];
    let domain = parts[1];
    if local.is_empty() {
        return Err(ValidationError::InvalidFormat(
            "missing local part before '@'".to_string(),
        ));
    }
    if domain.is_empty() || !domain.contains('.') {
        return Err(ValidationError::InvalidFormat(
            "domain must contain a '.'".to_string(),
        ));
    }
    Ok(())
}

/// A validated name type with a private field — the "Guess" pattern from Rust Book §9.3
/// Once constructed, the name is guaranteed to be valid (3-20 chars, alphanumeric + underscore)
/// Reinforces: structs, encapsulation (private fields), constructors returning Result
#[derive(Debug)]
pub struct ValidatedName {
    name: String,
}

impl ValidatedName {
    /// Create a new ValidatedName, returning Err if the name is invalid
    pub fn new(name: &str) -> Result<Self, ValidationError> {
        validate_username(name)?;
        Ok(ValidatedName {
            name: name.to_string(),
        })
    }

    /// Get the validated name as a string slice
    pub fn value(&self) -> &str {
        &self.name
    }
}

/// Validate a complete registration (username + password + email)
/// Reinforces: chaining ? with custom errors, combining multiple validations
pub fn validate_registration(
    username: &str,
    password: &str,
    email: &str,
) -> Result<(), ValidationError> {
    validate_username(username)?;
    validate_password(password)?;
    validate_email(email)?;
    Ok(())
}

// ============================================
// Topic 4: Error Conversion — From trait
// Learn: impl From<E1> for E2, automatic conversion with ?
// Reinforces: structs (UserProfile), HashMap collection, borrowing
// ============================================

/// Application-level error that wraps multiple error sources
#[derive(Debug, PartialEq)]
pub enum AppError {
    Parse(String),
    Validation(ValidationError),
    NotFound(String),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AppError::Validation(e) => write!(f, "Validation error: {}", e),
            AppError::NotFound(item) => write!(f, "{} not found", item),
            AppError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e.to_string())
    }
}

impl From<ValidationError> for AppError {
    fn from(e: ValidationError) -> Self {
        AppError::Validation(e)
    }
}

/// Register a user: validate username, parse age string, return greeting
pub fn register_user(username: &str, age_str: &str) -> Result<String, AppError> {
    validate_username(username)?; // ValidationError auto-converts to AppError
    let age: i32 = age_str.parse()?; // ParseIntError auto-converts to AppError
    Ok(format!("Welcome {}, age {}", username, age))
}

/// Parse a config line "key=value" into (key, i32-value)
pub fn parse_config_line(line: &str) -> Result<(String, i32), AppError> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(AppError::Custom("missing '=' separator".to_string()));
    }
    let key = parts[0].trim().to_string();
    let value: i32 = parts[1].trim().parse()?;
    Ok((key, value))
}

/// A user profile struct with owned String fields
/// Reinforces: struct definition, owned fields, methods
#[derive(Debug)]
pub struct UserProfile {
    pub username: String,
    pub age: i32,
    pub email: String,
}

impl UserProfile {
    /// Display-friendly summary of the user profile
    pub fn summary(&self) -> String {
        format!("{} (age {}): {}", self.username, self.age, self.email)
    }
}

/// Create a user profile with full validation
/// Validates username, parses age, validates email — all errors auto-convert via From
/// Reinforces: struct construction, combining multiple error sources
pub fn create_user_profile(
    username: &str,
    age_str: &str,
    email: &str,
) -> Result<UserProfile, AppError> {
    validate_username(username)?;
    let age: i32 = age_str.parse()?;
    if age < 0 || age > 150 {
        return Err(AppError::Custom(format!("invalid age: {}", age)));
    }
    validate_email(email)?;
    Ok(UserProfile {
        username: username.to_string(),
        age,
        email: email.to_string(),
    })
}

/// Parse multiple config lines into a HashMap
/// Reinforces: HashMap building, iterating over slices
pub fn parse_config_map(lines: &[&str]) -> Result<HashMap<String, i32>, AppError> {
    let mut map = HashMap::new();
    for line in lines {
        let (key, value) = parse_config_line(line)?;
        map.insert(key, value);
    }
    Ok(map)
}

/// Find a user by username and return their summary string
/// Reinforces: slice iteration, Option → Result conversion (ok_or_else)
pub fn find_user_summary(users: &[UserProfile], name: &str) -> Result<String, AppError> {
    users
        .iter()
        .find(|u| u.username == name)
        .map(|u| u.summary())
        .ok_or_else(|| AppError::NotFound(format!("user '{}'", name)))
}

// ============================================
// Topic 5: Collecting Results
// Learn: collect() into Result, partition results
// Reinforces: Vec operations, iteration, tuples, reusing custom errors
// ============================================

/// Parse all strings to i32, failing on first error (collect into Result)
pub fn parse_all_strict(items: &[&str]) -> Result<Vec<i32>, String> {
    items
        .iter()
        .map(|s| s.parse::<i32>().map_err(|e| format!("'{}': {}", s, e)))
        .collect()
}

/// Parse all strings to i32, ignoring errors (keep only successes)
pub fn parse_all_lenient(items: &[&str]) -> Vec<i32> {
    items.iter().filter_map(|s| s.parse::<i32>().ok()).collect()
}

/// Partition results into (successes, errors)
pub fn partition_results(items: &[&str]) -> (Vec<i32>, Vec<String>) {
    let mut successes = Vec::new();
    let mut errors = Vec::new();
    for s in items {
        match s.parse::<i32>() {
            Ok(n) => successes.push(n),
            Err(e) => errors.push(format!("'{}': {}", s, e)),
        }
    }
    (successes, errors)
}

/// Sum all parseable numbers, returning (sum, error_count)
pub fn sum_with_error_count(items: &[&str]) -> (i32, usize) {
    let mut sum = 0;
    let mut errors = 0;
    for s in items {
        match s.parse::<i32>() {
            Ok(n) => sum += n,
            Err(_) => errors += 1,
        }
    }
    (sum, errors)
}

/// Parse and filter: only keep numbers within [min, max] range
/// Returns (valid_numbers, rejection_messages)
/// Reinforces: Vec, range checks, match guards, tuples
pub fn filter_valid_in_range(
    items: &[&str],
    min: i32,
    max: i32,
) -> (Vec<i32>, Vec<String>) {
    let mut valid = Vec::new();
    let mut rejected = Vec::new();
    for s in items {
        match s.parse::<i32>() {
            Ok(n) if (min..=max).contains(&n) => valid.push(n),
            Ok(n) => rejected.push(format!("{} out of range {}..={}", n, min, max)),
            Err(_) => rejected.push(format!("'{}' is not a number", s)),
        }
    }
    (valid, rejected)
}

/// Parse a space-separated row of numbers into a Vec<f64>
/// Reinforces: String splitting, Vec collection via iterators
pub fn parse_matrix_row(s: &str) -> Result<Vec<f64>, String> {
    if s.trim().is_empty() {
        return Err("empty row".to_string());
    }
    s.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|e| format!("invalid number '{}': {}", token, e))
        })
        .collect()
}

/// Validate multiple usernames, partitioning into valid and invalid
/// Returns (valid_names, Vec of (name, error) pairs)
/// Reinforces: Vec of tuples, reusing custom error types from Topic 3
pub fn validate_usernames(
    names: &[&str],
) -> (Vec<String>, Vec<(String, ValidationError)>) {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();
    for &name in names {
        match validate_username(name) {
            Ok(()) => valid.push(name.to_string()),
            Err(e) => invalid.push((name.to_string(), e)),
        }
    }
    (valid, invalid)
}

// ============================================
// Topic 6: Advanced — Result Combinators & Patterns
// Learn: and_then, or_else, map_err, custom validation types,
//        Option ↔ Result bridging
// Reinforces: struct encapsulation (Guess pattern §9.3), match on strings
// ============================================

/// Chain operations: parse -> validate range (1-100) -> double
pub fn parse_validate_double(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("parse error: {}", e))
        .and_then(|n| {
            if (1..=100).contains(&n) {
                Ok(n)
            } else {
                Err(format!("{} is out of range 1-100", n))
            }
        })
        .map(|n| n * 2)
}

/// Try primary parse, fall back to secondary
pub fn parse_with_fallback(primary: &str, fallback: &str) -> Result<i32, String> {
    primary
        .parse::<i32>()
        .or_else(|_| fallback.parse::<i32>())
        .map_err(|e| format!("both failed: {}", e))
}

/// Transform errors to a standard format
pub fn standardize_error(r: Result<i32, String>) -> Result<i32, String> {
    r.map_err(|e| format!("[ERROR] {}", e))
}

/// Safely compute: parse a, parse b, divide a/b (f64)
pub fn safe_compute(a: &str, b: &str) -> Result<f64, String> {
    let x = a
        .parse::<f64>()
        .map_err(|e| format!("invalid first operand: {}", e))?;
    let y = b
        .parse::<f64>()
        .map_err(|e| format!("invalid second operand: {}", e))?;
    if y == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(x / y)
    }
}

/// A bounded integer type — the "Guess" pattern from Rust Book §9.3
/// The value is guaranteed to be within [min, max] once constructed.
/// Reinforces: struct encapsulation, private fields, constructor returning Result
#[derive(Debug)]
pub struct BoundedI32 {
    value: i32,
    min: i32,
    max: i32,
}

impl BoundedI32 {
    /// Create a new BoundedI32, returning Err if value is out of range
    pub fn new(value: i32, min: i32, max: i32) -> Result<Self, String> {
        if min > max {
            return Err(format!(
                "invalid bounds: min ({}) > max ({})",
                min, max
            ));
        }
        if value < min || value > max {
            return Err(format!(
                "{} is out of range {}..={}",
                value, min, max
            ));
        }
        Ok(BoundedI32 { value, min, max })
    }

    /// Get the bounded value
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Get the minimum bound
    pub fn min(&self) -> i32 {
        self.min
    }

    /// Get the maximum bound
    pub fn max(&self) -> i32 {
        self.max
    }
}

impl fmt::Display for BoundedI32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}..={})", self.value, self.min, self.max)
    }
}

/// Convert an Option<i32> to a Result with a custom error message
/// Demonstrates the Option → Result bridge (ok_or_else)
pub fn option_to_result(opt: Option<i32>, err_msg: &str) -> Result<i32, String> {
    opt.ok_or_else(|| err_msg.to_string())
}

/// Convert a Result<i32, String> to an Option<i32>, discarding the error
/// Demonstrates the Result → Option bridge (.ok())
pub fn result_to_option(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}

/// Try to parse each item in the slice, return the first successful parse
/// Reinforces: slice iteration, early return on success
pub fn first_valid_parse(items: &[&str]) -> Result<i32, String> {
    for item in items {
        if let Ok(n) = item.parse::<i32>() {
            return Ok(n);
        }
    }
    Err("no valid integer found".to_string())
}

/// Safely apply a mathematical operation described by a string
/// Operations: "double", "negate", "abs", "square"
/// Reinforces: match on string slices, combining parse + match
pub fn apply_operation(value: &str, op: &str) -> Result<i32, String> {
    let n = value
        .parse::<i32>()
        .map_err(|e| format!("parse error: {}", e))?;
    match op {
        "double" => Ok(n * 2),
        "negate" => Ok(-n),
        "abs" => Ok(n.abs()),
        "square" => Ok(n.saturating_mul(n)),
        _ => Err(format!("unknown operation: '{}'", op)),
    }
}