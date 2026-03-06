// ============================================
// Level 3, Project 2: HashMaps — Collections
// Learn: HashMap, BTreeMap, entry API, counting, grouping
// ============================================

use std::collections::{BTreeMap, HashMap};

// ============================================
// Topic 1: HashMap Basics
// Learn: insert, get, contains_key, remove, len
// ============================================

/// Count how many times each word appears in a string
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    map
}

/// Count how many times each character appears
pub fn char_frequency(text: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in text.chars() {
        if !c.is_whitespace() {
            *map.entry(c).or_insert(0) += 1;
        }
    }
    map
}

/// Create a HashMap from two parallel slices (keys, values)
pub fn zip_to_map<'a>(keys: &[&'a str], values: &[i32]) -> HashMap<&'a str, i32> {
    keys.iter()
        .zip(values.iter())
        .map(|(&k, &v)| (k, v))
        .collect()
}

/// Invert a map: swap keys and values (assumes values are unique)
pub fn invert_map(map: &HashMap<String, String>) -> HashMap<String, String> {
    map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

// ============================================
// Topic 2: Entry API
// Learn: entry(), or_insert, or_insert_with, and_modify
// ============================================

/// Group words by their first letter
pub fn group_by_first_letter(words: &[&str]) -> HashMap<char, Vec<String>> {
    let mut map: HashMap<char, Vec<String>> = HashMap::new();
    for word in words {
        if let Some(first) = word.chars().next() {
            map.entry(first.to_lowercase().next().unwrap())
                .or_default()
                .push(word.to_string());
        }
    }
    map
}

/// Group numbers by even/odd
pub fn group_by_parity(numbers: &[i32]) -> HashMap<&'static str, Vec<i32>> {
    let mut map = HashMap::new();
    for &n in numbers {
        let key = if n % 2 == 0 { "even" } else { "odd" };
        map.entry(key).or_insert_with(Vec::new).push(n);
    }
    map
}

/// Track scores: add score to existing total or start fresh
pub fn accumulate_scores(entries: &[(&str, i32)]) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for &(name, score) in entries {
        map.entry(name.to_string())
            .and_modify(|total| *total += score)
            .or_insert(score);
    }
    map
}

/// Count consecutive duplicates using entry API
pub fn count_runs(items: &[&str]) -> Vec<(String, usize)> {
    if items.is_empty() {
        return vec![];
    }
    let mut result: Vec<(String, usize)> = vec![];
    for &item in items {
        if let Some(last) = result.last_mut() {
            if last.0 == item {
                last.1 += 1;
                continue;
            }
        }
        result.push((item.to_string(), 1));
    }
    result
}

// ============================================
// Topic 3: BTreeMap — Ordered Maps
// Learn: BTreeMap, range queries, first/last
// ============================================

/// Create a sorted frequency table (BTreeMap so keys are sorted)
pub fn sorted_word_count(text: &str) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    map
}

/// Return all keys in a range (inclusive start, exclusive end)
pub fn keys_in_range(map: &BTreeMap<i32, String>, start: i32, end: i32) -> Vec<i32> {
    map.range(start..end).map(|(&k, _)| k).collect()
}

/// Get the minimum and maximum keys
pub fn min_max_keys(map: &BTreeMap<i32, String>) -> Option<(i32, i32)> {
    let min = map.keys().next()?;
    let max = map.keys().next_back()?;
    Some((*min, *max))
}

/// Merge two BTreeMaps; on conflict, keep the value from the second map
pub fn merge_btree(a: &BTreeMap<String, i32>, b: &BTreeMap<String, i32>) -> BTreeMap<String, i32> {
    let mut result = a.clone();
    for (k, v) in b {
        result.insert(k.clone(), *v);
    }
    result
}

// ============================================
// Topic 4: Map Transformations
// Learn: Transforming, filtering, reducing maps
// ============================================

/// Filter a map to keep only entries where value >= threshold
pub fn filter_by_value(map: &HashMap<String, i32>, threshold: i32) -> HashMap<String, i32> {
    map.iter()
        .filter(|(_, v)| **v >= threshold)
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

/// Transform all values with a function
pub fn map_values(map: &HashMap<String, i32>, f: fn(i32) -> i32) -> HashMap<String, i32> {
    map.iter().map(|(k, v)| (k.clone(), f(*v))).collect()
}

/// Sum all values in a map
pub fn sum_values(map: &HashMap<String, i32>) -> i32 {
    map.values().sum()
}

/// Find the key with the maximum value
pub fn key_with_max_value(map: &HashMap<String, i32>) -> Option<String> {
    map.iter().max_by_key(|(_, v)| **v).map(|(k, _)| k.clone())
}

/// Return keys sorted by their values (ascending)
pub fn keys_sorted_by_value(map: &HashMap<String, i32>) -> Vec<String> {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by_key(|(_, v)| **v);
    entries.into_iter().map(|(k, _)| k.clone()).collect()
}

// ============================================
// Topic 5: Nested Maps & Complex Keys
// Learn: HashMap<K, HashMap<K2, V>>, composite keys
// ============================================

/// Grade book: map of student -> map of subject -> grade
pub type GradeBook = HashMap<String, HashMap<String, f64>>;

/// Add a grade to the grade book
pub fn add_grade(book: &mut GradeBook, student: &str, subject: &str, grade: f64) {
    book.entry(student.to_string())
        .or_default()
        .insert(subject.to_string(), grade);
}

/// Get a student's average grade
pub fn student_average(book: &GradeBook, student: &str) -> Option<f64> {
    let grades = book.get(student)?;
    if grades.is_empty() {
        return None;
    }
    let sum: f64 = grades.values().sum();
    Some(sum / grades.len() as f64)
}

/// Get all students who have a grade in a given subject
pub fn students_in_subject(book: &GradeBook, subject: &str) -> Vec<String> {
    book.iter()
        .filter(|(_, subjects)| subjects.contains_key(subject))
        .map(|(student, _)| student.clone())
        .collect()
}

/// Get the top student (highest average)
pub fn top_student(book: &GradeBook) -> Option<String> {
    book.keys()
        .filter_map(|student| student_average(book, student).map(|avg| (student.clone(), avg)))
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(name, _)| name)
}

// ============================================
// Topic 6: Advanced — Set Operations via HashMap
// Learn: Using maps as sets, intersection, difference
// ============================================

/// Return elements common to both slices
pub fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_b: HashMap<i32, ()> = b.iter().map(|&v| (v, ())).collect();
    let mut seen = HashMap::new();
    a.iter()
        .filter(|&&v| set_b.contains_key(&v) && seen.insert(v, ()).is_none())
        .copied()
        .collect()
}

/// Return elements in a but not in b
pub fn difference(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_b: HashMap<i32, ()> = b.iter().map(|&v| (v, ())).collect();
    let mut seen = HashMap::new();
    a.iter()
        .filter(|&&v| !set_b.contains_key(&v) && seen.insert(v, ()).is_none())
        .copied()
        .collect()
}

/// Check if a is a subset of b (every element of a is in b)
pub fn is_subset(a: &[i32], b: &[i32]) -> bool {
    let set_b: HashMap<i32, ()> = b.iter().map(|&v| (v, ())).collect();
    a.iter().all(|v| set_b.contains_key(v))
}

/// Find the most frequent element
pub fn most_frequent(items: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(val, _)| val)
}

// ============================================
// Topic 7: Extra Practice
// Learn: More HashMap algorithms and patterns
// ============================================

/// Two-sum using HashMap: find two values that sum to target.
pub fn two_sum_map(items: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &v) in items.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - v)) {
            return Some((j, i));
        }
        seen.insert(v, i);
    }
    None
}

/// Return the top N keys by value (descending).
pub fn top_n_keys(map: &HashMap<String, i32>, n: usize) -> Vec<String> {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|(_, a), (_, b)| b.cmp(a));
    entries.into_iter().take(n).map(|(k, _)| k.clone()).collect()
}

/// Build a histogram: count occurrences of each value.
pub fn value_histogram(items: &[i32]) -> HashMap<i32, usize> {
    let mut map = HashMap::new();
    for &item in items { *map.entry(item).or_insert(0) += 1; }
    map
}

/// Merge two hashmaps, summing values for shared keys.
pub fn merge_sum(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result = a.clone();
    for (k, v) in b { *result.entry(k.clone()).or_insert(0) += v; }
    result
}

