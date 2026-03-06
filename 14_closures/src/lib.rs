// ============================================
// Level 5, Project 1: Closures & Fn Traits
// Learn: Fn, FnMut, FnOnce, higher-order functions, closure capture
// ============================================

// ============================================
// Topic 1: Closure Basics — Capture & Call
// Learn: Closures capture variables, || syntax, move
// ============================================

/// Apply a function to each element
pub fn map_vec<T, U>(items: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    items.iter().map(f).collect()
}

/// Keep items matching a predicate
pub fn filter_vec<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool) -> Vec<T> {
    items.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Reduce items to a single value
pub fn reduce<T: Clone>(items: &[T], initial: T, f: impl Fn(T, &T) -> T) -> T {
    items.iter().fold(initial, f)
}

/// Apply a transform N times
pub fn apply_n<T>(mut value: T, n: usize, f: impl Fn(T) -> T) -> T {
    for _ in 0..n {
        value = f(value);
    }
    value
}

/// Check if any element satisfies a predicate
pub fn any_of<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> bool {
    items.iter().any(predicate)
}

/// Check if all elements satisfy a predicate
pub fn all_of<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> bool {
    items.iter().all(predicate)
}

// ============================================
// Topic 2: Fn vs FnMut vs FnOnce
// Learn: When each trait is required
// ============================================

/// Count how many items match (Fn — read-only)
pub fn count_matching<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> usize {
    items.iter().filter(|x| predicate(x)).count()
}

/// For each item, call a mutable callback (FnMut — mutation allowed)
pub fn for_each<T>(items: &[T], mut f: impl FnMut(&T)) {
    for item in items {
        f(item);
    }
}

/// Collect items into a string using a mutable formatter (FnMut)
pub fn build_string<T>(items: &[T], formatter: impl FnMut(&T) -> String) -> String {
    items.iter().map(formatter).collect::<Vec<_>>().join(", ")
}

/// Consume a value through a callback (FnOnce — takes ownership)
pub fn consume_with<T, R>(value: T, f: impl FnOnce(T) -> R) -> R {
    f(value)
}

/// Try to produce a value, falling back to a default generator (FnOnce)
pub fn unwrap_or_else<T>(opt: Option<T>, default: impl FnOnce() -> T) -> T {
    match opt {
        Some(v) => v,
        None => default(),
    }
}

// ============================================
// Topic 3: Returning Closures
// Learn: impl Fn, Box<dyn Fn>, closure factories
// ============================================

/// Create an adder: returns a closure that adds N
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// Create a multiplier: returns a closure that multiplies by N
pub fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

/// Create a threshold checker
pub fn make_threshold(threshold: i32) -> impl Fn(i32) -> bool {
    move |x| x >= threshold
}

/// Create a string repeater
pub fn make_repeater(n: usize) -> impl Fn(&str) -> String {
    move |s| s.repeat(n)
}

/// Compose two functions: f(g(x))
pub fn compose<A, B, C>(
    f: impl Fn(B) -> C + 'static,
    g: impl Fn(A) -> B + 'static,
) -> Box<dyn Fn(A) -> C> {
    Box::new(move |x| f(g(x)))
}

/// Negate a predicate
pub fn negate<T>(predicate: impl Fn(&T) -> bool + 'static) -> Box<dyn Fn(&T) -> bool> {
    Box::new(move |x| !predicate(x))
}

// ============================================
// Topic 4: Higher-Order Functions — Combinators
// Learn: Building complex behavior from simple closures
// ============================================

/// Chain two transforms: apply f then g
pub fn chain<T>(f: impl Fn(T) -> T, g: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| g(f(x))
}

/// Apply the first predicate that matches
pub fn first_match<'a, T>(items: &[T], predicates: &[&'a dyn Fn(&T) -> bool]) -> Option<usize> {
    for (i, pred) in predicates.iter().enumerate() {
        if items.iter().any(pred) {
            return Some(i);
        }
    }
    None
}

/// Partition items into two groups based on a predicate
pub fn partition<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool) -> (Vec<T>, Vec<T>) {
    let mut pass = Vec::new();
    let mut fail = Vec::new();
    for item in items {
        if predicate(item) {
            pass.push(item.clone());
        } else {
            fail.push(item.clone());
        }
    }
    (pass, fail)
}

/// Group items by a key function
pub fn group_by<T: Clone, K: Eq + std::hash::Hash>(
    items: &[T],
    key_fn: impl Fn(&T) -> K,
) -> std::collections::HashMap<K, Vec<T>> {
    let mut map = std::collections::HashMap::new();
    for item in items {
        map.entry(key_fn(item))
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    map
}

/// Find the max by a key function
pub fn max_by_key<T, K: Ord>(items: &[T], key_fn: impl Fn(&T) -> K) -> Option<&T> {
    items.iter().max_by_key(|item| key_fn(item))
}

// ============================================
// Topic 5: Closure Patterns — Callbacks & Strategies
// Learn: Strategy pattern, event handlers, middleware
// ============================================

/// Type alias for a validation rule
type ValidationRule<T> = Box<dyn Fn(&T) -> Result<(), String>>;

/// A validator that holds a list of validation rules
pub struct Validator<T> {
    rules: Vec<ValidationRule<T>>,
}

impl<T> Validator<T> {
    pub fn new() -> Self {
        Validator { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: impl Fn(&T) -> Result<(), String> + 'static) {
        self.rules.push(Box::new(rule));
    }

    /// Run all rules, collecting errors
    pub fn validate(&self, value: &T) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self
            .rules
            .iter()
            .filter_map(|rule| rule(value).err())
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl<T> Default for Validator<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple middleware chain: each middleware transforms input
pub struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T> Pipeline<T> {
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    pub fn add_step(&mut self, step: impl Fn(T) -> T + 'static) {
        self.steps.push(Box::new(step));
    }

    pub fn execute(&self, mut value: T) -> T {
        for step in &self.steps {
            value = step(value);
        }
        value
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

impl<T> Default for Pipeline<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 6: Advanced — Memoization & Lazy Evaluation
// Learn: Caching with closures, deferred computation
// ============================================

use std::collections::HashMap;

/// A memoized function wrapper
pub struct Memoize<A: Eq + std::hash::Hash + Clone, R: Clone> {
    func: Box<dyn Fn(A) -> R>,
    cache: HashMap<A, R>,
}

impl<A: Eq + std::hash::Hash + Clone, R: Clone> Memoize<A, R> {
    pub fn new(func: impl Fn(A) -> R + 'static) -> Self {
        Memoize {
            func: Box::new(func),
            cache: HashMap::new(),
        }
    }

    pub fn call(&mut self, arg: A) -> R {
        if let Some(cached) = self.cache.get(&arg) {
            return cached.clone();
        }
        let result = (self.func)(arg.clone());
        self.cache.insert(arg, result.clone());
        result
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

/// A lazy value that computes on first access
pub struct Lazy<T> {
    init: Option<Box<dyn FnOnce() -> T>>,
    value: Option<T>,
}

impl<T> Lazy<T> {
    pub fn new(init: impl FnOnce() -> T + 'static) -> Self {
        Lazy {
            init: Some(Box::new(init)),
            value: None,
        }
    }

    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            let init = self.init.take().expect("Lazy already initialized");
            self.value = Some(init());
        }
        self.value.as_ref().unwrap()
    }

    pub fn is_initialized(&self) -> bool {
        self.value.is_some()
    }
}
