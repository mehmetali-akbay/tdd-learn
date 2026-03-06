use hashmaps::*;
use std::collections::{BTreeMap, HashMap};

// ===== Topic 1: HashMap Basics =====

#[test]
fn test_word_count() {
    let counts = word_count("the cat sat on the mat");
    assert_eq!(counts["the"], 2);
    assert_eq!(counts["cat"], 1);
    assert_eq!(counts["mat"], 1);
}

#[test]
fn test_word_count_case_insensitive() {
    let counts = word_count("Hello hello HELLO");
    assert_eq!(counts["hello"], 3);
    assert_eq!(counts.len(), 1);
}

#[test]
fn test_char_frequency() {
    let freq = char_frequency("aabbc");
    assert_eq!(freq[&'a'], 2);
    assert_eq!(freq[&'b'], 2);
    assert_eq!(freq[&'c'], 1);
}

#[test]
fn test_zip_to_map() {
    let map = zip_to_map(&["a", "b", "c"], &[1, 2, 3]);
    assert_eq!(map["a"], 1);
    assert_eq!(map["c"], 3);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_invert_map() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), "1".to_string());
    map.insert("b".to_string(), "2".to_string());
    let inv = invert_map(&map);
    assert_eq!(inv["1"], "a");
    assert_eq!(inv["2"], "b");
}

// ===== Topic 2: Entry API =====

#[test]
fn test_group_by_first_letter() {
    let groups = group_by_first_letter(&["apple", "avocado", "banana", "blueberry"]);
    assert_eq!(groups[&'a'].len(), 2);
    assert_eq!(groups[&'b'].len(), 2);
}

#[test]
fn test_group_by_parity() {
    let groups = group_by_parity(&[1, 2, 3, 4, 5]);
    assert_eq!(groups["odd"], vec![1, 3, 5]);
    assert_eq!(groups["even"], vec![2, 4]);
}

#[test]
fn test_accumulate_scores() {
    let scores = accumulate_scores(&[("alice", 10), ("bob", 20), ("alice", 5)]);
    assert_eq!(scores["alice"], 15);
    assert_eq!(scores["bob"], 20);
}

#[test]
fn test_count_runs() {
    assert_eq!(
        count_runs(&["a", "a", "b", "b", "b", "a"]),
        vec![
            ("a".to_string(), 2),
            ("b".to_string(), 3),
            ("a".to_string(), 1),
        ]
    );
    assert_eq!(count_runs(&[]), vec![]);
}

// ===== Topic 3: BTreeMap =====

#[test]
fn test_sorted_word_count() {
    let counts = sorted_word_count("banana apple banana cherry apple apple");
    let keys: Vec<_> = counts.keys().cloned().collect();
    assert_eq!(keys, vec!["apple", "banana", "cherry"]); // sorted!
    assert_eq!(counts["apple"], 3);
}

#[test]
fn test_keys_in_range() {
    let mut map = BTreeMap::new();
    map.insert(1, "one".to_string());
    map.insert(3, "three".to_string());
    map.insert(5, "five".to_string());
    map.insert(7, "seven".to_string());
    assert_eq!(keys_in_range(&map, 2, 6), vec![3, 5]);
}

#[test]
fn test_min_max_keys() {
    let mut map = BTreeMap::new();
    map.insert(5, "five".to_string());
    map.insert(1, "one".to_string());
    map.insert(9, "nine".to_string());
    assert_eq!(min_max_keys(&map), Some((1, 9)));
    assert_eq!(min_max_keys(&BTreeMap::<i32, String>::new()), None);
}

#[test]
fn test_merge_btree() {
    let mut a = BTreeMap::new();
    a.insert("x".to_string(), 1);
    a.insert("y".to_string(), 2);
    let mut b = BTreeMap::new();
    b.insert("y".to_string(), 99);
    b.insert("z".to_string(), 3);
    let merged = merge_btree(&a, &b);
    assert_eq!(merged["x"], 1);
    assert_eq!(merged["y"], 99); // b wins
    assert_eq!(merged["z"], 3);
}

// ===== Topic 4: Map Transformations =====

#[test]
fn test_filter_by_value() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 10);
    map.insert("b".to_string(), 5);
    map.insert("c".to_string(), 20);
    let filtered = filter_by_value(&map, 10);
    assert_eq!(filtered.len(), 2);
    assert!(filtered.contains_key("a"));
    assert!(filtered.contains_key("c"));
}

#[test]
fn test_map_values() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    let doubled = map_values(&map, |v| v * 2);
    assert_eq!(doubled["a"], 2);
    assert_eq!(doubled["b"], 4);
}

#[test]
fn test_sum_values() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 10);
    map.insert("b".to_string(), 20);
    assert_eq!(sum_values(&map), 30);
}

#[test]
fn test_key_with_max_value() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 10);
    map.insert("b".to_string(), 30);
    map.insert("c".to_string(), 20);
    assert_eq!(key_with_max_value(&map), Some("b".to_string()));
    assert_eq!(key_with_max_value(&HashMap::<String, i32>::new()), None);
}

#[test]
fn test_keys_sorted_by_value() {
    let mut map = HashMap::new();
    map.insert("c".to_string(), 3);
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    assert_eq!(keys_sorted_by_value(&map), vec!["a", "b", "c"]);
}

// ===== Topic 5: Nested Maps =====

#[test]
fn test_add_grade() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 95.0);
    add_grade(&mut book, "Alice", "English", 88.0);
    assert_eq!(book["Alice"]["Math"], 95.0);
    assert_eq!(book["Alice"]["English"], 88.0);
}

#[test]
fn test_student_average() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    add_grade(&mut book, "Alice", "English", 80.0);
    assert!((student_average(&book, "Alice").unwrap() - 85.0).abs() < 0.01);
    assert_eq!(student_average(&book, "Unknown"), None);
}

#[test]
fn test_students_in_subject() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    add_grade(&mut book, "Bob", "Math", 85.0);
    add_grade(&mut book, "Charlie", "English", 88.0);
    let mut students = students_in_subject(&book, "Math");
    students.sort();
    assert_eq!(students, vec!["Alice", "Bob"]);
}

#[test]
fn test_top_student() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    add_grade(&mut book, "Bob", "Math", 95.0);
    add_grade(&mut book, "Alice", "English", 80.0);
    add_grade(&mut book, "Bob", "English", 90.0);
    assert_eq!(top_student(&book), Some("Bob".to_string())); // avg 92.5 vs 85
}

// ===== Topic 6: Set Operations =====

#[test]
fn test_intersection() {
    assert_eq!(intersection(&[1, 2, 3, 4], &[3, 4, 5, 6]), vec![3, 4]);
    assert_eq!(intersection(&[1, 2], &[3, 4]), vec![]);
}

#[test]
fn test_difference() {
    assert_eq!(difference(&[1, 2, 3, 4], &[3, 4, 5, 6]), vec![1, 2]);
}

#[test]
fn test_is_subset() {
    assert!(is_subset(&[1, 2], &[1, 2, 3, 4]));
    assert!(!is_subset(&[1, 5], &[1, 2, 3]));
    assert!(is_subset(&[], &[1, 2, 3]));
}

#[test]
fn test_most_frequent() {
    assert_eq!(most_frequent(&[1, 2, 2, 3, 3, 3]), Some(3));
    assert_eq!(most_frequent(&[]), None);
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_two_sum_map() {
    assert_eq!(two_sum_map(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(two_sum_map(&[1, 2, 3], 10), None);
}

#[test]
fn test_top_n_keys() {
    let mut m = HashMap::new();
    m.insert("a".into(), 10);
    m.insert("b".into(), 30);
    m.insert("c".into(), 20);
    let top = top_n_keys(&m, 2);
    assert_eq!(top[0], "b");
    assert_eq!(top[1], "c");
}

#[test]
fn test_value_histogram() {
    let hist = value_histogram(&[1, 2, 2, 3, 3, 3]);
    assert_eq!(hist[&1], 1);
    assert_eq!(hist[&3], 3);
}

#[test]
fn test_merge_sum() {
    let mut a = HashMap::new();
    a.insert("x".into(), 10);
    let mut b = HashMap::new();
    b.insert("x".into(), 5);
    b.insert("y".into(), 3);
    let merged = merge_sum(&a, &b);
    assert_eq!(merged["x"], 15);
    assert_eq!(merged["y"], 3);
}

