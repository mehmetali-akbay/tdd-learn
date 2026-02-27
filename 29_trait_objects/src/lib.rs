// ============================================
// Level 7, Project 3: Trait Objects — OOP & Dynamic Dispatch
// Learn: dyn Trait, Box<dyn Trait>, State pattern, Strategy, Any
// ============================================

use std::any::Any;
use std::fmt;

// ============================================
// Topic 1: Trait Objects Basics
// Learn: dyn Trait, Box<dyn Trait>, object safety
// ============================================

pub trait Drawable {
    fn draw(&self) -> String;
    fn name(&self) -> &str;
}

pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

pub struct Square {
    pub side: f64,
}

impl Drawable for Square {
    fn draw(&self) -> String {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

pub struct Text {
    pub content: String,
}

impl Drawable for Text {
    fn draw(&self) -> String {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

/// Create a Drawable from a description string.
pub fn create_drawable(kind: &str) -> Box<dyn Drawable> {
    todo!()
}

// ============================================
// Topic 2: Heterogeneous Collections
// Learn: Vec<Box<dyn Trait>>, processing mixed types
// ============================================

/// Draw all shapes and return their descriptions.
pub fn draw_all(items: &[Box<dyn Drawable>]) -> Vec<String> {
    todo!()
}

/// Find a drawable by name.
pub fn find_by_name<'a>(items: &'a [Box<dyn Drawable>], name: &str) -> Option<&'a dyn Drawable> {
    todo!()
}

/// A canvas that holds mixed drawable items.
pub struct Canvas {
    items: Vec<Box<dyn Drawable>>,
}

impl Canvas {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&mut self, item: Box<dyn Drawable>) {
        todo!()
    }

    pub fn render(&self) -> Vec<String> {
        todo!()
    }

    pub fn count(&self) -> usize {
        todo!()
    }
}

// ============================================
// Topic 3: State Pattern
// Learn: State machine with trait objects (Rust Book Ch 18.3)
// ============================================

trait PostState {
    fn request_review(self: Box<Self>) -> Box<dyn PostState>;
    fn approve(self: Box<Self>) -> Box<dyn PostState>;
    fn reject(self: Box<Self>) -> Box<dyn PostState>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn state_name(&self) -> &str;
}

struct Draft;
struct PendingReview;
struct Published;

impl PostState for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn approve(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn reject(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn state_name(&self) -> &str {
        todo!()
    }
}

impl PostState for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn approve(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn reject(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn state_name(&self) -> &str {
        todo!()
    }
}

impl PostState for Published {
    fn request_review(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn approve(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn reject(self: Box<Self>) -> Box<dyn PostState> {
        todo!()
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        todo!()
    }
    fn state_name(&self) -> &str {
        todo!()
    }
}

pub struct Post {
    state: Option<Box<dyn PostState>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_text(&mut self, text: &str) {
        todo!()
    }

    pub fn content(&self) -> &str {
        todo!()
    }

    pub fn request_review(&mut self) {
        todo!()
    }

    pub fn approve(&mut self) {
        todo!()
    }

    pub fn reject(&mut self) {
        todo!()
    }

    pub fn state_name(&self) -> &str {
        todo!()
    }
}

// ============================================
// Topic 4: Strategy Pattern
// Learn: Swappable algorithms via trait objects
// ============================================

pub trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
    fn name(&self) -> &str;
}

pub struct BubbleSort;
pub struct InsertionSort;
pub struct QuickSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

impl SortStrategy for InsertionSort {
    fn sort(&self, data: &mut Vec<i32>) {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

pub struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    pub fn new(strategy: Box<dyn SortStrategy>) -> Self {
        todo!()
    }

    pub fn sort(&self, data: &mut Vec<i32>) {
        todo!()
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
        todo!()
    }

    pub fn strategy_name(&self) -> &str {
        todo!()
    }
}

// ============================================
// Topic 5: Any & Downcasting
// Learn: Runtime type info, downcast_ref
// ============================================

/// Store any value and retrieve it by type.
pub struct AnyBox {
    value: Box<dyn Any>,
}

impl AnyBox {
    pub fn new<T: Any>(value: T) -> Self {
        todo!()
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        todo!()
    }

    pub fn is<T: Any>(&self) -> bool {
        todo!()
    }
}

/// A heterogeneous collection with type-safe retrieval.
pub struct TypeMap {
    entries: Vec<Box<dyn Any>>,
}

impl TypeMap {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert<T: Any>(&mut self, value: T) {
        todo!()
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        todo!()
    }

    pub fn count(&self) -> usize {
        todo!()
    }
}

// ============================================
// Topic 6: Advanced — Static vs Dynamic Dispatch
// Learn: Monomorphization vs vtable, performance trade-offs
// ============================================

pub trait Formatter {
    fn format(&self, input: &str) -> String;
}

pub struct UpperFormatter;
pub struct LowerFormatter;
pub struct TitleFormatter;

impl Formatter for UpperFormatter {
    fn format(&self, input: &str) -> String {
        todo!()
    }
}

impl Formatter for LowerFormatter {
    fn format(&self, input: &str) -> String {
        todo!()
    }
}

impl Formatter for TitleFormatter {
    fn format(&self, input: &str) -> String {
        todo!()
    }
}

/// Static dispatch — monomorphized at compile time.
pub fn format_static(input: &str, formatter: &impl Formatter) -> String {
    todo!()
}

/// Dynamic dispatch — uses vtable at runtime.
pub fn format_dynamic(input: &str, formatter: &dyn Formatter) -> String {
    todo!()
}

/// Process a list of formatters (requires dynamic dispatch).
pub fn format_all(input: &str, formatters: &[&dyn Formatter]) -> Vec<String> {
    todo!()
}

// ============================================
// Tests
// ============================================
