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
    todo!()
}

/// Return a reference to the last element of a slice
pub fn last<T>(items: &[T]) -> Option<&T> {
    todo!()
}

/// Check if a slice contains an element
pub fn contains<T: PartialEq>(items: &[T], target: &T) -> bool {
    todo!()
}

/// Return the larger of two values
pub fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    todo!()
}

/// Return the smaller of two values
/// Reinforces: mirrors max_of_two, solidifies generic understanding
pub fn min_of_two<T: PartialOrd>(a: T, b: T) -> T {
    todo!()
}

/// Swap two values, returning them in reversed order
pub fn swap<T>(a: T, b: T) -> (T, T) {
    todo!()
}

/// Return a new vec with duplicates removed (preserving first-seen order)
/// Reinforces: Vec operations, PartialEq + Clone bounds
pub fn deduplicate<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    todo!()
}

/// Find the first element matching a predicate, return a reference
/// Reinforces: closures with slices, Option<&T>
pub fn find_first<T>(items: &[T], predicate: fn(&T) -> bool) -> Option<&T> {
    todo!()
}

/// Zip two slices into pairs, truncating to the shorter length
/// Reinforces: multiple generic params on functions, slices, lifetimes
pub fn zip_slices<'a, A, B>(a: &'a [A], b: &'a [B]) -> Vec<(&'a A, &'a B)> {
    todo!()
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
        todo!()
    }

    /// Get a reference to the inner value
    pub fn get(&self) -> &T {
        todo!()
    }

    /// Set the inner value (mutable borrow)
    pub fn set(&mut self, value: T) {
        todo!()
    }

    /// Consume the wrapper and return the inner value (ownership transfer)
    pub fn into_inner(self) -> T {
        todo!()
    }

    /// Transform the inner value with a function, returning a new Wrapper
    /// Reinforces: FnOnce, type transformation
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Wrapper<U> {
        todo!()
    }

    /// Return the inner value if it satisfies a predicate, otherwise a default
    /// Reinforces: closures, Option-like semantics
    pub fn unwrap_or(self, default: T, predicate: fn(&T) -> bool) -> T {
        todo!()
    }

    /// Combine two Wrappers into a Wrapper of a tuple
    /// Reinforces: multiple type params, tuples
    pub fn zip<U>(self, other: Wrapper<U>) -> Wrapper<(T, U)> {
        todo!()
    }
}

/// Display wrapper's content when T: Display
/// Reinforces: conditional impl (Rust Book pattern), Display trait
/// Format: "Wrapper({value})"
impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
        todo!()
    }

    /// Get a reference to the first element
    pub fn first(&self) -> &T {
        todo!()
    }

    /// Get a reference to the second element
    pub fn second(&self) -> &U {
        todo!()
    }

    /// Swap the elements, returning Pair<U, T>
    pub fn swap(self) -> Pair<U, T> {
        todo!()
    }

    /// Transform the first element
    pub fn map_first<V>(self, f: impl FnOnce(T) -> V) -> Pair<V, U> {
        todo!()
    }

    /// Transform the second element
    pub fn map_second<V>(self, f: impl FnOnce(U) -> V) -> Pair<T, V> {
        todo!()
    }

    /// Mixup: take first from self and second from another pair (Rust Book §10.1)
    /// This demonstrates method-level generic params (X2, Y2)
    /// separate from the struct's generic params (T, U)
    pub fn mixup<X2, Y2>(self, other: Pair<X2, Y2>) -> Pair<T, Y2> {
        todo!()
    }

    /// Convert the pair into a tuple
    /// Reinforces: destructuring, tuples
    pub fn into_tuple(self) -> (T, U) {
        todo!()
    }
}

/// Display for Pair when both types implement Display
/// Format: "({first}, {second})"
impl<T: fmt::Display, U: fmt::Display> fmt::Display for Pair<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
        todo!()
    }

    /// Check if this is Nothing
    pub fn is_nothing(&self) -> bool {
        todo!()
    }

    /// Unwrap the value, panicking with "called unwrap on Nothing" if Nothing
    pub fn unwrap(self) -> T {
        todo!()
    }

    /// Return the value if Just, otherwise return the provided default
    /// Reinforces: pattern matching, ownership
    pub fn unwrap_or(self, default: T) -> T {
        todo!()
    }

    /// Transform the inner value, if present
    /// Reinforces: FnOnce, type transformation
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Maybe<U> {
        todo!()
    }

    /// Convert to a standard Option<T>
    pub fn to_option(self) -> Option<T> {
        todo!()
    }

    /// Create from a standard Option<T>
    pub fn from_option(opt: Option<T>) -> Maybe<T> {
        todo!()
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
        todo!()
    }

    /// Check if this is a Right value
    pub fn is_right(&self) -> bool {
        todo!()
    }

    /// Get the Left value as an Option
    /// Reinforces: pattern matching, Option
    pub fn left(self) -> Option<L> {
        todo!()
    }

    /// Get the Right value as an Option
    pub fn right(self) -> Option<R> {
        todo!()
    }

    /// Map over the Left value, leaving Right unchanged
    pub fn map_left<L2>(self, f: impl FnOnce(L) -> L2) -> Either<L2, R> {
        todo!()
    }

    /// Map over the Right value, leaving Left unchanged
    pub fn map_right<R2>(self, f: impl FnOnce(R) -> R2) -> Either<L, R2> {
        todo!()
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
        todo!()
    }

    /// Create a stack from a Vec (bottom to top)
    /// Reinforces: Vec ownership, From-like pattern
    pub fn from_vec(v: Vec<T>) -> Self {
        todo!()
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: T) {
        todo!()
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    /// Number of elements on the stack
    pub fn size(&self) -> usize {
        todo!()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Convert the stack into a Vec (bottom to top)
    pub fn into_vec(self) -> Vec<T> {
        todo!()
    }

    /// Reverse the stack (bottom becomes top)
    /// Reinforces: Vec::reverse, mutability
    pub fn reverse(&mut self) {
        todo!()
    }

    /// Drain all elements into a Vec (top to bottom order)
    /// The stack is left empty after this operation
    /// Reinforces: ownership, Vec manipulation
    pub fn drain_to_vec(&mut self) -> Vec<T> {
        todo!()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        todo!()
    }
}

/// Display the stack as "[elem1, elem2, ...]" (bottom to top)
impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
    todo!()
}

/// Clamp a value between min and max bounds
pub fn clamp<T: PartialOrd + Copy>(value: T, min: T, max: T) -> T {
    todo!()
}

/// Sort three values and return as a tuple (smallest, middle, largest)
pub fn sort_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> (T, T, T) {
    todo!()
}

/// Check if value is between min and max (inclusive)
pub fn is_between<T: PartialOrd>(value: &T, min: &T, max: &T) -> bool {
    todo!()
}

/// Median of three values (the middle one when sorted)
pub fn median_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    todo!()
}

/// Check if a slice is sorted in ascending order
/// Uses a `where` clause (Rust Book style)
pub fn is_sorted<T>(items: &[T]) -> bool
where
    T: PartialOrd,
{
    todo!()
}

/// Count items in a slice matching a predicate
pub fn count_where<T>(items: &[T], predicate: fn(&T) -> bool) -> usize {
    todo!()
}

/// Partition a slice into two Vecs: (matching, not matching)
/// Uses a `where` clause with Clone bound
/// Reinforces: Vec, closures, tuple returns
pub fn partition<T>(items: &[T], predicate: fn(&T) -> bool) -> (Vec<T>, Vec<T>)
where
    T: Clone,
{
    todo!()
}

/// Format all items in a slice using Display, joined by a separator
/// Reinforces: Display (09), String building, generic iteration
pub fn format_joined<T>(items: &[T], separator: &str) -> String
where
    T: fmt::Display,
{
    todo!()
}

/// Group items by a key function, returning a HashMap
/// Reinforces: HashMap (07), closures, trait bounds
pub fn group_by<T, K>(items: &[T], key_fn: fn(&T) -> K) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + std::hash::Hash,
{
    todo!()
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
    /// Create a new Bounded value, clamping to the range [min, max]
    pub fn new(value: T, min: T, max: T) -> Self {
        todo!()
    }

    /// Get the current value
    pub fn value(&self) -> T {
        todo!()
    }

    /// Set a new value, clamping to bounds
    pub fn set(&mut self, value: T) {
        todo!()
    }

    /// Check if the value is at the minimum
    pub fn is_at_min(&self) -> bool {
        todo!()
    }

    /// Check if the value is at the maximum
    pub fn is_at_max(&self) -> bool {
        todo!()
    }
}

/// Display as "{value} [{min}, {max}]"
impl<T: PartialOrd + Copy + fmt::Display> fmt::Display for Bounded<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// ============================================
// Topic 7: Fn Bounds, Closures & Pipeline
// Learn: Generic functions as parameters, FnOnce/Fn, compose, pipeline
// Reinforces: closures, Result (08), type transformations, ownership
// ============================================

/// Apply a function to a value and return the result
pub fn apply<T, U>(value: T, f: impl FnOnce(T) -> U) -> U {
    todo!()
}

/// Apply a function twice: f(f(x))
/// Reinforces: Fn vs FnOnce (needs Fn since called twice)
pub fn apply_twice<T>(value: T, f: impl Fn(T) -> T) -> T {
    todo!()
}

/// Compose two functions: (f ∘ g)(x) = f(g(x))
pub fn compose<A, B, C>(
    f: impl Fn(B) -> C,
    g: impl Fn(A) -> B,
) -> impl Fn(A) -> C {
    todo!();
    // Hint: return a closure that applies g then f
    #[allow(unreachable_code)]
    move |x: A| -> C { f(g(x)) }
}

/// A pipeline that chains transformations on a value
pub struct Pipeline<T> {
    value: T,
}

impl<T> Pipeline<T> {
    /// Start a pipeline with an initial value
    pub fn new(value: T) -> Self {
        todo!()
    }

    /// Apply a transformation step and continue the pipeline
    pub fn then<U>(self, f: impl FnOnce(T) -> U) -> Pipeline<U> {
        todo!()
    }

    /// Finish the pipeline and return the final value
    pub fn execute(self) -> T {
        todo!()
    }
}

/// Filter a vec of Results, keeping only Ok values and transforming them
/// Reinforces: Result (08), iterators, closures
pub fn filter_map_ok<T, E, U>(items: Vec<Result<T, E>>, f: impl Fn(T) -> U) -> Vec<U> {
    todo!()
}

/// Try to apply a function that can fail, wrapping the result
/// Reinforces: Result, closures, error handling (08)
pub fn try_apply<T, U, E>(value: T, f: impl FnOnce(T) -> Result<U, E>) -> Result<U, E> {
    todo!()
}

/// Apply a function to each element, collecting into a Result<Vec<U>, E>
/// Stops at the first error (like Iterator::collect into Result)
/// Reinforces: Result (08), Vec, closures, error propagation
pub fn try_map<T, U, E>(
    items: &[T],
    f: impl Fn(&T) -> Result<U, E>,
) -> Result<Vec<U>, E> {
    todo!()
}

/// Fold (reduce) a slice with an initial accumulator and a function
/// Reinforces: iteration, closures, generic accumulation
pub fn fold<T, A>(items: &[T], initial: A, f: impl Fn(A, &T) -> A) -> A {
    todo!()
}