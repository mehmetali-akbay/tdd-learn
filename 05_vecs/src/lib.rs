// ============================================
// Topic 1: Vec Basics
// Learn: Creating, pushing, popping, indexing
// ============================================

use std::{
    collections::{HashMap, HashSet},
    vec,
};

/// Create and return a new empty Vec<i32>
pub fn create_empty_vec() -> Vec<i32> {
    let vec: Vec<i32> = Vec::new();
    vec
}

/// Create a vec containing [1, 2, 3, 4, 5] using the vec! macro
pub fn create_sample_vec() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

/// Push an element to the end of the vec and return it
pub fn push_element(mut v: Vec<i32>, elem: i32) -> Vec<i32> {
    v.push(elem);
    v
}

/// Remove and return the last element, or None if empty
pub fn pop_last(v: &mut Vec<i32>) -> Option<i32> {
    v.pop()
}

/// Return the length of the vec
pub fn vec_length(v: &[i32]) -> usize {
    v.len()
}

/// Return the element at the given index, or None if out of bounds
pub fn get_element(v: &[i32], index: usize) -> Option<i32> {
    v.get(index).copied()
}

/// Return true if the vec contains the given value
pub fn vec_contains(v: &[i32], value: i32) -> bool {
    v.contains(&value)
}

// ============================================
// Topic 2: Vec Iteration
// Learn: Iterating, summing, finding, counting
// ============================================

/// Sum all elements in the vec
pub fn sum_all(v: &[i32]) -> i32 {
    v.iter().sum()
}

/// Find and return the maximum element, or None if empty
pub fn find_max(v: &[i32]) -> Option<i32> {
    v.iter().max().copied()
}

/// Find and return the minimum element, or None if empty
pub fn find_min(v: &[i32]) -> Option<i32> {
    v.iter().min().copied()
}

/// Return true if all elements are positive (> 0)
pub fn all_positive(v: &[i32]) -> bool {
    v.iter().all(|&e| e > 0)
}

/// Return true if any element equals the target
pub fn any_equals(v: &[i32], target: i32) -> bool {
    v.iter().any(|&e| e == target)
}

/// Count how many elements are greater than the threshold
pub fn count_above(v: &[i32], threshold: i32) -> usize {
    v.iter().filter(|&&e| e > threshold).count()
}

/// Calculate the average of all elements, return None if empty
pub fn average(v: &[i32]) -> Option<f64> {
    if v.is_empty() {
        None
    } else {
        Some(v.iter().sum::<i32>() as f64 / v.len() as f64)
    }
}

// ============================================
// Topic 3: Vec Transform
// Learn: map, filter, collect transformations
// ============================================

/// Double every element in the vec
/// Example: [1, 2, 3] => [2, 4, 6]
pub fn double_all(v: &[i32]) -> Vec<i32> {
    v.iter().map(|&e| e * 2).collect()
}

/// Keep only even numbers
/// Example: [1, 2, 3, 4, 5] => [2, 4]
pub fn filter_even(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|&&e| e % 2 == 0).copied().collect()
}

/// Square every element
/// Example: [1, 2, 3] => [1, 4, 9]
pub fn square_all(v: &[i32]) -> Vec<i32> {
    v.iter().map(|&e| e * e).collect()
}

/// Filter strings that are longer than `min_len`
pub fn filter_by_length(words: &[&str], min_len: usize) -> Vec<String> {
    words
        .iter()
        .filter(|&&e| e.len() > min_len)
        .copied()
        .map(String::from)
        .collect()
}

/// Convert all strings to uppercase
pub fn all_to_uppercase(words: &[&str]) -> Vec<String> {
    words.iter().map(|&e| e.to_uppercase()).collect()
}

/// Add an index prefix to each element
/// Example: ["a", "b"] => ["0: a", "1: b"]
pub fn add_index_prefix(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .map(|(i, val)| format!("{i}: {val}"))
        .collect()
}

// ============================================
// Topic 4: Vec Sorting & Dedup
// Learn: sort, sort_by, dedup, unique
// ============================================

/// Sort the vec in ascending order
pub fn sort_ascending(v: &[i32]) -> Vec<i32> {
    let mut sorted = v.to_vec();
    sorted.sort();
    sorted
}

/// Sort the vec in descending order
pub fn sort_descending(v: &[i32]) -> Vec<i32> {
    let mut sorted = v.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted
}

/// Sort strings by their length (shortest first)
pub fn sort_by_length(words: &[&str]) -> Vec<String> {
    let mut sorted = words.to_vec();
    sorted.sort_by(|a, b| a.len().cmp(&b.len()));
    sorted.iter().map(|a| a.to_string()).collect()
}

/// Remove consecutive duplicate elements
/// Example: [1, 1, 2, 2, 3, 1] => [1, 2, 3, 1]
pub fn remove_consecutive_dupes(v: &[i32]) -> Vec<i32> {
    let mut result = v.to_vec();
    result.dedup();
    result
}

/// Return only unique elements (remove all duplicates, preserve order)
/// Example: [3, 1, 2, 1, 3] => [3, 1, 2]
pub fn unique_elements(v: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    v.iter().filter(|&e| seen.insert(e)).copied().collect()
}

// ============================================
// Topic 5: Vec Slices, Chunks & Combining
// Learn: take, split, windows, chunks, flatten, zip
// ============================================

/// Return the first `n` elements as a new vec
/// If n > length, return all elements
pub fn take_first(v: &[i32], n: usize) -> Vec<i32> {
    v.iter().take(n).cloned().collect()
}

/// Split the vec at index `pos` and return both halves
pub fn split_at_index(v: &[i32], pos: usize) -> (Vec<i32>, Vec<i32>) {
    let (left, right) = v.split_at(pos.min(v.len()));
    (left.to_vec(), right.to_vec())
}

/// Return chunks of size `n`
/// Example: [1,2,3,4,5] with n=2 => [[1,2], [3,4], [5]]
pub fn make_chunks(v: &[i32], size: usize) -> Vec<Vec<i32>> {
    v.chunks(size).map(|e| e.to_vec()).collect()
}

/// Return sliding windows of size `n`
/// Example: [1,2,3,4] with n=2 => [[1,2], [2,3], [3,4]]
pub fn sliding_windows(v: &[i32], size: usize) -> Vec<Vec<i32>> {
    v.windows(size).map(|e| e.to_vec()).collect()
}

/// Flatten a vec of vecs into a single vec
/// Example: [[1,2], [3,4]] => [1,2,3,4]
pub fn flatten(nested: &[Vec<i32>]) -> Vec<i32> {
    nested.iter().flatten().copied().collect()
}

/// Zip two vecs into a vec of tuples
/// Example: [1,2,3] + ["a","b","c"] => [(1,"a"), (2,"b"), (3,"c")]
pub fn zip_vecs(nums: &[i32], words: &[&str]) -> Vec<(i32, String)> {
    nums.iter()
        .zip(words)
        .map(|(&n, &w)| (n, w.to_string()))
        .collect()
}

// ============================================
// Topic 6: Advanced Vec Challenges
// Learn: Algorithms, HashMap combos, matrices
// ============================================

/// Two Sum: find two indices whose values sum to target
/// Return Some((i, j)) where i < j, or None if no pair found
pub fn two_sum(v: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in v.iter().enumerate() {
        let target_value = target - num;
        if let Some(&first_index) = map.get(&target_value) {
            return Some((first_index, i));
        }
        map.insert(num, i);
    }
    None
}

/// Running sum: each element is the sum of all previous elements plus itself
/// Example: [1, 2, 3, 4] => [1, 3, 6, 10]
pub fn running_sum(v: &[i32]) -> Vec<i32> {
    let mut sum = 0;
    v.iter()
        .map(|e| {
            sum += e;
            sum
        })
        .collect()
}

/// Merge two already-sorted vecs into one sorted vec
pub fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut r: Vec<i32> = Vec::with_capacity(a.len()+b.len());

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            r.push(a[i]);
            i += 1;
        } else {
            r.push(b[j]);
            j += 1;
        }
    }

    while i < a.len() {
        r.push(a[i]);
        i += 1;
    }

    while j < b.len() {
        r.push(b[j]);
        j += 1;
    }
    r
}

/// Partition: split into (matching, non_matching) based on a predicate
pub fn partition(v: &[i32], predicate: fn(i32) -> bool) -> (Vec<i32>, Vec<i32>) {
    v.iter().copied().partition(|&e| predicate(e))
}

/// Group consecutive equal elements
/// Example: [1,1,2,2,2,3,1,1] => [[1,1],[2,2,2],[3],[1,1]]
pub fn group_consecutive(v: &[i32]) -> Vec<Vec<i32>> {
   v.chunk_by(|a,b| a == b).map(|e| e.to_vec()).collect()
}

/// Moving average with a window size
/// Example: [1.0, 2.0, 3.0, 4.0, 5.0] window=3 => [2.0, 3.0, 4.0]
pub fn moving_average(v: &[f64], window: usize) -> Vec<f64> {
    v.windows(window).map(|e| e.iter().sum::<f64>()/window as f64).collect()
}

/// Rotate a vec left by `n` positions
/// Example: rotate_left([1,2,3,4,5], 2) => [3,4,5,1,2]
pub fn rotate_left(v: &[i32], n: usize) -> Vec<i32> {
    todo!()
}

/// Check if a vec is sorted in ascending order
pub fn is_sorted(v: &[i32]) -> bool {
    todo!()
}

/// Find the majority element (appears more than n/2 times)
/// Return None if no majority element exists
pub fn majority_element(v: &[i32]) -> Option<i32> {
    todo!()
}

/// Find all pairs that sum to target, returned as (smaller, larger), sorted
pub fn find_pairs_with_sum(v: &[i32], target: i32) -> Vec<(i32, i32)> {
    todo!()
}

/// Matrix transpose: swap rows and columns
/// Example: [[1,2,3],[4,5,6]] => [[1,4],[2,5],[3,6]]
pub fn transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    todo!()
}

/// Product of all elements except self (without using division)
/// Example: [1,2,3,4] => [24, 12, 8, 6]
pub fn product_except_self(v: &[i32]) -> Vec<i32> {
    todo!()
}

// ============================================
// Topic 7: Extra Practice
// Learn: More algorithms for repetition and muscle memory
// ============================================

/// Interleave two vecs: [1,2,3] + [a,b,c] => [1,a,2,b,3,c]
pub fn interleave(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

/// Rotate right by n positions: [1,2,3,4,5], 2 => [4,5,1,2,3]
pub fn rotate_right(v: &[i32], n: usize) -> Vec<i32> {
    todo!()
}

/// Check if a vec reads the same forwards and backwards
pub fn is_palindrome(v: &[i32]) -> bool {
    todo!()
}

/// Find the nth largest element (1-indexed: 1 = largest)
pub fn nth_largest(v: &[i32], n: usize) -> Option<i32> {
    todo!()
}

/// Return elements with their original indices where predicate matches
pub fn enumerate_filter(v: &[i32], predicate: fn(i32) -> bool) -> Vec<(usize, i32)> {
    todo!()
}
