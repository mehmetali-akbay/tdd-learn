// ============================================
// Level 3, Project 4: Iterators — Custom Iterators & Chains
// Learn: Iterator trait, custom iterators, method chaining,
//        adaptor patterns, infinite sequences, combinator pipelines
// ============================================

use std::collections::HashMap;
use std::collections::HashSet;

// ============================================
// Topic 1: Iterator Basics — Method Chains
// Learn: map, filter, fold, collect, enumerate, zip, sum, product
// Reinforces: 05_vecs (collect, Vec), 14_closures (Fn trait, HOFs)
// ============================================

/// Sum of squares: 1² + 2² + ... + n².
pub fn sum_of_squares(n: u32) -> u64 {
    (1..=n).map(|x| (x as u64) * (x as u64)).sum()
}

/// Product of elements (return 1 for empty).
pub fn product(items: &[i32]) -> i64 {
    items.iter().fold(1i64, |acc, &x| acc * x as i64)
}

/// Flatten a vec of vecs into a single vec.
pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|v| v.iter().cloned()).collect()
}

/// Zip two slices and sum corresponding elements.
pub fn zip_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

/// Get elements at even indices (0, 2, 4, ...).
pub fn even_indexed<T: Clone>(items: &[T]) -> Vec<T> {
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v.clone())
        .collect()
}

/// Find the running maximum: [3, 1, 4, 1, 5] => [3, 3, 4, 4, 5].
pub fn running_max(items: &[i32]) -> Vec<i32> {
    items
        .iter()
        .scan(i32::MIN, |max, &x| {
            *max = (*max).max(x);
            Some(*max)
        })
        .collect()
}

/// Enumerate items and format as "index: value" strings.
pub fn enumerate_format<T: std::fmt::Display>(items: &[T]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, v)| format!("{}: {}", i, v))
        .collect()
}

/// Zip two slices into pairs. Stops at the shorter slice.
pub fn zip_pairs<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter().zip(b.iter()).map(|(x, y)| (x.clone(), y.clone())).collect()
}

/// Compute the dot product of two numeric slices.
pub fn dot_product(a: &[i32], b: &[i32]) -> i64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| x as i64 * y as i64)
        .sum()
}

// ============================================
// Topic 2: Chained Transformations
// Learn: Complex pipelines, take_while, skip_while, dedup, windows
// Reinforces: 06_strings (string processing), 13_files (line operations)
// ============================================

/// Extract words, lowercase, sort, deduplicate.
pub fn unique_words_sorted(text: &str) -> Vec<String> {
    let mut words: Vec<String> = text.split_whitespace().map(|w| w.to_lowercase()).collect();
    words.sort();
    words.dedup();
    words
}

/// Take elements while they are ascending.
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

/// Skip leading zeros and collect the rest.
pub fn skip_leading_zeros(items: &[i32]) -> Vec<i32> {
    items.iter().skip_while(|&&x| x == 0).copied().collect()
}

/// Group consecutive equal elements: [1,1,2,2,2,3] => [[1,1],[2,2,2],[3]].
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

/// Pairwise differences: [1, 3, 6, 10] => [2, 3, 4].
pub fn pairwise_diff(items: &[i32]) -> Vec<i32> {
    items.windows(2).map(|w| w[1] - w[0]).collect()
}

/// Sliding window averages: [1,2,3,4,5] with size 3 => [2.0, 3.0, 4.0].
pub fn window_averages(items: &[f64], size: usize) -> Vec<f64> {
    if size == 0 || items.len() < size {
        return vec![];
    }
    items
        .windows(size)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect()
}

/// Take every Nth element from a slice.
pub fn every_nth<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    if n == 0 {
        return vec![];
    }
    items.iter().step_by(n).cloned().collect()
}

/// Flatten and sort: flatten nested vecs then sort the result.
pub fn flatten_and_sort(nested: &[Vec<i32>]) -> Vec<i32> {
    let mut flat: Vec<i32> = nested.iter().flat_map(|v| v.iter().copied()).collect();
    flat.sort();
    flat
}

// ============================================
// Topic 3: Custom Iterator — Counter
// Learn: Implementing Iterator trait, next(), stateful iteration
// Reinforces: 04_structs (struct design), 09_traits (trait impl)
// ============================================

/// A counter that counts from start by step.
pub struct Counter {
    current: i32,
    step: i32,
    limit: Option<i32>,
}

impl Counter {
    /// Count from start by step, no limit.
    pub fn new(start: i32, step: i32) -> Self {
        Counter {
            current: start,
            step,
            limit: None,
        }
    }

    /// Count from start by step, up to (not including) limit.
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
        if let Some(limit) = self.limit
            && ((self.step > 0 && self.current >= limit)
                || (self.step < 0 && self.current <= limit))
        {
            return None;
        }
        let val = self.current;
        self.current += self.step;
        Some(val)
    }
}

// ============================================
// Topic 4: Custom Iterator — Fibonacci & Sequences
// Learn: Stateful iterators, infinite sequences, take/take_while
// Reinforces: 10_generics (generic thinking), 08_results (Option)
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

/// Collect the first n Fibonacci numbers.
pub fn first_n_fib(n: usize) -> Vec<u64> {
    Fibonacci::new().take(n).collect()
}

/// Sum of Fibonacci numbers below a limit.
pub fn fib_sum_below(limit: u64) -> u64 {
    Fibonacci::new().take_while(|&x| x < limit).sum()
}

/// Collatz sequence: start from n, if even n/2, if odd 3n+1, until 1.
pub struct Collatz {
    current: Option<u64>,
}

impl Collatz {
    pub fn new(start: u64) -> Self {
        Collatz {
            current: if start == 0 { None } else { Some(start) },
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current?;
        if val == 1 {
            self.current = None;
        } else if val % 2 == 0 {
            self.current = Some(val / 2);
        } else {
            self.current = Some(3 * val + 1);
        }
        Some(val)
    }
}

/// Collect the Collatz sequence starting from n.
pub fn collatz_sequence(n: u64) -> Vec<u64> {
    Collatz::new(n).collect()
}

/// Length of the Collatz sequence starting from n.
pub fn collatz_length(n: u64) -> usize {
    Collatz::new(n).count()
}

// ============================================
// Topic 5: Custom Iterator — Chunks & Adapters
// Learn: Iterator adaptors, chunks, cycle, stepping
// Reinforces: 10_generics (generic structs), 05_vecs (slicing)
// ============================================

/// An iterator that yields chunks of a given size from a vec.
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

/// Split a slice into chunks.
pub fn chunk_slice<T: Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    Chunks::new(items.to_vec(), size).collect()
}

/// A cycling iterator that repeats a slice N times.
pub struct CycleN<T> {
    data: Vec<T>,
    total: usize,
    index: usize,
}

impl<T: Clone> CycleN<T> {
    pub fn new(data: Vec<T>, times: usize) -> Self {
        CycleN {
            total: data.len() * times,
            data,
            index: 0,
        }
    }
}

impl<T: Clone> Iterator for CycleN<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() || self.index >= self.total {
            return None;
        }
        let item = self.data[self.index % self.data.len()].clone();
        self.index += 1;
        Some(item)
    }
}

/// Cycle a slice N times.
pub fn cycle_n<T: Clone>(items: &[T], times: usize) -> Vec<T> {
    CycleN::new(items.to_vec(), times).collect()
}

/// A step-by iterator that skips elements.
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first: bool,
}

impl<I: Iterator> StepBy<I> {
    pub fn new(iter: I, step: usize) -> Self {
        StepBy {
            iter,
            step: step.max(1),
            first: true,
        }
    }
}

impl<I: Iterator> Iterator for StepBy<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return self.iter.next();
        }
        for _ in 0..self.step - 1 {
            self.iter.next();
        }
        self.iter.next()
    }
}

/// Collect every Nth element using the custom StepBy iterator.
pub fn step_by_collect<T>(items: Vec<T>, step: usize) -> Vec<T> {
    StepBy::new(items.into_iter(), step).collect()
}

// ============================================
// Topic 6: Advanced — Iterator Combinators
// Learn: scan, chain, interleave, windows, complex pipelines
// Reinforces: 14_closures (closure composition), 07_hashmaps (grouping)
// ============================================

/// Running sum: [1, 2, 3] => [1, 3, 6].
pub fn running_sum(items: &[i32]) -> Vec<i32> {
    items
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

/// Interleave two slices: [a,b,c], [1,2,3] => [a,1,b,2,c,3].
pub fn interleave<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter()
        .zip(b.iter())
        .flat_map(|(x, y)| vec![x.clone(), y.clone()])
        .chain(a.iter().skip(b.len()).cloned())
        .chain(b.iter().skip(a.len()).cloned())
        .collect()
}

/// Chain multiple iterators: flatten a vec of vecs into one stream.
pub fn chain_all(vecs: Vec<Vec<i32>>) -> Vec<i32> {
    vecs.into_iter().flatten().collect()
}

/// Chain two slices and collect unique elements (preserving first occurrence).
pub fn unique_from_both(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    a.iter()
        .chain(b.iter())
        .filter(|&&x| seen.insert(x))
        .copied()
        .collect()
}

/// Cumulative max using scan.
pub fn cumulative_max(items: &[i32]) -> Vec<i32> {
    items
        .iter()
        .scan(i32::MIN, |max, &x| {
            *max = (*max).max(x);
            Some(*max)
        })
        .collect()
}

/// Group consecutive elements into (value, count) runs.
pub fn group_runs(items: &[i32]) -> Vec<(i32, usize)> {
    if items.is_empty() {
        return vec![];
    }
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

/// Zip with index: return (index, value) for items matching a predicate.
pub fn indexed_filter(items: &[i32], pred: fn(i32) -> bool) -> Vec<(usize, i32)> {
    items
        .iter()
        .enumerate()
        .filter(|&(_, &x)| pred(x))
        .map(|(i, &x)| (i, x))
        .collect()
}

/// Frequency count of elements in a slice.
pub fn frequency_count<T: Eq + std::hash::Hash + Clone>(items: &[T]) -> HashMap<T, usize> {
    let mut map = HashMap::new();
    for item in items {
        *map.entry(item.clone()).or_insert(0) += 1;
    }
    map
}

/// Find the top N most frequent elements.
pub fn top_n_frequent(items: &[i32], n: usize) -> Vec<(i32, usize)> {
    let freq = frequency_count(items);
    let mut pairs: Vec<(i32, usize)> = freq.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.into_iter().take(n).collect()
}

// ============================================
// Topic 7: Iterator Utilities & Conversions
// Learn: Practical iterator utilities, batching, folding patterns
// Reinforces: 08_results (Option/Result with iterators), 06_strings (collect)
// ============================================

/// Batch process: apply a function to batches of items.
pub fn batch_process<T: Clone, R>(items: &[T], batch_size: usize, f: impl Fn(&[T]) -> R) -> Vec<R> {
    Chunks::new(items.to_vec(), batch_size).map(|batch| f(&batch)).collect()
}

/// Partition a slice by a predicate, returning (matching, rest).
pub fn partition_by<T: Clone>(items: &[T], pred: impl Fn(&T) -> bool) -> (Vec<T>, Vec<T>) {
    let mut matching = Vec::new();
    let mut rest = Vec::new();
    for item in items {
        if pred(item) {
            matching.push(item.clone());
        } else {
            rest.push(item.clone());
        }
    }
    (matching, rest)
}

/// Map items and collect only the Some results (filter_map).
pub fn filter_map_vec<T, U>(items: &[T], f: impl Fn(&T) -> Option<U>) -> Vec<U> {
    items.iter().filter_map(f).collect()
}

/// Try to parse all strings as numbers, returning Err on first failure.
pub fn parse_all_numbers(items: &[&str]) -> Result<Vec<f64>, String> {
    items
        .iter()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| format!("cannot parse '{}'", s))
        })
        .collect()
}

/// Find the position and value of the maximum element.
pub fn max_with_index(items: &[i32]) -> Option<(usize, i32)> {
    items
        .iter()
        .enumerate()
        .max_by_key(|&(_, &val)| val)
        .map(|(i, &val)| (i, val))
}

/// Find the position and value of the minimum element.
pub fn min_with_index(items: &[i32]) -> Option<(usize, i32)> {
    items
        .iter()
        .enumerate()
        .min_by_key(|&(_, &val)| val)
        .map(|(i, &val)| (i, val))
}

/// Join items with a separator using iterators (like str::join but generic).
pub fn join_with<T: std::fmt::Display>(items: &[T], sep: &str) -> String {
    items
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}

/// Compute a histogram: given numeric items, count occurrences per bucket.
pub fn histogram(items: &[i32], bucket_size: i32) -> Vec<(i32, usize)> {
    if bucket_size <= 0 || items.is_empty() {
        return vec![];
    }
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &item in items {
        let bucket = (item / bucket_size) * bucket_size;
        *freq.entry(bucket).or_insert(0) += 1;
    }
    let mut pairs: Vec<(i32, usize)> = freq.into_iter().collect();
    pairs.sort_by_key(|&(bucket, _)| bucket);
    pairs
}

/// Scan to produce running averages.
pub fn running_average(items: &[f64]) -> Vec<f64> {
    items
        .iter()
        .enumerate()
        .scan(0.0_f64, |sum, (i, &x)| {
            *sum += x;
            Some(*sum / (i + 1) as f64)
        })
        .collect()
}

/// Unzip a slice of pairs into two vecs.
pub fn unzip_pairs<A: Clone, B: Clone>(pairs: &[(A, B)]) -> (Vec<A>, Vec<B>) {
    pairs.iter().cloned().unzip()
}
