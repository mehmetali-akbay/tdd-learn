use closures::*;

// =====================================================================
// Topic 1: Closure Basics — Capture & Call
// =====================================================================

#[test]
fn test_map_vec() {
    assert_eq!(map_vec(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
    assert_eq!(map_vec(&["hi", "bye"], |s| s.len()), vec![2, 3]);
}

#[test]
fn test_map_vec_empty() {
    assert_eq!(map_vec::<i32, i32>(&[], |x| x * 2), Vec::<i32>::new());
}

#[test]
fn test_filter_vec() {
    assert_eq!(filter_vec(&[1, 2, 3, 4, 5], |x| x % 2 == 0), vec![2, 4]);
}

#[test]
fn test_filter_vec_none_match() {
    assert_eq!(filter_vec(&[1, 3, 5], |x| x % 2 == 0), Vec::<i32>::new());
}

#[test]
fn test_reduce() {
    assert_eq!(reduce(&[1, 2, 3, 4], 0, |acc, x| acc + x), 10);
    assert_eq!(reduce(&[1, 2, 3], 1, |acc, x| acc * x), 6);
}

#[test]
fn test_reduce_empty() {
    assert_eq!(reduce::<i32>(&[], 0, |acc, x| acc + x), 0);
}

#[test]
fn test_apply_n() {
    assert_eq!(apply_n(1, 4, |x| x * 2), 16); // 1 -> 2 -> 4 -> 8 -> 16
    assert_eq!(apply_n(0, 0, |x| x + 1), 0); // zero times
}

#[test]
fn test_any_of() {
    assert!(any_of(&[1, 2, 3], |x| *x > 2));
    assert!(!any_of(&[1, 2, 3], |x| *x > 5));
}

#[test]
fn test_all_of() {
    assert!(all_of(&[2, 4, 6], |x| x % 2 == 0));
    assert!(!all_of(&[2, 3, 6], |x| x % 2 == 0));
}

#[test]
fn test_any_all_empty() {
    assert!(!any_of::<i32>(&[], |_| true));
    assert!(all_of::<i32>(&[], |_| false)); // vacuously true
}

#[test]
fn test_find_first() {
    assert_eq!(find_first(&[1, 2, 3, 4], |x| *x > 2), Some(&3));
    assert_eq!(find_first(&[1, 2, 3], |x| *x > 10), None);
}

#[test]
fn test_flat_map_vec() {
    assert_eq!(
        flat_map_vec(&[1, 2, 3], |x| vec![*x, x * 10]),
        vec![1, 10, 2, 20, 3, 30]
    );
}

#[test]
fn test_flat_map_vec_empty() {
    assert_eq!(
        flat_map_vec(&[1, 2, 3], |_| Vec::<i32>::new()),
        Vec::<i32>::new()
    );
}

#[test]
fn test_zip_with() {
    assert_eq!(
        zip_with(&[1, 2, 3], &[10, 20, 30], |a, b| a + b),
        vec![11, 22, 33]
    );
}

#[test]
fn test_zip_with_unequal_len() {
    assert_eq!(
        zip_with(&[1, 2], &[10, 20, 30], |a, b| a + b),
        vec![11, 22]
    );
}

#[test]
fn test_take_while_vec() {
    assert_eq!(take_while_vec(&[1, 2, 3, 4, 5], |x| *x < 4), vec![1, 2, 3]);
    assert_eq!(
        take_while_vec(&[10, 20, 30], |x| *x < 5),
        Vec::<i32>::new()
    );
}

// =====================================================================
// Topic 2: Fn vs FnMut vs FnOnce
// =====================================================================

#[test]
fn test_count_matching() {
    assert_eq!(count_matching(&[1, 2, 3, 4, 5], |x| x % 2 == 0), 2);
    assert_eq!(count_matching(&[1, 3, 5], |x| x % 2 == 0), 0);
}

#[test]
fn test_for_each() {
    let mut sum = 0;
    for_each(&[1, 2, 3], |x| sum += x);
    assert_eq!(sum, 6);
}

#[test]
fn test_for_each_collect() {
    let mut result = Vec::new();
    for_each(&["a", "b", "c"], |s| result.push(s.to_uppercase()));
    assert_eq!(result, vec!["A", "B", "C"]);
}

#[test]
fn test_build_string() {
    let mut counter = 0;
    let result = build_string(&[1, 2, 3], |x| {
        counter += 1;
        format!("{}:{}", counter, x)
    });
    assert_eq!(result, "1:1, 2:2, 3:3");
}

#[test]
fn test_consume_with() {
    let v = vec![1, 2, 3];
    let len = consume_with(v, |v| v.len());
    assert_eq!(len, 3);
}

#[test]
fn test_consume_with_string() {
    let s = String::from("hello");
    let upper = consume_with(s, |s| s.to_uppercase());
    assert_eq!(upper, "HELLO");
}

#[test]
fn test_unwrap_or_else() {
    assert_eq!(unwrap_or_else(Some(42), || 0), 42);
    assert_eq!(unwrap_or_else(None, || 99), 99);
}

#[test]
fn test_transform_in_place() {
    let mut nums = vec![1, 2, 3, 4];
    transform_in_place(&mut nums, |x| *x *= 2);
    assert_eq!(nums, vec![2, 4, 6, 8]);
}

#[test]
fn test_transform_in_place_strings() {
    let mut words = vec!["hello".to_string(), "world".to_string()];
    transform_in_place(&mut words, |s| *s = s.to_uppercase());
    assert_eq!(words, vec!["HELLO", "WORLD"]);
}

#[test]
fn test_scan_vec() {
    // Running sum
    let running_sums = scan_vec(&[1, 2, 3, 4], 0, |state, x| {
        *state += x;
        *state
    });
    assert_eq!(running_sums, vec![1, 3, 6, 10]);
}

#[test]
fn test_scan_vec_index() {
    // Attach an incrementing index
    let indexed = scan_vec(&["a", "b", "c"], 0usize, |idx, item| {
        let result = format!("{}:{}", idx, item);
        *idx += 1;
        result
    });
    assert_eq!(indexed, vec!["0:a", "1:b", "2:c"]);
}

#[test]
fn test_map_or_else() {
    let result: i32 = map_or_else(Some(5), || -1, |v| v * 2);
    assert_eq!(result, 10);
    let result: i32 = map_or_else(None, || -1, |v: i32| v * 2);
    assert_eq!(result, -1);
}

// =====================================================================
// Topic 3: Returning Closures
// =====================================================================

#[test]
fn test_make_adder() {
    let add5 = make_adder(5);
    assert_eq!(add5(3), 8);
    assert_eq!(add5(10), 15);
}

#[test]
fn test_make_multiplier() {
    let double = make_multiplier(2);
    assert_eq!(double(5), 10);
    assert_eq!(double(0), 0);
}

#[test]
fn test_make_threshold() {
    let is_adult = make_threshold(18);
    assert!(is_adult(21));
    assert!(is_adult(18));
    assert!(!is_adult(15));
}

#[test]
fn test_make_repeater() {
    let triple = make_repeater(3);
    assert_eq!(triple("ha"), "hahaha");
    assert_eq!(triple(""), "");
}

#[test]
fn test_compose() {
    let add1_then_double = compose(|x: i32| x * 2, |x: i32| x + 1);
    assert_eq!(add1_then_double(3), 8); // (3+1)*2
}

#[test]
fn test_compose_strings() {
    let trim_upper = compose(|s: String| s.to_uppercase(), |s: String| s.trim().to_string());
    assert_eq!(trim_upper("  hello  ".to_string()), "HELLO");
}

#[test]
fn test_negate() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = negate(is_even);
    assert!(is_odd(&3));
    assert!(!is_odd(&4));
}

#[test]
fn test_make_clamper() {
    let clamp_0_100 = make_clamper(0, 100);
    assert_eq!(clamp_0_100(50), 50);
    assert_eq!(clamp_0_100(-10), 0);
    assert_eq!(clamp_0_100(200), 100);
}

#[test]
fn test_make_prefix() {
    let greet = make_prefix("Hello, ");
    assert_eq!(greet("World"), "Hello, World");
    assert_eq!(greet("Rust"), "Hello, Rust");
}

#[test]
fn test_make_counter() {
    let mut counter = make_counter(0);
    assert_eq!(counter(), 0);
    assert_eq!(counter(), 1);
    assert_eq!(counter(), 2);
}

#[test]
fn test_make_counter_nonzero_start() {
    let mut counter = make_counter(10);
    assert_eq!(counter(), 10);
    assert_eq!(counter(), 11);
}

// =====================================================================
// Topic 4: Higher-Order Combinators
// =====================================================================

#[test]
fn test_chain() {
    let add1_then_double = chain(|x: i32| x + 1, |x| x * 2);
    assert_eq!(add1_then_double(3), 8); // (3+1)*2
}

#[test]
fn test_chain_identity() {
    let identity = chain(|x: i32| x, |x| x);
    assert_eq!(identity(42), 42);
}

#[test]
fn test_first_match() {
    let items = vec![1, 5, 10, 15];
    let preds: Vec<&dyn Fn(&i32) -> bool> = vec![
        &|x: &i32| *x > 20, // no match
        &|x: &i32| *x > 8,  // matches 10,15
    ];
    assert_eq!(first_match(&items, &preds), Some(1));
}

#[test]
fn test_first_match_none() {
    let items = vec![1, 2, 3];
    let preds: Vec<&dyn Fn(&i32) -> bool> = vec![&|x: &i32| *x > 100];
    assert_eq!(first_match(&items, &preds), None);
}

#[test]
fn test_partition() {
    let (evens, odds) = partition(&[1, 2, 3, 4, 5], |x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4]);
    assert_eq!(odds, vec![1, 3, 5]);
}

#[test]
fn test_partition_all_pass() {
    let (pass, fail) = partition(&[2, 4, 6], |x| x % 2 == 0);
    assert_eq!(pass, vec![2, 4, 6]);
    assert!(fail.is_empty());
}

#[test]
fn test_group_by() {
    let groups = group_by(
        &[1, 2, 3, 4, 5, 6],
        |x| if x % 2 == 0 { "even" } else { "odd" },
    );
    assert_eq!(groups["even"].len(), 3);
    assert_eq!(groups["odd"].len(), 3);
}

#[test]
fn test_group_by_length() {
    let groups = group_by(&["hi", "hey", "hello", "yo"], |w| w.len());
    assert_eq!(groups[&2].len(), 2); // "hi", "yo"
    assert_eq!(groups[&3].len(), 1); // "hey"
    assert_eq!(groups[&5].len(), 1); // "hello"
}

#[test]
fn test_max_by_key() {
    let words = vec!["hi", "hello", "hey"];
    assert_eq!(max_by_key(&words, |w| w.len()), Some(&"hello"));
}

#[test]
fn test_max_by_key_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(max_by_key(&empty, |x| *x), None);
}

#[test]
fn test_min_by_key() {
    assert_eq!(min_by_key(&[3, 1, 4, 1, 5], |x| *x), Some(&1));
    assert_eq!(min_by_key(&Vec::<i32>::new(), |x| *x), None);
}

#[test]
fn test_sort_by_key_fn() {
    let words = vec!["banana", "apple", "cherry", "date"];
    let sorted = sort_by_key_fn(&words, |w| w.len());
    assert_eq!(sorted, vec!["date", "apple", "banana", "cherry"]);
}

#[test]
fn test_unique_by() {
    let items = vec!["hello", "HELLO", "world", "WORLD", "hi"];
    let unique = unique_by(&items, |w| w.to_lowercase());
    assert_eq!(unique, vec!["hello", "world", "hi"]);
}

#[test]
fn test_unique_by_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(unique_by(&empty, |x| *x), Vec::<i32>::new());
}

#[test]
fn test_window_map() {
    let sums = window_map(&[1, 2, 3, 4, 5], 3, |w| w.iter().sum::<i32>());
    assert_eq!(sums, vec![6, 9, 12]);
}

#[test]
fn test_window_map_pairs() {
    let diffs = window_map(&[10, 20, 15, 30], 2, |w| w[1] - w[0]);
    assert_eq!(diffs, vec![10, -5, 15]);
}

// =====================================================================
// Topic 5: Callbacks & Strategies
// =====================================================================

#[test]
fn test_validator_pass() {
    let mut v = Validator::new();
    v.add_rule(|x: &i32| {
        if *x > 0 {
            Ok(())
        } else {
            Err("must be positive".to_string())
        }
    });
    assert!(v.validate(&5).is_ok());
}

#[test]
fn test_validator_fail() {
    let mut v = Validator::new();
    v.add_rule(|x: &i32| {
        if *x > 0 {
            Ok(())
        } else {
            Err("must be positive".to_string())
        }
    });
    v.add_rule(|x: &i32| {
        if *x < 100 {
            Ok(())
        } else {
            Err("must be under 100".to_string())
        }
    });
    let result = v.validate(&-5);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().len(), 1);
    assert_eq!(v.rule_count(), 2);
}

#[test]
fn test_validator_multiple_errors() {
    let mut v = Validator::new();
    v.add_rule(|x: &i32| {
        if *x > 0 {
            Ok(())
        } else {
            Err("must be positive".into())
        }
    });
    v.add_rule(|x: &i32| {
        if *x % 2 == 0 {
            Ok(())
        } else {
            Err("must be even".into())
        }
    });
    let result = v.validate(&-3);
    assert_eq!(result.unwrap_err().len(), 2);
}

#[test]
fn test_validator_first_error() {
    let mut v = Validator::new();
    v.add_rule(|x: &i32| {
        if *x > 0 {
            Ok(())
        } else {
            Err("not positive".into())
        }
    });
    v.add_rule(|x: &i32| {
        if *x < 100 {
            Ok(())
        } else {
            Err("too large".into())
        }
    });
    assert!(v.validate_first_error(&5).is_ok());
    assert_eq!(v.validate_first_error(&-1), Err("not positive".to_string()));
}

#[test]
fn test_validator_empty_rules() {
    let v = Validator::<i32>::new();
    assert!(v.validate(&42).is_ok());
    assert_eq!(v.rule_count(), 0);
}

#[test]
fn test_pipeline() {
    let mut p = Pipeline::new();
    p.add_step(|x: i32| x + 1);
    p.add_step(|x| x * 2);
    p.add_step(|x| x - 3);
    assert_eq!(p.execute(5), 9); // ((5+1)*2)-3 = 9
    assert_eq!(p.step_count(), 3);
}

#[test]
fn test_pipeline_string() {
    let mut p = Pipeline::new();
    p.add_step(|s: String| s.trim().to_string());
    p.add_step(|s| s.to_uppercase());
    assert_eq!(p.execute("  hello  ".to_string()), "HELLO");
}

#[test]
fn test_pipeline_conditional_step() {
    let mut p = Pipeline::new();
    p.add_step(|x: i32| x + 1);
    // Only double if value is even
    p.add_conditional_step(|x| x % 2 == 0, |x| x * 2);
    // 5 + 1 = 6 (even) -> 12
    assert_eq!(p.execute(5), 12);
    // 4 + 1 = 5 (odd) -> 5 (skipped)
    assert_eq!(p.execute(4), 5);
}

#[test]
fn test_pipeline_no_steps() {
    let p = Pipeline::<i32>::new();
    assert_eq!(p.execute(42), 42);
    assert_eq!(p.step_count(), 0);
}

// =====================================================================
// Topic 6: Memoization & Lazy Evaluation
// =====================================================================

#[test]
fn test_memoize() {
    let mut memo = Memoize::new(|x: i32| x * x);
    assert_eq!(memo.call(5), 25);
    assert_eq!(memo.call(5), 25); // cached
    assert_eq!(memo.call(3), 9);
    assert_eq!(memo.cache_size(), 2);
}

#[test]
fn test_memoize_contains() {
    let mut memo = Memoize::new(|x: i32| x + 1);
    assert!(!memo.contains(&5));
    memo.call(5);
    assert!(memo.contains(&5));
    assert!(!memo.contains(&10));
}

#[test]
fn test_memoize_clear() {
    let mut memo = Memoize::new(|x: i32| x + 1);
    memo.call(1);
    memo.call(2);
    assert_eq!(memo.cache_size(), 2);
    memo.clear_cache();
    assert_eq!(memo.cache_size(), 0);
    assert!(!memo.contains(&1));
}

#[test]
fn test_memoize_strings() {
    let mut memo = Memoize::new(|s: String| s.len());
    assert_eq!(memo.call("hello".to_string()), 5);
    assert_eq!(memo.call("hi".to_string()), 2);
    assert_eq!(memo.cache_size(), 2);
}

#[test]
fn test_lazy() {
    let mut lazy = Lazy::new(|| 42);
    assert!(!lazy.is_initialized());
    assert_eq!(*lazy.get(), 42);
    assert!(lazy.is_initialized());
    assert_eq!(*lazy.get(), 42); // same value
}

#[test]
fn test_lazy_expensive() {
    let mut lazy = Lazy::new(|| "computed".to_string());
    assert!(!lazy.is_initialized());
    assert_eq!(lazy.get(), "computed");
    assert!(lazy.is_initialized());
}

#[test]
fn test_lazy_vec() {
    let mut lazy = Lazy::new(|| vec![1, 2, 3]);
    assert!(!lazy.is_initialized());
    assert_eq!(lazy.get().len(), 3);
    assert!(lazy.is_initialized());
}

// =====================================================================
// Topic 7: Closure-based Generators
// =====================================================================

#[test]
fn test_generate() {
    assert_eq!(generate(5, |i| i * i), vec![0, 1, 4, 9, 16]);
    assert_eq!(generate(0, |i| i), Vec::<usize>::new());
}

#[test]
fn test_repeat_with_fn() {
    let mut n = 0;
    let result = repeat_with_fn(4, || {
        n += 1;
        n
    });
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_repeat_with_fn_constant() {
    assert_eq!(repeat_with_fn(3, || 42), vec![42, 42, 42]);
}

#[test]
fn test_iterate() {
    assert_eq!(iterate(1, |x| x * 2, 5), vec![1, 2, 4, 8, 16]);
    assert_eq!(iterate(1, |x| x + 1, 0), Vec::<i32>::new());
}

#[test]
fn test_iterate_fibonacci() {
    // Using iterate with a tuple to get Fibonacci-like sequence
    let fibs = iterate((0, 1), |&(a, b)| (b, a + b), 6);
    let fib_vals: Vec<i32> = fibs.iter().map(|&(a, _)| a).collect();
    assert_eq!(fib_vals, vec![0, 1, 1, 2, 3, 5]);
}

#[test]
fn test_unfold() {
    // Count down from 5
    let countdown = unfold(5, |n| {
        if n > 0 {
            Some((n, n - 1))
        } else {
            None
        }
    }, 10);
    assert_eq!(countdown, vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_unfold_limited() {
    // Infinite-like sequence, limited by max
    let ones = unfold((), |_| Some((1, ())), 4);
    assert_eq!(ones, vec![1, 1, 1, 1]);
}

#[test]
fn test_from_fn_vec() {
    let mut count = 0;
    let result = from_fn_vec(
        || {
            count += 1;
            if count <= 3 {
                Some(count * 10)
            } else {
                None
            }
        },
        10,
    );
    assert_eq!(result, vec![10, 20, 30]);
}

#[test]
fn test_from_fn_vec_max_reached() {
    let result = from_fn_vec(|| Some(1), 3);
    assert_eq!(result, vec![1, 1, 1]);
}

#[test]
fn test_successors_vec() {
    let powers = successors_vec(1, |x| Some(x * 2), 5);
    assert_eq!(powers, vec![1, 2, 4, 8, 16]);
}

#[test]
fn test_successors_vec_terminating() {
    let halves = successors_vec(16, |x| if *x > 1 { Some(x / 2) } else { None }, 10);
    assert_eq!(halves, vec![16, 8, 4, 2, 1]);
}

#[test]
fn test_take_until() {
    assert_eq!(take_until(&[1, 2, 3, 4, 5], |x| *x > 3), vec![1, 2, 3]);
    assert_eq!(
        take_until(&[10, 20, 30], |x| *x > 100),
        vec![10, 20, 30]
    );
}

#[test]
fn test_take_until_immediate() {
    assert_eq!(take_until(&[1, 2, 3], |x| *x == 1), Vec::<i32>::new());
}

// =====================================================================
// Integration / Cross-topic tests
// =====================================================================

#[test]
fn test_compose_with_pipeline() {
    // Compose functions, then use in pipeline
    let mut p = Pipeline::new();
    p.add_step(|x: i32| x + 10);
    p.add_step(|x| x * 2);
    assert_eq!(p.execute(5), 30); // (5+10)*2
}

#[test]
fn test_filter_then_reduce() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let evens = filter_vec(&items, |x| x % 2 == 0);
    let sum = reduce(&evens, 0, |acc, x| acc + x);
    assert_eq!(sum, 12);
}

#[test]
fn test_generate_then_group() {
    let nums = generate(10, |i| i as i32);
    let groups = group_by(&nums, |x| if x % 2 == 0 { "even" } else { "odd" });
    assert_eq!(groups["even"].len(), 5); // 0,2,4,6,8
    assert_eq!(groups["odd"].len(), 5); // 1,3,5,7,9
}

#[test]
fn test_memoize_with_map() {
    let mut memo = Memoize::new(|x: i32| x * x);
    let inputs = vec![1, 2, 3, 2, 1];
    let results: Vec<i32> = inputs.iter().map(|x| memo.call(*x)).collect();
    assert_eq!(results, vec![1, 4, 9, 4, 1]);
    assert_eq!(memo.cache_size(), 3); // only 3 unique inputs
}

#[test]
fn test_validator_with_closures() {
    let min = 1;
    let max = 100;
    let mut v = Validator::new();
    v.add_rule(move |x: &i32| {
        if *x >= min {
            Ok(())
        } else {
            Err(format!("must be >= {}", min))
        }
    });
    v.add_rule(move |x: &i32| {
        if *x <= max {
            Ok(())
        } else {
            Err(format!("must be <= {}", max))
        }
    });
    assert!(v.validate(&50).is_ok());
    assert!(v.validate(&0).is_err());
    assert!(v.validate(&101).is_err());
}
