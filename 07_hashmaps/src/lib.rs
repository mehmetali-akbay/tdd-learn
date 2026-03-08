// =============================================================
// Module 07: Hash Maps — Associative Collections
// =============================================================
// Covers: The Rust Book Chapter 8, Section 3
//   1. Creating Hash Maps
//   2. Accessing Values
//   3. Ownership and Hash Maps
//   4. Updating — Entry API
//   5. Counting and Grouping Patterns
//   6. BTreeMap — Ordered Maps
//   7. Map Transformations
//   8. HashSet Fundamentals
//   9. Nested Maps and Complex Structures
//  10. Advanced HashMap Patterns
//
// Reinforces prior topics:
//   - Ownership & borrowing (String moved into map, &str keys)
//   - Option<T> (get() returns Option<&V>)
//   - Iterators (keys(), values(), iter(), collect())
//   - Pattern matching (match on Option, entry variants)
//   - Structs with methods (composite data in maps)
// =============================================================

use std::collections::{BTreeMap, HashMap, HashSet};

// ── Topic 1: Creating Hash Maps ─────────────────────────────
// HashMap::new() + insert, collect() from iterators, HashMap::from()
// The Rust Book shows: new + insert, and collecting from zipped iterators.

/// Create a score board: {"Alice": 100, "Bob": 85, "Charlie": 92}.
/// Demonstrates the basic HashMap::new() + insert pattern.
pub fn create_scores() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("Alice".to_string(), 100);
    map.insert("Bob".to_string(), 85);
    map.insert("Charlie".to_string(), 92);
    map
}

/// Create a HashMap from a slice of (key, value) tuples.
/// Demonstrates collecting from an iterator of pairs.
/// (Reinforces: iterators, map(), collect(), String ownership)
pub fn from_tuples(pairs: &[(&str, i32)]) -> HashMap<String, i32> {
    pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
}

/// Create a HashMap from two parallel slices (keys, values).
/// If slices differ in length, zip stops at the shorter one.
/// (Reinforces: zip, iterators, collect)
pub fn zip_to_map<'a>(keys: &[&'a str], values: &[i32]) -> HashMap<&'a str, i32> {
    keys.iter()
        .zip(values.iter())
        .map(|(&k, &v)| (k, v))
        .collect()
}

/// Create a HashMap from keys, giving each the same default value.
/// Example: from_keys_with_default(&["x","y","z"], 0) → {"x":0, "y":0, "z":0}
pub fn from_keys_with_default(keys: &[&str], default: i32) -> HashMap<String, i32> {
    keys.iter().map(|&k| (k.to_string(), default)).collect()
}

/// Create a HashMap using the array literal syntax: HashMap::from().
/// Returns: {"one": 1, "two": 2, "three": 3}
pub fn from_array_literal() -> HashMap<&'static str, i32> {
    HashMap::from([("one", 1), ("two", 2), ("three", 3)])
}

// ── Topic 2: Accessing Values ───────────────────────────────
// get() → Option<&V>, contains_key(), keys(), values(), iteration.
// The Rust Book shows: get().copied().unwrap_or(0) pattern.

/// Safely get a value by key, returning a default if not found.
/// Uses the .get().copied().unwrap_or() pattern from the Rust Book.
/// (Reinforces: Option<&V>, copied(), unwrap_or())
pub fn get_or_default(map: &HashMap<String, i32>, key: &str, default: i32) -> i32 {
    map.get(key).copied().unwrap_or(default)
}

/// Look up multiple keys, returning a Vec of Option values.
/// Uses get().copied() for each key.
/// (Reinforces: iterating over keys, get() → Option<&V>, copied())
pub fn get_many(map: &HashMap<String, i32>, keys: &[&str]) -> Vec<Option<i32>> {
    keys.iter().map(|&k| map.get(k).copied()).collect()
}

/// Check if ALL the given keys exist in the map.
/// (Reinforces: contains_key(), all())
pub fn contains_all_keys(map: &HashMap<String, i32>, keys: &[&str]) -> bool {
    keys.iter().all(|&k| map.contains_key(k))
}

/// Return all keys, sorted alphabetically.
/// HashMap iteration order is arbitrary — sorting gives determinism.
/// (Reinforces: keys(), cloned(), collect(), sort())
pub fn all_keys_sorted(map: &HashMap<String, i32>) -> Vec<String> {
    let mut keys: Vec<String> = map.keys().cloned().collect();
    keys.sort();
    keys
}

/// Return all values, sorted ascending.
/// (Reinforces: values(), copied(), collect(), sort())
pub fn all_values_sorted(map: &HashMap<String, i32>) -> Vec<i32> {
    let mut vals: Vec<i32> = map.values().copied().collect();
    vals.sort();
    vals
}

/// Count how many entries have a value equal to `target`.
/// (Reinforces: values(), filter(), count())
pub fn count_by_value(map: &HashMap<String, i32>, target: i32) -> usize {
    map.values().filter(|&&v| v == target).count()
}

// ── Topic 3: Ownership and Hash Maps ────────────────────────
// The Rust Book emphasizes: String keys are MOVED into the map.
// Copy types (like i32) are copied. References need lifetimes.
// insert() returns Option<V> — the OLD value if the key existed.

/// Insert a key-value pair, returning the OLD value if the key already existed.
/// Demonstrates that insert() returns Option<V>.
/// (Reinforces: ownership — String key is moved, i32 value is copied)
pub fn insert_returns_old(
    map: &mut HashMap<String, i32>,
    key: &str,
    value: i32,
) -> Option<i32> {
    map.insert(key.to_string(), value)
}

/// Swap keys and values. Assumes all values are unique.
/// Both key and value must be cloned since we're reading from one map
/// and building another.
/// (Reinforces: clone(), iterating map entries, ownership transfer)
pub fn invert_map(map: &HashMap<String, String>) -> HashMap<String, String> {
    map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

/// Clone a map and extend it with entries from another map.
/// Keys from `extra` overwrite keys from `base` on conflict.
/// (Reinforces: clone(), extend() — consumes an iterator of pairs)
pub fn clone_and_extend(
    base: &HashMap<String, i32>,
    extra: &HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut result = base.clone();
    result.extend(extra.clone());
    result
}

/// Merge two maps: on conflict, keep the value from `first`.
/// Uses entry().or_insert() — only inserts if key is absent.
/// (Reinforces: entry API for conflict resolution)
pub fn merge_prefer_first(
    first: &HashMap<String, i32>,
    second: &HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut result = first.clone();
    for (k, v) in second {
        result.entry(k.clone()).or_insert(*v);
    }
    result
}

/// Merge two maps: on conflict, keep the value from `second`.
/// Uses insert() which always overwrites.
pub fn merge_prefer_second(
    first: &HashMap<String, i32>,
    second: &HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut result = first.clone();
    for (k, v) in second {
        result.insert(k.clone(), *v);
    }
    result
}

// ── Topic 4: Updating — Entry API ───────────────────────────
// The Rust Book covers three update strategies:
//   1. Overwrite: insert() replaces the old value
//   2. Insert-if-absent: entry().or_insert()
//   3. Update based on old value: entry().and_modify().or_insert()
// Also: or_insert_with(), values_mut(), retain(), remove()

/// Insert a value only if the key doesn't already exist.
/// Uses entry().or_insert(). Returns the final value for that key.
/// (Reinforces: Entry enum, or_insert returns &mut V)
pub fn insert_if_absent(map: &mut HashMap<String, i32>, key: &str, value: i32) -> i32 {
    *map.entry(key.to_string()).or_insert(value)
}

/// Push a value into the Vec associated with a key, creating the Vec if needed.
/// Uses entry().or_insert_with() — the closure is only called if key is absent.
/// Returns the new length of the Vec.
/// (Reinforces: or_insert_with for lazy initialization)
/// Note: or_default() is equivalent here since Vec implements Default,
/// but we use or_insert_with() explicitly to teach the pattern.
#[allow(clippy::unwrap_or_default)]
pub fn push_to_key(
    map: &mut HashMap<String, Vec<String>>,
    key: &str,
    value: &str,
) -> usize {
    let vec = map.entry(key.to_string()).or_insert_with(Vec::new);
    vec.push(value.to_string());
    vec.len()
}

/// Upsert pattern: increment if present, insert 1 if absent.
/// Uses and_modify().or_insert() — the idiomatic Rust upsert.
pub fn upsert_counter(map: &mut HashMap<String, i32>, key: &str) {
    map.entry(key.to_string())
        .and_modify(|count| *count += 1)
        .or_insert(1);
}

/// Multiply all values in the map by a factor, in place.
/// Demonstrates iterating with values_mut().
pub fn scale_all_values(map: &mut HashMap<String, i32>, factor: i32) {
    for v in map.values_mut() {
        *v *= factor;
    }
}

/// Remove all entries where the value is negative.
/// Demonstrates retain() — keeps only entries where the predicate is true.
/// Returns how many entries were removed.
pub fn remove_negative_values(map: &mut HashMap<String, i32>) -> usize {
    let before = map.len();
    map.retain(|_, v| *v >= 0);
    before - map.len()
}

/// Remove a key and return its value (if present).
/// Demonstrates remove() returning Option<V>.
pub fn take_value(map: &mut HashMap<String, i32>, key: &str) -> Option<i32> {
    map.remove(key)
}

/// Clear all entries from the map. Returns how many were removed.
/// After clear(), the map is empty but retains its allocated memory.
pub fn clear_map(map: &mut HashMap<String, i32>) -> usize {
    let count = map.len();
    map.clear();
    count
}

// ── Topic 5: Counting and Grouping Patterns ─────────────────
// The classic HashMap use cases from the Rust Book:
//   word counting, frequency tables, grouping by criteria.

/// Count how many times each word appears in a string.
/// The classic Rust Book word-counting example (Listing 8-25).
/// Words are lowercased for case-insensitive counting.
/// (Reinforces: split_whitespace, entry().or_insert(0), *count += 1)
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    map
}

/// Count how many times each character appears (ignoring whitespace).
pub fn char_frequency(text: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in text.chars() {
        if !c.is_whitespace() {
            *map.entry(c).or_insert(0) += 1;
        }
    }
    map
}

/// Group words by their first letter (lowercased).
/// (Reinforces: entry().or_default(), Option handling)
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

/// Group numbers by even/odd.
/// (Reinforces: or_insert_with for lazy Vec creation)
pub fn group_by_parity(numbers: &[i32]) -> HashMap<&'static str, Vec<i32>> {
    let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
    for &n in numbers {
        let key = if n % 2 == 0 { "even" } else { "odd" };
        map.entry(key).or_default().push(n);
    }
    map
}

/// Track scores: add to existing total or start fresh.
/// Demonstrates and_modify().or_insert().
pub fn accumulate_scores(entries: &[(&str, i32)]) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for &(name, score) in entries {
        map.entry(name.to_string())
            .and_modify(|total| *total += score)
            .or_insert(score);
    }
    map
}

/// Count consecutive duplicate runs.
/// Example: ["a","a","b","a"] → [("a",2), ("b",1), ("a",1)]
/// (Note: Vec algorithm, but demonstrates grouping/counting logic)
pub fn count_runs(items: &[&str]) -> Vec<(String, usize)> {
    if items.is_empty() {
        return vec![];
    }
    let mut result: Vec<(String, usize)> = vec![];
    for &item in items {
        if let Some(last) = result.last_mut()
            && last.0 == item
        {
            last.1 += 1;
            continue;
        }
        result.push((item.to_string(), 1));
    }
    result
}

/// Group strings by their length.
/// Example: ["hi", "ok", "hey"] → {2: ["hi","ok"], 3: ["hey"]}
pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for &word in words {
        map.entry(word.len()).or_default().push(word.to_string());
    }
    map
}

// ── Topic 6: BTreeMap — Ordered Maps ────────────────────────
// BTreeMap keeps keys sorted. Useful for range queries and
// ordered iteration. Same API as HashMap for basic operations.

/// Create a sorted frequency table (BTreeMap so keys are ordered).
pub fn sorted_word_count(text: &str) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    map
}

/// Return all keys in a range [start, end).
pub fn keys_in_range(map: &BTreeMap<i32, String>, start: i32, end: i32) -> Vec<i32> {
    map.range(start..end).map(|(&k, _)| k).collect()
}

/// Get the minimum and maximum keys.
pub fn min_max_keys(map: &BTreeMap<i32, String>) -> Option<(i32, i32)> {
    let min = map.keys().next()?;
    let max = map.keys().next_back()?;
    Some((*min, *max))
}

/// Merge two BTreeMaps; on conflict, keep the value from `b`.
pub fn merge_btree(
    a: &BTreeMap<String, i32>,
    b: &BTreeMap<String, i32>,
) -> BTreeMap<String, i32> {
    let mut result = a.clone();
    for (k, v) in b {
        result.insert(k.clone(), *v);
    }
    result
}

/// Get the Nth key in sorted order (0-indexed).
/// Returns None if n >= map.len().
pub fn btree_nth_key(map: &BTreeMap<String, i32>, n: usize) -> Option<String> {
    map.keys().nth(n).cloned()
}

/// Sum all values whose keys fall in the range [start, end).
pub fn btree_range_sum(map: &BTreeMap<i32, i32>, start: i32, end: i32) -> i32 {
    map.range(start..end).map(|(_, &v)| v).sum()
}

// ── Topic 7: Map Transformations ────────────────────────────
// Filtering, mapping, reducing, partitioning maps.

/// Filter a map to keep only entries where value >= threshold.
pub fn filter_by_value(map: &HashMap<String, i32>, threshold: i32) -> HashMap<String, i32> {
    map.iter()
        .filter(|(_, v)| **v >= threshold)
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

/// Transform all values with a function.
pub fn map_values(map: &HashMap<String, i32>, f: fn(i32) -> i32) -> HashMap<String, i32> {
    map.iter().map(|(k, v)| (k.clone(), f(*v))).collect()
}

/// Sum all values in a map.
pub fn sum_values(map: &HashMap<String, i32>) -> i32 {
    map.values().sum()
}

/// Find the key with the maximum value. Ties broken arbitrarily.
pub fn key_with_max_value(map: &HashMap<String, i32>) -> Option<String> {
    map.iter().max_by_key(|(_, v)| **v).map(|(k, _)| k.clone())
}

/// Find the key with the minimum value. Ties broken arbitrarily.
pub fn key_with_min_value(map: &HashMap<String, i32>) -> Option<String> {
    map.iter().min_by_key(|(_, v)| **v).map(|(k, _)| k.clone())
}

/// Return keys sorted by their values (ascending).
pub fn keys_sorted_by_value(map: &HashMap<String, i32>) -> Vec<String> {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by_key(|(_, v)| **v);
    entries.into_iter().map(|(k, _)| k.clone()).collect()
}

/// Split a map into two: (passes_predicate, fails_predicate).
/// (Reinforces: partition-like logic with maps)
pub fn partition_map(
    map: &HashMap<String, i32>,
    predicate: fn(&i32) -> bool,
) -> (HashMap<String, i32>, HashMap<String, i32>) {
    let mut pass = HashMap::new();
    let mut fail = HashMap::new();
    for (k, v) in map {
        if predicate(v) {
            pass.insert(k.clone(), *v);
        } else {
            fail.insert(k.clone(), *v);
        }
    }
    (pass, fail)
}

// ── Topic 8: HashSet Fundamentals ───────────────────────────
// HashSet<T> is a collection of unique values. It's implemented
// as HashMap<T, ()> under the hood. Perfect for membership
// testing, deduplication, and set-theoretic operations.

/// Remove duplicates from a slice, preserving first-occurrence order.
/// (Reinforces: HashSet::insert returns false if already present)
pub fn unique_elements(items: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    items.iter().filter(|&&x| seen.insert(x)).copied().collect()
}

/// Check if all elements in a slice are unique (no duplicates).
pub fn is_all_unique(items: &[i32]) -> bool {
    let mut seen = HashSet::new();
    items.iter().all(|x| seen.insert(x))
}

/// Return elements present in BOTH slices (set intersection).
/// Uses proper HashSet::intersection(). Result is sorted.
pub fn set_intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    let mut result: Vec<i32> = set_a.intersection(&set_b).copied().collect();
    result.sort();
    result
}

/// Return elements in either slice (set union), deduplicated and sorted.
pub fn set_union(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    let mut result: Vec<i32> = set_a.union(&set_b).copied().collect();
    result.sort();
    result
}

/// Return elements in `a` but NOT in `b` (set difference). Sorted.
pub fn set_difference(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    let mut result: Vec<i32> = set_a.difference(&set_b).copied().collect();
    result.sort();
    result
}

/// Return elements in `a` or `b` but NOT in both (symmetric difference). Sorted.
pub fn set_symmetric_difference(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    let mut result: Vec<i32> = set_a.symmetric_difference(&set_b).copied().collect();
    result.sort();
    result
}

/// Check if every element of `a` is also in `b` (subset test).
pub fn is_subset(a: &[i32], b: &[i32]) -> bool {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    set_a.is_subset(&set_b)
}

/// Check if `a` and `b` share NO elements (disjoint test).
pub fn is_disjoint(a: &[i32], b: &[i32]) -> bool {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    set_a.is_disjoint(&set_b)
}

// ── Topic 9: Nested Maps & Complex Structures ───────────────
// HashMap<K, HashMap<K2, V>>, multi-level lookups.
// Includes one of the Rust Book's suggested exercises:
//   "Using a hash map and vectors, create a text interface to allow
//    a user to add employee names to a department in a company."

/// Grade book: student → subject → grade.
pub type GradeBook = HashMap<String, HashMap<String, f64>>;

/// Add a grade to the grade book.
pub fn add_grade(book: &mut GradeBook, student: &str, subject: &str, grade: f64) {
    book.entry(student.to_string())
        .or_default()
        .insert(subject.to_string(), grade);
}

/// Get a student's average grade.
/// Returns None if the student doesn't exist or has no grades.
pub fn student_average(book: &GradeBook, student: &str) -> Option<f64> {
    let grades = book.get(student)?;
    if grades.is_empty() {
        return None;
    }
    let sum: f64 = grades.values().sum();
    Some(sum / grades.len() as f64)
}

/// Get all students who have a grade in a given subject, sorted.
pub fn students_in_subject(book: &GradeBook, subject: &str) -> Vec<String> {
    let mut result: Vec<String> = book
        .iter()
        .filter(|(_, subjects)| subjects.contains_key(subject))
        .map(|(student, _)| student.clone())
        .collect();
    result.sort();
    result
}

/// Get the top student (highest average). Ties broken arbitrarily.
pub fn top_student(book: &GradeBook) -> Option<String> {
    book.keys()
        .filter_map(|student| student_average(book, student).map(|avg| (student.clone(), avg)))
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(name, _)| name)
}

/// Calculate the average grade for each subject across all students.
/// Returns: {"Math": 87.5, "English": 82.0, ...}
pub fn subject_averages(book: &GradeBook) -> HashMap<String, f64> {
    let mut totals: HashMap<String, (f64, usize)> = HashMap::new();
    for subjects in book.values() {
        for (subject, &grade) in subjects {
            let entry = totals.entry(subject.clone()).or_insert((0.0, 0));
            entry.0 += grade;
            entry.1 += 1;
        }
    }
    totals
        .into_iter()
        .map(|(subject, (sum, count))| (subject, sum / count as f64))
        .collect()
}

/// Department directory: add a person to a department.
/// Keeps each department's list sorted alphabetically.
/// (This is one of the Rust Book's suggested exercises!)
pub fn add_to_department(
    directory: &mut HashMap<String, Vec<String>>,
    department: &str,
    person: &str,
) {
    let people = directory.entry(department.to_string()).or_default();
    people.push(person.to_string());
    people.sort();
}

/// Get all people in a department, sorted alphabetically.
/// Returns an empty Vec if the department doesn't exist.
pub fn people_in_department(
    directory: &HashMap<String, Vec<String>>,
    department: &str,
) -> Vec<String> {
    directory.get(department).cloned().unwrap_or_default()
}

/// Get all people across all departments, sorted and deduplicated.
/// (Rust Book exercise: "all people in the company, sorted alphabetically")
pub fn all_people_sorted(directory: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut all: Vec<String> = directory.values().flatten().cloned().collect();
    all.sort();
    all.dedup();
    all
}

// ── Topic 10: Advanced HashMap Patterns ─────────────────────
// Common algorithm patterns using hash maps and hash sets.

/// Two-sum: find indices of two values that sum to target.
/// Returns the first valid pair found (smaller index first).
/// (Classic interview problem solved in O(n) with a HashMap)
pub fn two_sum(items: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &v) in items.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - v)) {
            return Some((j, i));
        }
        seen.insert(v, i);
    }
    None
}

/// Find the most frequently occurring element.
/// Ties broken arbitrarily. Returns None for empty input.
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

/// Return the top N keys by value (descending).
pub fn top_n_keys(map: &HashMap<String, i32>, n: usize) -> Vec<String> {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|(_, a), (_, b)| b.cmp(a));
    entries.into_iter().take(n).map(|(k, _)| k.clone()).collect()
}

/// Build a histogram: count occurrences of each value.
pub fn value_histogram(items: &[i32]) -> HashMap<i32, usize> {
    let mut map = HashMap::new();
    for &item in items {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}

/// Merge two hashmaps, summing values for shared keys.
pub fn merge_sum(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result = a.clone();
    for (k, v) in b {
        *result.entry(k.clone()).or_insert(0) += v;
    }
    result
}

/// Group words that are anagrams of each other.
/// Two words are anagrams if they contain the same letters.
/// Example: ["eat","tea","tan","ate","nat","bat"]
///        → [["ate","eat","tea"], ["bat"], ["nat","tan"]]
/// Each group is sorted; groups are sorted by their first element.
pub fn anagram_groups(words: &[&str]) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for &word in words {
        let mut key: Vec<char> = word.to_lowercase().chars().collect();
        key.sort();
        let key: String = key.into_iter().collect();
        map.entry(key).or_default().push(word.to_string());
    }
    let mut groups: Vec<Vec<String>> = map.into_values().collect();
    for group in &mut groups {
        group.sort();
    }
    groups.sort_by(|a, b| a[0].cmp(&b[0]));
    groups
}

/// Find the first character in a string that doesn't repeat.
/// Returns None if all characters repeat or string is empty.
/// Example: "aabcbd" → Some('c')
pub fn first_unique_char(s: &str) -> Option<char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    for &c in &chars {
        *counts.entry(c).or_insert(0) += 1;
    }
    chars.into_iter().find(|c| counts[c] == 1)
}

/// Find the length of the longest consecutive sequence.
/// Example: [100, 4, 200, 1, 3, 2] → 4 (the sequence 1,2,3,4)
/// Uses HashSet for O(n) average-case lookups.
pub fn longest_consecutive(items: &[i32]) -> usize {
    let set: HashSet<i32> = items.iter().copied().collect();
    let mut best = 0;
    for &num in &set {
        // Only start counting from the beginning of a sequence
        if !set.contains(&(num - 1)) {
            let mut current = num;
            let mut length = 1;
            while set.contains(&(current + 1)) {
                current += 1;
                length += 1;
            }
            best = best.max(length);
        }
    }
    best
}
