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
        List::Nil
    }

    /// Push to the front
    pub fn push(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }

    /// Count elements
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    /// Get the head value
    pub fn head(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(val, _) => Some(val),
        }
    }
}

impl<T: Clone> List<T> {
    /// Convert to a Vec
    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = self;
        loop {
            match current {
                List::Nil => break,
                List::Cons(val, tail) => {
                    result.push(val.clone());
                    current = tail;
                }
            }
        }
        result
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
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
        match self {
            Tree::Leaf(v) => *v,
            Tree::Node(left, right) => left.sum() + right.sum(),
        }
    }
}

impl<T> Tree<T> {
    /// Count all leaves
    pub fn count_leaves(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Node(left, right) => left.count_leaves() + right.count_leaves(),
        }
    }

    /// Tree depth
    pub fn depth(&self) -> usize {
        match self {
            Tree::Leaf(_) => 0,
            Tree::Node(left, right) => 1 + left.depth().max(right.depth()),
        }
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
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "circle"
    }
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> &str {
        "square"
    }
}

/// Total area of a collection of boxed shapes
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
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
        Rc::new(SharedConfig {
            name: name.to_string(),
            debug,
        })
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
        Component {
            label: label.to_string(),
            config,
        }
    }

    pub fn is_debug(&self) -> bool {
        self.config.debug
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
        Rc::new(GraphNode {
            value,
            children: vec![],
        })
    }

    pub fn with_children(value: i32, children: Vec<Rc<GraphNode>>) -> Rc<Self> {
        Rc::new(GraphNode { value, children })
    }

    /// Count all descendants (including self)
    pub fn count_nodes(node: &Rc<GraphNode>) -> usize {
        1 + node
            .children
            .iter()
            .map(GraphNode::count_nodes)
            .sum::<usize>()
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
        Logger {
            messages: RefCell::new(Vec::new()),
        }
    }

    /// Log a message (note: &self, not &mut self!)
    pub fn log(&self, msg: &str) {
        self.messages.borrow_mut().push(msg.to_string());
    }

    /// Get all logged messages
    pub fn messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }

    /// Count logged messages
    pub fn count(&self) -> usize {
        self.messages.borrow().len()
    }

    /// Clear all messages
    pub fn clear(&self) {
        self.messages.borrow_mut().clear();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// A mutable counter that can be shared via Rc<RefCell<>>
#[derive(Debug)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new(initial: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Counter { value: initial }))
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn decrement(&mut self) {
        self.value -= 1;
    }

    pub fn value(&self) -> i32 {
        self.value
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
        Rc::new(BankAccount {
            owner: owner.to_string(),
            balance: RefCell::new(balance),
        })
    }

    pub fn deposit(&self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("deposit must be positive".to_string());
        }
        *self.balance.borrow_mut() += amount;
        Ok(*self.balance.borrow())
    }

    pub fn withdraw(&self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("withdrawal must be positive".to_string());
        }
        let current = *self.balance.borrow();
        if amount > current {
            return Err("insufficient funds".to_string());
        }
        *self.balance.borrow_mut() -= amount;
        Ok(*self.balance.borrow())
    }

    pub fn get_balance(&self) -> f64 {
        *self.balance.borrow()
    }
}

/// Transfer between two accounts
pub fn transfer(from: &BankAccount, to: &BankAccount, amount: f64) -> Result<(), String> {
    from.withdraw(amount)?;
    to.deposit(amount)?;
    Ok(())
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
        counter.set(counter.get() + 1);
        Tracked { value, counter }
    }
}

impl<T> Drop for Tracked<T> {
    fn drop(&mut self) {
        self.counter.set(self.counter.get() - 1);
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
        History {
            current: initial,
            history: Vec::new(),
            max_history,
        }
    }

    pub fn set(&mut self, new_value: T) {
        self.history.push(self.current.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        self.current = new_value;
    }

    pub fn get(&self) -> &T {
        &self.current
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.current = prev;
            true
        } else {
            false
        }
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
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
        EventEmitter {
            listeners: Vec::new(),
        }
    }

    pub fn on(&mut self, callback: Listener) {
        self.listeners.push(callback);
    }

    pub fn emit(&self, event: &str) {
        for listener in &self.listeners {
            (listener.borrow_mut())(event);
        }
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.len()
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}
