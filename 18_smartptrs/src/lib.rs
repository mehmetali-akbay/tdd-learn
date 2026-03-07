// ============================================
// Level 4, Project 4: Smart Pointers — Box, Rc, RefCell, Arc
// Learn: Heap allocation, shared ownership, interior mutability, thread safety
// ============================================

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// Type alias for listener callbacks in the observer pattern
type Listener = Rc<RefCell<dyn FnMut(&str)>>;

// ============================================
// Topic 1: Box<T> — Heap Allocation
// Learn: Box for recursive types, trait objects, tree operations
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

    /// Get the nth element (0-indexed)
    pub fn nth(&self, n: usize) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(val, tail) => {
                if n == 0 {
                    Some(val)
                } else {
                    tail.nth(n - 1)
                }
            }
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

    /// Create a list from a Vec (first element becomes head)
    pub fn from_vec(v: &[T]) -> Self {
        let mut list = List::Nil;
        for item in v.iter().rev() {
            list = List::Cons(item.clone(), Box::new(list));
        }
        list
    }

    /// Reverse the list
    pub fn reverse(&self) -> Self {
        let mut result = List::Nil;
        let mut current = self;
        loop {
            match current {
                List::Nil => break,
                List::Cons(val, tail) => {
                    result = List::Cons(val.clone(), Box::new(result));
                    current = tail;
                }
            }
        }
        result
    }

    /// Append another list to this one
    pub fn append(&self, other: &List<T>) -> Self {
        match self {
            List::Nil => {
                // Clone other
                let mut items = Vec::new();
                let mut cur = other;
                loop {
                    match cur {
                        List::Nil => break,
                        List::Cons(v, tail) => {
                            items.push(v.clone());
                            cur = tail;
                        }
                    }
                }
                List::from_vec(&items)
            }
            List::Cons(val, tail) => List::Cons(val.clone(), Box::new(tail.append(other))),
        }
    }

    /// Map a function over the list, producing a new list
    pub fn map<U: Clone, F: Fn(&T) -> U>(&self, f: F) -> List<U> {
        match self {
            List::Nil => List::Nil,
            List::Cons(val, tail) => List::Cons(f(val), Box::new(tail.map(f))),
        }
    }
}

impl<T: PartialEq> List<T> {
    /// Check if the list contains a value
    pub fn contains(&self, target: &T) -> bool {
        match self {
            List::Nil => false,
            List::Cons(val, tail) => val == target || tail.contains(target),
        }
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

    /// Collect all leaf values into a Vec (left-to-right)
    pub fn flatten(&self) -> Vec<&T> {
        match self {
            Tree::Leaf(v) => vec![v],
            Tree::Node(left, right) => {
                let mut result = left.flatten();
                result.extend(right.flatten());
                result
            }
        }
    }
}

impl<T: Clone> Tree<T> {
    /// Map a function over all leaf values
    pub fn map_leaves<U: Clone, F: Fn(&T) -> U>(&self, f: &F) -> Tree<U> {
        match self {
            Tree::Leaf(v) => Tree::Leaf(f(v)),
            Tree::Node(left, right) => {
                Tree::Node(Box::new(left.map_leaves(f)), Box::new(right.map_leaves(f)))
            }
        }
    }
}

impl<T: PartialEq> Tree<T> {
    /// Check if any leaf contains a specific value
    pub fn contains(&self, target: &T) -> bool {
        match self {
            Tree::Leaf(v) => v == target,
            Tree::Node(left, right) => left.contains(target) || right.contains(target),
        }
    }
}

impl<T: PartialOrd + Copy> Tree<T> {
    /// Find the maximum leaf value
    pub fn max_leaf(&self) -> T {
        match self {
            Tree::Leaf(v) => *v,
            Tree::Node(left, right) => {
                let lm = left.max_leaf();
                let rm = right.max_leaf();
                if lm >= rm {
                    lm
                } else {
                    rm
                }
            }
        }
    }

    /// Find the minimum leaf value
    pub fn min_leaf(&self) -> T {
        match self {
            Tree::Leaf(v) => *v,
            Tree::Node(left, right) => {
                let lm = left.min_leaf();
                let rm = right.min_leaf();
                if lm <= rm {
                    lm
                } else {
                    rm
                }
            }
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

/// Find the shape with the largest area, returning its name
pub fn largest_shape_name(shapes: &[Box<dyn Shape>]) -> Option<&str> {
    shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
        .map(|s| s.name())
}

/// Describe a shape: "circle (area=12.57)"
pub fn describe_shape(shape: &dyn Shape) -> String {
    format!("{} (area={:.2})", shape.name(), shape.area())
}

// ============================================
// Topic 2: Rc<T> — Shared Ownership
// Learn: Reference counting, Rc::clone, strong_count, graph structures
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

    /// Summarize the config
    pub fn summary(&self) -> String {
        format!(
            "Config '{}' (debug={})",
            self.name,
            if self.debug { "on" } else { "off" }
        )
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

    /// Describe the component
    pub fn describe(&self) -> String {
        format!("Component '{}' using {}", self.label, self.config.summary())
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

    /// Compute the depth of the graph (longest path from this node)
    pub fn max_depth(node: &Rc<GraphNode>) -> usize {
        if node.children.is_empty() {
            0
        } else {
            1 + node
                .children
                .iter()
                .map(GraphNode::max_depth)
                .max()
                .unwrap_or(0)
        }
    }

    /// Collect all values reachable from this node (including self)
    pub fn all_values(node: &Rc<GraphNode>) -> Vec<i32> {
        let mut result = vec![node.value];
        for child in &node.children {
            result.extend(GraphNode::all_values(child));
        }
        result
    }

    /// Search for a value in the graph, returning true if found
    pub fn find_value(node: &Rc<GraphNode>, target: i32) -> bool {
        if node.value == target {
            return true;
        }
        node.children
            .iter()
            .any(|child| GraphNode::find_value(child, target))
    }
}

// ============================================
// Topic 3: RefCell<T> — Interior Mutability
// Learn: Borrow checking at runtime, borrow/borrow_mut, shared logging
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

    /// Get the last logged message
    pub fn last(&self) -> Option<String> {
        self.messages.borrow().last().cloned()
    }

    /// Check if any message contains a substring
    pub fn contains_message(&self, substr: &str) -> bool {
        self.messages.borrow().iter().any(|m| m.contains(substr))
    }

    /// Filter messages matching a predicate
    pub fn filter_messages<F: Fn(&str) -> bool>(&self, predicate: F) -> Vec<String> {
        self.messages
            .borrow()
            .iter()
            .filter(|m| predicate(m))
            .cloned()
            .collect()
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

    /// Reset the counter to a specific value
    pub fn reset(&mut self, value: i32) {
        self.value = value;
    }
}

/// Helper to read a shared counter's value without manual borrow
pub fn get_shared_value(counter: &Rc<RefCell<Counter>>) -> i32 {
    counter.borrow().value()
}

// ============================================
// Topic 4: Rc<RefCell<T>> — Shared Mutable State
// Learn: Combining Rc and RefCell for shared mutable data, transactions
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

    /// Produce a balance summary: "Alice: $100.00"
    pub fn balance_summary(&self) -> String {
        format!("{}: ${:.2}", self.owner, self.get_balance())
    }
}

/// Transfer between two accounts
pub fn transfer(from: &BankAccount, to: &BankAccount, amount: f64) -> Result<(), String> {
    from.withdraw(amount)?;
    to.deposit(amount)?;
    Ok(())
}

/// Calculate total balance across multiple accounts
pub fn total_balance(accounts: &[Rc<BankAccount>]) -> f64 {
    accounts.iter().map(|a| a.get_balance()).sum()
}

/// A shared cache using Rc<RefCell<HashMap>>
#[derive(Debug)]
pub struct SharedCache {
    data: RefCell<HashMap<String, String>>,
}

impl SharedCache {
    pub fn new() -> Rc<Self> {
        Rc::new(SharedCache {
            data: RefCell::new(HashMap::new()),
        })
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: &str, value: &str) {
        self.data
            .borrow_mut()
            .insert(key.to_string(), value.to_string());
    }

    /// Get a value by key
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().get(key).cloned()
    }

    /// Check if a key exists
    pub fn contains(&self, key: &str) -> bool {
        self.data.borrow().contains_key(key)
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }

    /// Remove a key, returning its value
    pub fn remove(&self, key: &str) -> Option<String> {
        self.data.borrow_mut().remove(key)
    }

    /// Clear the cache
    pub fn clear(&self) {
        self.data.borrow_mut().clear();
    }
}

impl Default for SharedCache {
    fn default() -> Self {
        SharedCache {
            data: RefCell::new(HashMap::new()),
        }
    }
}

// ============================================
// Topic 5: Drop Trait & Custom Smart Pointers
// Learn: Drop for cleanup, custom wrapper types, undo history
// ============================================

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

    /// Get the current instance count
    pub fn instance_count(&self) -> usize {
        self.counter.get()
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

    /// Check if undo is possible
    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    /// Peek at the previous value without undoing
    pub fn peek_prev(&self) -> Option<&T> {
        self.history.last()
    }

    /// Get the full history as a slice
    pub fn get_history(&self) -> &[T] {
        &self.history
    }
}

/// A smart pointer that logs when it's created and dropped
pub struct Verbose<T: std::fmt::Debug> {
    pub value: T,
    log: Rc<RefCell<Vec<String>>>,
}

impl<T: std::fmt::Debug> Verbose<T> {
    pub fn new(value: T, log: Rc<RefCell<Vec<String>>>) -> Self {
        log.borrow_mut()
            .push(format!("Created: {:?}", value));
        Verbose { value, log }
    }
}

impl<T: std::fmt::Debug> Drop for Verbose<T> {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("Dropped: {:?}", self.value));
    }
}

// ============================================
// Topic 6: Observer Pattern & Callbacks
// Learn: Rc<RefCell<>> for callbacks, dynamic dispatch, event handling
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

    /// Remove all listeners
    pub fn clear_listeners(&mut self) {
        self.listeners.clear();
    }

    /// Check if any listeners are registered
    pub fn has_listeners(&self) -> bool {
        !self.listeners.is_empty()
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

/// A typed event bus that routes events by name
pub struct EventBus {
    handlers: HashMap<String, Vec<Listener>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            handlers: HashMap::new(),
        }
    }

    /// Register a handler for a specific event name
    pub fn subscribe(&mut self, event: &str, callback: Listener) {
        self.handlers
            .entry(event.to_string())
            .or_default()
            .push(callback);
    }

    /// Emit a named event to all subscribers
    pub fn publish(&self, event: &str, data: &str) {
        if let Some(handlers) = self.handlers.get(event) {
            for handler in handlers {
                (handler.borrow_mut())(data);
            }
        }
    }

    /// Count handlers for a given event
    pub fn handler_count(&self, event: &str) -> usize {
        self.handlers.get(event).map_or(0, |h| h.len())
    }

    /// List all event names that have handlers
    pub fn event_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.handlers.keys().cloned().collect();
        names.sort();
        names
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 7: Arc<T> & Mutex<T> — Thread-Safe Smart Pointers
// Learn: Arc for shared ownership across threads, Mutex for safe mutation
// ============================================

/// A thread-safe counter using Arc<Mutex<>>
#[derive(Debug)]
pub struct AtomicCounter {
    value: Mutex<i64>,
}

impl AtomicCounter {
    pub fn new(initial: i64) -> Arc<Self> {
        Arc::new(AtomicCounter {
            value: Mutex::new(initial),
        })
    }

    pub fn increment(&self) {
        *self.value.lock().unwrap() += 1;
    }

    pub fn decrement(&self) {
        *self.value.lock().unwrap() -= 1;
    }

    pub fn get(&self) -> i64 {
        *self.value.lock().unwrap()
    }

    pub fn reset(&self, value: i64) {
        *self.value.lock().unwrap() = value;
    }

    /// Add a specific amount
    pub fn add(&self, amount: i64) {
        *self.value.lock().unwrap() += amount;
    }
}

/// A thread-safe shared log
#[derive(Debug)]
pub struct SharedLog {
    entries: Mutex<Vec<String>>,
}

impl SharedLog {
    pub fn new() -> Arc<Self> {
        Arc::new(SharedLog {
            entries: Mutex::new(Vec::new()),
        })
    }

    /// Append a log entry
    pub fn append(&self, msg: &str) {
        self.entries.lock().unwrap().push(msg.to_string());
    }

    /// Get all entries
    pub fn entries(&self) -> Vec<String> {
        self.entries.lock().unwrap().clone()
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.lock().unwrap().is_empty()
    }

    /// Clear all entries
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

impl Default for SharedLog {
    fn default() -> Self {
        SharedLog {
            entries: Mutex::new(Vec::new()),
        }
    }
}

/// A thread-safe key-value store using Arc<Mutex<HashMap>>
#[derive(Debug)]
pub struct ConcurrentMap<V: Clone> {
    data: Mutex<HashMap<String, V>>,
}

impl<V: Clone> ConcurrentMap<V> {
    pub fn new() -> Arc<Self> {
        Arc::new(ConcurrentMap {
            data: Mutex::new(HashMap::new()),
        })
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: &str, value: V) {
        self.data.lock().unwrap().insert(key.to_string(), value);
    }

    /// Get a cloned value by key
    pub fn get(&self, key: &str) -> Option<V> {
        self.data.lock().unwrap().get(key).cloned()
    }

    /// Remove a key
    pub fn remove(&self, key: &str) -> Option<V> {
        self.data.lock().unwrap().remove(key)
    }

    /// Count the entries
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }

    /// Get all keys sorted
    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.data.lock().unwrap().keys().cloned().collect();
        keys.sort();
        keys
    }
}

impl<V: Clone> Default for ConcurrentMap<V> {
    fn default() -> Self {
        ConcurrentMap {
            data: Mutex::new(HashMap::new()),
        }
    }
}
