use hashmaps::*;
use std::collections::{BTreeMap, HashMap};

// =============================================================================
// Topic 1: Creating Hash Maps
// =============================================================================

#[test]
fn test_create_scores_contains_expected_keys() {
    let scores = create_scores();
    assert!(scores.contains_key("Alice"));
    assert!(scores.contains_key("Bob"));
    assert!(scores.contains_key("Charlie"));
}

#[test]
fn test_create_scores_correct_values() {
    let scores = create_scores();
    assert_eq!(scores["Alice"], 100);
    assert_eq!(scores["Bob"], 85);
    assert_eq!(scores["Charlie"], 92);
}

#[test]
fn test_create_scores_length() {
    assert_eq!(create_scores().len(), 3);
}

#[test]
fn test_from_tuples_basic() {
    let map = from_tuples(&[("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(map["a"], 1);
    assert_eq!(map["b"], 2);
    assert_eq!(map["c"], 3);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_from_tuples_empty() {
    let map = from_tuples(&[]);
    assert!(map.is_empty());
}

#[test]
fn test_from_tuples_duplicate_keys_last_wins() {
    let map = from_tuples(&[("x", 1), ("x", 2)]);
    // collect() from iterator: last value wins for duplicate keys
    assert_eq!(map.len(), 1);
    assert!(map["x"] == 1 || map["x"] == 2);
}

#[test]
fn test_zip_to_map_basic() {
    let map = zip_to_map(&["a", "b", "c"], &[1, 2, 3]);
    assert_eq!(map["a"], 1);
    assert_eq!(map["b"], 2);
    assert_eq!(map["c"], 3);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_zip_to_map_unequal_lengths() {
    let map = zip_to_map(&["a", "b", "c"], &[1, 2]);
    assert_eq!(map.len(), 2);
    assert!(!map.contains_key("c"));
}

#[test]
fn test_zip_to_map_empty() {
    let map = zip_to_map(&[], &[]);
    assert!(map.is_empty());
}

#[test]
fn test_from_keys_with_default() {
    let map = from_keys_with_default(&["x", "y", "z"], 0);
    assert_eq!(map["x"], 0);
    assert_eq!(map["y"], 0);
    assert_eq!(map["z"], 0);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_from_keys_with_default_nonzero() {
    let map = from_keys_with_default(&["a"], 42);
    assert_eq!(map["a"], 42);
}

#[test]
fn test_from_keys_with_default_empty() {
    let map = from_keys_with_default(&[], 99);
    assert!(map.is_empty());
}

#[test]
fn test_from_array_literal() {
    let map = from_array_literal();
    assert_eq!(map["one"], 1);
    assert_eq!(map["two"], 2);
    assert_eq!(map["three"], 3);
    assert_eq!(map.len(), 3);
}

// =============================================================================
// Topic 2: Accessing Values
// =============================================================================

#[test]
fn test_get_or_default_key_exists() {
    let map = from_tuples(&[("score", 42)]);
    assert_eq!(get_or_default(&map, "score", 0), 42);
}

#[test]
fn test_get_or_default_key_missing() {
    let map = from_tuples(&[("score", 42)]);
    assert_eq!(get_or_default(&map, "missing", -1), -1);
}

#[test]
fn test_get_or_default_empty_map() {
    let map: HashMap<String, i32> = HashMap::new();
    assert_eq!(get_or_default(&map, "anything", 99), 99);
}

#[test]
fn test_get_many_all_found() {
    let map = from_tuples(&[("a", 1), ("b", 2), ("c", 3)]);
    let results = get_many(&map, &["a", "c"]);
    assert_eq!(results, vec![Some(1), Some(3)]);
}

#[test]
fn test_get_many_some_missing() {
    let map = from_tuples(&[("a", 1), ("b", 2)]);
    let results = get_many(&map, &["a", "z", "b"]);
    assert_eq!(results, vec![Some(1), None, Some(2)]);
}

#[test]
fn test_get_many_all_missing() {
    let map = from_tuples(&[("a", 1)]);
    let results = get_many(&map, &["x", "y"]);
    assert_eq!(results, vec![None, None]);
}

#[test]
fn test_contains_all_keys_true() {
    let map = from_tuples(&[("a", 1), ("b", 2), ("c", 3)]);
    assert!(contains_all_keys(&map, &["a", "b"]));
}

#[test]
fn test_contains_all_keys_false() {
    let map = from_tuples(&[("a", 1), ("b", 2)]);
    assert!(!contains_all_keys(&map, &["a", "z"]));
}

#[test]
fn test_contains_all_keys_empty_keys() {
    let map = from_tuples(&[("a", 1)]);
    assert!(contains_all_keys(&map, &[])); // vacuously true
}

#[test]
fn test_all_keys_sorted() {
    let map = from_tuples(&[("cherry", 3), ("apple", 1), ("banana", 2)]);
    assert_eq!(all_keys_sorted(&map), vec!["apple", "banana", "cherry"]);
}

#[test]
fn test_all_keys_sorted_empty() {
    let map: HashMap<String, i32> = HashMap::new();
    assert!(all_keys_sorted(&map).is_empty());
}

#[test]
fn test_all_values_sorted() {
    let map = from_tuples(&[("a", 30), ("b", 10), ("c", 20)]);
    assert_eq!(all_values_sorted(&map), vec![10, 20, 30]);
}

#[test]
fn test_all_values_sorted_with_duplicates() {
    let map = from_tuples(&[("a", 5), ("b", 5), ("c", 1)]);
    assert_eq!(all_values_sorted(&map), vec![1, 5, 5]);
}

#[test]
fn test_count_by_value_found() {
    let map = from_tuples(&[("a", 10), ("b", 20), ("c", 10), ("d", 10)]);
    assert_eq!(count_by_value(&map, 10), 3);
}

#[test]
fn test_count_by_value_not_found() {
    let map = from_tuples(&[("a", 10), ("b", 20)]);
    assert_eq!(count_by_value(&map, 99), 0);
}

#[test]
fn test_count_by_value_all_match() {
    let map = from_tuples(&[("a", 7), ("b", 7), ("c", 7)]);
    assert_eq!(count_by_value(&map, 7), 3);
}

// =============================================================================
// Topic 3: Ownership and Hash Maps
// =============================================================================

#[test]
fn test_insert_returns_old_new_key() {
    let mut map = HashMap::new();
    let old = insert_returns_old(&mut map, "a", 1);
    assert_eq!(old, None);
    assert_eq!(map["a"], 1);
}

#[test]
fn test_insert_returns_old_existing_key() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 1);
    let old = insert_returns_old(&mut map, "a", 99);
    assert_eq!(old, Some(1));
    assert_eq!(map["a"], 99);
}

#[test]
fn test_insert_returns_old_multiple() {
    let mut map = HashMap::new();
    assert_eq!(insert_returns_old(&mut map, "x", 10), None);
    assert_eq!(insert_returns_old(&mut map, "x", 20), Some(10));
    assert_eq!(insert_returns_old(&mut map, "x", 30), Some(20));
    assert_eq!(map["x"], 30);
}

#[test]
fn test_invert_map_basic() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), "1".to_string());
    map.insert("b".to_string(), "2".to_string());
    let inv = invert_map(&map);
    assert_eq!(inv["1"], "a");
    assert_eq!(inv["2"], "b");
    assert_eq!(inv.len(), 2);
}

#[test]
fn test_invert_map_empty() {
    let map: HashMap<String, String> = HashMap::new();
    assert!(invert_map(&map).is_empty());
}

#[test]
fn test_clone_and_extend_no_conflict() {
    let base = from_tuples(&[("a", 1), ("b", 2)]);
    let extra = from_tuples(&[("c", 3)]);
    let result = clone_and_extend(&base, &extra);
    assert_eq!(result.len(), 3);
    assert_eq!(result["a"], 1);
    assert_eq!(result["c"], 3);
}

#[test]
fn test_clone_and_extend_with_conflict() {
    let base = from_tuples(&[("a", 1), ("b", 2)]);
    let extra = from_tuples(&[("b", 99)]);
    let result = clone_and_extend(&base, &extra);
    assert_eq!(result["b"], 99); // extra wins
}

#[test]
fn test_clone_and_extend_empty_extra() {
    let base = from_tuples(&[("a", 1)]);
    let extra: HashMap<String, i32> = HashMap::new();
    let result = clone_and_extend(&base, &extra);
    assert_eq!(result, base);
}

#[test]
fn test_merge_prefer_first_conflict() {
    let first = from_tuples(&[("a", 1), ("b", 2)]);
    let second = from_tuples(&[("b", 99), ("c", 3)]);
    let result = merge_prefer_first(&first, &second);
    assert_eq!(result["a"], 1);
    assert_eq!(result["b"], 2); // first wins
    assert_eq!(result["c"], 3);
}

#[test]
fn test_merge_prefer_first_no_conflict() {
    let first = from_tuples(&[("a", 1)]);
    let second = from_tuples(&[("b", 2)]);
    let result = merge_prefer_first(&first, &second);
    assert_eq!(result.len(), 2);
}

#[test]
fn test_merge_prefer_second_conflict() {
    let first = from_tuples(&[("a", 1), ("b", 2)]);
    let second = from_tuples(&[("b", 99), ("c", 3)]);
    let result = merge_prefer_second(&first, &second);
    assert_eq!(result["b"], 99); // second wins
    assert_eq!(result["a"], 1);
    assert_eq!(result["c"], 3);
}

#[test]
fn test_merge_prefer_second_no_conflict() {
    let first = from_tuples(&[("x", 10)]);
    let second = from_tuples(&[("y", 20)]);
    let result = merge_prefer_second(&first, &second);
    assert_eq!(result.len(), 2);
}

// =============================================================================
// Topic 4: Updating — Entry API
// =============================================================================

#[test]
fn test_insert_if_absent_new_key() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let val = insert_if_absent(&mut map, "a", 42);
    assert_eq!(val, 42);
    assert_eq!(map["a"], 42);
}

#[test]
fn test_insert_if_absent_existing_key() {
    let mut map = from_tuples(&[("a", 10)]);
    let val = insert_if_absent(&mut map, "a", 42);
    assert_eq!(val, 10); // kept the original
    assert_eq!(map["a"], 10);
}

#[test]
fn test_push_to_key_new_key() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let len = push_to_key(&mut map, "fruits", "apple");
    assert_eq!(len, 1);
    assert_eq!(map["fruits"], vec!["apple"]);
}

#[test]
fn test_push_to_key_existing_key() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    push_to_key(&mut map, "fruits", "apple");
    let len = push_to_key(&mut map, "fruits", "banana");
    assert_eq!(len, 2);
    assert_eq!(map["fruits"], vec!["apple", "banana"]);
}

#[test]
fn test_push_to_key_multiple_keys() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    push_to_key(&mut map, "fruits", "apple");
    push_to_key(&mut map, "vegs", "carrot");
    assert_eq!(map.len(), 2);
}

#[test]
fn test_upsert_counter_new_key() {
    let mut map: HashMap<String, i32> = HashMap::new();
    upsert_counter(&mut map, "clicks");
    assert_eq!(map["clicks"], 1);
}

#[test]
fn test_upsert_counter_existing_key() {
    let mut map: HashMap<String, i32> = HashMap::new();
    upsert_counter(&mut map, "clicks");
    upsert_counter(&mut map, "clicks");
    upsert_counter(&mut map, "clicks");
    assert_eq!(map["clicks"], 3);
}

#[test]
fn test_upsert_counter_multiple_keys() {
    let mut map: HashMap<String, i32> = HashMap::new();
    upsert_counter(&mut map, "a");
    upsert_counter(&mut map, "b");
    upsert_counter(&mut map, "a");
    assert_eq!(map["a"], 2);
    assert_eq!(map["b"], 1);
}

#[test]
fn test_scale_all_values_double() {
    let mut map = from_tuples(&[("a", 1), ("b", 2), ("c", 3)]);
    scale_all_values(&mut map, 2);
    assert_eq!(map["a"], 2);
    assert_eq!(map["b"], 4);
    assert_eq!(map["c"], 6);
}

#[test]
fn test_scale_all_values_zero() {
    let mut map = from_tuples(&[("a", 10), ("b", 20)]);
    scale_all_values(&mut map, 0);
    assert_eq!(map["a"], 0);
    assert_eq!(map["b"], 0);
}

#[test]
fn test_scale_all_values_negative() {
    let mut map = from_tuples(&[("x", 5)]);
    scale_all_values(&mut map, -1);
    assert_eq!(map["x"], -5);
}

#[test]
fn test_remove_negative_values_some() {
    let mut map = from_tuples(&[("a", 10), ("b", -5), ("c", -1), ("d", 0)]);
    let removed = remove_negative_values(&mut map);
    assert_eq!(removed, 2);
    assert_eq!(map.len(), 2);
    assert!(map.contains_key("a"));
    assert!(map.contains_key("d")); // 0 is not negative
}

#[test]
fn test_remove_negative_values_none() {
    let mut map = from_tuples(&[("a", 1), ("b", 2)]);
    let removed = remove_negative_values(&mut map);
    assert_eq!(removed, 0);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_remove_negative_values_all() {
    let mut map = from_tuples(&[("a", -1), ("b", -2)]);
    let removed = remove_negative_values(&mut map);
    assert_eq!(removed, 2);
    assert!(map.is_empty());
}

#[test]
fn test_take_value_present() {
    let mut map = from_tuples(&[("a", 42), ("b", 10)]);
    let val = take_value(&mut map, "a");
    assert_eq!(val, Some(42));
    assert_eq!(map.len(), 1);
    assert!(!map.contains_key("a"));
}

#[test]
fn test_take_value_absent() {
    let mut map = from_tuples(&[("a", 1)]);
    let val = take_value(&mut map, "z");
    assert_eq!(val, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_clear_map() {
    let mut map = from_tuples(&[("a", 1), ("b", 2), ("c", 3)]);
    let count = clear_map(&mut map);
    assert_eq!(count, 3);
    assert!(map.is_empty());
}

#[test]
fn test_clear_map_empty() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let count = clear_map(&mut map);
    assert_eq!(count, 0);
}

// =============================================================================
// Topic 5: Counting and Grouping Patterns
// =============================================================================

#[test]
fn test_word_count_basic() {
    let counts = word_count("the cat sat on the mat");
    assert_eq!(counts["the"], 2);
    assert_eq!(counts["cat"], 1);
    assert_eq!(counts["mat"], 1);
    assert_eq!(counts.len(), 5);
}

#[test]
fn test_word_count_case_insensitive() {
    let counts = word_count("Hello hello HELLO");
    assert_eq!(counts["hello"], 3);
    assert_eq!(counts.len(), 1);
}

#[test]
fn test_word_count_empty() {
    let counts = word_count("");
    assert!(counts.is_empty());
}

#[test]
fn test_word_count_single_word() {
    let counts = word_count("rust");
    assert_eq!(counts["rust"], 1);
}

#[test]
fn test_char_frequency_basic() {
    let freq = char_frequency("aabbc");
    assert_eq!(freq[&'a'], 2);
    assert_eq!(freq[&'b'], 2);
    assert_eq!(freq[&'c'], 1);
}

#[test]
fn test_char_frequency_ignores_whitespace() {
    let freq = char_frequency("a b c");
    assert_eq!(freq.len(), 3);
    assert!(!freq.contains_key(&' '));
}

#[test]
fn test_char_frequency_empty() {
    let freq = char_frequency("");
    assert!(freq.is_empty());
}

#[test]
fn test_group_by_first_letter_basic() {
    let groups = group_by_first_letter(&["apple", "avocado", "banana", "blueberry"]);
    assert_eq!(groups[&'a'].len(), 2);
    assert_eq!(groups[&'b'].len(), 2);
}

#[test]
fn test_group_by_first_letter_case_insensitive() {
    let groups = group_by_first_letter(&["Apple", "apricot"]);
    assert_eq!(groups[&'a'].len(), 2);
}

#[test]
fn test_group_by_first_letter_empty() {
    let groups = group_by_first_letter(&[]);
    assert!(groups.is_empty());
}

#[test]
fn test_group_by_parity_mixed() {
    let groups = group_by_parity(&[1, 2, 3, 4, 5]);
    assert_eq!(groups["odd"], vec![1, 3, 5]);
    assert_eq!(groups["even"], vec![2, 4]);
}

#[test]
fn test_group_by_parity_all_even() {
    let groups = group_by_parity(&[2, 4, 6]);
    assert_eq!(groups["even"], vec![2, 4, 6]);
    assert!(!groups.contains_key("odd"));
}

#[test]
fn test_group_by_parity_empty() {
    let groups = group_by_parity(&[]);
    assert!(groups.is_empty());
}

#[test]
fn test_accumulate_scores_basic() {
    let scores = accumulate_scores(&[("alice", 10), ("bob", 20), ("alice", 5)]);
    assert_eq!(scores["alice"], 15);
    assert_eq!(scores["bob"], 20);
}

#[test]
fn test_accumulate_scores_single_entry() {
    let scores = accumulate_scores(&[("x", 100)]);
    assert_eq!(scores["x"], 100);
    assert_eq!(scores.len(), 1);
}

#[test]
fn test_accumulate_scores_empty() {
    let scores = accumulate_scores(&[]);
    assert!(scores.is_empty());
}

#[test]
fn test_count_runs_basic() {
    assert_eq!(
        count_runs(&["a", "a", "b", "b", "b", "a"]),
        vec![
            ("a".to_string(), 2),
            ("b".to_string(), 3),
            ("a".to_string(), 1),
        ]
    );
}

#[test]
fn test_count_runs_no_duplicates() {
    assert_eq!(
        count_runs(&["a", "b", "c"]),
        vec![
            ("a".to_string(), 1),
            ("b".to_string(), 1),
            ("c".to_string(), 1),
        ]
    );
}

#[test]
fn test_count_runs_single() {
    assert_eq!(count_runs(&["x"]), vec![("x".to_string(), 1)]);
}

#[test]
fn test_count_runs_empty() {
    assert_eq!(count_runs(&[]), Vec::<(String, usize)>::new());
}

#[test]
fn test_group_by_length_basic() {
    let groups = group_by_length(&["hi", "ok", "hey", "bye", "a"]);
    assert_eq!(groups[&2].len(), 2);
    assert_eq!(groups[&3].len(), 2);
    assert_eq!(groups[&1].len(), 1);
}

#[test]
fn test_group_by_length_empty() {
    let groups = group_by_length(&[]);
    assert!(groups.is_empty());
}

#[test]
fn test_group_by_length_same_length() {
    let groups = group_by_length(&["abc", "def", "ghi"]);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[&3].len(), 3);
}

// =============================================================================
// Topic 6: BTreeMap — Ordered Maps
// =============================================================================

#[test]
fn test_sorted_word_count_keys_in_order() {
    let counts = sorted_word_count("banana apple banana cherry apple apple");
    let keys: Vec<_> = counts.keys().cloned().collect();
    assert_eq!(keys, vec!["apple", "banana", "cherry"]);
}

#[test]
fn test_sorted_word_count_correct_counts() {
    let counts = sorted_word_count("banana apple banana cherry apple apple");
    assert_eq!(counts["apple"], 3);
    assert_eq!(counts["banana"], 2);
    assert_eq!(counts["cherry"], 1);
}

#[test]
fn test_keys_in_range_basic() {
    let mut map = BTreeMap::new();
    map.insert(1, "one".to_string());
    map.insert(3, "three".to_string());
    map.insert(5, "five".to_string());
    map.insert(7, "seven".to_string());
    assert_eq!(keys_in_range(&map, 2, 6), vec![3, 5]);
}

#[test]
fn test_keys_in_range_empty_result() {
    let mut map = BTreeMap::new();
    map.insert(1, "one".to_string());
    map.insert(10, "ten".to_string());
    assert_eq!(keys_in_range(&map, 3, 8), vec![]);
}

#[test]
fn test_keys_in_range_all_included() {
    let mut map = BTreeMap::new();
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    assert_eq!(keys_in_range(&map, 1, 10), vec![2, 3]);
}

#[test]
fn test_min_max_keys_basic() {
    let mut map = BTreeMap::new();
    map.insert(5, "five".to_string());
    map.insert(1, "one".to_string());
    map.insert(9, "nine".to_string());
    assert_eq!(min_max_keys(&map), Some((1, 9)));
}

#[test]
fn test_min_max_keys_single() {
    let mut map = BTreeMap::new();
    map.insert(42, "answer".to_string());
    assert_eq!(min_max_keys(&map), Some((42, 42)));
}

#[test]
fn test_min_max_keys_empty() {
    let map: BTreeMap<i32, String> = BTreeMap::new();
    assert_eq!(min_max_keys(&map), None);
}

#[test]
fn test_merge_btree_conflict() {
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

#[test]
fn test_merge_btree_no_conflict() {
    let mut a = BTreeMap::new();
    a.insert("a".to_string(), 1);
    let mut b = BTreeMap::new();
    b.insert("b".to_string(), 2);
    let merged = merge_btree(&a, &b);
    assert_eq!(merged.len(), 2);
}

#[test]
fn test_btree_nth_key_valid() {
    let mut map = BTreeMap::new();
    map.insert("apple".to_string(), 1);
    map.insert("banana".to_string(), 2);
    map.insert("cherry".to_string(), 3);
    assert_eq!(btree_nth_key(&map, 0), Some("apple".to_string()));
    assert_eq!(btree_nth_key(&map, 1), Some("banana".to_string()));
    assert_eq!(btree_nth_key(&map, 2), Some("cherry".to_string()));
}

#[test]
fn test_btree_nth_key_out_of_bounds() {
    let mut map = BTreeMap::new();
    map.insert("a".to_string(), 1);
    assert_eq!(btree_nth_key(&map, 5), None);
}

#[test]
fn test_btree_nth_key_empty() {
    let map: BTreeMap<String, i32> = BTreeMap::new();
    assert_eq!(btree_nth_key(&map, 0), None);
}

#[test]
fn test_btree_range_sum_basic() {
    let mut map = BTreeMap::new();
    map.insert(1, 10);
    map.insert(3, 30);
    map.insert(5, 50);
    map.insert(7, 70);
    assert_eq!(btree_range_sum(&map, 2, 6), 80); // 30 + 50
}

#[test]
fn test_btree_range_sum_empty_range() {
    let mut map = BTreeMap::new();
    map.insert(1, 10);
    map.insert(10, 100);
    assert_eq!(btree_range_sum(&map, 3, 8), 0);
}

#[test]
fn test_btree_range_sum_all() {
    let mut map = BTreeMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(btree_range_sum(&map, 0, 100), 30);
}

// =============================================================================
// Topic 7: Map Transformations
// =============================================================================

#[test]
fn test_filter_by_value_some_pass() {
    let map = from_tuples(&[("a", 10), ("b", 5), ("c", 20)]);
    let filtered = filter_by_value(&map, 10);
    assert_eq!(filtered.len(), 2);
    assert!(filtered.contains_key("a"));
    assert!(filtered.contains_key("c"));
}

#[test]
fn test_filter_by_value_none_pass() {
    let map = from_tuples(&[("a", 1), ("b", 2)]);
    let filtered = filter_by_value(&map, 100);
    assert!(filtered.is_empty());
}

#[test]
fn test_filter_by_value_all_pass() {
    let map = from_tuples(&[("a", 50), ("b", 60)]);
    let filtered = filter_by_value(&map, 10);
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_map_values_double() {
    let map = from_tuples(&[("a", 1), ("b", 2)]);
    let doubled = map_values(&map, |v| v * 2);
    assert_eq!(doubled["a"], 2);
    assert_eq!(doubled["b"], 4);
}

#[test]
fn test_map_values_negate() {
    let map = from_tuples(&[("x", 5), ("y", -3)]);
    let negated = map_values(&map, |v| -v);
    assert_eq!(negated["x"], -5);
    assert_eq!(negated["y"], 3);
}

#[test]
fn test_sum_values_basic() {
    let map = from_tuples(&[("a", 10), ("b", 20), ("c", 30)]);
    assert_eq!(sum_values(&map), 60);
}

#[test]
fn test_sum_values_empty() {
    let map: HashMap<String, i32> = HashMap::new();
    assert_eq!(sum_values(&map), 0);
}

#[test]
fn test_sum_values_negative() {
    let map = from_tuples(&[("a", 10), ("b", -5)]);
    assert_eq!(sum_values(&map), 5);
}

#[test]
fn test_key_with_max_value_basic() {
    let map = from_tuples(&[("a", 10), ("b", 30), ("c", 20)]);
    assert_eq!(key_with_max_value(&map), Some("b".to_string()));
}

#[test]
fn test_key_with_max_value_empty() {
    let map: HashMap<String, i32> = HashMap::new();
    assert_eq!(key_with_max_value(&map), None);
}

#[test]
fn test_key_with_min_value_basic() {
    let map = from_tuples(&[("a", 10), ("b", 30), ("c", 5)]);
    assert_eq!(key_with_min_value(&map), Some("c".to_string()));
}

#[test]
fn test_key_with_min_value_empty() {
    let map: HashMap<String, i32> = HashMap::new();
    assert_eq!(key_with_min_value(&map), None);
}

#[test]
fn test_keys_sorted_by_value_basic() {
    let map = from_tuples(&[("c", 3), ("a", 1), ("b", 2)]);
    assert_eq!(keys_sorted_by_value(&map), vec!["a", "b", "c"]);
}

#[test]
fn test_keys_sorted_by_value_empty() {
    let map: HashMap<String, i32> = HashMap::new();
    assert!(keys_sorted_by_value(&map).is_empty());
}

#[test]
fn test_partition_map_positive_negative() {
    let map = from_tuples(&[("a", 5), ("b", -3), ("c", 0), ("d", -1)]);
    let (pos, neg) = partition_map(&map, |v| *v >= 0);
    assert_eq!(pos.len(), 2); // a=5, c=0
    assert_eq!(neg.len(), 2); // b=-3, d=-1
    assert!(pos.contains_key("a"));
    assert!(pos.contains_key("c"));
    assert!(neg.contains_key("b"));
    assert!(neg.contains_key("d"));
}

#[test]
fn test_partition_map_all_pass() {
    let map = from_tuples(&[("a", 1), ("b", 2)]);
    let (pass, fail) = partition_map(&map, |v| *v > 0);
    assert_eq!(pass.len(), 2);
    assert!(fail.is_empty());
}

// =============================================================================
// Topic 8: HashSet Fundamentals
// =============================================================================

#[test]
fn test_unique_elements_with_dupes() {
    assert_eq!(unique_elements(&[1, 2, 2, 3, 1, 4, 3]), vec![1, 2, 3, 4]);
}

#[test]
fn test_unique_elements_no_dupes() {
    assert_eq!(unique_elements(&[5, 10, 15]), vec![5, 10, 15]);
}

#[test]
fn test_unique_elements_empty() {
    assert_eq!(unique_elements(&[]), vec![]);
}

#[test]
fn test_is_all_unique_true() {
    assert!(is_all_unique(&[1, 2, 3, 4]));
}

#[test]
fn test_is_all_unique_false() {
    assert!(!is_all_unique(&[1, 2, 3, 2]));
}

#[test]
fn test_is_all_unique_empty() {
    assert!(is_all_unique(&[]));
}

#[test]
fn test_set_intersection_overlap() {
    assert_eq!(set_intersection(&[1, 2, 3, 4], &[3, 4, 5, 6]), vec![3, 4]);
}

#[test]
fn test_set_intersection_no_overlap() {
    assert_eq!(set_intersection(&[1, 2], &[3, 4]), vec![]);
}

#[test]
fn test_set_intersection_empty() {
    assert_eq!(set_intersection(&[], &[1, 2]), vec![]);
}

#[test]
fn test_set_intersection_with_duplicates_in_input() {
    // Input duplicates are collapsed by HashSet
    assert_eq!(set_intersection(&[1, 1, 2, 2], &[2, 2, 3, 3]), vec![2]);
}

#[test]
fn test_set_union_basic() {
    assert_eq!(set_union(&[1, 2, 3], &[3, 4, 5]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_set_union_identical() {
    assert_eq!(set_union(&[1, 2], &[1, 2]), vec![1, 2]);
}

#[test]
fn test_set_union_empty() {
    assert_eq!(set_union(&[], &[1, 2]), vec![1, 2]);
}

#[test]
fn test_set_difference_basic() {
    assert_eq!(set_difference(&[1, 2, 3, 4], &[3, 4, 5, 6]), vec![1, 2]);
}

#[test]
fn test_set_difference_none() {
    assert_eq!(set_difference(&[1, 2], &[1, 2, 3]), vec![]);
}

#[test]
fn test_set_difference_all() {
    assert_eq!(set_difference(&[1, 2], &[3, 4]), vec![1, 2]);
}

#[test]
fn test_set_symmetric_difference_basic() {
    assert_eq!(
        set_symmetric_difference(&[1, 2, 3], &[3, 4, 5]),
        vec![1, 2, 4, 5]
    );
}

#[test]
fn test_set_symmetric_difference_identical() {
    assert_eq!(set_symmetric_difference(&[1, 2, 3], &[1, 2, 3]), vec![]);
}

#[test]
fn test_set_symmetric_difference_disjoint() {
    assert_eq!(
        set_symmetric_difference(&[1, 2], &[3, 4]),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn test_is_subset_true() {
    assert!(is_subset(&[1, 2], &[1, 2, 3, 4]));
}

#[test]
fn test_is_subset_false() {
    assert!(!is_subset(&[1, 5], &[1, 2, 3]));
}

#[test]
fn test_is_subset_empty_is_subset_of_anything() {
    assert!(is_subset(&[], &[1, 2, 3]));
}

#[test]
fn test_is_subset_equal_sets() {
    assert!(is_subset(&[1, 2, 3], &[1, 2, 3]));
}

#[test]
fn test_is_disjoint_true() {
    assert!(is_disjoint(&[1, 2], &[3, 4]));
}

#[test]
fn test_is_disjoint_false() {
    assert!(!is_disjoint(&[1, 2, 3], &[3, 4, 5]));
}

#[test]
fn test_is_disjoint_empty() {
    assert!(is_disjoint(&[], &[1, 2]));
}

// =============================================================================
// Topic 9: Nested Maps & Complex Structures
// =============================================================================

#[test]
fn test_add_grade_new_student() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 95.0);
    assert_eq!(book["Alice"]["Math"], 95.0);
}

#[test]
fn test_add_grade_existing_student() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 95.0);
    add_grade(&mut book, "Alice", "English", 88.0);
    assert_eq!(book["Alice"].len(), 2);
}

#[test]
fn test_add_grade_overwrite() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 70.0);
    add_grade(&mut book, "Alice", "Math", 95.0);
    assert_eq!(book["Alice"]["Math"], 95.0);
}

#[test]
fn test_student_average_basic() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    add_grade(&mut book, "Alice", "English", 80.0);
    let avg = student_average(&book, "Alice").unwrap();
    assert!((avg - 85.0).abs() < 0.01);
}

#[test]
fn test_student_average_unknown() {
    let book = GradeBook::new();
    assert_eq!(student_average(&book, "Nobody"), None);
}

#[test]
fn test_student_average_single_grade() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Bob", "Art", 100.0);
    assert_eq!(student_average(&book, "Bob"), Some(100.0));
}

#[test]
fn test_students_in_subject_found() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    add_grade(&mut book, "Bob", "Math", 85.0);
    add_grade(&mut book, "Charlie", "English", 88.0);
    assert_eq!(students_in_subject(&book, "Math"), vec!["Alice", "Bob"]);
}

#[test]
fn test_students_in_subject_not_found() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    assert!(students_in_subject(&book, "Physics").is_empty());
}

#[test]
fn test_top_student_basic() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    add_grade(&mut book, "Alice", "English", 80.0); // avg 85
    add_grade(&mut book, "Bob", "Math", 95.0);
    add_grade(&mut book, "Bob", "English", 90.0); // avg 92.5
    assert_eq!(top_student(&book), Some("Bob".to_string()));
}

#[test]
fn test_top_student_empty() {
    let book = GradeBook::new();
    assert_eq!(top_student(&book), None);
}

#[test]
fn test_subject_averages_basic() {
    let mut book = GradeBook::new();
    add_grade(&mut book, "Alice", "Math", 90.0);
    add_grade(&mut book, "Bob", "Math", 80.0);
    add_grade(&mut book, "Alice", "English", 70.0);
    let avgs = subject_averages(&book);
    assert!((avgs["Math"] - 85.0).abs() < 0.01);
    assert!((avgs["English"] - 70.0).abs() < 0.01);
}

#[test]
fn test_subject_averages_empty() {
    let book = GradeBook::new();
    assert!(subject_averages(&book).is_empty());
}

#[test]
fn test_add_to_department_new() {
    let mut dir = HashMap::new();
    add_to_department(&mut dir, "Engineering", "Charlie");
    add_to_department(&mut dir, "Engineering", "Alice");
    // Should be sorted
    assert_eq!(dir["Engineering"], vec!["Alice", "Charlie"]);
}

#[test]
fn test_add_to_department_multiple_departments() {
    let mut dir = HashMap::new();
    add_to_department(&mut dir, "Engineering", "Alice");
    add_to_department(&mut dir, "Sales", "Bob");
    assert_eq!(dir.len(), 2);
}

#[test]
fn test_people_in_department_exists() {
    let mut dir = HashMap::new();
    add_to_department(&mut dir, "Sales", "Bob");
    add_to_department(&mut dir, "Sales", "Alice");
    assert_eq!(people_in_department(&dir, "Sales"), vec!["Alice", "Bob"]);
}

#[test]
fn test_people_in_department_not_exists() {
    let dir: HashMap<String, Vec<String>> = HashMap::new();
    assert!(people_in_department(&dir, "Marketing").is_empty());
}

#[test]
fn test_all_people_sorted_basic() {
    let mut dir = HashMap::new();
    add_to_department(&mut dir, "Engineering", "Charlie");
    add_to_department(&mut dir, "Sales", "Alice");
    add_to_department(&mut dir, "Engineering", "Bob");
    assert_eq!(
        all_people_sorted(&dir),
        vec!["Alice", "Bob", "Charlie"]
    );
}

#[test]
fn test_all_people_sorted_empty() {
    let dir: HashMap<String, Vec<String>> = HashMap::new();
    assert!(all_people_sorted(&dir).is_empty());
}

// =============================================================================
// Topic 10: Advanced HashMap Patterns
// =============================================================================

#[test]
fn test_two_sum_found() {
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn test_two_sum_not_found() {
    assert_eq!(two_sum(&[1, 2, 3], 10), None);
}

#[test]
fn test_two_sum_negative() {
    assert_eq!(two_sum(&[3, -1, 4, 1], 0), Some((1, 3)));
}

#[test]
fn test_two_sum_empty() {
    assert_eq!(two_sum(&[], 5), None);
}

#[test]
fn test_most_frequent_basic() {
    assert_eq!(most_frequent(&[1, 2, 2, 3, 3, 3]), Some(3));
}

#[test]
fn test_most_frequent_single() {
    assert_eq!(most_frequent(&[42]), Some(42));
}

#[test]
fn test_most_frequent_empty() {
    assert_eq!(most_frequent(&[]), None);
}

#[test]
fn test_top_n_keys_basic() {
    let mut m = HashMap::new();
    m.insert("a".into(), 10);
    m.insert("b".into(), 30);
    m.insert("c".into(), 20);
    let top = top_n_keys(&m, 2);
    assert_eq!(top, vec!["b", "c"]);
}

#[test]
fn test_top_n_keys_n_exceeds_len() {
    let mut m = HashMap::new();
    m.insert("a".into(), 1);
    let top = top_n_keys(&m, 5);
    assert_eq!(top.len(), 1);
}

#[test]
fn test_top_n_keys_zero() {
    let m = from_tuples(&[("a", 1)]);
    assert!(top_n_keys(&m, 0).is_empty());
}

#[test]
fn test_value_histogram_basic() {
    let hist = value_histogram(&[1, 2, 2, 3, 3, 3]);
    assert_eq!(hist[&1], 1);
    assert_eq!(hist[&2], 2);
    assert_eq!(hist[&3], 3);
}

#[test]
fn test_value_histogram_empty() {
    let hist = value_histogram(&[]);
    assert!(hist.is_empty());
}

#[test]
fn test_merge_sum_shared_keys() {
    let mut a = HashMap::new();
    a.insert("x".into(), 10);
    a.insert("y".into(), 5);
    let mut b = HashMap::new();
    b.insert("x".into(), 5);
    b.insert("z".into(), 3);
    let merged = merge_sum(&a, &b);
    assert_eq!(merged["x"], 15); // summed
    assert_eq!(merged["y"], 5);
    assert_eq!(merged["z"], 3);
}

#[test]
fn test_merge_sum_disjoint() {
    let a = from_tuples(&[("a", 1)]);
    let b = from_tuples(&[("b", 2)]);
    let merged = merge_sum(&a, &b);
    assert_eq!(merged.len(), 2);
    assert_eq!(merged["a"], 1);
    assert_eq!(merged["b"], 2);
}

#[test]
fn test_anagram_groups_basic() {
    let groups = anagram_groups(&["eat", "tea", "tan", "ate", "nat", "bat"]);
    assert_eq!(
        groups,
        vec![
            vec!["ate", "eat", "tea"],
            vec!["bat"],
            vec!["nat", "tan"],
        ]
    );
}

#[test]
fn test_anagram_groups_no_anagrams() {
    let groups = anagram_groups(&["abc", "def", "ghi"]);
    assert_eq!(groups.len(), 3);
    // Each group has exactly 1 element
    assert!(groups.iter().all(|g| g.len() == 1));
}

#[test]
fn test_anagram_groups_all_anagrams() {
    let groups = anagram_groups(&["abc", "bca", "cab"]);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].len(), 3);
}

#[test]
fn test_anagram_groups_empty() {
    let groups = anagram_groups(&[]);
    assert!(groups.is_empty());
}

#[test]
fn test_first_unique_char_basic() {
    assert_eq!(first_unique_char("aabcbd"), Some('c'));
}

#[test]
fn test_first_unique_char_first_position() {
    assert_eq!(first_unique_char("abcabc"), None); // all repeat
}

#[test]
fn test_first_unique_char_first_is_unique() {
    assert_eq!(first_unique_char("xaabb"), Some('x'));
}

#[test]
fn test_first_unique_char_empty() {
    assert_eq!(first_unique_char(""), None);
}

#[test]
fn test_first_unique_char_single() {
    assert_eq!(first_unique_char("z"), Some('z'));
}

#[test]
fn test_longest_consecutive_basic() {
    assert_eq!(longest_consecutive(&[100, 4, 200, 1, 3, 2]), 4);
}

#[test]
fn test_longest_consecutive_single() {
    assert_eq!(longest_consecutive(&[42]), 1);
}

#[test]
fn test_longest_consecutive_empty() {
    assert_eq!(longest_consecutive(&[]), 0);
}

#[test]
fn test_longest_consecutive_with_duplicates() {
    assert_eq!(longest_consecutive(&[1, 2, 2, 3, 3, 4]), 4);
}

#[test]
fn test_longest_consecutive_disjoint() {
    assert_eq!(longest_consecutive(&[10, 20, 30]), 1);
}
