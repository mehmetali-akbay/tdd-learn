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
        match self {
            JsonValue::Null => false,
            JsonValue::Bool(b) => *b,
            JsonValue::Num(n) => *n != 0.0,
            JsonValue::Str(s) => !s.is_empty(),
            JsonValue::Array(arr) => !arr.is_empty(),
        }
    }

    /// Return a human-readable type name
    pub fn type_name(&self) -> &'static str {
        match self {
            JsonValue::Null => "null",
            JsonValue::Bool(_) => "boolean",
            JsonValue::Num(_) => "number",
            JsonValue::Str(_) => "string",
            JsonValue::Array(_) => "array",
        }
    }

    /// Convert to a display string
    pub fn to_display_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Num(n) => n.to_string(),
            JsonValue::Str(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_display_string()).collect();
                format!("[{}]", items.join(", "))
            }
        }
    }

    /// Check if this is a Null
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    /// Try to extract a number, return None if not a Num
    pub fn as_num(&self) -> Option<f64> {
        match self {
            JsonValue::Num(n) => Some(*n),
            _ => None,
        }
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
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    /// How many seconds to wait in this state
    pub fn wait_time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    /// Should traffic stop?
    pub fn is_stop(&self) -> bool {
        matches!(self, TrafficLight::Red | TrafficLight::Yellow)
    }

    /// Can traffic go?
    pub fn can_go(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }

    /// Return the color name as a string
    pub fn color_name(&self) -> &'static str {
        match self {
            TrafficLight::Red => "red",
            TrafficLight::Yellow => "yellow",
            TrafficLight::Green => "green",
        }
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
        Self::default()
    }

    /// Add an element to the front
    pub fn push(self, value: i32) -> Self {
        List::Cons(value, Box::new(self))
    }

    /// Count the number of elements
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, rest) => 1 + rest.len(),
        }
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    /// Get the first element
    pub fn head(&self) -> Option<i32> {
        match self {
            List::Nil => None,
            List::Cons(val, _) => Some(*val),
        }
    }

    /// Check if the list contains a value
    pub fn contains(&self, target: i32) -> bool {
        match self {
            List::Nil => false,
            List::Cons(val, rest) => *val == target || rest.contains(target),
        }
    }

    /// Sum all elements
    pub fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons(val, rest) => val + rest.sum(),
        }
    }

    /// Convert to a Vec
    pub fn to_vec(&self) -> Vec<i32> {
        match self {
            List::Nil => vec![],
            List::Cons(val, rest) => {
                let mut v = vec![*val];
                v.extend(rest.to_vec());
                v
            }
        }
    }

    /// Create a list from a Vec
    pub fn from_vec(v: &[i32]) -> Self {
        v.iter().rev().fold(List::Nil, |acc, &val| acc.push(val))
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
        Box::new(Expr::Num(n))
    }

    /// Evaluate the expression, returning None on division by zero
    pub fn eval(&self) -> Option<f64> {
        match self {
            Expr::Num(n) => Some(*n),
            Expr::Add(a, b) => Some(a.eval()? + b.eval()?),
            Expr::Sub(a, b) => Some(a.eval()? - b.eval()?),
            Expr::Mul(a, b) => Some(a.eval()? * b.eval()?),
            Expr::Div(a, b) => {
                let divisor = b.eval()?;
                if divisor == 0.0 {
                    None
                } else {
                    Some(a.eval()? / divisor)
                }
            }
            Expr::Neg(e) => Some(-e.eval()?),
        }
    }

    /// Pretty-print the expression
    pub fn to_string_repr(&self) -> String {
        match self {
            Expr::Num(n) => format!("{}", n),
            Expr::Add(a, b) => format!("({} + {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Sub(a, b) => format!("({} - {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Mul(a, b) => format!("({} * {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Div(a, b) => format!("({} / {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Neg(e) => format!("(-{})", e.to_string_repr()),
        }
    }

    /// Count the number of operations (non-Num nodes)
    pub fn count_ops(&self) -> usize {
        match self {
            Expr::Num(_) => 0,
            Expr::Neg(e) => 1 + e.count_ops(),
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                1 + a.count_ops() + b.count_ops()
            }
        }
    }

    /// Check if the expression contains a division
    pub fn contains_div(&self) -> bool {
        match self {
            Expr::Num(_) => false,
            Expr::Div(_, _) => true,
            Expr::Neg(e) => e.contains_div(),
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) => {
                a.contains_div() || b.contains_div()
            }
        }
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
        match self {
            AppError::NotFound(item) => format!("{} not found", item),
            AppError::InvalidInput(msg) => format!("Invalid input: {}", msg),
            AppError::Unauthorized => "Unauthorized access".to_string(),
            AppError::ParseError(msg) => format!("Parse error: {}", msg),
        }
    }

    /// Check if this is a user error (InvalidInput or ParseError)
    pub fn is_user_error(&self) -> bool {
        matches!(self, AppError::InvalidInput(_) | AppError::ParseError(_))
    }
}

/// Parse a string as an age (0-150), returning AppError on failure
pub fn parse_age(s: &str) -> Result<u8, AppError> {
    let num: i32 = s
        .parse()
        .map_err(|_| AppError::ParseError(format!("'{}' is not a number", s)))?;
    if !(0..=150).contains(&num) {
        Err(AppError::InvalidInput(format!(
            "age {} is out of range 0-150",
            num
        )))
    } else {
        Ok(num as u8)
    }
}

/// Divide two numbers, returning an error for division by zero
pub fn divide_safe(a: f64, b: f64) -> Result<f64, AppError> {
    if b == 0.0 {
        Err(AppError::InvalidInput("division by zero".to_string()))
    } else {
        Ok(a / b)
    }
}

/// Look up a user by ID from a hardcoded "database"
pub fn lookup_user(id: u32) -> Result<String, AppError> {
    match id {
        1 => Ok("Alice".to_string()),
        2 => Ok("Bob".to_string()),
        3 => Ok("Charlie".to_string()),
        _ => Err(AppError::NotFound(format!("user {}", id))),
    }
}

/// Chain: parse age, then check if adult (>= 18)
pub fn parse_and_check_adult(s: &str) -> Result<bool, AppError> {
    let age = parse_age(s)?;
    Ok(age >= 18)
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
        match self {
            Payload::Text(s) => s.len(),
            Payload::Binary(b) => b.len(),
            Payload::Json(j) => j.to_display_string().len(),
            Payload::Empty => 0,
        }
    }

    /// Check if the payload is text-based (Text or Json)
    pub fn is_text(&self) -> bool {
        matches!(self, Payload::Text(_) | Payload::Json(_))
    }

    /// Check if the payload is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Payload::Empty => true,
            Payload::Text(s) => s.is_empty(),
            Payload::Binary(b) => b.is_empty(),
            Payload::Json(j) => j.is_null(),
        }
    }

    /// Convert to text representation
    pub fn to_text(&self) -> String {
        match self {
            Payload::Text(s) => s.clone(),
            Payload::Binary(b) => format!("<{} bytes>", b.len()),
            Payload::Json(j) => j.to_display_string(),
            Payload::Empty => "<empty>".to_string(),
        }
    }

    /// Merge two payloads (text + text concatenated, otherwise first wins)
    pub fn merge(self, other: Payload) -> Payload {
        match (self, other) {
            (Payload::Text(a), Payload::Text(b)) => Payload::Text(format!("{}{}", a, b)),
            (Payload::Binary(mut a), Payload::Binary(b)) => {
                a.extend(b);
                Payload::Binary(a)
            }
            (Payload::Empty, other) => other,
            (this, Payload::Empty) => this,
            (this, _) => this,
        }
    }
}

// ============================================
// Topic 7: Extra Practice
// Learn: More type conversion and newtype exercises
// ============================================

/// Clamp a value to the range [min, max].
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min { min } else if value > max { max } else { value }
}

/// Safely convert u32 to u8, returning None if out of range.
pub fn safe_u32_to_u8(n: u32) -> Option<u8> {
    if n > 255 { None } else { Some(n as u8) }
}

/// Convert a temperature from Fahrenheit to Celsius and classify it.
pub fn temp_classify(fahrenheit: f64) -> &'static str {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    match celsius as i32 {
        i32::MIN..=-1 => "freezing",
        0..=15 => "cold",
        16..=25 => "comfortable",
        26..=35 => "warm",
        _ => "hot",
    }
}

/// Map a value from one range to another (linear interpolation).
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let t = (value - from_min) / (from_max - from_min);
    to_min + t * (to_max - to_min)
}
