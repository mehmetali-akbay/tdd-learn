// ============================================
// Level 7, Project 3: Trait Objects — OOP & Dynamic Dispatch
// ============================================
use std::any::Any;
use std::fmt;

// Topic 1: Trait Objects Basics
pub trait Drawable {
    fn draw(&self) -> String;
    fn name(&self) -> &str;
}
pub struct Circle {
    pub radius: f64,
}
impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Drawing circle with radius {}", self.radius)
    }
    fn name(&self) -> &str {
        "circle"
    }
}
pub struct Square {
    pub side: f64,
}
impl Drawable for Square {
    fn draw(&self) -> String {
        format!("Drawing square with side {}", self.side)
    }
    fn name(&self) -> &str {
        "square"
    }
}
pub struct Text {
    pub content: String,
}
impl Drawable for Text {
    fn draw(&self) -> String {
        format!("Drawing text: {}", self.content)
    }
    fn name(&self) -> &str {
        "text"
    }
}
/// Create a Drawable from a description string.
pub fn create_drawable(kind: &str) -> Box<dyn Drawable> {
    match kind {
        "circle" => Box::new(Circle { radius: 1.0 }),
        "square" => Box::new(Square { side: 1.0 }),
        _ => Box::new(Text {
            content: kind.to_string(),
        }),
    }
}

// Topic 2: Heterogeneous Collections
/// Draw all shapes and return their descriptions.
pub fn draw_all(items: &[Box<dyn Drawable>]) -> Vec<String> {
    items.iter().map(|i| i.draw()).collect()
}
/// Find a drawable by name.
pub fn find_by_name<'a>(items: &'a [Box<dyn Drawable>], name: &str) -> Option<&'a dyn Drawable> {
    items.iter().find(|i| i.name() == name).map(|i| i.as_ref())
}
/// A canvas that holds mixed drawable items.
pub struct Canvas {
    items: Vec<Box<dyn Drawable>>,
}
impl Canvas {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add(&mut self, item: Box<dyn Drawable>) {
        self.items.push(item);
    }
    pub fn render(&self) -> Vec<String> {
        self.items.iter().map(|i| i.draw()).collect()
    }
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

// Topic 3: State Pattern
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
        Box::new(PendingReview)
    }
    fn approve(self: Box<Self>) -> Box<dyn PostState> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn PostState> {
        self
    }
    fn state_name(&self) -> &str {
        "draft"
    }
}
impl PostState for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn PostState> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn PostState> {
        Box::new(Published)
    }
    fn reject(self: Box<Self>) -> Box<dyn PostState> {
        Box::new(Draft)
    }
    fn state_name(&self) -> &str {
        "pending_review"
    }
}
impl PostState for Published {
    fn request_review(self: Box<Self>) -> Box<dyn PostState> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn PostState> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn PostState> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn state_name(&self) -> &str {
        "published"
    }
}
pub struct Post {
    state: Option<Box<dyn PostState>>,
    content: String,
}
impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
    pub fn state_name(&self) -> &str {
        self.state.as_ref().unwrap().state_name()
    }
}

// Topic 4: Strategy Pattern
pub trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
    fn name(&self) -> &str;
}
pub struct BubbleSort;
pub struct InsertionSort;
pub struct QuickSort;
impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let n = data.len();
        for i in 0..n {
            for j in 0..n.saturating_sub(i + 1) {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
    fn name(&self) -> &str {
        "bubble_sort"
    }
}
impl SortStrategy for InsertionSort {
    fn sort(&self, data: &mut Vec<i32>) {
        for i in 1..data.len() {
            let key = data[i];
            let mut j = i;
            while j > 0 && data[j - 1] > key {
                data[j] = data[j - 1];
                j -= 1;
            }
            data[j] = key;
        }
    }
    fn name(&self) -> &str {
        "insertion_sort"
    }
}
impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        data.sort();
    }
    fn name(&self) -> &str {
        "quick_sort"
    }
}
pub struct Sorter {
    strategy: Box<dyn SortStrategy>,
}
impl Sorter {
    pub fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Self { strategy }
    }
    pub fn sort(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }
    pub fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
        self.strategy = strategy;
    }
    pub fn strategy_name(&self) -> &str {
        self.strategy.name()
    }
}

// Topic 5: Any & Downcasting
/// Store any value and retrieve it by type.
pub struct AnyBox {
    value: Box<dyn Any>,
}
impl AnyBox {
    pub fn new<T: Any>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }
    }
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }
    pub fn is<T: Any>(&self) -> bool {
        self.value.is::<T>()
    }
}

/// A heterogeneous collection with type-safe retrieval.
pub struct TypeMap {
    entries: Vec<Box<dyn Any>>,
}
impl TypeMap {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn insert<T: Any>(&mut self, value: T) {
        self.entries.push(Box::new(value));
    }
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.entries.iter().find_map(|e| e.downcast_ref::<T>())
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

// Topic 6: Static vs Dynamic Dispatch
pub trait Formatter {
    fn format(&self, input: &str) -> String;
}
pub struct UpperFormatter;
pub struct LowerFormatter;
pub struct TitleFormatter;
impl Formatter for UpperFormatter {
    fn format(&self, input: &str) -> String {
        input.to_uppercase()
    }
}
impl Formatter for LowerFormatter {
    fn format(&self, input: &str) -> String {
        input.to_lowercase()
    }
}
impl Formatter for TitleFormatter {
    fn format(&self, input: &str) -> String {
        input
            .split_whitespace()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().to_string() + &c.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
/// Static dispatch — monomorphized at compile time.
pub fn format_static(input: &str, formatter: &impl Formatter) -> String {
    formatter.format(input)
}
/// Dynamic dispatch — uses vtable at runtime.
pub fn format_dynamic(input: &str, formatter: &dyn Formatter) -> String {
    formatter.format(input)
}
/// Process a list of formatters (requires dynamic dispatch).
pub fn format_all(input: &str, formatters: &[&dyn Formatter]) -> Vec<String> {
    formatters.iter().map(|f| f.format(input)).collect()
}
