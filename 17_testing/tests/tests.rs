use testing_patterns::*;

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
fn test_is_even() {
    assert!(is_even(4));
    assert!(!is_even(3));
    assert!(is_even(0));
}
#[test]
fn test_max_of() {
    assert_eq!(max_of(3, 5), 5);
    assert_eq!(max_of(5, 3), 5);
    assert_eq!(max_of(3, 3), 3);
}
#[test]
fn test_greet() {
    let r = greet("Alice");
    assert!(r.contains("Alice"));
    assert_ne!(r, "");
}
#[test]
fn test_clamp() {
    assert_eq!(clamp(5, 1, 10), 5);
    assert_eq!(clamp(-1, 0, 10), 0);
    assert_eq!(clamp(15, 0, 10), 10);
}
#[test]
fn test_divide_normal() {
    assert!((divide(10.0, 3.0) - 3.3333333333).abs() < 1e-6);
}
#[test]
#[should_panic(expected = "zero")]
fn test_divide_by_zero() {
    divide(1.0, 0.0);
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
    assert!(parse_positive("-5").is_err());
}
#[test]
#[should_panic(expected = "out of bounds")]
fn test_get_element_panic() {
    get_element(&[1, 2, 3], 5);
}
#[test]
fn test_get_element_valid() {
    assert_eq!(get_element(&[10, 20, 30], 1), 20);
}
#[test]
fn test_validate_email_valid() -> Result<(), String> {
    validate_email("test@example.com")?;
    Ok(())
}
#[test]
fn test_validate_email_invalid() {
    assert!(validate_email("not-an-email").is_err());
}

fn make_user(name: &str, age: u32, active: bool) -> User {
    let mut u = User::new(name, age);
    u.active = active;
    u
}
#[test]
fn test_user_creation() {
    let u = User::new("Alice", 30);
    assert_eq!(u.name, "Alice");
    assert_eq!(u.age, 30);
    assert!(u.active);
}
#[test]
fn test_is_adult() {
    assert!(User::new("Alice", 18).is_adult());
    assert!(!User::new("Bob", 17).is_adult());
}
#[test]
fn test_display_name() {
    let u = User::new("Alice", 30);
    let d = u.display_name();
    assert!(d.contains("Alice"));
    assert!(d.contains("30"));
}
#[test]
fn test_active_adults() {
    let users = vec![
        make_user("Alice", 30, true),
        make_user("Bob", 15, true),
        make_user("Charlie", 25, false),
        make_user("Diana", 20, true),
    ];
    assert_eq!(active_adults(&users).len(), 2);
}
#[test]
fn test_find_user() {
    let users = vec![User::new("Alice", 30), User::new("Bob", 25)];
    assert!(find_user(&users, "alice").is_some());
    assert!(find_user(&users, "ALICE").is_some());
    assert!(find_user(&users, "Charlie").is_none());
}
#[test]
fn test_fib_small() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(10), 55);
}
#[test]
#[ignore]
fn test_fib_large() {
    assert_eq!(fib(30), 832040);
}
#[test]
fn test_is_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(7));
    assert!(!is_prime(9));
}
#[test]
fn test_primes_up_to() {
    assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
}
#[test]
#[ignore]
fn test_primes_up_to_large() {
    assert_eq!(primes_up_to(1000).len(), 168);
}
#[test]
fn test_stack_new() {
    let s: Stack<i32> = Stack::new(5);
    assert!(s.is_empty());
    assert!(!s.is_full());
    assert_eq!(s.len(), 0);
}
#[test]
fn test_stack_push_pop() {
    let mut s = Stack::new(3);
    assert!(s.push(1).is_ok());
    assert!(s.push(2).is_ok());
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}
#[test]
fn test_stack_peek() {
    let mut s = Stack::new(3);
    assert_eq!(s.peek(), None);
    s.push(42).unwrap();
    assert_eq!(s.peek(), Some(&42));
    assert_eq!(s.len(), 1);
}
#[test]
fn test_stack_capacity() {
    let mut s = Stack::new(2);
    assert!(s.push(1).is_ok());
    assert!(s.push(2).is_ok());
    assert!(s.push(3).is_err());
    assert!(s.is_full());
}
#[test]
fn test_stack_empty_boundary() {
    let mut s: Stack<i32> = Stack::new(1);
    assert!(s.is_empty());
    s.push(1).unwrap();
    assert!(!s.is_empty());
    assert!(s.is_full());
    s.pop();
    assert!(s.is_empty());
}
#[test]
fn test_sort_is_sorted() {
    let sorted = sort_vec(vec![3, 1, 4, 1, 5, 9]);
    for w in sorted.windows(2) {
        assert!(w[0] <= w[1]);
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
fn test_unique_preserves_order() {
    assert_eq!(unique(&[3, 1, 2, 1, 3]), vec![3, 1, 2]);
}
#[test]
fn test_unique_already_unique() {
    assert_eq!(unique(&[1, 2, 3]), vec![1, 2, 3]);
}
#[test]
fn test_reverse_string_roundtrip() {
    let o = "hello world";
    assert_eq!(reverse_string(&reverse_string(o)), o);
}
#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("abc"), "cba");
}
#[test]
fn test_caesar_roundtrip() {
    let o = "hello";
    assert_eq!(caesar_decode(&caesar_encode(o, 3), 3), o);
}
#[test]
fn test_caesar_encode() {
    assert_eq!(caesar_encode("abc", 1), "bcd");
}
#[test]
fn test_caesar_wrap_around() {
    assert_eq!(caesar_encode("xyz", 3), "abc");
}
