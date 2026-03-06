// ============================================
// Level 7, Project 1: Advanced Traits
// Learn: Associated types, operator overloading, supertraits, newtype
// ============================================

use std::fmt;
use std::ops::{Add, Index, Mul, Neg, Sub};

// ============================================
// Topic 1: Associated Types
// Learn: type Item, Iterator pattern
// ============================================

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
        todo!()
    }
}

impl Container for NamedList {
    type Item = String;

    fn items(&self) -> &[Self::Item] {
        todo!()
    }
}

/// An iterator that yields squared values.
pub struct SquareIter {
    current: u32,
    max: u32,
}

impl SquareIter {
    pub fn new(max: u32) -> Self {
        todo!()
    }
}

impl Iterator for SquareIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// ============================================
// Topic 2: Operator Overloading
// Learn: Add, Sub, Mul, Neg, Index
// ============================================

/// A 2D vector supporting arithmetic operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        todo!()
    }

    pub fn magnitude(&self) -> f64 {
        todo!()
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Self::Output {
        todo!()
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        todo!()
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
        todo!()
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        todo!()
    }
}

// ============================================
// Topic 3: Supertraits
// Learn: trait A: B + C
// ============================================

/// A trait requiring Display + Debug.
pub trait Printable: fmt::Display + fmt::Debug {
    fn print_debug(&self) -> String {
        format!("{:?}", self)
    }
    fn print_display(&self) -> String {
        format!("{}", self)
    }
}

/// A labeled value that implements Printable.
#[derive(Debug)]
pub struct LabeledValue {
    pub label: String,
    pub value: f64,
}

impl LabeledValue {
    pub fn new(label: &str, value: f64) -> Self {
        todo!()
    }
}

impl fmt::Display for LabeledValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Printable for LabeledValue {}

/// A trait hierarchy: Drawable requires Sized + Display.
pub trait Drawable: fmt::Display {
    fn draw(&self) -> String;
    fn bounding_box(&self) -> (f64, f64, f64, f64); // x, y, width, height
}

// ============================================
// Topic 4: Newtype Pattern
// Learn: Wrapping types, external traits on external types
// ============================================

/// Wrapper around Vec<String> to implement Display.
pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Email newtype that validates format.
#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn as_str(&self) -> &str {
        todo!()
    }

    pub fn domain(&self) -> &str {
        todo!()
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Meters newtype for type-safe arithmetic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(pub f64);

/// Kilometers newtype.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kilometers(pub f64);

impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        todo!()
    }
}

impl From<Meters> for Kilometers {
    fn from(m: Meters) -> Self {
        todo!()
    }
}

impl Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// ============================================
// Topic 5: Fully Qualified Syntax
// Learn: <Type as Trait>::method(), disambiguation
// ============================================

pub trait Pilot {
    fn fly(&self) -> String;
}

pub trait Wizard {
    fn fly(&self) -> String;
}

pub struct Human;

impl Human {
    pub fn fly(&self) -> String {
        todo!()
    }
}

impl Pilot for Human {
    fn fly(&self) -> String {
        todo!()
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        todo!()
    }
}

/// Trait with an associated function (no self).
pub trait Animal {
    fn name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn name() -> String {
        todo!()
    }
}

impl Animal for Dog {
    fn name() -> String {
        todo!()
    }
}

// ============================================
// Topic 6: Advanced — Blanket Implementations
// Learn: Conditional impls, impl<T: Display>
// ============================================

/// A trait that provides a formatted summary.
pub trait Summarize {
    fn summarize(&self) -> String;
}

/// Blanket implementation: anything that implements Display gets Summarize.
impl<T: fmt::Display> Summarize for T {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)
    }
}

/// A trait for converting to a prettified JSON-like string.
pub trait PrettyPrint {
    fn pretty(&self) -> String;
}

/// A point that implements Display and thus gets Summarize for free.
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        todo!()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Implement PrettyPrint for anything that is Debug.
impl<T: fmt::Debug> PrettyPrint for T {
    fn pretty(&self) -> String {
        format!("{:#?}", self)
    }
}

// ============================================
// Tests
// ============================================
