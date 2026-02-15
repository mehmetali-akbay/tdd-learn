use vecs::*;

// ===== Topic 1: Vec Basics =====

#[test]
fn test_create_empty_vec() {
    let v = create_empty_vec();
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);
}

#[test]
fn test_create_sample_vec() {
    assert_eq!(create_sample_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_push_element() {
    let v = vec![1, 2, 3];
    assert_eq!(push_element(v, 4), vec![1, 2, 3, 4]);
    assert_eq!(push_element(vec![], 1), vec![1]);
}

#[test]
fn test_pop_last() {
    let mut v = vec![1, 2, 3];
    assert_eq!(pop_last(&mut v), Some(3));
    assert_eq!(v, vec![1, 2]);
    assert_eq!(pop_last(&mut vec![]), None);
}

#[test]
fn test_vec_length() {
    assert_eq!(vec_length(&[1, 2, 3]), 3);
    assert_eq!(vec_length(&[]), 0);
}

#[test]
fn test_get_element() {
    let v = vec![10, 20, 30];
    assert_eq!(get_element(&v, 0), Some(10));
    assert_eq!(get_element(&v, 2), Some(30));
    assert_eq!(get_element(&v, 99), None);
}

#[test]
fn test_vec_contains() {
    let v = vec![1, 2, 3, 4, 5];
    assert!(vec_contains(&v, 3));
    assert!(!vec_contains(&v, 99));
}

// ===== Topic 2: Vec Iteration =====

#[test]
fn test_sum_all() {
    assert_eq!(sum_all(&[1, 2, 3, 4, 5]), 15);
    assert_eq!(sum_all(&[]), 0);
    assert_eq!(sum_all(&[-1, 1]), 0);
}

#[test]
fn test_find_max() {
    assert_eq!(find_max(&[1, 5, 3, 2, 4]), Some(5));
    assert_eq!(find_max(&[-10, -5, -1]), Some(-1));
    assert_eq!(find_max(&[]), None);
}

#[test]
fn test_find_min() {
    assert_eq!(find_min(&[1, 5, 3, 2, 4]), Some(1));
    assert_eq!(find_min(&[-10, -5, -1]), Some(-10));
    assert_eq!(find_min(&[]), None);
}

#[test]
fn test_all_positive() {
    assert!(all_positive(&[1, 2, 3]));
    assert!(!all_positive(&[1, -1, 3]));
    assert!(!all_positive(&[0]));
    assert!(all_positive(&[])); // vacuous truth
}

#[test]
fn test_any_equals() {
    assert!(any_equals(&[1, 2, 3], 2));
    assert!(!any_equals(&[1, 2, 3], 99));
    assert!(!any_equals(&[], 1));
}

#[test]
fn test_count_above() {
    assert_eq!(count_above(&[1, 2, 3, 4, 5], 3), 2);
    assert_eq!(count_above(&[1, 2, 3], 10), 0);
    assert_eq!(count_above(&[], 0), 0);
}

#[test]
fn test_average() {
    assert_eq!(average(&[1, 2, 3, 4, 5]), Some(3.0));
    assert_eq!(average(&[10]), Some(10.0));
    assert_eq!(average(&[]), None);
}

// ===== Topic 3: Vec Transform =====

#[test]
fn test_double_all() {
    assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
    assert_eq!(double_all(&[]), vec![]);
    assert_eq!(double_all(&[0, -1]), vec![0, -2]);
}

#[test]
fn test_filter_even() {
    assert_eq!(filter_even(&[1, 2, 3, 4, 5]), vec![2, 4]);
    assert_eq!(filter_even(&[1, 3, 5]), vec![]);
    assert_eq!(filter_even(&[2, 4, 6]), vec![2, 4, 6]);
}

#[test]
fn test_square_all() {
    assert_eq!(square_all(&[1, 2, 3]), vec![1, 4, 9]);
    assert_eq!(square_all(&[-2, -3]), vec![4, 9]);
    assert_eq!(square_all(&[]), vec![]);
}

#[test]
fn test_filter_by_length() {
    let words = vec!["hi", "hello", "hey", "rust"];
    assert_eq!(filter_by_length(&words, 3), vec!["hello", "rust"]);
    assert_eq!(filter_by_length(&words, 10), Vec::<String>::new());
}

#[test]
fn test_all_to_uppercase() {
    assert_eq!(
        all_to_uppercase(&["hello", "world"]),
        vec!["HELLO", "WORLD"]
    );
    assert_eq!(all_to_uppercase(&[]), Vec::<String>::new());
}

#[test]
fn test_add_index_prefix() {
    assert_eq!(
        add_index_prefix(&["a", "b", "c"]),
        vec!["0: a", "1: b", "2: c"]
    );
}

// ===== Topic 4: Vec Sorting & Dedup =====

#[test]
fn test_sort_ascending() {
    assert_eq!(sort_ascending(&[3, 1, 4, 1, 5]), vec![1, 1, 3, 4, 5]);
    assert_eq!(sort_ascending(&[]), vec![]);
}

#[test]
fn test_sort_descending() {
    assert_eq!(sort_descending(&[3, 1, 4, 1, 5]), vec![5, 4, 3, 1, 1]);
}

#[test]
fn test_sort_by_length() {
    assert_eq!(
        sort_by_length(&["hello", "hi", "hey"]),
        vec!["hi", "hey", "hello"]
    );
}

#[test]
fn test_remove_consecutive_dupes() {
    assert_eq!(
        remove_consecutive_dupes(&[1, 1, 2, 2, 3, 1]),
        vec![1, 2, 3, 1]
    );
    assert_eq!(remove_consecutive_dupes(&[1, 1, 1]), vec![1]);
}

#[test]
fn test_unique_elements() {
    assert_eq!(unique_elements(&[3, 1, 2, 1, 3]), vec![3, 1, 2]);
    assert_eq!(unique_elements(&[1, 1, 1]), vec![1]);
    assert_eq!(unique_elements(&[]), vec![]);
}

// ===== Topic 5: Vec Slices, Chunks & Combining =====

#[test]
fn test_take_first() {
    assert_eq!(take_first(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
    assert_eq!(take_first(&[1, 2], 10), vec![1, 2]);
    assert_eq!(take_first(&[], 5), vec![]);
}

#[test]
fn test_split_at_index() {
    assert_eq!(
        split_at_index(&[1, 2, 3, 4, 5], 3),
        (vec![1, 2, 3], vec![4, 5])
    );
    assert_eq!(split_at_index(&[1, 2, 3], 0), (vec![], vec![1, 2, 3]));
}

#[test]
fn test_make_chunks() {
    assert_eq!(
        make_chunks(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
}

#[test]
fn test_sliding_windows() {
    assert_eq!(
        sliding_windows(&[1, 2, 3, 4], 2),
        vec![vec![1, 2], vec![2, 3], vec![3, 4]]
    );
}

#[test]
fn test_flatten() {
    assert_eq!(
        flatten(&[vec![1, 2], vec![3, 4], vec![5]]),
        vec![1, 2, 3, 4, 5]
    );
    assert_eq!(flatten(&[]), vec![]);
}

#[test]
fn test_zip_vecs() {
    assert_eq!(
        zip_vecs(&[1, 2, 3], &["a", "b", "c"]),
        vec![
            (1, "a".to_string()),
            (2, "b".to_string()),
            (3, "c".to_string())
        ]
    );
}

// ===== Topic 6: Advanced Vec Challenges =====

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
    assert_eq!(two_sum(&[1, 2, 3], 100), None);
    assert_eq!(two_sum(&[], 5), None);
}

#[test]
fn test_running_sum() {
    assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(&[1, 1, 1]), vec![1, 2, 3]);
    assert_eq!(running_sum(&[]), vec![]);
}

#[test]
fn test_merge_sorted() {
    assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(merge_sorted(&[1, 2, 3], &[]), vec![1, 2, 3]);
    assert_eq!(merge_sorted(&[], &[4, 5, 6]), vec![4, 5, 6]);
    assert_eq!(merge_sorted(&[1, 1], &[1, 1]), vec![1, 1, 1, 1]);
}

#[test]
fn test_partition() {
    fn is_even(x: i32) -> bool {
        x % 2 == 0
    }
    assert_eq!(
        partition(&[1, 2, 3, 4, 5], is_even),
        (vec![2, 4], vec![1, 3, 5])
    );
    assert_eq!(partition(&[], is_even), (vec![], vec![]));
}

#[test]
fn test_group_consecutive() {
    assert_eq!(
        group_consecutive(&[1, 1, 2, 2, 2, 3, 1, 1]),
        vec![vec![1, 1], vec![2, 2, 2], vec![3], vec![1, 1]]
    );
    assert_eq!(group_consecutive(&[1]), vec![vec![1]]);
    assert_eq!(group_consecutive(&[]), Vec::<Vec<i32>>::new());
}

#[test]
fn test_moving_average() {
    let result = moving_average(&[1.0, 2.0, 3.0, 4.0, 5.0], 3);
    assert_eq!(result.len(), 3);
    assert!((result[0] - 2.0).abs() < 0.001);
    assert!((result[1] - 3.0).abs() < 0.001);
    assert!((result[2] - 4.0).abs() < 0.001);
}

#[test]
fn test_rotate_left() {
    assert_eq!(rotate_left(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
    assert_eq!(rotate_left(&[1, 2, 3], 0), vec![1, 2, 3]);
    assert_eq!(rotate_left(&[1, 2, 3], 3), vec![1, 2, 3]); // full rotation
    assert_eq!(rotate_left(&[], 5), vec![]);
}

#[test]
fn test_is_sorted() {
    assert!(is_sorted(&[1, 2, 3, 4, 5]));
    assert!(is_sorted(&[1, 1, 2, 3]));
    assert!(!is_sorted(&[3, 1, 2]));
    assert!(is_sorted(&[]));
    assert!(is_sorted(&[42]));
}

#[test]
fn test_majority_element() {
    assert_eq!(majority_element(&[3, 3, 3, 2, 1]), Some(3));
    assert_eq!(majority_element(&[1, 2, 3]), None);
    assert_eq!(majority_element(&[1, 1, 2, 2]), None); // 2 is not > n/2
}

#[test]
fn test_find_pairs_with_sum() {
    assert_eq!(
        find_pairs_with_sum(&[1, 2, 3, 4, 5], 6),
        vec![(1, 5), (2, 4)]
    );
    assert_eq!(find_pairs_with_sum(&[1, 1, 1], 2), vec![(1, 1)]);
    assert_eq!(find_pairs_with_sum(&[1, 2, 3], 100), vec![]);
}

#[test]
fn test_transpose() {
    assert_eq!(
        transpose(&[vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
    assert_eq!(transpose(&[vec![1], vec![2], vec![3]]), vec![vec![1, 2, 3]]);
}

#[test]
fn test_product_except_self() {
    assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(product_except_self(&[2, 3]), vec![3, 2]);
    assert_eq!(product_except_self(&[5]), vec![1]);
}

// ===== Topic 7: Extra Practice =====

#[test]
fn test_interleave() {
    assert_eq!(interleave(&[1, 2, 3], &[10, 20, 30]), vec![1, 10, 2, 20, 3, 30]);
    assert_eq!(interleave(&[1, 2], &[10, 20, 30]), vec![1, 10, 2, 20, 30]);
    assert_eq!(interleave(&[], &[1]), vec![1]);
}

#[test]
fn test_rotate_right() {
    assert_eq!(rotate_right(&[1, 2, 3, 4, 5], 2), vec![4, 5, 1, 2, 3]);
    assert_eq!(rotate_right(&[1, 2, 3], 0), vec![1, 2, 3]);
    assert_eq!(rotate_right(&[], 5), vec![]);
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(&[1, 2, 3, 2, 1]));
    assert!(is_palindrome(&[1, 2, 2, 1]));
    assert!(!is_palindrome(&[1, 2, 3]));
    assert!(is_palindrome(&[]));
}

#[test]
fn test_nth_largest() {
    assert_eq!(nth_largest(&[3, 1, 5, 2, 4], 1), Some(5));
    assert_eq!(nth_largest(&[3, 1, 5, 2, 4], 3), Some(3));
    assert_eq!(nth_largest(&[3, 1, 5], 0), None);
    assert_eq!(nth_largest(&[3, 1, 5], 4), None);
}

#[test]
fn test_enumerate_filter() {
    let result = enumerate_filter(&[10, 3, 20, 5, 30], |x| x >= 10);
    assert_eq!(result, vec![(0, 10), (2, 20), (4, 30)]);
}
