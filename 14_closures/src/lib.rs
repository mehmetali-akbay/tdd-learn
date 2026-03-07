// ============================================
// Level 5, Project 1: Closures & Fn Traits
// Learn: Fn, FnMut, FnOnce, higher-order functions, closure capture,
//        returning closures, combinators, callbacks, memoization, generators
// ============================================

use std::collections::HashMap;
use std::collections::HashSet;

// ============================================
// Topic 1: Closure Basics — Capture & Call
// Learn: Closures capture variables, || syntax, move, basic HOFs
// Reinforces: 05_vecs (iterators, collect), 10_generics (generic bounds)
// ============================================

/// Apply a function to each element.
pub fn map_vec<T, U>(items: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    items.iter().map(f).collect()
}

/// Keep items matching a predicate.
pub fn filter_vec<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool) -> Vec<T> {
    items.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Reduce items to a single value (fold).
pub fn reduce<T: Clone>(items: &[T], initial: T, f: impl Fn(T, &T) -> T) -> T {
    items.iter().fold(initial, f)
}

/// Apply a transform N times.
pub fn apply_n<T>(mut value: T, n: usize, f: impl Fn(T) -> T) -> T {
    for _ in 0..n {
        value = f(value);
    }
    value
}

/// Check if any element satisfies a predicate.
pub fn any_of<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> bool {
    items.iter().any(predicate)
}

/// Check if all elements satisfy a predicate.
pub fn all_of<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> bool {
    items.iter().all(predicate)
}

/// Find the first element matching a predicate.
pub fn find_first<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> Option<&T> {
    items.iter().find(|x| predicate(x))
}

/// Flat-map: apply f to each element, flatten the resulting Vecs.
pub fn flat_map_vec<T, U>(items: &[T], f: impl Fn(&T) -> Vec<U>) -> Vec<U> {
    items.iter().flat_map(f).collect()
}

/// Zip two slices together using a combining function.
pub fn zip_with<A, B, C>(a: &[A], b: &[B], f: impl Fn(&A, &B) -> C) -> Vec<C> {
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

/// Take elements while predicate is true.
pub fn take_while_vec<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool) -> Vec<T> {
    items.iter().take_while(|x| predicate(x)).cloned().collect()
}

// ============================================
// Topic 2: Fn vs FnMut vs FnOnce
// Learn: When each trait is required, mutation vs ownership
// Reinforces: 03_ownership (move semantics), 09_traits (trait bounds)
// ============================================

/// Count how many items match (Fn — read-only).
pub fn count_matching<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> usize {
    items.iter().filter(|x| predicate(x)).count()
}

/// For each item, call a mutable callback (FnMut — mutation allowed).
pub fn for_each<T>(items: &[T], mut f: impl FnMut(&T)) {
    for item in items {
        f(item);
    }
}

/// Collect items into a string using a mutable formatter (FnMut).
pub fn build_string<T>(items: &[T], formatter: impl FnMut(&T) -> String) -> String {
    items.iter().map(formatter).collect::<Vec<_>>().join(", ")
}

/// Consume a value through a callback (FnOnce — takes ownership).
pub fn consume_with<T, R>(value: T, f: impl FnOnce(T) -> R) -> R {
    f(value)
}

/// Try to produce a value, falling back to a default generator (FnOnce).
pub fn unwrap_or_else<T>(opt: Option<T>, default: impl FnOnce() -> T) -> T {
    match opt {
        Some(v) => v,
        None => default(),
    }
}

/// Transform each element in a mutable slice in-place (FnMut).
pub fn transform_in_place<T>(items: &mut [T], mut f: impl FnMut(&mut T)) {
    for item in items.iter_mut() {
        f(item);
    }
}

/// Stateful map: accumulate state while mapping (FnMut).
pub fn scan_vec<T, S, R>(items: &[T], initial: S, mut f: impl FnMut(&mut S, &T) -> R) -> Vec<R> {
    let mut state = initial;
    items.iter().map(|item| f(&mut state, item)).collect()
}

/// Map an Option with separate functions for Some and None (FnOnce).
pub fn map_or_else<T, U>(
    opt: Option<T>,
    default: impl FnOnce() -> U,
    f: impl FnOnce(T) -> U,
) -> U {
    match opt {
        Some(v) => f(v),
        None => default(),
    }
}

// ============================================
// Topic 3: Returning Closures
// Learn: impl Fn, Box<dyn Fn>, closure factories, move captures
// Reinforces: 19_smartptrs (Box), 09_traits (dyn trait)
// ============================================

/// Create an adder: returns a closure that adds N.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// Create a multiplier: returns a closure that multiplies by N.
pub fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

/// Create a threshold checker.
pub fn make_threshold(threshold: i32) -> impl Fn(i32) -> bool {
    move |x| x >= threshold
}

/// Create a string repeater.
pub fn make_repeater(n: usize) -> impl Fn(&str) -> String {
    move |s| s.repeat(n)
}

/// Compose two functions: f(g(x)).
pub fn compose<A, B, C>(
    f: impl Fn(B) -> C + 'static,
    g: impl Fn(A) -> B + 'static,
) -> Box<dyn Fn(A) -> C> {
    Box::new(move |x| f(g(x)))
}

/// Negate a predicate.
pub fn negate<T>(predicate: impl Fn(&T) -> bool + 'static) -> Box<dyn Fn(&T) -> bool> {
    Box::new(move |x| !predicate(x))
}

/// Create a clamper: returns a closure that clamps a value to [min, max].
pub fn make_clamper(min: i32, max: i32) -> impl Fn(i32) -> i32 {
    move |x| x.clamp(min, max)
}

/// Create a prefixer: returns a closure that prepends a string.
pub fn make_prefix(prefix: &str) -> impl Fn(&str) -> String {
    let prefix = prefix.to_string();
    move |s| format!("{}{}", prefix, s)
}

/// Create a counter: returns a FnMut closure that counts from start.
pub fn make_counter(start: usize) -> impl FnMut() -> usize {
    let mut count = start;
    move || {
        let current = count;
        count += 1;
        current
    }
}

// ============================================
// Topic 4: Higher-Order Functions — Combinators
// Learn: Building complex behavior from simple closures
// Reinforces: 07_hashmaps (grouping), 11_iterators (chaining)
// ============================================

/// Chain two transforms: apply f then g.
pub fn chain<T>(f: impl Fn(T) -> T, g: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| g(f(x))
}

/// Apply the first predicate that matches any item in the slice.
pub fn first_match<'a, T>(items: &[T], predicates: &[&'a dyn Fn(&T) -> bool]) -> Option<usize> {
    for (i, pred) in predicates.iter().enumerate() {
        if items.iter().any(pred) {
            return Some(i);
        }
    }
    None
}

/// Partition items into two groups based on a predicate.
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

/// Group items by a key function.
pub fn group_by<T: Clone, K: Eq + std::hash::Hash>(
    items: &[T],
    key_fn: impl Fn(&T) -> K,
) -> HashMap<K, Vec<T>> {
    let mut map = HashMap::new();
    for item in items {
        map.entry(key_fn(item))
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    map
}

/// Find the max by a key function.
pub fn max_by_key<T, K: Ord>(items: &[T], key_fn: impl Fn(&T) -> K) -> Option<&T> {
    items.iter().max_by_key(|item| key_fn(item))
}

/// Find the min by a key function.
pub fn min_by_key<T, K: Ord>(items: &[T], key_fn: impl Fn(&T) -> K) -> Option<&T> {
    items.iter().min_by_key(|item| key_fn(item))
}

/// Sort items by a key function (returns a new Vec).
pub fn sort_by_key_fn<T: Clone, K: Ord>(items: &[T], key_fn: impl Fn(&T) -> K) -> Vec<T> {
    let mut sorted = items.to_vec();
    sorted.sort_by_key(|item| key_fn(item));
    sorted
}

/// Remove duplicates by a key function (preserving first occurrence).
pub fn unique_by<T: Clone, K: Eq + std::hash::Hash>(
    items: &[T],
    key_fn: impl Fn(&T) -> K,
) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for item in items {
        if seen.insert(key_fn(item)) {
            result.push(item.clone());
        }
    }
    result
}

/// Apply a function to each sliding window of size N.
pub fn window_map<T, R>(items: &[T], window_size: usize, f: impl Fn(&[T]) -> R) -> Vec<R> {
    items.windows(window_size).map(f).collect()
}

// ============================================
// Topic 5: Closure Patterns — Callbacks & Strategies
// Learn: Strategy pattern, validation, middleware pipelines
// Reinforces: 04_structs (struct methods), 09_traits (trait objects)
// ============================================

/// Type alias for a validation rule.
type ValidationRule<T> = Box<dyn Fn(&T) -> Result<(), String>>;

/// A validator that holds a list of validation rules.
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

    /// Run all rules, collecting errors.
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

    /// Run rules, returning the first error found.
    pub fn validate_first_error(&self, value: &T) -> Result<(), String> {
        for rule in &self.rules {
            rule(value)?;
        }
        Ok(())
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

/// A middleware pipeline: each step transforms a value.
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

    /// Add a step that only runs when the condition is true.
    pub fn add_conditional_step(
        &mut self,
        condition: impl Fn(&T) -> bool + 'static,
        step: impl Fn(T) -> T + 'static,
    ) {
        self.steps.push(Box::new(move |value: T| {
            if condition(&value) {
                step(value)
            } else {
                value
            }
        }));
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
// Reinforces: 07_hashmaps (cache), 10_generics (generic structs)
// ============================================

/// A memoized function wrapper.
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

    /// Check if a value is already cached.
    pub fn contains(&self, arg: &A) -> bool {
        self.cache.contains_key(arg)
    }
}

/// A lazy value that computes on first access.
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

// ============================================
// Topic 7: Closure-based Generators
// Learn: Creating sequences with closures, iterator-like patterns
// Reinforces: 11_iterators (lazy iteration), 08_results (Option)
// ============================================

/// Generate N values using an index-based function.
pub fn generate<T>(n: usize, f: impl Fn(usize) -> T) -> Vec<T> {
    (0..n).map(f).collect()
}

/// Call a closure N times, collecting results.
pub fn repeat_with_fn<T>(n: usize, mut f: impl FnMut() -> T) -> Vec<T> {
    (0..n).map(|_| f()).collect()
}

/// Build a sequence by repeatedly applying f to the previous value.
pub fn iterate<T: Clone>(seed: T, f: impl Fn(&T) -> T, n: usize) -> Vec<T> {
    let mut result = Vec::with_capacity(n);
    if n == 0 {
        return result;
    }
    result.push(seed);
    for i in 1..n {
        let next = f(&result[i - 1]);
        result.push(next);
    }
    result
}

/// Unfold a sequence from a seed state, producing values until None.
pub fn unfold<S, T>(seed: S, mut f: impl FnMut(S) -> Option<(T, S)>, max: usize) -> Vec<T> {
    let mut state = seed;
    let mut result = Vec::new();
    for _ in 0..max {
        match f(state) {
            Some((value, next_state)) => {
                result.push(value);
                state = next_state;
            }
            None => break,
        }
    }
    result
}

/// Collect values from a closure that returns Option until None.
pub fn from_fn_vec<T>(mut f: impl FnMut() -> Option<T>, max: usize) -> Vec<T> {
    let mut result = Vec::new();
    for _ in 0..max {
        match f() {
            Some(v) => result.push(v),
            None => break,
        }
    }
    result
}

/// Build a sequence of successors starting from the first value.
pub fn successors_vec<T: Clone>(first: T, f: impl Fn(&T) -> Option<T>, max: usize) -> Vec<T> {
    let mut result = vec![first];
    while result.len() < max {
        match f(result.last().unwrap()) {
            Some(next) => result.push(next),
            None => break,
        }
    }
    result
}

/// Take elements until the predicate is true (exclusive).
pub fn take_until<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool) -> Vec<T> {
    let mut result = Vec::new();
    for item in items {
        if predicate(item) {
            break;
        }
        result.push(item.clone());
    }
    result
}
