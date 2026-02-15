// ============================================
// Level 3, Project 4: Iterators — Custom Iterators & Chains
// Learn: Iterator trait, custom iterators, method chaining
// ============================================

// ============================================
// Topic 1: Iterator Basics — Method Chains
// Learn: map, filter, fold, collect, enumerate, zip
// ============================================

/// Sum of squares: 1² + 2² + ... + n²
pub fn sum_of_squares(n: u32) -> u64 {
    (1..=n).map(|x| (x as u64) * (x as u64)).sum()
}

/// Product of elements (return 1 for empty)
pub fn product(items: &[i32]) -> i64 {
    items.iter().fold(1i64, |acc, &x| acc * x as i64)
}

/// Flatten a vec of vecs into a single vec
pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|v| v.iter().cloned()).collect()
}

/// Zip two slices and sum corresponding elements
pub fn zip_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

/// Get elements at even indices (0, 2, 4, ...)
pub fn even_indexed<T: Clone>(items: &[T]) -> Vec<T> {
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v.clone())
        .collect()
}

/// Find the running maximum: [3, 1, 4, 1, 5] => [3, 3, 4, 4, 5]
pub fn running_max(items: &[i32]) -> Vec<i32> {
    items
        .iter()
        .scan(i32::MIN, |max, &x| {
            *max = (*max).max(x);
            Some(*max)
        })
        .collect()
}

// ============================================
// Topic 2: Chained Transformations
// Learn: Complex pipelines, take_while, skip_while
// ============================================

/// Extract words, lowercase, sort, deduplicate
pub fn unique_words_sorted(text: &str) -> Vec<String> {
    let mut words: Vec<String> = text.split_whitespace().map(|w| w.to_lowercase()).collect();
    words.sort();
    words.dedup();
    words
}

/// Take elements while they are ascending
pub fn take_while_ascending(items: &[i32]) -> Vec<i32> {
    if items.is_empty() {
        return vec![];
    }
    let mut result = vec![items[0]];
    for window in items.windows(2) {
        if window[1] >= window[0] {
            result.push(window[1]);
        } else {
            break;
        }
    }
    result
}

/// Skip leading zeros and collect the rest
pub fn skip_leading_zeros(items: &[i32]) -> Vec<i32> {
    items.iter().skip_while(|&&x| x == 0).copied().collect()
}

/// Group consecutive equal elements: [1,1,2,2,2,3] => [[1,1],[2,2,2],[3]]
pub fn group_consecutive(items: &[i32]) -> Vec<Vec<i32>> {
    if items.is_empty() {
        return vec![];
    }
    let mut result: Vec<Vec<i32>> = vec![vec![items[0]]];
    for &item in &items[1..] {
        if item == *result.last().unwrap().last().unwrap() {
            result.last_mut().unwrap().push(item);
        } else {
            result.push(vec![item]);
        }
    }
    result
}

// ============================================
// Topic 3: Custom Iterator — Counter
// Learn: Implementing Iterator trait
// ============================================

/// A counter that counts from start by step
pub struct Counter {
    current: i32,
    step: i32,
    limit: Option<i32>,
}

impl Counter {
    /// Count from start by step, no limit
    pub fn new(start: i32, step: i32) -> Self {
        Counter {
            current: start,
            step,
            limit: None,
        }
    }

    /// Count from start by step, up to (not including) limit
    pub fn with_limit(start: i32, step: i32, limit: i32) -> Self {
        Counter {
            current: start,
            step,
            limit: Some(limit),
        }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.limit {
            if (self.step > 0 && self.current >= limit) || (self.step < 0 && self.current <= limit)
            {
                return None;
            }
        }
        let val = self.current;
        self.current += self.step;
        Some(val)
    }
}

// ============================================
// Topic 4: Custom Iterator — Fibonacci
// Learn: Stateful iterators, infinite sequences
// ============================================

/// Fibonacci iterator: 0, 1, 1, 2, 3, 5, 8, ...
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.a;
        let next_b = self.a + self.b;
        self.a = self.b;
        self.b = next_b;
        Some(val)
    }
}

/// Collect the first n Fibonacci numbers
pub fn first_n_fib(n: usize) -> Vec<u64> {
    Fibonacci::new().take(n).collect()
}

/// Sum of Fibonacci numbers below a limit
pub fn fib_sum_below(limit: u64) -> u64 {
    Fibonacci::new().take_while(|&x| x < limit).sum()
}

// ============================================
// Topic 5: Custom Iterator — Range Chunks
// Learn: Iterator adaptors, chunks
// ============================================

/// An iterator that yields chunks of a given size from a vec
pub struct Chunks<T> {
    data: Vec<T>,
    chunk_size: usize,
    pos: usize,
}

impl<T: Clone> Chunks<T> {
    pub fn new(data: Vec<T>, chunk_size: usize) -> Self {
        Chunks {
            data,
            chunk_size: chunk_size.max(1),
            pos: 0,
        }
    }
}

impl<T: Clone> Iterator for Chunks<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let end = (self.pos + self.chunk_size).min(self.data.len());
        let chunk = self.data[self.pos..end].to_vec();
        self.pos = end;
        Some(chunk)
    }
}

/// Split a slice into chunks
pub fn chunk_slice<T: Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    Chunks::new(items.to_vec(), size).collect()
}

// ============================================
// Topic 6: Advanced — Iterator Combinators
// Learn: scan, chain, interleave, windows
// ============================================

/// Running sum: [1, 2, 3] => [1, 3, 6]
pub fn running_sum(items: &[i32]) -> Vec<i32> {
    items
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

/// Interleave two slices: [a,b,c], [1,2,3] => [a,1,b,2,c,3]
pub fn interleave<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter()
        .zip(b.iter())
        .flat_map(|(x, y)| vec![x.clone(), y.clone()])
        .chain(a.iter().skip(b.len()).cloned())
        .chain(b.iter().skip(a.len()).cloned())
        .collect()
}

/// Chain multiple iterators: flatten a vec of vecs into one stream
pub fn chain_all(vecs: Vec<Vec<i32>>) -> Vec<i32> {
    vecs.into_iter().flatten().collect()
}

/// Sliding window averages: [1,2,3,4,5] with size 3 => [2.0, 3.0, 4.0]
pub fn window_averages(items: &[f64], size: usize) -> Vec<f64> {
    if size == 0 || items.len() < size {
        return vec![];
    }
    items
        .windows(size)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect()
}

/// Pairwise differences: [1, 3, 6, 10] => [2, 3, 4]
pub fn pairwise_diff(items: &[i32]) -> Vec<i32> {
    items.windows(2).map(|w| w[1] - w[0]).collect()
}

// ============================================
// Topic 7: Extra Practice
// Learn: More iterator combinator practice
// ============================================

/// Chain two iterators and collect unique elements.
pub fn unique_from_both(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut seen = std::collections::HashSet::new();
    a.iter().chain(b.iter())
        .filter(|&&x| seen.insert(x))
        .copied()
        .collect()
}

/// Use scan to create running max.
pub fn cumulative_max(items: &[i32]) -> Vec<i32> {
    items.iter()
        .scan(i32::MIN, |max, &x| {
            *max = (*max).max(x);
            Some(*max)
        })
        .collect()
}

/// Group consecutive elements using iterators.
pub fn group_runs(items: &[i32]) -> Vec<(i32, usize)> {
    if items.is_empty() { return vec![]; }
    let mut result: Vec<(i32, usize)> = vec![(items[0], 1)];
    for &x in &items[1..] {
        if x == result.last().unwrap().0 {
            result.last_mut().unwrap().1 += 1;
        } else {
            result.push((x, 1));
        }
    }
    result
}

/// Zip with index: return (index, value) for items matching predicate.
pub fn indexed_filter(items: &[i32], pred: fn(i32) -> bool) -> Vec<(usize, i32)> {
    items.iter().enumerate()
        .filter(|&(_, &x)| pred(x))
        .map(|(i, &x)| (i, x))
        .collect()
}
