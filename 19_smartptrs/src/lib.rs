// ============================================
// Level 4, Project 4: Smart Pointers — Box, Rc, RefCell, Arc
// Learn: Heap allocation, shared ownership, interior mutability
// ============================================

use std::cell::RefCell;
use std::rc::Rc;

/// Type alias for listener callbacks in the observer pattern
type Listener = Rc<RefCell<dyn FnMut(&str)>>;

// ============================================
// Topic 1: Box<T> — Heap Allocation
// Learn: Box for recursive types, trait objects
// ============================================

/// A recursive linked list using Box
#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn new() -> Self {
        todo!()
    }

    /// Push to the front
    pub fn push(self, value: T) -> Self {
        todo!()
    }

    /// Count elements
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Get the head value
    pub fn head(&self) -> Option<&T> {
        todo!()
    }
}

impl<T: Clone> List<T> {
    /// Convert to a Vec
    pub fn to_vec(&self) -> Vec<T> {
        todo!()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        todo!()
    }
}

/// A binary tree using Box
#[derive(Debug, PartialEq)]
pub enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

impl<T: std::ops::Add<Output = T> + Copy> Tree<T> {
    /// Sum all leaf values
    pub fn sum(&self) -> T {
        todo!()
    }
}

impl<T> Tree<T> {
    /// Count all leaves
    pub fn count_leaves(&self) -> usize {
        todo!()
    }

    /// Tree depth
    pub fn depth(&self) -> usize {
        todo!()
    }
}

/// Trait object via Box: any shape with an area
pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> &str {
        todo!()
    }
}

/// Total area of a collection of boxed shapes
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!()
}

// ============================================
// Topic 2: Rc<T> — Shared Ownership
// Learn: Reference counting, Rc::clone, strong_count
// ============================================

/// A shared config that multiple components can reference
#[derive(Debug)]
pub struct SharedConfig {
    pub name: String,
    pub debug: bool,
}

impl SharedConfig {
    pub fn new(name: &str, debug: bool) -> Rc<Self> {
        todo!()
    }
}

/// A component that holds a shared config
#[derive(Debug)]
pub struct Component {
    pub label: String,
    pub config: Rc<SharedConfig>,
}

impl Component {
    pub fn new(label: &str, config: Rc<SharedConfig>) -> Self {
        todo!()
    }

    pub fn is_debug(&self) -> bool {
        todo!()
    }
}

/// A shared graph node: value + list of shared children
#[derive(Debug)]
pub struct GraphNode {
    pub value: i32,
    pub children: Vec<Rc<GraphNode>>,
}

impl GraphNode {
    pub fn leaf(value: i32) -> Rc<Self> {
        todo!()
    }

    pub fn with_children(value: i32, children: Vec<Rc<GraphNode>>) -> Rc<Self> {
        todo!()
    }

    /// Count all descendants (including self)
    pub fn count_nodes(node: &Rc<GraphNode>) -> usize {
        todo!()
    }
}

// ============================================
// Topic 3: RefCell<T> — Interior Mutability
// Learn: Borrow checking at runtime, borrow/borrow_mut
// ============================================

/// A logger that can be mutated through shared references
#[derive(Debug)]
pub struct Logger {
    messages: RefCell<Vec<String>>,
}

impl Logger {
    pub fn new() -> Self {
        todo!()
    }

    /// Log a message (note: &self, not &mut self!)
    pub fn log(&self, msg: &str) {
        todo!()
    }

    /// Get all logged messages
    pub fn messages(&self) -> Vec<String> {
        todo!()
    }

    /// Count logged messages
    pub fn count(&self) -> usize {
        todo!()
    }

    /// Clear all messages
    pub fn clear(&self) {
        todo!()
    }
}

impl Default for Logger {
    fn default() -> Self {
        todo!()
    }
}

/// A mutable counter that can be shared via Rc<RefCell<>>
#[derive(Debug)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new(initial: i32) -> Rc<RefCell<Self>> {
        todo!()
    }

    pub fn increment(&mut self) {
        todo!()
    }

    pub fn decrement(&mut self) {
        todo!()
    }

    pub fn value(&self) -> i32 {
        todo!()
    }
}

// ============================================
// Topic 4: Rc<RefCell<T>> — Shared Mutable State
// Learn: Combining Rc and RefCell for shared mutable data
// ============================================

/// A bank account with shared mutable balance
#[derive(Debug)]
pub struct BankAccount {
    pub owner: String,
    pub balance: RefCell<f64>,
}

impl BankAccount {
    pub fn new(owner: &str, balance: f64) -> Rc<Self> {
        todo!()
    }

    pub fn deposit(&self, amount: f64) -> Result<f64, String> {
        todo!()
    }

    pub fn withdraw(&self, amount: f64) -> Result<f64, String> {
        todo!()
    }

    pub fn get_balance(&self) -> f64 {
        todo!()
    }
}

/// Transfer between two accounts
pub fn transfer(from: &BankAccount, to: &BankAccount, amount: f64) -> Result<(), String> {
    todo!()
}

// ============================================
// Topic 5: Drop Trait & Custom Smart Pointers
// Learn: Drop for cleanup, custom wrapper types
// ============================================

use std::cell::Cell;

/// A simple wrapper that tracks how many instances exist
pub struct Tracked<T> {
    pub value: T,
    counter: Rc<Cell<usize>>,
}

impl<T> Tracked<T> {
    pub fn new(value: T, counter: Rc<Cell<usize>>) -> Self {
        todo!()
    }
}

impl<T> Drop for Tracked<T> {
    fn drop(&mut self) {
        todo!()
    }
}

/// A wrapper that remembers the last N values it held
#[derive(Debug)]
pub struct History<T: Clone> {
    current: T,
    history: Vec<T>,
    max_history: usize,
}

impl<T: Clone> History<T> {
    pub fn new(initial: T, max_history: usize) -> Self {
        todo!()
    }

    pub fn set(&mut self, new_value: T) {
        todo!()
    }

    pub fn get(&self) -> &T {
        todo!()
    }

    pub fn undo(&mut self) -> bool {
        todo!()
    }

    pub fn history_len(&self) -> usize {
        todo!()
    }
}

// ============================================
// Topic 6: Advanced — Observer Pattern
// Learn: Rc<RefCell<>> for callbacks, dynamic dispatch
// ============================================

/// An event emitter that notifies observers
pub struct EventEmitter {
    listeners: Vec<Listener>,
}

impl EventEmitter {
    pub fn new() -> Self {
        todo!()
    }

    pub fn on(&mut self, callback: Listener) {
        todo!()
    }

    pub fn emit(&self, event: &str) {
        todo!()
    }

    pub fn listener_count(&self) -> usize {
        todo!()
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        todo!()
    }
}
