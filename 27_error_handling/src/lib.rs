// ============================================
// Level 8, Project 3: Error Handling — Production Patterns
// Learn: thiserror, anyhow, error composition, panic vs Result
// ============================================

use std::fmt;
use std::num::ParseIntError;
use thiserror::Error;

// ============================================
// Topic 1: thiserror Basics
// Learn: #[derive(Error)], struct and enum errors
// ============================================

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("field '{field}' is required")]
    RequiredField { field: String },

    #[error("field '{field}' must be at most {max} characters, got {actual}")]
    TooLong {
        field: String,
        max: usize,
        actual: usize,
    },

    #[error("invalid email format: {0}")]
    InvalidEmail(String),
}

/// Validate a username (non-empty, max 20 chars).
pub fn validate_username(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::RequiredField {
            field: "username".into(),
        });
    }
    if name.len() > 20 {
        return Err(ValidationError::TooLong {
            field: "username".into(),
            max: 20,
            actual: name.len(),
        });
    }
    Ok(())
}

/// Validate an email (must contain @ and .).
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(ValidationError::InvalidEmail(email.to_string()));
    }
    Ok(())
}

#[derive(Debug, Error)]
#[error("parse error at line {line}, column {col}: {message}")]
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub message: String,
}

/// Simulate parsing that can fail.
pub fn parse_line(input: &str, line: usize) -> Result<i32, ParseError> {
    input.parse::<i32>().map_err(|_| ParseError {
        line,
        col: 0,
        message: format!("cannot parse '{}'", input),
    })
}

// ============================================
// Topic 2: anyhow for Applications
// Learn: anyhow::Result, context(), bail!
// ============================================

/// Process a config string: parse key=value pairs.
pub fn process_config(input: &str) -> anyhow::Result<Vec<(String, String)>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            anyhow::bail!("invalid config line: '{}'", line);
        }
        result.push((parts[0].to_string(), parts[1].to_string()));
    }
    Ok(result)
}

/// Load a value by key from config, with context on failure.
pub fn get_config_value(config: &[(String, String)], key: &str) -> anyhow::Result<String> {
    config
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .ok_or_else(|| anyhow::anyhow!("key '{}' not found in config", key))
}

/// Chain multiple fallible operations with context.
pub fn parse_and_validate_port(input: &str) -> anyhow::Result<u16> {
    let n: u32 = input
        .parse()
        .map_err(|e: ParseIntError| anyhow::anyhow!(e))
        .with_context(|| format!("failed to parse '{}' as port number", input))?;
    if n == 0 || n > 65535 {
        anyhow::bail!("port {} out of valid range 1-65535", n);
    }
    Ok(n as u16)
}

use anyhow::Context;

// ============================================
// Topic 3: Error Composition
// Learn: #[from], #[source], multiple error sources
// ============================================

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("network error: {0}")]
    Network(#[from] NetworkError),

    #[error("parse error: {0}")]
    Parse(#[from] ParseIntError),

    #[error("not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Error)]
#[error("database connection failed: {reason}")]
pub struct DatabaseError {
    pub reason: String,
}

#[derive(Debug, Error)]
#[error("network request failed: {url}")]
pub struct NetworkError {
    pub url: String,
}

/// Simulate a database lookup.
pub fn db_lookup(id: i32) -> Result<String, ServiceError> {
    if id == 999 {
        return Err(ServiceError::NotFound(format!("id {}", id)));
    }
    Ok(format!("record_{}", id))
}

/// Simulate a network fetch.
pub fn network_fetch(url: &str) -> Result<String, ServiceError> {
    if url.is_empty() {
        return Err(NetworkError {
            url: url.to_string(),
        }
        .into());
    }
    Ok(format!("response from {}", url))
}

/// Combine multiple error sources.
pub fn fetch_and_parse(url: &str) -> Result<i32, ServiceError> {
    let response = network_fetch(url)?;
    let number: i32 = response.len().to_string().parse()?;
    Ok(number)
}

// ============================================
// Topic 4: Custom Error Hierarchies
// Learn: Domain errors, mapping between layers
// ============================================

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("key not found: {0}")]
    KeyNotFound(String),

    #[error("storage full")]
    Full,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("storage failure: {0}")]
    Storage(#[from] StorageError),

    #[error("invalid input: {0}")]
    Input(String),

    #[error("unauthorized")]
    Unauthorized,
}

/// A simple in-memory store for demonstration.
pub struct Store {
    items: Vec<(String, String)>,
    capacity: usize,
}

impl Store {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }

    pub fn get(&self, key: &str) -> Result<&str, StorageError> {
        self.items
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .ok_or_else(|| StorageError::KeyNotFound(key.to_string()))
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), StorageError> {
        if self.items.len() >= self.capacity {
            return Err(StorageError::Full);
        }
        self.items.push((key, value));
        Ok(())
    }
}

/// Application-level function that maps storage errors to app errors.
pub fn app_get(store: &Store, key: &str, is_authed: bool) -> Result<String, AppError> {
    if !is_authed {
        return Err(AppError::Unauthorized);
    }
    let value = store.get(key)?;
    Ok(value.to_string())
}

// ============================================
// Topic 5: panic vs Result Strategies
// Learn: When to panic, catch_unwind
// ============================================

/// This function should panic — called when invariant is broken.
pub fn assert_valid_index(index: usize, len: usize) {
    if index >= len {
        panic!("index {} out of bounds for length {}", index, len);
    }
}

/// Catch a panic and convert to Result.
pub fn catch_panic<F: FnOnce() -> i32 + std::panic::UnwindSafe>(f: F) -> Result<i32, String> {
    std::panic::catch_unwind(f).map_err(|_| "panic occurred".to_string())
}

/// A function that converts panics from a bad API into Results.
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }
    Ok(a / b)
}

// ============================================
// Topic 6: Advanced — Error Chains & Reporting
// Learn: Error chains, source(), Display formatting
// ============================================

/// Walk an error chain and collect all messages.
pub fn error_chain(err: &dyn std::error::Error) -> Vec<String> {
    let mut chain = vec![err.to_string()];
    let mut current = err.source();
    while let Some(cause) = current {
        chain.push(cause.to_string());
        current = cause.source();
    }
    chain
}

/// Format an error with its full chain for logging.
pub fn format_error_chain(err: &dyn std::error::Error) -> String {
    error_chain(err).join(" -> ")
}

pub fn anyhow_with_backtrace() -> anyhow::Result<()> {
    anyhow::bail!("a simulated failure that captures a backtrace")
}

pub fn extract_backtrace(err: &anyhow::Error) -> Option<String> {
    // anyhow::Error provides a backtrace() method
    Some(err.backtrace().to_string())
}

#[derive(Debug, Error)]
#[error("high-level operation failed")]
pub struct HighLevelError {
    #[source]
    pub cause: MidLevelError,
}

#[derive(Debug, Error)]
#[error("mid-level operation failed")]
pub struct MidLevelError {
    #[source]
    pub cause: LowLevelError,
}

#[derive(Debug, Error)]
#[error("low-level I/O error: {details}")]
pub struct LowLevelError {
    pub details: String,
}

/// Create a nested error chain for testing.
pub fn create_error_chain() -> HighLevelError {
    HighLevelError {
        cause: MidLevelError {
            cause: LowLevelError {
                details: "disk read failed".to_string(),
            },
        },
    }
}
