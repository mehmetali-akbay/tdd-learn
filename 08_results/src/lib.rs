// ============================================
// Level 3, Project 1: Results — Error Handling
// Learn: Result<T,E>, ? operator, custom errors, error conversion
// ============================================

use std::fmt;
use std::num::ParseIntError;

// ============================================
// Topic 1: Result Basics
// Learn: Ok/Err, unwrap_or, map, and_then
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

// ============================================
// Topic 2: The ? Operator
// Learn: Early return with ?, propagating errors
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

/// Calculate the average of string numbers: "1,2,3" => 2
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

// ============================================
// Topic 3: Custom Error Types
// Learn: Define error enums, implement Display, std::error::Error
// ============================================

/// Validation error type
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort(usize),
    TooLong(usize),
    InvalidChar(char),
    Empty,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::TooShort(len) => write!(f, "too short: {} chars", len),
            ValidationError::TooLong(len) => write!(f, "too long: {} chars", len),
            ValidationError::InvalidChar(c) => write!(f, "invalid character: '{}'", c),
            ValidationError::Empty => write!(f, "cannot be empty"),
        }
    }
}

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

/// Validate a password: at least 8 chars, must have uppercase, lowercase, digit
pub fn validate_password(pw: &str) -> Result<(), ValidationError> {
    if pw.is_empty() {
        return Err(ValidationError::Empty);
    }
    if pw.len() < 8 {
        return Err(ValidationError::TooShort(pw.len()));
    }
    if !pw.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::InvalidChar('a'));
    }
    if !pw.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::InvalidChar('A'));
    }
    if !pw.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidChar('x'));
    }
    Ok(())
}

// ============================================
// Topic 4: Error Conversion — From trait
// Learn: impl From<E1> for E2, automatic conversion with ?
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

// ============================================
// Topic 5: Collecting Results
// Learn: collect() into Result, partition results
// ============================================

/// Parse all strings to i32, failing on first error
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

/// Sum all parseable numbers, returning error count
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

// ============================================
// Topic 6: Advanced — Result Combinators
// Learn: and_then, or_else, map_err, flatten
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

/// Safely compute: parse a, parse b, divide a/b
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

// ============================================
// Topic 7: Extra Practice
// Learn: More error handling and Result exercises
// ============================================

/// Parse two f64 numbers from strings and add them.
pub fn parse_and_add(a: &str, b: &str) -> Result<f64, String> {
    let x: f64 = a.parse().map_err(|_| format!("invalid: {}", a))?;
    let y: f64 = b.parse().map_err(|_| format!("invalid: {}", b))?;
    Ok(x + y)
}

/// Validate an email has @ and ., returning descriptive error.
pub fn validate_email_simple(email: &str) -> Result<(), String> {
    if !email.contains('@') { return Err("missing @".into()); }
    if !email.contains('.') { return Err("missing .".into()); }
    Ok(())
}

/// Safe division returning f64 Result.
pub fn safe_divide_f64(a: &str, b: &str) -> Result<f64, String> {
    let x: f64 = a.parse().map_err(|_| "invalid numerator".to_string())?;
    let y: f64 = b.parse().map_err(|_| "invalid denominator".to_string())?;
    if y == 0.0 { return Err("division by zero".into()); }
    Ok(x / y)
}

/// Collect results: parse all strings, fail on first error.
pub fn parse_all_ints(items: &[&str]) -> Result<Vec<i32>, String> {
    items.iter()
        .map(|s| s.parse::<i32>().map_err(|_| format!("not a number: {}", s)))
        .collect()
}

