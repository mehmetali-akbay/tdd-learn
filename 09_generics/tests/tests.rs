use generics::*;

// ============================================
// Topic 1: Generic Functions
// ============================================

#[test]
fn test_first() {
    assert_eq!(first(&[1, 2, 3]), Some(&1));
    assert_eq!(first(&["a", "b"]), Some(&"a"));
    assert_eq!(first::<i32>(&[]), None);
}

#[test]
fn test_last() {
    assert_eq!(last(&[1, 2, 3]), Some(&3));
    assert_eq!(last(&["x"]), Some(&"x"));
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
fn test_min_of_two() {
    assert_eq!(min_of_two(3, 5), 3);
    assert_eq!(min_of_two(10, 2), 2);
    assert_eq!(min_of_two(4, 4), 4);
    assert_eq!(min_of_two('a', 'z'), 'a');
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

#[test]
fn test_find_first() {
    assert_eq!(find_first(&[1, 2, 3], |x| *x > 1), Some(&2));
    assert_eq!(find_first(&[1, 2, 3], |x| *x > 5), None);
}

#[test]
fn test_find_first_strings() {
    let words = vec!["short", "longer", "longest"];
    assert_eq!(find_first(&words, |w| w.len() > 5), Some(&"longer"));
}

#[test]
fn test_zip_slices() {
    let result = zip_slices(&[1, 2], &["a", "b", "c"]);
    assert_eq!(result.len(), 2);
    assert_eq!(*result[0].0, 1);
    assert_eq!(*result[0].1, "a");
    assert_eq!(*result[1].0, 2);
    assert_eq!(*result[1].1, "b");
}

#[test]
fn test_zip_slices_empty() {
    let result = zip_slices::<i32, &str>(&[], &["a"]);
    assert!(result.is_empty());
}

// ============================================
// Topic 2: Wrapper<T>
// ============================================

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

#[test]
fn test_wrapper_unwrap_or_passes() {
    let w = Wrapper::new(10);
    let result = w.unwrap_or(0, |x| *x > 5);
    assert_eq!(result, 10);
}

#[test]
fn test_wrapper_unwrap_or_fails() {
    let w = Wrapper::new(3);
    let result = w.unwrap_or(0, |x| *x > 5);
    assert_eq!(result, 0);
}

#[test]
fn test_wrapper_zip() {
    let a = Wrapper::new(1);
    let b = Wrapper::new("hello");
    let zipped = a.zip(b);
    assert_eq!(*zipped.get(), (1, "hello"));
}

#[test]
fn test_wrapper_display() {
    let w = Wrapper::new(42);
    assert_eq!(format!("{}", w), "Wrapper(42)");
}

#[test]
fn test_wrapper_display_string() {
    let w = Wrapper::new("hello".to_string());
    assert_eq!(format!("{}", w), "Wrapper(hello)");
}

// ============================================
// Topic 3: Pair<T, U>
// ============================================

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

#[test]
fn test_pair_mixup() {
    let p1 = Pair::new(5, 10.4);
    let p2 = Pair::new("Hello", 'c');
    let p3 = p1.mixup(p2);
    assert_eq!(*p3.first(), 5);
    assert_eq!(*p3.second(), 'c');
}

#[test]
fn test_pair_mixup_different_types() {
    let p1 = Pair::new("rust", true);
    let p2 = Pair::new(42, vec![1, 2, 3]);
    let p3 = p1.mixup(p2);
    assert_eq!(*p3.first(), "rust");
    assert_eq!(*p3.second(), vec![1, 2, 3]);
}

#[test]
fn test_pair_into_tuple() {
    let p = Pair::new(1, "hello");
    assert_eq!(p.into_tuple(), (1, "hello"));
}

#[test]
fn test_pair_display() {
    let p = Pair::new(1, "world");
    assert_eq!(format!("{}", p), "(1, world)");
}

// ============================================
// Topic 4: Generic Enums — Maybe<T> & Either<L, R>
// ============================================

#[test]
fn test_maybe_just() {
    let m = Maybe::Just(42);
    assert!(m.is_just());
    assert!(!m.is_nothing());
    assert_eq!(m.unwrap(), 42);
}

#[test]
fn test_maybe_nothing() {
    let m: Maybe<i32> = Maybe::Nothing;
    assert!(!m.is_just());
    assert!(m.is_nothing());
}

#[test]
#[should_panic(expected = "called unwrap on Nothing")]
fn test_maybe_unwrap_nothing_panics() {
    let m: Maybe<i32> = Maybe::Nothing;
    m.unwrap();
}

#[test]
fn test_maybe_unwrap_or() {
    assert_eq!(Maybe::Just(10).unwrap_or(0), 10);
    assert_eq!(Maybe::<i32>::Nothing.unwrap_or(0), 0);
}

#[test]
fn test_maybe_map() {
    let m = Maybe::Just(5);
    let mapped = m.map(|x| x * 2);
    assert_eq!(mapped, Maybe::Just(10));
}

#[test]
fn test_maybe_map_nothing() {
    let m: Maybe<i32> = Maybe::Nothing;
    let mapped = m.map(|x| x * 2);
    assert_eq!(mapped, Maybe::Nothing);
}

#[test]
fn test_maybe_map_type_change() {
    let m = Maybe::Just(42);
    let mapped = m.map(|x| format!("val={}", x));
    assert_eq!(mapped, Maybe::Just("val=42".to_string()));
}

#[test]
fn test_maybe_to_option() {
    assert_eq!(Maybe::Just(5).to_option(), Some(5));
    assert_eq!(Maybe::<i32>::Nothing.to_option(), None);
}

#[test]
fn test_maybe_from_option() {
    assert_eq!(Maybe::from_option(Some(5)), Maybe::Just(5));
    assert_eq!(Maybe::from_option(None::<i32>), Maybe::Nothing);
}

#[test]
fn test_either_left() {
    let e: Either<i32, &str> = Either::Left(42);
    assert!(e.is_left());
    assert!(!e.is_right());
    assert_eq!(e.left(), Some(42));
}

#[test]
fn test_either_right() {
    let e: Either<i32, &str> = Either::Right("hello");
    assert!(!e.is_left());
    assert!(e.is_right());
    assert_eq!(e.right(), Some("hello"));
}

#[test]
fn test_either_left_returns_none_for_right() {
    let e: Either<i32, &str> = Either::Right("hello");
    assert_eq!(e.left(), None);
}

#[test]
fn test_either_right_returns_none_for_left() {
    let e: Either<i32, &str> = Either::Left(42);
    assert_eq!(e.right(), None);
}

#[test]
fn test_either_map_left() {
    let e: Either<i32, &str> = Either::Left(5);
    let mapped = e.map_left(|x| x * 10);
    assert_eq!(mapped, Either::Left(50));
}

#[test]
fn test_either_map_left_unchanged_right() {
    let e: Either<i32, &str> = Either::Right("hello");
    let mapped = e.map_left(|x| x * 10);
    assert_eq!(mapped, Either::Right("hello"));
}

#[test]
fn test_either_map_right() {
    let e: Either<i32, String> = Either::Right("hello".to_string());
    let mapped = e.map_right(|s| s.len());
    assert_eq!(mapped, Either::Right(5));
}

#[test]
fn test_either_map_right_unchanged_left() {
    let e: Either<i32, String> = Either::Left(42);
    let mapped = e.map_right(|s| s.len());
    assert_eq!(mapped, Either::Left(42));
}

// ============================================
// Topic 5: Stack<T>
// ============================================

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
fn test_stack_into_vec() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.into_vec(), vec![1, 2, 3]);
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

#[test]
fn test_stack_from_vec() {
    let stack = Stack::from_vec(vec![1, 2, 3]);
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&3));
}

#[test]
fn test_stack_reverse() {
    let mut stack = Stack::from_vec(vec![1, 2, 3]);
    stack.reverse();
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(3));
}

#[test]
fn test_stack_drain_to_vec() {
    let mut stack = Stack::from_vec(vec![1, 2, 3]);
    let drained = stack.drain_to_vec();
    assert_eq!(drained, vec![3, 2, 1]); // top to bottom
    assert!(stack.is_empty());
}

#[test]
fn test_stack_default() {
    let stack: Stack<f64> = Stack::default();
    assert!(stack.is_empty());
}

#[test]
fn test_stack_display() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(format!("{}", stack), "[1, 2, 3]");
}

// ============================================
// Topic 6: Trait Bounds & Constraints
// ============================================

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

#[test]
fn test_is_sorted() {
    assert!(is_sorted(&[1, 2, 3, 4]));
    assert!(!is_sorted(&[1, 3, 2]));
    assert!(is_sorted(&["a", "b", "c"]));
    assert!(is_sorted::<i32>(&[]));
    assert!(is_sorted(&[42]));
}

#[test]
fn test_count_where() {
    assert_eq!(count_where(&[1, 2, 3, 4, 5], |x| *x > 3), 2);
    assert_eq!(count_where(&[1, 2, 3, 4, 5], |x| *x % 2 == 0), 2);
}

#[test]
fn test_partition() {
    let (evens, odds) = partition(&[1, 2, 3, 4, 5, 6], |x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5]);
}

#[test]
fn test_partition_strings() {
    let words = vec!["hi", "hello", "yo", "hey"];
    let (short, long) = partition(&words, |w| w.len() <= 2);
    assert_eq!(short, vec!["hi", "yo"]);
    assert_eq!(long, vec!["hello", "hey"]);
}

#[test]
fn test_format_joined() {
    assert_eq!(format_joined(&[1, 2, 3], ", "), "1, 2, 3");
    assert_eq!(format_joined(&["a", "b", "c"], " | "), "a | b | c");
    assert_eq!(format_joined::<i32>(&[], ", "), "");
}

#[test]
fn test_group_by() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let grouped = group_by(&items, |x| x % 2 == 0);
    assert_eq!(grouped[&true], vec![2, 4, 6]);
    assert_eq!(grouped[&false], vec![1, 3, 5]);
}

#[test]
fn test_group_by_string_length() {
    let words = vec!["hi", "hey", "hello", "yo", "yes"];
    let grouped = group_by(&words, |w| w.len());
    assert_eq!(grouped[&2], vec!["hi", "yo"]);
    assert_eq!(grouped[&3], vec!["hey", "yes"]);
    assert_eq!(grouped[&5], vec!["hello"]);
}

#[test]
fn test_bounded_new_clamps() {
    let b = Bounded::new(15, 0, 10);
    assert_eq!(b.value(), 10);
}

#[test]
fn test_bounded_in_range() {
    let b = Bounded::new(5, 0, 10);
    assert_eq!(b.value(), 5);
    assert!(!b.is_at_min());
    assert!(!b.is_at_max());
}

#[test]
fn test_bounded_set_clamps() {
    let mut b = Bounded::new(5, 0, 10);
    b.set(20);
    assert_eq!(b.value(), 10);
    assert!(b.is_at_max());
    b.set(-5);
    assert_eq!(b.value(), 0);
    assert!(b.is_at_min());
}

#[test]
fn test_bounded_display() {
    let b = Bounded::new(5, 0, 10);
    assert_eq!(format!("{}", b), "5 [0, 10]");
}

#[test]
fn test_bounded_float() {
    let b = Bounded::new(1.5_f64, 0.0_f64, 1.0_f64);
    assert!((b.value() - 1.0_f64).abs() < 1e-9);
}

// ============================================
// Topic 7: Fn Bounds, Closures & Pipeline
// ============================================

#[test]
fn test_apply() {
    assert_eq!(apply(5, |x| x * 2), 10);
    assert_eq!(apply("hello", |s| s.len()), 5);
}

#[test]
fn test_apply_twice() {
    assert_eq!(apply_twice(2, |x| x * 3), 18); // 2*3=6, 6*3=18
    assert_eq!(apply_twice(1, |x| x + 10), 21); // 1+10=11, 11+10=21
}

#[test]
fn test_compose() {
    let double = |x: i32| x * 2;
    let add_one = |x: i32| x + 1;
    let double_then_add = compose(add_one, double);
    assert_eq!(double_then_add(5), 11); // (5*2)+1
}

#[test]
fn test_compose_type_change() {
    let to_string = |x: i32| format!("{}", x);
    let length = |s: String| s.len();
    let int_to_len = compose(length, to_string);
    assert_eq!(int_to_len(42), 2);
    assert_eq!(int_to_len(1000), 4);
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
fn test_pipeline_vec() {
    let result = Pipeline::new(vec![3, 1, 4, 1, 5])
        .then(|mut v| { v.sort(); v })
        .then(|v| v.into_iter().filter(|x| *x > 2).collect::<Vec<_>>())
        .execute();
    assert_eq!(result, vec![3, 4, 5]);
}

#[test]
fn test_filter_map_ok() {
    let items: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3), Err("nope"), Ok(5)];
    let doubled = filter_map_ok(items, |x| x * 2);
    assert_eq!(doubled, vec![2, 6, 10]);
}

#[test]
fn test_filter_map_ok_all_errors() {
    let items: Vec<Result<i32, &str>> = vec![Err("a"), Err("b")];
    let result = filter_map_ok(items, |x| x);
    assert!(result.is_empty());
}

#[test]
fn test_try_apply_success() {
    let result = try_apply(10, |x| {
        if x > 0 { Ok(x * 2) } else { Err("negative") }
    });
    assert_eq!(result, Ok(20));
}

#[test]
fn test_try_apply_failure() {
    let result = try_apply(-5, |x| {
        if x > 0 { Ok(x * 2) } else { Err("negative") }
    });
    assert_eq!(result, Err("negative"));
}

#[test]
fn test_try_map_all_ok() {
    let items = vec![1, 2, 3];
    let result = try_map(&items, |x| {
        if *x > 0 { Ok(x * 10) } else { Err("negative") }
    });
    assert_eq!(result, Ok(vec![10, 20, 30]));
}

#[test]
fn test_try_map_with_error() {
    let items = vec![1, -2, 3];
    let result = try_map(&items, |x| {
        if *x > 0 { Ok(x * 10) } else { Err("negative") }
    });
    assert_eq!(result, Err("negative"));
}

#[test]
fn test_fold_sum() {
    let items = vec![1, 2, 3, 4, 5];
    let sum = fold(&items, 0, |acc, x| acc + x);
    assert_eq!(sum, 15);
}

#[test]
fn test_fold_product() {
    let items = vec![1, 2, 3, 4];
    let product = fold(&items, 1, |acc, x| acc * x);
    assert_eq!(product, 24);
}

#[test]
fn test_fold_string_concat() {
    let words = vec!["hello", "world"];
    let result = fold(&words, String::new(), |acc, w| {
        if acc.is_empty() { w.to_string() } else { format!("{} {}", acc, w) }
    });
    assert_eq!(result, "hello world");
}

#[test]
fn test_fold_empty() {
    let items: Vec<i32> = vec![];
    let sum = fold(&items, 42, |acc, x| acc + x);
    assert_eq!(sum, 42); // returns initial
}