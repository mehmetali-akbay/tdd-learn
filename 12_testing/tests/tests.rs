use testing_patterns::*;

// =====================================================================
// Topic 1: Basic Assertions — assert!, assert_eq!, assert_ne!, messages
// =====================================================================

#[test]
fn test_add_positive() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add_negative() {
    assert_eq!(add(-1, -1), -2);
}

#[test]
fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(10, 4), 6);
    assert_eq!(subtract(3, 5), -2);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(3, 7), 21);
    assert_eq!(multiply(-2, 5), -10);
    assert_eq!(multiply(0, 999), 0);
}

#[test]
fn test_is_even() {
    assert!(is_even(4), "4 should be even");
    assert!(is_even(0), "0 should be even");
    assert!(is_even(-2), "negative even numbers count too");
    assert!(!is_even(3));
}

#[test]
fn test_is_odd() {
    assert!(is_odd(3));
    assert!(is_odd(-1));
    assert!(!is_odd(0));
}

#[test]
fn test_max_of() {
    assert_eq!(max_of(3, 5), 5);
    assert_eq!(max_of(5, 3), 5);
    assert_eq!(max_of(3, 3), 3);
}

#[test]
fn test_min_of() {
    assert_eq!(min_of(3, 5), 3);
    assert_eq!(min_of(-1, -5), -5);
    assert_eq!(min_of(7, 7), 7);
}

#[test]
fn test_abs_diff() {
    assert_eq!(abs_diff(10, 3), 7);
    assert_eq!(abs_diff(3, 10), 7);
    assert_eq!(abs_diff(5, 5), 0);
}

#[test]
fn test_clamp() {
    assert_eq!(clamp(5, 1, 10), 5);
    assert_eq!(clamp(-1, 0, 10), 0);
    assert_eq!(clamp(15, 0, 10), 10);
}

#[test]
fn test_clamp_at_boundary() {
    assert_eq!(clamp(0, 0, 10), 0, "at lower boundary");
    assert_eq!(clamp(10, 0, 10), 10, "at upper boundary");
}

#[test]
fn test_greet_contains_name() {
    let result = greet("Alice");
    assert!(
        result.contains("Alice"),
        "Greeting should contain the name, got: {result}"
    );
}

#[test]
fn test_greet_not_empty() {
    assert_ne!(greet("Bob"), "", "Greeting should not be empty");
}

// =====================================================================
// Topic 2: #[should_panic] & Result-Returning Tests
// =====================================================================

#[test]
fn test_divide_normal() {
    assert!((divide(10.0, 3.0) - 3.333_333_333_3).abs() < 1e-6);
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_divide_by_zero_panics() {
    divide(1.0, 0.0);
}

#[test]
fn test_get_element_valid() {
    assert_eq!(get_element(&[10, 20, 30], 1), 20);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_get_element_out_of_bounds() {
    get_element(&[1, 2, 3], 5);
}

#[test]
fn test_parse_positive_valid() -> Result<(), String> {
    let r = parse_positive("42")?;
    assert_eq!(r, 42);
    Ok(())
}

#[test]
fn test_parse_positive_invalid() {
    assert!(parse_positive("abc").is_err());
}

#[test]
fn test_parse_positive_negative() {
    let err = parse_positive("-5").unwrap_err();
    assert!(
        err.contains("negative"),
        "Error should mention 'negative', got: {err}"
    );
}

#[test]
fn test_validate_email_valid() -> Result<(), String> {
    validate_email("test@example.com")?;
    Ok(())
}

#[test]
fn test_validate_email_missing_at() {
    let err = validate_email("not-an-email.com").unwrap_err();
    assert!(err.contains("@"), "Error should mention @, got: {err}");
}

#[test]
fn test_validate_email_missing_dot() {
    let err = validate_email("user@localhost").unwrap_err();
    assert!(err.contains("dot"), "Error should mention dot, got: {err}");
}

#[test]
fn test_safe_divide_ok() -> Result<(), String> {
    let result = safe_divide(10.0, 4.0)?;
    assert_eq!(result, 2.5);
    Ok(())
}

#[test]
fn test_safe_divide_zero() {
    assert!(safe_divide(1.0, 0.0).is_err());
}

#[test]
fn test_checked_sqrt_ok() -> Result<(), String> {
    let result = checked_sqrt(25.0)?;
    assert!((result - 5.0).abs() < 1e-10);
    Ok(())
}

#[test]
fn test_checked_sqrt_negative() {
    assert!(checked_sqrt(-1.0).is_err());
}

// =====================================================================
// Topic 3: Struct Testing with Helpers & Fixtures
// =====================================================================

/// Helper: create a user with custom active status (test fixture pattern).
fn make_user(name: &str, age: u32, active: bool) -> User {
    let mut u = User::new(name, age);
    if !active {
        u.deactivate();
    }
    u
}

/// Helper: create a standard set of test users.
fn sample_users() -> Vec<User> {
    vec![
        make_user("Alice", 30, true),
        make_user("Bob", 15, true),
        make_user("Charlie", 25, false),
        make_user("Diana", 20, true),
    ]
}

#[test]
fn test_user_creation() {
    let u = User::new("Alice", 30);
    assert_eq!(u.name, "Alice");
    assert_eq!(u.age, 30);
    assert!(u.active);
}

#[test]
fn test_user_is_adult() {
    assert!(User::new("Alice", 18).is_adult());
    assert!(User::new("Bob", 50).is_adult());
}

#[test]
fn test_user_is_not_adult() {
    assert!(!User::new("Kid", 17).is_adult());
    assert!(!User::new("Baby", 0).is_adult());
}

#[test]
fn test_user_display_name() {
    let u = User::new("Alice", 30);
    let d = u.display_name();
    assert!(d.contains("Alice"));
    assert!(d.contains("30"));
}

#[test]
fn test_user_deactivate() {
    let mut u = User::new("Alice", 30);
    assert!(u.active);
    u.deactivate();
    assert!(!u.active);
}

#[test]
fn test_user_display_trait() {
    let u = User::new("Alice", 30);
    let s = u.to_string();
    assert!(s.contains("Alice"));
    assert!(s.contains("active"));
}

#[test]
fn test_user_clone_eq() {
    let u = User::new("Alice", 30);
    let u2 = u.clone();
    assert_eq!(u, u2);
}

#[test]
fn test_active_adults() {
    let users = sample_users();
    let adults = active_adults(&users);
    // Alice (30, active) and Diana (20, active) qualify; Bob (15) too young, Charlie (25) inactive
    assert_eq!(adults.len(), 2);
    assert!(adults.iter().any(|u| u.name == "Alice"));
    assert!(adults.iter().any(|u| u.name == "Diana"));
}

#[test]
fn test_find_user_case_insensitive() {
    let users = sample_users();
    assert!(find_user(&users, "alice").is_some());
    assert!(find_user(&users, "ALICE").is_some());
    assert_eq!(find_user(&users, "alice").unwrap().age, 30);
}

#[test]
fn test_find_user_missing() {
    let users = sample_users();
    assert!(find_user(&users, "Eve").is_none());
}

#[test]
fn test_oldest_user() {
    let users = sample_users();
    let oldest = oldest_user(&users).unwrap();
    assert_eq!(oldest.name, "Alice");
    assert_eq!(oldest.age, 30);
}

#[test]
fn test_user_summary() {
    let users = sample_users();
    let (active, inactive) = user_summary(&users);
    assert_eq!(active, 3);
    assert_eq!(inactive, 1);
}

// =====================================================================
// Topic 4: Expensive Computations — #[ignore] Tests
// =====================================================================

#[test]
fn test_fib_base_cases() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
}

#[test]
fn test_fib_small() {
    assert_eq!(fib(10), 55);
}

#[test]
#[ignore] // slow recursive implementation
fn test_fib_large() {
    assert_eq!(fib(30), 832040);
}

#[test]
fn test_is_prime_small() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(7));
    assert!(!is_prime(9));
}

#[test]
fn test_primes_up_to() {
    assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
    assert_eq!(primes_up_to(1), vec![]);
}

#[test]
#[ignore] // larger sieve
fn test_primes_up_to_large() {
    assert_eq!(primes_up_to(1000).len(), 168);
}

#[test]
fn test_nth_prime() {
    assert_eq!(nth_prime(0), 2);
    assert_eq!(nth_prime(1), 3);
    assert_eq!(nth_prime(4), 11);
}

#[test]
fn test_collatz_steps() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(2), 1); // 2 → 1
    assert_eq!(collatz_steps(6), 8); // 6→3→10→5→16→8→4→2→1
}

// =====================================================================
// Topic 5: Data Structures — Boundary & State Testing
// =====================================================================

#[test]
fn test_stack_new_is_empty() {
    let s: Stack<i32> = Stack::new(5);
    assert!(s.is_empty());
    assert!(!s.is_full());
    assert_eq!(s.len(), 0);
}

#[test]
fn test_stack_push_pop_lifo() {
    let mut s = Stack::new(3);
    s.push(1).unwrap();
    s.push(2).unwrap();
    s.push(3).unwrap();
    assert_eq!(s.pop(), Some(3)); // LIFO
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn test_stack_peek_does_not_remove() {
    let mut s = Stack::new(3);
    assert_eq!(s.peek(), None);
    s.push(42).unwrap();
    assert_eq!(s.peek(), Some(&42));
    assert_eq!(s.len(), 1); // peek didn't remove
}

#[test]
fn test_stack_overflow() {
    let mut s = Stack::new(2);
    assert!(s.push(1).is_ok());
    assert!(s.push(2).is_ok());
    assert!(s.push(3).is_err()); // capacity exceeded
    assert!(s.is_full());
}

#[test]
fn test_stack_empty_pop() {
    let mut s: Stack<i32> = Stack::new(5);
    assert_eq!(s.pop(), None);
}

#[test]
fn test_stack_full_boundary_transition() {
    let mut s: Stack<i32> = Stack::new(1);
    assert!(s.is_empty());
    s.push(1).unwrap();
    assert!(!s.is_empty());
    assert!(s.is_full());
    s.pop();
    assert!(s.is_empty());
    assert!(!s.is_full());
}

#[test]
fn test_stack_clear() {
    let mut s = Stack::new(5);
    s.push(1).unwrap();
    s.push(2).unwrap();
    s.clear();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
}

#[test]
fn test_stack_into_vec() {
    let mut s = Stack::new(5);
    s.push(10).unwrap();
    s.push(20).unwrap();
    assert_eq!(s.into_vec(), vec![10, 20]);
}

#[test]
fn test_sort_is_sorted() {
    let sorted = sort_vec(vec![3, 1, 4, 1, 5, 9]);
    for w in sorted.windows(2) {
        assert!(w[0] <= w[1], "not sorted: {} > {}", w[0], w[1]);
    }
}

#[test]
fn test_sort_preserves_length() {
    let input = vec![5, 3, 1, 4, 2];
    assert_eq!(sort_vec(input.clone()).len(), input.len());
}

#[test]
fn test_sort_preserves_elements() {
    let input = vec![3, 1, 2];
    let mut sorted = sort_vec(input.clone());
    let mut original = input;
    sorted.sort();
    original.sort();
    assert_eq!(sorted, original);
}

#[test]
fn test_sort_empty() {
    assert_eq!(sort_vec(vec![]), Vec::<i32>::new());
}

#[test]
fn test_sort_idempotent() {
    let input = vec![5, 3, 1, 4];
    let once = sort_vec(input);
    let twice = sort_vec(once.clone());
    assert_eq!(once, twice, "sorting twice should give the same result");
}

#[test]
fn test_unique_preserves_order() {
    assert_eq!(unique(&[3, 1, 2, 1, 3]), vec![3, 1, 2]);
}

#[test]
fn test_unique_already_unique() {
    assert_eq!(unique(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_unique_all_same() {
    assert_eq!(unique(&[7, 7, 7]), vec![7]);
}

#[test]
fn test_chunk_vec() {
    assert_eq!(
        chunk_vec(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
}

#[test]
fn test_chunk_vec_empty() {
    assert!(chunk_vec(&[], 3).is_empty());
}

#[test]
fn test_flatten() {
    let nested = vec![vec![1, 2], vec![3], vec![4, 5]];
    assert_eq!(flatten(&nested), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_flatten_empty() {
    let nested: Vec<Vec<i32>> = vec![vec![], vec![]];
    assert!(flatten(&nested).is_empty());
}

#[test]
fn test_chunk_flatten_roundtrip() {
    // chunk then flatten should give back the original
    let original = vec![1, 2, 3, 4, 5];
    let chunked = chunk_vec(&original, 2);
    assert_eq!(flatten(&chunked), original);
}

// =====================================================================
// Topic 6: Roundtrip & Property Testing
// =====================================================================

#[test]
fn test_caesar_encode() {
    assert_eq!(caesar_encode("abc", 1), "bcd");
}

#[test]
fn test_caesar_wrap_around() {
    assert_eq!(caesar_encode("xyz", 3), "abc");
}

#[test]
fn test_caesar_preserves_non_alpha() {
    assert_eq!(caesar_encode("Hello, World!", 5), "Mjqqt, Btwqi!");
}

#[test]
fn test_caesar_roundtrip() {
    let original = "The quick brown Fox!";
    let encoded = caesar_encode(original, 13);
    let decoded = caesar_decode(&encoded, 13);
    assert_eq!(decoded, original, "encode then decode should give original");
}

#[test]
fn test_caesar_shift_26_is_identity() {
    let text = "Hello";
    assert_eq!(
        caesar_encode(text, 26),
        text,
        "shift of 26 should be identity"
    );
}

#[test]
fn test_rle_encode() {
    assert_eq!(run_length_encode("aaabbc"), "3a2b1c");
}

#[test]
fn test_rle_encode_single_chars() {
    assert_eq!(run_length_encode("abc"), "1a1b1c");
}

#[test]
fn test_rle_encode_empty() {
    assert_eq!(run_length_encode(""), "");
}

#[test]
fn test_rle_decode() -> Result<(), String> {
    assert_eq!(run_length_decode("3a2b1c")?, "aaabbc");
    Ok(())
}

#[test]
fn test_rle_roundtrip() -> Result<(), String> {
    let original = "aabbccdddd";
    let encoded = run_length_encode(original);
    let decoded = run_length_decode(&encoded)?;
    assert_eq!(decoded, original, "encode→decode should be identity");
    Ok(())
}

#[test]
fn test_rle_decode_invalid() {
    assert!(run_length_decode("3").is_err(), "trailing digits are invalid");
}

#[test]
fn test_hex_encode() {
    assert_eq!(to_hex_string(&[0xff, 0x00, 0xab]), "ff00ab");
}

#[test]
fn test_hex_decode() -> Result<(), String> {
    assert_eq!(from_hex_string("ff00ab")?, vec![0xff, 0x00, 0xab]);
    Ok(())
}

#[test]
fn test_hex_roundtrip() -> Result<(), String> {
    let original = vec![0xde, 0xad, 0xbe, 0xef];
    let hex = to_hex_string(&original);
    let decoded = from_hex_string(&hex)?;
    assert_eq!(decoded, original);
    Ok(())
}

#[test]
fn test_hex_empty() -> Result<(), String> {
    assert_eq!(to_hex_string(&[]), "");
    assert_eq!(from_hex_string("")?, vec![]);
    Ok(())
}

#[test]
fn test_hex_invalid_odd() {
    assert!(from_hex_string("abc").is_err());
}

#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("abc"), "cba");
}

#[test]
fn test_reverse_string_roundtrip() {
    let original = "hello world";
    assert_eq!(
        reverse_string(&reverse_string(original)),
        original,
        "double reverse should be identity"
    );
}

// =====================================================================
// Topic 7: Validation & Trait Testing — Parameterized Style
// =====================================================================

#[test]
fn test_classify_number_parameterized() {
    // Table-driven test: (input, expected_output)
    let cases = [
        (42, "positive"),
        (-7, "negative"),
        (0, "zero"),
        (1, "positive"),
        (-1, "negative"),
    ];
    for (input, expected) in &cases {
        assert_eq!(
            classify_number(*input),
            *expected,
            "classify_number({input}) should be {expected}"
        );
    }
}

#[test]
fn test_fizzbuzz_parameterized() {
    // Table-driven test
    let cases = [
        (1, "1"),
        (3, "Fizz"),
        (5, "Buzz"),
        (15, "FizzBuzz"),
        (7, "7"),
        (30, "FizzBuzz"),
        (9, "Fizz"),
        (10, "Buzz"),
    ];
    for (input, expected) in &cases {
        assert_eq!(
            fizzbuzz(*input),
            *expected,
            "fizzbuzz({input}) should be {expected}"
        );
    }
}

#[test]
fn test_range_validator_in_range() {
    let v = RangeValidator { min: 1, max: 100 };
    assert!(v.validate(&50).is_ok());
    assert!(v.validate(&1).is_ok()); // boundary
    assert!(v.validate(&100).is_ok()); // boundary
}

#[test]
fn test_range_validator_out_of_range() {
    let v = RangeValidator { min: 1, max: 100 };
    assert!(v.validate(&0).is_err());
    assert!(v.validate(&101).is_err());
}

#[test]
fn test_range_validator_error_message() {
    let v = RangeValidator { min: 0, max: 10 };
    let err = v.validate(&20).unwrap_err();
    assert!(err.contains("20"), "error should contain the value");
    assert!(err.contains("out of range"), "error should describe the issue");
}

#[test]
fn test_length_validator_ok() {
    let v = LengthValidator {
        min_len: 3,
        max_len: 10,
    };
    assert!(v.validate(&"hello".to_string()).is_ok());
}

#[test]
fn test_length_validator_too_short() {
    let v = LengthValidator {
        min_len: 3,
        max_len: 10,
    };
    let err = v.validate(&"ab".to_string()).unwrap_err();
    assert!(err.contains("short"), "error should say 'short', got: {err}");
}

#[test]
fn test_length_validator_too_long() {
    let v = LengthValidator {
        min_len: 1,
        max_len: 5,
    };
    let err = v.validate(&"toolongstring".to_string()).unwrap_err();
    assert!(err.contains("long"), "error should say 'long', got: {err}");
}

#[test]
fn test_validate_all_pass() {
    let v1 = RangeValidator { min: 0, max: 100 };
    let v2 = RangeValidator { min: -50, max: 50 };
    let errors = validate_all(&25, &[&v1, &v2]);
    assert!(errors.is_empty(), "all validators should pass");
}

#[test]
fn test_validate_all_some_fail() {
    let v1 = RangeValidator { min: 0, max: 100 };
    let v2 = RangeValidator { min: 0, max: 10 };
    let errors = validate_all(&50, &[&v1, &v2]);
    assert_eq!(errors.len(), 1, "one validator should fail");
}

#[test]
fn test_validate_all_all_fail() {
    let v1 = RangeValidator { min: 0, max: 5 };
    let v2 = RangeValidator { min: 10, max: 20 };
    let errors = validate_all(&7, &[&v1, &v2]);
    assert_eq!(errors.len(), 2, "both validators should fail");
}