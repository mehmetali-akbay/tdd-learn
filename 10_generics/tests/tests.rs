use generics::*;

// ===== Topic 1: Generic Functions =====

#[test]
fn test_first() {
    assert_eq!(first(&[1, 2, 3]), Some(&1));
    assert_eq!(first(&["a", "b"]), Some(&"a"));
    assert_eq!(first::<i32>(&[]), None);
}

#[test]
fn test_last() {
    assert_eq!(last(&[1, 2, 3]), Some(&3));
    assert_eq!(last::<i32>(&[]), None);
}

#[test]
fn test_contains() {
    assert!(contains(&[1, 2, 3], &2));
    assert!(!contains(&[1, 2, 3], &4));
    assert!(contains(&["a", "b", "c"], &"b"));
}

#[test]
fn test_max_of_two() {
    assert_eq!(max_of_two(3, 5), 5);
    assert_eq!(max_of_two(10, 2), 10);
    assert_eq!(max_of_two(4, 4), 4);
    assert_eq!(max_of_two(1.5, 2.5), 2.5);
}

#[test]
fn test_swap() {
    assert_eq!(swap(1, 2), (2, 1));
    assert_eq!(swap("a", "b"), ("b", "a"));
}

#[test]
fn test_deduplicate() {
    assert_eq!(deduplicate(&[1, 2, 2, 3, 1, 3]), vec![1, 2, 3]);
    assert_eq!(deduplicate(&["a", "b", "a"]), vec!["a", "b"]);
    assert_eq!(deduplicate::<i32>(&[]), Vec::<i32>::new());
}

// ===== Topic 2: Wrapper<T> =====

#[test]
fn test_wrapper_new_get() {
    let w = Wrapper::new(42);
    assert_eq!(*w.get(), 42);
}

#[test]
fn test_wrapper_set() {
    let mut w = Wrapper::new(1);
    w.set(99);
    assert_eq!(*w.get(), 99);
}

#[test]
fn test_wrapper_into_inner() {
    let w = Wrapper::new("hello".to_string());
    let s: String = w.into_inner();
    assert_eq!(s, "hello");
}

#[test]
fn test_wrapper_map() {
    let w = Wrapper::new(5);
    let doubled = w.map(|x| x * 2);
    assert_eq!(*doubled.get(), 10);
}

#[test]
fn test_wrapper_map_type_change() {
    let w = Wrapper::new(42);
    let s = w.map(|n| n.to_string());
    assert_eq!(*s.get(), "42");
}

// ===== Topic 3: Pair<T, U> =====

#[test]
fn test_pair_new() {
    let p = Pair::new(1, "hello");
    assert_eq!(*p.first(), 1);
    assert_eq!(*p.second(), "hello");
}

#[test]
fn test_pair_swap() {
    let p = Pair::new(1, "hello");
    let swapped = p.swap();
    assert_eq!(*swapped.first(), "hello");
    assert_eq!(*swapped.second(), 1);
}

#[test]
fn test_pair_map_first() {
    let p = Pair::new(5, "hello");
    let mapped = p.map_first(|x| x * 10);
    assert_eq!(*mapped.first(), 50);
    assert_eq!(*mapped.second(), "hello");
}

#[test]
fn test_pair_map_second() {
    let p = Pair::new("hello", 3);
    let mapped = p.map_second(|x| x + 1);
    assert_eq!(*mapped.first(), "hello");
    assert_eq!(*mapped.second(), 4);
}

// ===== Topic 4: Stack<T> =====

#[test]
fn test_stack_new_empty() {
    let stack: Stack<i32> = Stack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_stack_push_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_stack_peek() {
    let mut stack = Stack::new();
    assert_eq!(stack.peek(), None);
    stack.push("hello");
    assert_eq!(stack.peek(), Some(&"hello"));
    assert_eq!(stack.size(), 1); // peek doesn't remove
}

#[test]
fn test_stack_to_vec() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_stack_with_strings() {
    let mut stack = Stack::new();
    stack.push("rust".to_string());
    stack.push("is".to_string());
    stack.push("great".to_string());
    assert_eq!(stack.pop(), Some("great".to_string()));
    assert_eq!(stack.size(), 2);
}

// ===== Topic 5: Generic Constraints =====

#[test]
fn test_min_of_three() {
    assert_eq!(min_of_three(3, 1, 2), 1);
    assert_eq!(min_of_three(5, 5, 5), 5);
    assert_eq!(min_of_three(1.5, 0.5, 2.5), 0.5);
}

#[test]
fn test_clamp() {
    assert_eq!(clamp(5, 0, 10), 5); // in range
    assert_eq!(clamp(-5, 0, 10), 0); // below min
    assert_eq!(clamp(15, 0, 10), 10); // above max
    assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
}

#[test]
fn test_sort_three() {
    assert_eq!(sort_three(3, 1, 2), (1, 2, 3));
    assert_eq!(sort_three(1, 1, 1), (1, 1, 1));
    assert_eq!(sort_three(5, 3, 8), (3, 5, 8));
}

#[test]
fn test_is_between() {
    assert!(is_between(&5, &1, &10));
    assert!(is_between(&1, &1, &10)); // inclusive
    assert!(!is_between(&0, &1, &10));
    assert!(is_between(&"b", &"a", &"c"));
}

#[test]
fn test_median_of_three() {
    assert_eq!(median_of_three(1, 3, 2), 2);
    assert_eq!(median_of_three(5, 5, 5), 5);
    assert_eq!(median_of_three(10, 1, 5), 5);
}

// ===== Topic 6: Advanced — Fn Bounds & Pipeline =====

#[test]
fn test_apply() {
    assert_eq!(apply(5, |x| x * 2), 10);
    assert_eq!(apply("hello", |s| s.len()), 5);
}

#[test]
fn test_compose() {
    let double = |x: i32| x * 2;
    let add_one = |x: i32| x + 1;
    let double_then_add = compose(add_one, double);
    assert_eq!(double_then_add(5), 11); // (5*2)+1
}

#[test]
fn test_pipeline() {
    let result = Pipeline::new(5)
        .then(|x| x * 2)
        .then(|x| x + 1)
        .then(|x| x.to_string())
        .execute();
    assert_eq!(result, "11");
}

#[test]
fn test_pipeline_strings() {
    let result = Pipeline::new("  Hello World  ".to_string())
        .then(|s| s.trim().to_string())
        .then(|s| s.to_lowercase())
        .execute();
    assert_eq!(result, "hello world");
}

#[test]
fn test_filter_map_result() {
    let items: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3), Err("nope"), Ok(5)];
    let doubled = filter_map_result(items, |x| x * 2);
    assert_eq!(doubled, vec![2, 6, 10]);
}

#[test]
fn test_filter_map_result_all_errors() {
    let items: Vec<Result<i32, &str>> = vec![Err("a"), Err("b")];
    let result = filter_map_result(items, |x| x);
    assert!(result.is_empty());
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_max_of() {
    assert_eq!(max_of(3, 5), 5);
    assert_eq!(max_of("apple", "banana"), "banana");
}

#[test]
fn test_is_sorted_generic() {
    assert!(is_sorted(&[1, 2, 3, 4]));
    assert!(!is_sorted(&[1, 3, 2]));
    assert!(is_sorted(&["a", "b", "c"]));
}

#[test]
fn test_count_where() {
    assert_eq!(count_where(&[1, 2, 3, 4, 5], |x| *x > 3), 2);
}

#[test]
fn test_find_first() {
    assert_eq!(find_first(&[1, 2, 3], |x| *x > 1), Some(&2));
    assert_eq!(find_first(&[1, 2, 3], |x| *x > 5), None);
}

#[test]
fn test_zip_slices() {
    let result = zip_slices(&[1, 2], &["a", "b", "c"]);
    assert_eq!(result.len(), 2);
    assert_eq!(*result[0].0, 1);
    assert_eq!(*result[0].1, "a");
}
