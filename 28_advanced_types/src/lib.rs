// ============================================
// Level 7, Project 2: Advanced Types
// ============================================
use std::fmt;
use std::marker::PhantomData;

// Topic 1: Type Aliases
#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    NotFound(String),
    InvalidInput(String),
    Unauthorized,
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(s) => write!(f, "not found: {}", s),
            AppError::InvalidInput(s) => write!(f, "invalid input: {}", s),
            AppError::Unauthorized => write!(f, "unauthorized"),
        }
    }
}
pub type AppResult<T> = Result<T, AppError>;

/// Look up an item by name. Returns AppResult.
pub fn find_item(items: &[(&str, i32)], name: &str) -> AppResult<i32> {
    items
        .iter()
        .find(|(k, _)| *k == name)
        .map(|(_, v)| *v)
        .ok_or_else(|| AppError::NotFound(name.to_string()))
}
/// Validate and parse input. Returns AppResult.
pub fn parse_age(input: &str) -> AppResult<u32> {
    input
        .parse::<u32>()
        .map_err(|_| AppError::InvalidInput(format!("cannot parse '{}' as age", input)))
}
/// Process requires auth. Returns AppResult.
pub fn authorized_action(is_authed: bool, action: &str) -> AppResult<String> {
    if !is_authed {
        return Err(AppError::Unauthorized);
    }
    Ok(format!("executed: {}", action))
}

// Topic 2: Newtype Validation
#[derive(Debug, Clone, PartialEq)]
pub struct NonEmptyString(String);
impl NonEmptyString {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        if s.is_empty() {
            Err("string cannot be empty")
        } else {
            Ok(Self(s.to_string()))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(f64);
impl Percentage {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if !(0.0..=100.0).contains(&value) {
            Err("percentage must be 0-100")
        } else {
            Ok(Self(value))
        }
    }
    pub fn value(&self) -> f64 {
        self.0
    }
    /// Convert to a 0.0-1.0 ratio.
    pub fn as_ratio(&self) -> f64 {
        self.0 / 100.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Port(u16);
impl Port {
    pub fn new(n: u16) -> Result<Self, &'static str> {
        if n == 0 {
            Err("port must be 1-65535")
        } else {
            Ok(Self(n))
        }
    }
    pub fn value(&self) -> u16 {
        self.0
    }
    pub fn is_privileged(&self) -> bool {
        self.0 < 1024
    }
}

// Topic 3: Never Type
/// Parse or panic — demonstrates a function that always returns or diverges.
pub fn parse_or_die(s: &str) -> i32 {
    s.parse()
        .unwrap_or_else(|_| panic!("failed to parse '{}'", s))
}
/// Process a Result where the error case diverges (panics).
pub fn unwrap_or_die<T: fmt::Debug>(result: Result<T, String>) -> T {
    match result {
        Ok(v) => v,
        Err(e) => panic!("unwrap failed: {}", e),
    }
}
/// A function that returns a value computed using a match
/// where one arm diverges with `continue`-like behavior.
/// Filters a list, parsing strings to i32 and keeping only Ok values.
pub fn parse_all_or_skip(items: &[&str]) -> Vec<i32> {
    items.iter().filter_map(|s| s.parse::<i32>().ok()).collect()
}

// Topic 4: DSTs & Sized
/// Print any Display type — works with both sized and unsized types.
pub fn print_it<T: fmt::Display + ?Sized>(item: &T) -> String {
    format!("{}", item)
}
/// Get the length of anything that can be a str reference.
pub fn str_len(s: &str) -> usize {
    s.len()
}
/// A generic wrapper that works with ?Sized types.
pub struct Ref<'a, T: ?Sized> {
    pub value: &'a T,
}
impl<'a, T: fmt::Display + ?Sized> Ref<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }
    pub fn display(&self) -> String {
        format!("{}", self.value)
    }
}

// Topic 5: Function Pointers
/// Apply a function pointer to each element.
pub fn map_with_fn(items: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    items.iter().map(|&x| f(x)).collect()
}
/// Apply different operations based on a string name.
pub fn get_operation(name: &str) -> Option<fn(i32, i32) -> i32> {
    match name {
        "add" => Some(|a, b| a + b),
        "sub" => Some(|a, b| a - b),
        "mul" => Some(|a, b| a * b),
        _ => None,
    }
}
/// Create a list of functions and apply them in sequence.
pub fn apply_pipeline(value: i32, fns: &[fn(i32) -> i32]) -> i32 {
    fns.iter().fold(value, |acc, f| f(acc))
}
/// Use a closure where fn pointer is expected (closure must not capture).
pub fn transform_strings(items: &[&str], f: fn(&str) -> String) -> Vec<String> {
    items.iter().map(|s| f(s)).collect()
}

// Topic 6: PhantomData & Type-State
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
        Self {
            url: None,
            method: None,
            body: None,
            _url: PhantomData,
            _method: PhantomData,
        }
    }
}
impl<M> RequestBuilder<Unset, M> {
    pub fn url(self, url: &str) -> RequestBuilder<Set, M> {
        RequestBuilder {
            url: Some(url.to_string()),
            method: self.method,
            body: self.body,
            _url: PhantomData,
            _method: PhantomData,
        }
    }
}
impl<U> RequestBuilder<U, Unset> {
    pub fn method(self, method: &str) -> RequestBuilder<U, Set> {
        RequestBuilder {
            url: self.url,
            method: Some(method.to_string()),
            body: self.body,
            _url: PhantomData,
            _method: PhantomData,
        }
    }
}
impl<U, M> RequestBuilder<U, M> {
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
}
impl RequestBuilder<Set, Set> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.unwrap(),
            method: self.method.unwrap(),
            body: self.body,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
}
