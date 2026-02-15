// ============================================
// Level 7, Project 1: Advanced Traits
// ============================================
use std::fmt;
use std::ops::{Add, Index, Mul, Neg, Sub};

// Topic 1: Associated Types
/// A container trait with an associated type.
pub trait Container {
    type Item;
    fn items(&self) -> &[Self::Item];
    fn len(&self) -> usize {
        self.items().len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A named list that implements Container.
pub struct NamedList {
    pub name: String,
    elements: Vec<String>,
}
impl NamedList {
    pub fn new(name: &str, elements: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            elements,
        }
    }
}
impl Container for NamedList {
    type Item = String;
    fn items(&self) -> &[Self::Item] {
        &self.elements
    }
}

/// An iterator that yields squared values.
pub struct SquareIter {
    current: u32,
    max: u32,
}
impl SquareIter {
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}
impl Iterator for SquareIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current > self.max {
            None
        } else {
            Some(self.current * self.current)
        }
    }
}

// Topic 2: Operator Overloading
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Self::Output {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

/// A matrix that supports indexing by (row, col).
pub struct Matrix {
    data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}
impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Self { data, rows, cols }
    }
}
impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

// Topic 3: Supertraits
/// A trait requiring Display + Debug.
pub trait Printable: fmt::Display + fmt::Debug {
    fn print_debug(&self) -> String {
        format!("{:?}", self)
    }
    fn print_display(&self) -> String {
        format!("{}", self)
    }
}
#[derive(Debug)]
pub struct LabeledValue {
    pub label: String,
    pub value: f64,
}
impl LabeledValue {
    pub fn new(label: &str, value: f64) -> Self {
        Self {
            label: label.to_string(),
            value,
        }
    }
}
impl fmt::Display for LabeledValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.label, self.value)
    }
}
impl Printable for LabeledValue {}
/// A trait hierarchy: Drawable requires Sized + Display.
pub trait Drawable: fmt::Display {
    fn draw(&self) -> String;
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

// Topic 4: Newtype
/// Wrapper around Vec<String> to implement Display.
pub struct Wrapper(pub Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);
impl Email {
    pub fn new(s: &str) -> Result<Self, String> {
        if s.contains('@') && s.contains('.') {
            Ok(Self(s.to_string()))
        } else {
            Err("invalid email".into())
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn domain(&self) -> &str {
        self.0.split('@').last().unwrap_or("")
    }
}
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(pub f64);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kilometers(pub f64);
impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        Meters(km.0 * 1000.0)
    }
}
impl From<Meters> for Kilometers {
    fn from(m: Meters) -> Self {
        Kilometers(m.0 / 1000.0)
    }
}
impl Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Self) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

// Topic 5: Fully Qualified Syntax
pub trait Pilot {
    fn fly(&self) -> String;
}
pub trait Wizard {
    fn fly(&self) -> String;
}
pub struct Human;
impl Human {
    pub fn fly(&self) -> String {
        "*waving arms*".into()
    }
}
impl Pilot for Human {
    fn fly(&self) -> String {
        "This is your captain speaking from the cockpit".into()
    }
}
impl Wizard for Human {
    fn fly(&self) -> String {
        "Up! (using magic)".into()
    }
}
/// Trait with an associated function (no self).
pub trait Animal {
    fn name() -> String;
}
pub struct Dog;
impl Dog {
    pub fn name() -> String {
        "Spot the dog".into()
    }
}
impl Animal for Dog {
    fn name() -> String {
        "canine (animal trait)".into()
    }
}

// Topic 6: Blanket Implementations
/// A trait that provides a formatted summary.
pub trait Summarize {
    fn summarize(&self) -> String;
}
impl<T: fmt::Display> Summarize for T {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)
    }
}
/// A trait for converting to a prettified JSON-like string.
pub trait PrettyPrint {
    fn pretty(&self) -> String;
}
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl<T: fmt::Debug> PrettyPrint for T {
    fn pretty(&self) -> String {
        format!("{:#?}", self)
    }
}
