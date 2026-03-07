// ============================================
// Level 7, Project 3: Trait Objects — OOP & Dynamic Dispatch
// ============================================
use std::any::Any;

// ============================================
// Topic 1: Trait Objects Basics
// Learn: dyn Trait, Box<dyn Trait>, object safety, trait methods
// ============================================

pub trait Drawable {
    fn draw(&self) -> String;
    fn name(&self) -> &str;
    fn area(&self) -> f64;
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
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
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
    fn area(&self) -> f64 {
        self.side * self.side
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
    fn area(&self) -> f64 {
        0.0
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Drawable for Triangle {
    fn draw(&self) -> String {
        format!(
            "Drawing triangle with base {} and height {}",
            self.base, self.height
        )
    }
    fn name(&self) -> &str {
        "triangle"
    }
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

/// Create a Drawable from a description string.
pub fn create_drawable(kind: &str) -> Box<dyn Drawable> {
    match kind {
        "circle" => Box::new(Circle { radius: 1.0 }),
        "square" => Box::new(Square { side: 1.0 }),
        "triangle" => Box::new(Triangle {
            base: 1.0,
            height: 1.0,
        }),
        _ => Box::new(Text {
            content: kind.to_string(),
        }),
    }
}

/// Get the total area of a slice of drawables.
pub fn total_area(items: &[Box<dyn Drawable>]) -> f64 {
    items.iter().map(|i| i.area()).sum()
}

/// Get the largest drawable by area (returns its name).
pub fn largest_by_area(items: &[Box<dyn Drawable>]) -> Option<String> {
    items
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
        .map(|i| i.name().to_string())
}

// ============================================
// Topic 2: Heterogeneous Collections
// Learn: Vec<Box<dyn Trait>>, processing mixed types, Canvas
// ============================================

/// Draw all shapes and return their descriptions.
pub fn draw_all(items: &[Box<dyn Drawable>]) -> Vec<String> {
    items.iter().map(|i| i.draw()).collect()
}

/// Find a drawable by name.
pub fn find_by_name<'a>(items: &'a [Box<dyn Drawable>], name: &str) -> Option<&'a dyn Drawable> {
    items.iter().find(|i| i.name() == name).map(|i| i.as_ref())
}

/// Get all names from a collection of drawables.
pub fn get_all_names(items: &[Box<dyn Drawable>]) -> Vec<String> {
    items.iter().map(|i| i.name().to_string()).collect()
}

/// Filter drawables whose area exceeds a threshold.
pub fn filter_by_min_area(items: &[Box<dyn Drawable>], min_area: f64) -> Vec<String> {
    items
        .iter()
        .filter(|i| i.area() >= min_area)
        .map(|i| i.name().to_string())
        .collect()
}

/// A canvas that holds mixed drawable items.
pub struct Canvas {
    items: Vec<Box<dyn Drawable>>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn names(&self) -> Vec<String> {
        self.items.iter().map(|i| i.name().to_string()).collect()
    }

    pub fn total_area(&self) -> f64 {
        self.items.iter().map(|i| i.area()).sum()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn remove_by_name(&mut self, name: &str) -> bool {
        let before = self.items.len();
        self.items.retain(|i| i.name() != name);
        self.items.len() < before
    }
}

// ============================================
// Topic 3: State Pattern
// Learn: State machine with trait objects, transitions
// ============================================

trait PostState {
    fn request_review(self: Box<Self>) -> Box<dyn PostState>;
    fn approve(self: Box<Self>) -> Box<dyn PostState>;
    fn reject(self: Box<Self>) -> Box<dyn PostState>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn state_name(&self) -> &str;
    fn is_published(&self) -> bool {
        false
    }
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
    fn is_published(&self) -> bool {
        true
    }
}

pub struct Post {
    state: Option<Box<dyn PostState>>,
    content: String,
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn is_published(&self) -> bool {
        self.state.as_ref().unwrap().is_published()
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
pub struct SelectionSort;

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

impl SortStrategy for SelectionSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let n = data.len();
        for i in 0..n {
            let mut min_idx = i;
            for j in (i + 1)..n {
                if data[j] < data[min_idx] {
                    min_idx = j;
                }
            }
            data.swap(i, min_idx);
        }
    }
    fn name(&self) -> &str {
        "selection_sort"
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

/// Sort data using multiple strategies and return all results.
pub fn sort_with_all(data: &[i32], strategies: &[&dyn SortStrategy]) -> Vec<Vec<i32>> {
    strategies
        .iter()
        .map(|s| {
            let mut d = data.to_vec();
            s.sort(&mut d);
            d
        })
        .collect()
}

// ============================================
// Topic 5: Any & Downcasting
// Learn: Runtime type info, downcast_ref, type-safe heterogeneous storage
// ============================================

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

    pub fn replace<T: Any>(&mut self, value: T) -> Box<dyn Any> {
        std::mem::replace(&mut self.value, Box::new(value))
    }
}

/// A heterogeneous collection with type-safe retrieval.
pub struct TypeMap {
    entries: Vec<Box<dyn Any>>,
}

impl Default for TypeMap {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn get_all<T: Any>(&self) -> Vec<&T> {
        self.entries
            .iter()
            .filter_map(|e| e.downcast_ref::<T>())
            .collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn count_of<T: Any>(&self) -> usize {
        self.entries.iter().filter(|e| e.is::<T>()).count()
    }

    pub fn remove_all_of<T: Any>(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|e| !e.is::<T>());
        before - self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ============================================
// Topic 6: Static vs Dynamic Dispatch
// Learn: Monomorphization vs vtable, &impl Trait vs &dyn Trait
// ============================================

pub trait Formatter {
    fn format(&self, input: &str) -> String;
}

pub struct UpperFormatter;
pub struct LowerFormatter;
pub struct TitleFormatter;
pub struct TrimFormatter;
pub struct ReverseFormatter;

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

impl Formatter for TrimFormatter {
    fn format(&self, input: &str) -> String {
        input.trim().to_string()
    }
}

impl Formatter for ReverseFormatter {
    fn format(&self, input: &str) -> String {
        input.chars().rev().collect()
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

/// Chain formatters: apply each in sequence.
pub fn format_chain(input: &str, formatters: &[&dyn Formatter]) -> String {
    formatters
        .iter()
        .fold(input.to_string(), |acc, f| f.format(&acc))
}

// ============================================
// Topic 7: Observer Pattern
// Learn: Event-driven design with trait objects, decoupled communication
// ============================================

pub trait Observer {
    fn on_event(&mut self, event: &str);
    fn name(&self) -> &str;
}

pub struct Logger {
    pub logs: Vec<String>,
    label: String,
}

impl Logger {
    pub fn new(label: &str) -> Self {
        Self {
            logs: Vec::new(),
            label: label.to_string(),
        }
    }
}

impl Observer for Logger {
    fn on_event(&mut self, event: &str) {
        self.logs.push(format!("[{}] {}", self.label, event));
    }
    fn name(&self) -> &str {
        &self.label
    }
}

pub struct Counter {
    pub count: usize,
    label: String,
}

impl Counter {
    pub fn new(label: &str) -> Self {
        Self {
            count: 0,
            label: label.to_string(),
        }
    }
}

impl Observer for Counter {
    fn on_event(&mut self, _event: &str) {
        self.count += 1;
    }
    fn name(&self) -> &str {
        &self.label
    }
}

pub struct EventBus {
    observers: Vec<Box<dyn Observer>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn emit(&mut self, event: &str) {
        for obs in &mut self.observers {
            obs.on_event(event);
        }
    }

    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    pub fn observer_names(&self) -> Vec<String> {
        self.observers.iter().map(|o| o.name().to_string()).collect()
    }

    pub fn unsubscribe(&mut self, name: &str) -> bool {
        let before = self.observers.len();
        self.observers.retain(|o| o.name() != name);
        self.observers.len() < before
    }
}

// ============================================
// Topic 8: Command Pattern
// Learn: Encapsulating actions as trait objects, undo/redo
// ============================================

pub trait Command {
    fn execute(&self, state: &mut Vec<String>);
    fn undo(&self, state: &mut Vec<String>);
    fn description(&self) -> String;
}

pub struct AddCommand {
    pub item: String,
}

impl AddCommand {
    pub fn new(item: &str) -> Self {
        Self {
            item: item.to_string(),
        }
    }
}

impl Command for AddCommand {
    fn execute(&self, state: &mut Vec<String>) {
        state.push(self.item.clone());
    }
    fn undo(&self, state: &mut Vec<String>) {
        if let Some(pos) = state.iter().rposition(|s| s == &self.item) {
            state.remove(pos);
        }
    }
    fn description(&self) -> String {
        format!("Add '{}'", self.item)
    }
}

pub struct RemoveCommand {
    pub item: String,
}

impl RemoveCommand {
    pub fn new(item: &str) -> Self {
        Self {
            item: item.to_string(),
        }
    }
}

impl Command for RemoveCommand {
    fn execute(&self, state: &mut Vec<String>) {
        if let Some(pos) = state.iter().position(|s| s == &self.item) {
            state.remove(pos);
        }
    }
    fn undo(&self, state: &mut Vec<String>) {
        state.push(self.item.clone());
    }
    fn description(&self) -> String {
        format!("Remove '{}'", self.item)
    }
}

pub struct ClearCommand;

impl Command for ClearCommand {
    fn execute(&self, state: &mut Vec<String>) {
        state.clear();
    }
    fn undo(&self, _state: &mut Vec<String>) {
        // Clear cannot be undone without storing backup — intentional limitation
    }
    fn description(&self) -> String {
        "Clear all".to_string()
    }
}

pub struct CommandHistory {
    commands: Vec<Box<dyn Command>>,
    state: Vec<String>,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            state: Vec::new(),
        }
    }

    pub fn execute(&mut self, command: Box<dyn Command>) {
        command.execute(&mut self.state);
        self.commands.push(command);
    }

    pub fn undo_last(&mut self) -> bool {
        if let Some(cmd) = self.commands.pop() {
            cmd.undo(&mut self.state);
            true
        } else {
            false
        }
    }

    pub fn state(&self) -> &[String] {
        &self.state
    }

    pub fn history_len(&self) -> usize {
        self.commands.len()
    }

    pub fn history_descriptions(&self) -> Vec<String> {
        self.commands.iter().map(|c| c.description()).collect()
    }
}

// ============================================
// Topic 9: Trait Object Composition & Multi-trait
// Learn: Combining traits, supertrait objects, trait casting
// ============================================

pub trait Describable {
    fn describe(&self) -> String;
}

pub trait Priceable {
    fn price(&self) -> f64;
}

pub trait Product: Describable + Priceable {
    fn product_id(&self) -> u32;
}

pub struct Book {
    pub id: u32,
    pub title: String,
    pub cost: f64,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}", self.title)
    }
}

impl Priceable for Book {
    fn price(&self) -> f64 {
        self.cost
    }
}

impl Product for Book {
    fn product_id(&self) -> u32 {
        self.id
    }
}

pub struct Gadget {
    pub id: u32,
    pub name: String,
    pub cost: f64,
}

impl Describable for Gadget {
    fn describe(&self) -> String {
        format!("Gadget: {}", self.name)
    }
}

impl Priceable for Gadget {
    fn price(&self) -> f64 {
        self.cost
    }
}

impl Product for Gadget {
    fn product_id(&self) -> u32 {
        self.id
    }
}

/// Compute total price of a collection of products.
pub fn total_price(products: &[Box<dyn Product>]) -> f64 {
    products.iter().map(|p| p.price()).sum()
}

/// Describe all products.
pub fn describe_all(products: &[Box<dyn Product>]) -> Vec<String> {
    products.iter().map(|p| p.describe()).collect()
}

/// Find product by ID.
pub fn find_product_by_id(products: &[Box<dyn Product>], id: u32) -> Option<String> {
    products
        .iter()
        .find(|p| p.product_id() == id)
        .map(|p| p.describe())
}

/// Filter products above a certain price.
pub fn products_above_price(products: &[Box<dyn Product>], threshold: f64) -> Vec<String> {
    products
        .iter()
        .filter(|p| p.price() > threshold)
        .map(|p| p.describe())
        .collect()
}

/// Cheapest product description.
pub fn cheapest_product(products: &[Box<dyn Product>]) -> Option<String> {
    products
        .iter()
        .min_by(|a, b| a.price().partial_cmp(&b.price()).unwrap())
        .map(|p| p.describe())
}

// ============================================
// Topic 10: Closure Trait Objects
// Learn: Box<dyn Fn>, storing closures as trait objects, callback patterns
// ============================================

pub type TransformFn = Box<dyn Fn(i32) -> i32>;

/// Apply a boxed closure to a value.
pub fn apply_transform(value: i32, transform: &TransformFn) -> i32 {
    transform(value)
}

/// Apply a series of transforms in order.
pub fn apply_pipeline(value: i32, transforms: &[TransformFn]) -> i32 {
    transforms.iter().fold(value, |acc, f| f(acc))
}

/// A registry of named transforms.
pub struct TransformRegistry {
    transforms: Vec<(String, TransformFn)>,
}

impl Default for TransformRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformRegistry {
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }

    pub fn register(&mut self, name: &str, transform: TransformFn) {
        self.transforms.push((name.to_string(), transform));
    }

    pub fn apply(&self, name: &str, value: i32) -> Option<i32> {
        self.transforms
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, f)| f(value))
    }

    pub fn apply_all(&self, value: i32) -> Vec<(String, i32)> {
        self.transforms
            .iter()
            .map(|(n, f)| (n.clone(), f(value)))
            .collect()
    }

    pub fn count(&self) -> usize {
        self.transforms.len()
    }

    pub fn names(&self) -> Vec<String> {
        self.transforms.iter().map(|(n, _)| n.clone()).collect()
    }
}

/// Predicate filter using boxed closure trait objects.
pub type PredicateFn = Box<dyn Fn(i32) -> bool>;

/// Filter items using a trait object predicate.
pub fn filter_with_predicate(items: &[i32], predicate: &PredicateFn) -> Vec<i32> {
    items.iter().copied().filter(|&x| predicate(x)).collect()
}

/// Compose two predicates with AND logic.
pub fn and_predicate(a: PredicateFn, b: PredicateFn) -> PredicateFn {
    Box::new(move |x| a(x) && b(x))
}

/// Compose two predicates with OR logic.
pub fn or_predicate(a: PredicateFn, b: PredicateFn) -> PredicateFn {
    Box::new(move |x| a(x) || b(x))
}

/// Negate a predicate.
pub fn not_predicate(p: PredicateFn) -> PredicateFn {
    Box::new(move |x| !p(x))
}
