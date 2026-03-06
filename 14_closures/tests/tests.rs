use closures::*;

// ===== Topic 1: Closure Basics =====

#[test]
fn test_map_vec() {
    assert_eq!(map_vec(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
    assert_eq!(map_vec(&["hi", "bye"], |s| s.len()), vec![2, 3]);
}

#[test]
fn test_filter_vec() {
    assert_eq!(filter_vec(&[1, 2, 3, 4, 5], |x| x % 2 == 0), vec![2, 4]);
}

#[test]
fn test_reduce() {
    assert_eq!(reduce(&[1, 2, 3, 4], 0, |acc, x| acc + x), 10);
    assert_eq!(reduce(&[1, 2, 3], 1, |acc, x| acc * x), 6);
}

#[test]
fn test_apply_n() {
    assert_eq!(apply_n(1, 4, |x| x * 2), 16); // 1 -> 2 -> 4 -> 8 -> 16
    assert_eq!(apply_n(0, 0, |x| x + 1), 0); // zero times
}

#[test]
fn test_any_of_all_of() {
    assert!(any_of(&[1, 2, 3], |x| *x > 2));
    assert!(!any_of(&[1, 2, 3], |x| *x > 5));
    assert!(all_of(&[2, 4, 6], |x| x % 2 == 0));
    assert!(!all_of(&[2, 3, 6], |x| x % 2 == 0));
}

// ===== Topic 2: Fn vs FnMut vs FnOnce =====

#[test]
fn test_count_matching() {
    assert_eq!(count_matching(&[1, 2, 3, 4, 5], |x| x % 2 == 0), 2);
}

#[test]
fn test_for_each() {
    let mut sum = 0;
    for_each(&[1, 2, 3], |x| sum += x);
    assert_eq!(sum, 6);
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
fn test_unwrap_or_else() {
    assert_eq!(unwrap_or_else(Some(42), || 0), 42);
    assert_eq!(unwrap_or_else(None, || 99), 99);
}

// ===== Topic 3: Returning Closures =====

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
}

#[test]
fn test_make_threshold() {
    let is_adult = make_threshold(18);
    assert!(is_adult(21));
    assert!(!is_adult(15));
}

#[test]
fn test_make_repeater() {
    let triple = make_repeater(3);
    assert_eq!(triple("ha"), "hahaha");
}

#[test]
fn test_compose() {
    let add1_then_double = compose(|x: i32| x * 2, |x: i32| x + 1);
    assert_eq!(add1_then_double(3), 8); // (3+1)*2
}

#[test]
fn test_negate() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = negate(is_even);
    assert!(is_odd(&3));
    assert!(!is_odd(&4));
}

// ===== Topic 4: Higher-Order Combinators =====

#[test]
fn test_chain() {
    let add1_then_double = chain(|x: i32| x + 1, |x| x * 2);
    assert_eq!(add1_then_double(3), 8); // (3+1)*2
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
fn test_partition() {
    let (evens, odds) = partition(&[1, 2, 3, 4, 5], |x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4]);
    assert_eq!(odds, vec![1, 3, 5]);
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
fn test_max_by_key() {
    let words = vec!["hi", "hello", "hey"];
    assert_eq!(max_by_key(&words, |w| w.len()), Some(&"hello"));
}

// ===== Topic 5: Validator & Pipeline =====

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

// ===== Topic 6: Memoize & Lazy =====

#[test]
fn test_memoize() {
    let mut memo = Memoize::new(|x: i32| x * x);
    assert_eq!(memo.call(5), 25);
    assert_eq!(memo.call(5), 25); // cached
    assert_eq!(memo.call(3), 9);
    assert_eq!(memo.cache_size(), 2);
}

#[test]
fn test_memoize_clear() {
    let mut memo = Memoize::new(|x: i32| x + 1);
    memo.call(1);
    memo.call(2);
    assert_eq!(memo.cache_size(), 2);
    memo.clear_cache();
    assert_eq!(memo.cache_size(), 0);
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

// ===== Edge Cases =====

#[test]
fn test_map_vec_empty() {
    assert_eq!(map_vec::<i32, i32>(&[], |x| x * 2), Vec::<i32>::new());
}

#[test]
fn test_filter_vec_none_match() {
    assert_eq!(filter_vec(&[1, 3, 5], |x| x % 2 == 0), Vec::<i32>::new());
}

#[test]
fn test_reduce_empty() {
    assert_eq!(reduce::<i32>(&[], 0, |acc, x| acc + x), 0);
}

#[test]
fn test_any_of_empty() {
    assert!(!any_of::<i32>(&[], |_| true));
    assert!(all_of::<i32>(&[], |_| false)); // vacuously true
}

#[test]
fn test_count_matching_none() {
    assert_eq!(count_matching(&[1, 3, 5], |x| x % 2 == 0), 0);
}

#[test]
fn test_max_by_key_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(max_by_key(&empty, |x| *x), None);
}

#[test]
fn test_validator_empty_rules() {
    let v = Validator::<i32>::new();
    assert!(v.validate(&42).is_ok());
    assert_eq!(v.rule_count(), 0);
}

#[test]
fn test_pipeline_no_steps() {
    let p = Pipeline::<i32>::new();
    assert_eq!(p.execute(42), 42);
    assert_eq!(p.step_count(), 0);
}
