// ============================================
// Level 2, Project 4: Generics — Generic Types & Functions
// Learn: Type parameters, constraints, generic structs
// ============================================

// ============================================
// Topic 1: Generic Functions
// Learn: <T>, T: PartialOrd, T: Clone
// ============================================

/// Return a reference to the first element
pub fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

/// Return a reference to the last element
pub fn last<T>(items: &[T]) -> Option<&T> {
    items.last()
}

/// Check if a slice contains an element
pub fn contains<T: PartialEq>(items: &[T], target: &T) -> bool {
    items.iter().any(|item| item == target)
}

/// Return the larger of two values
pub fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

/// Swap two values
pub fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

/// Return a new vec with duplicates removed (preserving order)
pub fn deduplicate<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for item in items {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    result
}

// ============================================
// Topic 2: Wrapper<T> — Generic Struct
// Learn: Structs with type parameters
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

    /// Set the inner value
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Consume the wrapper and return the inner value
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Transform the inner value with a function
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Wrapper<U> {
        Wrapper {
            value: f(self.value),
        }
    }
}

// ============================================
// Topic 3: Pair<T, U> — Multiple Type Parameters
// Learn: Multiple generic params, different types
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
}

// ============================================
// Topic 4: Stack<T> — Generic Collection
// Learn: Building a collection with generics
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

    /// Push a value onto the stack
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Peek at the top value
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Number of elements
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Convert to a Vec (bottom to top)
    pub fn to_vec(self) -> Vec<T> {
        self.elements
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack::new()
    }
}

// ============================================
// Topic 5: Generic Constraints
// Learn: Multiple bounds: PartialOrd + Copy, where clauses
// ============================================

/// Return the minimum of three values
pub fn min_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    let min_ab = if a <= b { a } else { b };
    if min_ab <= c {
        min_ab
    } else {
        c
    }
}

/// Clamp a value between min and max
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
    if arr[0] > arr[1] {
        arr.swap(0, 1);
    }
    if arr[1] > arr[2] {
        arr.swap(1, 2);
    }
    if arr[0] > arr[1] {
        arr.swap(0, 1);
    }
    (arr[0], arr[1], arr[2])
}

/// Check if value is between min and max (inclusive)
pub fn is_between<T: PartialOrd>(value: &T, min: &T, max: &T) -> bool {
    value >= min && value <= max
}

/// Median of three values
pub fn median_of_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    let (_, mid, _) = sort_three(a, b, c);
    mid
}

// ============================================
// Topic 6: Advanced — Fn Bounds & Pipeline
// Learn: Generic functions as parameters, closures
// ============================================

/// Apply a function to a value
pub fn apply<T, U>(value: T, f: impl FnOnce(T) -> U) -> U {
    f(value)
}

/// Compose two functions: (f ∘ g)(x) = f(g(x))
pub fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

/// A pipeline that chains transformations
pub struct Pipeline<T> {
    value: T,
}

impl<T> Pipeline<T> {
    /// Start a pipeline with an initial value
    pub fn new(value: T) -> Self {
        Pipeline { value }
    }

    /// Apply a transformation and continue the pipeline
    pub fn then<U>(self, f: impl FnOnce(T) -> U) -> Pipeline<U> {
        Pipeline {
            value: f(self.value),
        }
    }

    /// Execute the pipeline and return the final value
    pub fn execute(self) -> T {
        self.value
    }
}

/// Filter a vec of Results, keeping only Ok values and transforming them
pub fn filter_map_result<T, E, U>(items: Vec<Result<T, E>>, f: impl Fn(T) -> U) -> Vec<U> {
    items
        .into_iter()
        .filter_map(|item| item.ok())
        .map(&f)
        .collect()
}

// ============================================
// Topic 7: Extra Practice
// Learn: More generic function exercises
// ============================================

/// Return the larger of two comparable values.
pub fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

/// Check if a slice is sorted in ascending order.
pub fn is_sorted<T: PartialOrd>(items: &[T]) -> bool {
    items.windows(2).all(|w| w[0] <= w[1])
}

/// Count items matching a predicate.
pub fn count_where<T>(items: &[T], predicate: fn(&T) -> bool) -> usize {
    items.iter().filter(|x| predicate(x)).count()
}

/// Find first item matching predicate, return reference.
pub fn find_first<T>(items: &[T], predicate: fn(&T) -> bool) -> Option<&T> {
    items.iter().find(|x| predicate(x))
}

/// Zip two slices into pairs, truncating to shorter length.
pub fn zip_slices<'a, A, B>(a: &'a [A], b: &'a [B]) -> Vec<(&'a A, &'a B)> {
    a.iter().zip(b.iter()).collect()
}
