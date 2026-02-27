// ============================================
// Level 2, Project 2: Types — Complex Enums
// Learn: Enums with data, recursive enums, state machines
// ============================================

// ============================================
// Topic 1: JSON Value — Enums with Mixed Data
// Learn: Enum variants with different data types
// ============================================

/// A simplified JSON value
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<JsonValue>),
}

impl JsonValue {
    /// Check if a value is "truthy":
    /// Null => false, Bool(b) => b, Num(0) => false, Str("") => false,
    /// Array([]) => false, everything else => true
    pub fn is_truthy(&self) -> bool {
        todo!()
    }

    /// Return a human-readable type name
    pub fn type_name(&self) -> &'static str {
        todo!()
    }

    /// Convert to a display string
    pub fn to_display_string(&self) -> String {
        todo!()
    }

    /// Check if this is a Null
    pub fn is_null(&self) -> bool {
        todo!()
    }

    /// Try to extract a number, return None if not a Num
    pub fn as_num(&self) -> Option<f64> {
        todo!()
    }
}

// ============================================
// Topic 2: Traffic Light — State Machine
// Learn: State transitions, methods on enums
// ============================================

/// A traffic light with three states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// Transition to the next state: Red -> Green -> Yellow -> Red
    pub fn next(&self) -> TrafficLight {
        todo!()
    }

    /// How many seconds to wait in this state
    pub fn wait_time(&self) -> u32 {
        todo!()
    }

    /// Should traffic stop?
    pub fn is_stop(&self) -> bool {
        todo!()
    }

    /// Can traffic go?
    pub fn can_go(&self) -> bool {
        todo!()
    }

    /// Return the color name as a string
    pub fn color_name(&self) -> &'static str {
        todo!()
    }
}

// ============================================
// Topic 3: Linked List — Recursive Enums
// Learn: Recursive enums, Box, pattern matching
// ============================================

/// A singly-linked list of i32 values
#[derive(Debug, Clone, PartialEq, Default)]
pub enum List {
    #[default]
    Nil,
    Cons(i32, Box<List>),
}

impl List {
    /// Create an empty list
    pub fn new() -> Self {
        todo!()
    }

    /// Add an element to the front
    pub fn push(self, value: i32) -> Self {
        todo!()
    }

    /// Count the number of elements
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Get the first element
    pub fn head(&self) -> Option<i32> {
        todo!()
    }

    /// Check if the list contains a value
    pub fn contains(&self, target: i32) -> bool {
        todo!()
    }

    /// Sum all elements
    pub fn sum(&self) -> i32 {
        todo!()
    }

    /// Convert to a Vec
    pub fn to_vec(&self) -> Vec<i32> {
        todo!()
    }

    /// Create a list from a Vec
    pub fn from_vec(v: &[i32]) -> Self {
        todo!()
    }
}

// ============================================
// Topic 4: Expression Tree — Recursive Matching
// Learn: Complex recursive enums, Box, evaluation
// ============================================

/// A math expression tree with more operations than Level 1
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    /// Helper: create a boxed Num
    pub fn num(n: f64) -> Box<Expr> {
        todo!()
    }

    /// Evaluate the expression, returning None on division by zero
    pub fn eval(&self) -> Option<f64> {
        todo!()
    }

    /// Pretty-print the expression
    pub fn to_string_repr(&self) -> String {
        todo!()
    }

    /// Count the number of operations (non-Num nodes)
    pub fn count_ops(&self) -> usize {
        todo!()
    }

    /// Check if the expression contains a division
    pub fn contains_div(&self) -> bool {
        todo!()
    }
}

// ============================================
// Topic 5: Custom Error Types
// Learn: Error enums, Result with custom errors
// ============================================

/// Application-level errors
#[derive(Debug, PartialEq)]
pub enum AppError {
    NotFound(String),
    InvalidInput(String),
    Unauthorized,
    ParseError(String),
}

impl AppError {
    /// Return a user-friendly error message
    pub fn message(&self) -> String {
        todo!()
    }

    /// Check if this is a user error (InvalidInput or ParseError)
    pub fn is_user_error(&self) -> bool {
        todo!()
    }
}

/// Parse a string as an age (0-150), returning AppError on failure
pub fn parse_age(s: &str) -> Result<u8, AppError> {
        todo!()
}

/// Divide two numbers, returning an error for division by zero
pub fn divide_safe(a: f64, b: f64) -> Result<f64, AppError> {
        todo!()
}

/// Look up a user by ID from a hardcoded "database"
pub fn lookup_user(id: u32) -> Result<String, AppError> {
        todo!()
}

/// Chain: parse age, then check if adult (>= 18)
pub fn parse_and_check_adult(s: &str) -> Result<bool, AppError> {
        todo!()
}

// ============================================
// Topic 6: Payload — Advanced Nested Enums
// Learn: Complex variants, methods on enums
// ============================================

/// A data payload with different formats
#[derive(Debug, Clone, PartialEq)]
pub enum Payload {
    Text(String),
    Binary(Vec<u8>),
    Json(JsonValue),
    Empty,
}

impl Payload {
    /// Size in bytes
    pub fn size(&self) -> usize {
        todo!()
    }

    /// Check if the payload is text-based (Text or Json)
    pub fn is_text(&self) -> bool {
        todo!()
    }

    /// Check if the payload is empty
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Convert to text representation
    pub fn to_text(&self) -> String {
        todo!()
    }

    /// Merge two payloads (text + text concatenated, otherwise first wins)
    pub fn merge(self, other: Payload) -> Payload {
        todo!()
    }
}

// ============================================
// Topic 7: Extra Practice
// Learn: More type conversion and newtype exercises
// ============================================

/// Clamp a value to the range [min, max].
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        todo!()
}

/// Safely convert u32 to u8, returning None if out of range.
pub fn safe_u32_to_u8(n: u32) -> Option<u8> {
        todo!()
}

/// Convert a temperature from Fahrenheit to Celsius and classify it.
pub fn temp_classify(fahrenheit: f64) -> &'static str {
        todo!()
}

/// Map a value from one range to another (linear interpolation).
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
        todo!()
}
