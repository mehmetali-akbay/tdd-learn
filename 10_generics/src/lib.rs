// ============================================
// Module 10: Generics — Generic Types & Functions
// Learn: Type parameters, constraints, generic structs/enums,
//        where clauses, multiple type params, impl blocks,
//        Fn bounds, closures, pipelines
// Reinforces: ownership, borrowing, slices, structs, enums,
//             pattern matching, Vec, String, HashMap, Result, traits
// ============================================

use std::collections::HashMap;
use std::fmt;

// ============================================
// Topic 1: Generic Functions
// Learn: <T>, trait bounds on T, returning T vs &T, Option
// Reinforces: slices, borrowing, Option, Vec
// ============================================

/// Return a reference to the first element of a slice
pub fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

/// Return a reference to the last element of a slice
pub fn last<T>(items: &[T]) -> Option<&T> {
    items.last()
}

/// Check if a slice contains an element
pub fn contains<T: PartialEq>(items: &[T], target: &T) -> bool {
    items.iter().any(|item| item == target)
}

/// Return the larger of two values
pub fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

/// Return the smaller of two values
/// Reinforces: mirrors max_of_two, solidifies generic understanding
pub fn min_of_two<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

/// Swap two values, returning them in reversed order
pub fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

/// Return a new vec with duplicates removed (preserving first-seen order)
/// Reinforces: Vec operations, PartialEq + Clone bounds
pub fn deduplicate<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for item in items {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    result
}

/// Find the first element matching a predicate, return a reference
/// Reinforces: closures with slices, Option<&T>
pub fn find_first<T>(items: &[T], predicate: fn(&T) -> bool) -> Option<&T> {
    items.iter().find(|x| predicate(x))
}

/// Zip two slices into pairs, truncating to the shorter length
/// Reinforces: multiple generic params on functions, slices, lifetimes
pub fn zip_slices<'a, A, B>(a: &'a [A], b: &'a [B]) -> Vec<(&'a A, &'a B)> {
    a.iter().zip(b.iter()).collect()
}

// ============================================
// Topic 2: Wrapper<T> — Generic Struct
// Learn: Structs with type parameters, impl<T>, consuming self vs &self
// Reinforces: ownership (move semantics), Option, Display
// ============================================

/// A simple wrapper around a value
#[derive(Debug, Clone, PartialEq)]
pub struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    /// Create a new wrapper
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }

    /// Get a reference to the inner value
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Set the inner value (mutable borrow)
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Consume the wrapper and return the inner value (ownership transfer)
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Transform the inner value with a function, returning a new Wrapper
    /// Reinforces: FnOnce, type transformation
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Wrapper<U> {
        Wrapper {
            value: f(self.value),
        }
    }

    /// Return the inner value if it satisfies a predicate, otherwise a default
    /// Reinforces: closures, Option-like semantics
    pub fn unwrap_or(self, default: T, predicate: fn(&T) -> bool) -> T {
        if predicate(&self.value) {
            self.value
        } else {
            default
        }
    }

    /// Combine two Wrappers into a Wrapper of a tuple
    /// Reinforces: multiple type params, tuples
    pub fn zip<U>(self, other: Wrapper<U>) -> Wrapper<(T, U)> {
        Wrapper {
            value: (self.value, other.value),
        }
    }
}

/// Display wrapper's content when T: Display
/// Reinforces: conditional impl (Rust Book pattern), Display trait
impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper({})", self.value)
    }
}

// ============================================
// Topic 3: Pair<T, U> — Multiple Type Parameters
// Learn: Multiple generic params, different types, mixup pattern (Rust Book)
// Reinforces: ownership/move, type transformations
// ============================================

/// A pair of two values that can have different types
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    /// Create a new pair
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    /// Get a reference to the first element
    pub fn first(&self) -> &T {
        &self.first
    }

    /// Get a reference to the second element
    pub fn second(&self) -> &U {
        &self.second
    }

    /// Swap the elements, returning Pair<U, T>
    pub fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }

    /// Transform the first element
    pub fn map_first<V>(self, f: impl FnOnce(T) -> V) -> Pair<V, U> {
        Pair {
            first: f(self.first),
            second: self.second,
        }
    }

    /// Transform the second element
    pub fn map_second<V>(self, f: impl FnOnce(U) -> V) -> Pair<T, V> {
        Pair {
            first: self.first,
            second: f(self.second),
        }
    }

    /// Mixup: take first from self and second from another pair (Rust Book §10.1)
    /// This demonstrates method-level generic params (X2, Y2)
    /// separate from the struct's generic params (T, U)
    pub fn mixup<X2, Y2>(self, other: Pair<X2, Y2>) -> Pair<T, Y2> {
        Pair {
            first: self.first,
            second: other.second,
        }
    }

    /// Convert the pair into a tuple
    /// Reinforces: destructuring, tuples
    pub fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
}

/// Display for Pair when both types implement Display
impl<T: fmt::Display, U: fmt::Display> fmt::Display for Pair<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

// ============================================
// Topic 4: Generic Enums — Maybe<T> & Either<L, R>
// Learn: Custom generic enums, pattern matching, map/unwrap
// (Mirrors Option<T> and Result<T,E> from Rust Book §10.1)
// Reinforces: enums (03), pattern matching, Option/Result semantics (08)
// ============================================

/// A custom Option-like enum to practice building generic enums
#[derive(Debug, Clone, PartialEq)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    /// Check if this is a Just value
    pub fn is_just(&self) -> bool {
        matches!(self, Maybe::Just(_))
    }

    /// Check if this is Nothing
    pub fn is_nothing(&self) -> bool {
        matches!(self, Maybe::Nothing)
    }

    /// Unwrap the value, panicking with a message if Nothing
    pub fn unwrap(self) -> T {
        match self {
            Maybe::Just(v) => v,
            Maybe::Nothing => panic!("called unwrap on Nothing"),
        }
    }

    /// Return the value if Just, otherwise return the provided default
    /// Reinforces: pattern matching, ownership
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Maybe::Just(v) => v,
            Maybe::Nothing => default,
        }
    }

    /// Transform the inner value, if present
    /// Reinforces: FnOnce, type transformation
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Maybe<U> {
        match self {
            Maybe::Just(v) => Maybe::Just(f(v)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    /// Convert to a standard Option<T>
    pub fn to_option(self) -> Option<T> {
        match self {
            Maybe::Just(v) => Some(v),
            Maybe::Nothing => None,
        }
    }

    /// Create from a standard Option<T>
    pub fn from_option(opt: Option<T>) -> Maybe<T> {
        match opt {
            Some(v) => Maybe::Just(v),
            None => Maybe::Nothing,
        }
    }
}

/// A custom Result-like enum with two type parameters
/// Reinforces: generic enums with 2 params (Rust Book §10.1)
#[derive(Debug, Clone, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Check if this is a Left value
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    /// Check if this is a Right value
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }

    /// Get the Left value as an Option
    /// Reinforces: pattern matching, Option
    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    /// Get the Right value as an Option
    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    /// Map over the Left value, leaving Right unchanged
    pub fn map_left<L2>(self, f: impl FnOnce(L) -> L2) -> Either<L2, R> {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    /// Map over the Right value, leaving Left unchanged
    pub fn map_right<R2>(self, f: impl FnOnce(R) -> R2) -> Either<L, R2> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }
}

// ============================================
// Topic 5: Stack<T> — Generic Collection
// Learn: Building a collection with generics, Default trait
// Reinforces: Vec (05), ownership, iterating
// ============================================

/// A generic stack (LIFO)
#[derive(Debug, Clone)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// Create an empty stack
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    /// Create a stack from a Vec (bottom to top)
    /// Reinforces: Vec ownership, From-like pattern
    pub fn from_vec(v: Vec<T>) -> Self {
        Stack { elements: v }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Number of elements on the stack
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Convert the stack into a Vec (bottom to top)
    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }

    /// Reverse the stack (bottom becomes top)
    /// Reinforces: Vec::reverse, mutability
    pub fn reverse(&mut self) {
        self.elements.reverse();
    }

    /// Drain all elements into a Vec (top to bottom order)
    /// The stack is left empty after this operation
    /// Reinforces: ownership, Vec manipulation
    pub fn drain_to_vec(&mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(item) = self.pop() {
            result.push(item);
        }
        result
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack::new()
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.elements.iter().map(|e| format!("{}", e)).collect();
        write!(f, "[{}]", items.join(", "))
    }
}

// ============================================
// Topic 6: Trait Bounds & Constraints
// Learn: Multiple bounds with +, where clauses, conditional impl,
//        impl for specific types (Rust Book §10.1)
// Reinforces: PartialOrd, Copy, Display (09), slices, HashMap (07)
// ============================================

/// Return the minimum of three values
pub fn min_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    let min_ab = if a <= b { a } else { b };
    if min_ab <= c { min_ab } else { c }
}

/// Clamp a value between min and max bounds
pub fn clamp<T: PartialOrd + Copy>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Sort three values and return as a tuple (smallest, middle, largest)
pub fn sort_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> (T, T, T) {
    let mut arr = [a, b, c];
    if arr[0] > arr[1] { arr.swap(0, 1); }
    if arr[1] > arr[2] { arr.swap(1, 2); }
    if arr[0] > arr[1] { arr.swap(0, 1); }
    (arr[0], arr[1], arr[2])
}

/// Check if value is between min and max (inclusive)
pub fn is_between<T: PartialOrd>(value: &T, min: &T, max: &T) -> bool {
    value >= min && value <= max
}

/// Median of three values (the middle one when sorted)
pub fn median_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    let (_, mid, _) = sort_three(a, b, c);
    mid
}

/// Check if a slice is sorted in ascending order
/// Uses a `where` clause (Rust Book style)
pub fn is_sorted<T>(items: &[T]) -> bool
where
    T: PartialOrd,
{
    items.windows(2).all(|w| w[0] <= w[1])
}

/// Count items in a slice matching a predicate
pub fn count_where<T>(items: &[T], predicate: fn(&T) -> bool) -> usize {
    items.iter().filter(|x| predicate(x)).count()
}

/// Partition a slice into two Vecs: (matching, not matching)
/// Uses a `where` clause with Clone bound
/// Reinforces: Vec, closures, tuple returns
pub fn partition<T>(items: &[T], predicate: fn(&T) -> bool) -> (Vec<T>, Vec<T>)
where
    T: Clone,
{
    let mut matching = Vec::new();
    let mut rest = Vec::new();
    for item in items {
        if predicate(item) {
            matching.push(item.clone());
        } else {
            rest.push(item.clone());
        }
    }
    (matching, rest)
}

/// Format all items in a slice using Display, joined by a separator
/// Reinforces: Display (09), String building, generic iteration
pub fn format_joined<T>(items: &[T], separator: &str) -> String
where
    T: fmt::Display,
{
    items
        .iter()
        .map(|item| format!("{}", item))
        .collect::<Vec<_>>()
        .join(separator)
}

/// Group items by a key function, returning a HashMap
/// Reinforces: HashMap (07), closures, trait bounds
pub fn group_by<T, K>(items: &[T], key_fn: fn(&T) -> K) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + std::hash::Hash,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        map.entry(key_fn(item)).or_default().push(item.clone());
    }
    map
}

/// A generic bounded value: ensures the value stays within [min, max]
/// Demonstrates: struct with constraints, conditional behavior
/// Reinforces: PartialOrd + Copy bounds, struct design (02)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounded<T: PartialOrd + Copy> {
    value: T,
    min: T,
    max: T,
}

impl<T: PartialOrd + Copy> Bounded<T> {
    /// Create a new Bounded value, clamping to the range
    pub fn new(value: T, min: T, max: T) -> Self {
        Bounded {
            value: clamp(value, min, max),
            min,
            max,
        }
    }

    /// Get the current value
    pub fn value(&self) -> T {
        self.value
    }

    /// Set a new value, clamping to bounds
    pub fn set(&mut self, value: T) {
        self.value = clamp(value, self.min, self.max);
    }

    /// Check if the value is at the minimum
    pub fn is_at_min(&self) -> bool {
        self.value == self.min
    }

    /// Check if the value is at the maximum
    pub fn is_at_max(&self) -> bool {
        self.value == self.max
    }
}

impl<T: PartialOrd + Copy + fmt::Display> fmt::Display for Bounded<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}, {}]", self.value, self.min, self.max)
    }
}

// ============================================
// Topic 7: Fn Bounds, Closures & Pipeline
// Learn: Generic functions as parameters, FnOnce/Fn, compose, pipeline
// Reinforces: closures, Result (08), type transformations, ownership
// ============================================

/// Apply a function to a value and return the result
pub fn apply<T, U>(value: T, f: impl FnOnce(T) -> U) -> U {
    f(value)
}

/// Apply a function twice: f(f(x))
/// Reinforces: Fn vs FnOnce (needs Fn since called twice)
pub fn apply_twice<T>(value: T, f: impl Fn(T) -> T) -> T {
    f(f(value))
}

/// Compose two functions: (f ∘ g)(x) = f(g(x))
pub fn compose<A, B, C>(
    f: impl Fn(B) -> C,
    g: impl Fn(A) -> B,
) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

/// A pipeline that chains transformations on a value
pub struct Pipeline<T> {
    value: T,
}

impl<T> Pipeline<T> {
    /// Start a pipeline with an initial value
    pub fn new(value: T) -> Self {
        Pipeline { value }
    }

    /// Apply a transformation step and continue the pipeline
    pub fn then<U>(self, f: impl FnOnce(T) -> U) -> Pipeline<U> {
        Pipeline { value: f(self.value) }
    }

    /// Finish the pipeline and return the final value
    pub fn execute(self) -> T {
        self.value
    }
}

/// Filter a vec of Results, keeping only Ok values and transforming them
/// Reinforces: Result (08), iterators, closures
pub fn filter_map_ok<T, E, U>(items: Vec<Result<T, E>>, f: impl Fn(T) -> U) -> Vec<U> {
    items
        .into_iter()
        .filter_map(|item| item.ok())
        .map(&f)
        .collect()
}

/// Try to apply a function that can fail, wrapping the result
/// Reinforces: Result, closures, error handling (08)
pub fn try_apply<T, U, E>(value: T, f: impl FnOnce(T) -> Result<U, E>) -> Result<U, E> {
    f(value)
}

/// Apply a function to each element, collecting into a Result<Vec<U>, E>
/// Stops at the first error (like Iterator::collect into Result)
/// Reinforces: Result (08), Vec, closures, error propagation
pub fn try_map<T, U, E>(
    items: &[T],
    f: impl Fn(&T) -> Result<U, E>,
) -> Result<Vec<U>, E> {
    items.iter().map(f).collect()
}

/// Fold (reduce) a slice with an initial accumulator and a function
/// Reinforces: iteration, closures, generic accumulation
pub fn fold<T, A>(items: &[T], initial: A, f: impl Fn(A, &T) -> A) -> A {
    let mut acc = initial;
    for item in items {
        acc = f(acc, item);
    }
    acc
}