use enums_deep::*;
use std::borrow::Cow;

#[test]
fn test_first_and_last_normal() {
    assert_eq!(first_and_last(&[1, 2, 3]), Some((1, 3)));
}
#[test]
fn test_first_and_last_single() {
    assert_eq!(first_and_last(&[42]), Some((42, 42)));
}
#[test]
fn test_first_and_last_empty() {
    assert_eq!(first_and_last(&[]), None);
}
#[test]
fn test_sum_range_normal() {
    assert_eq!(sum_range(&[10, 20, 30, 40, 50], 1, 4), 90);
}
#[test]
fn test_sum_range_out_of_bounds() {
    assert_eq!(sum_range(&[1, 2, 3], 0, 10), 0);
}
#[test]
fn test_windows_of() {
    assert_eq!(
        windows_of(&[1, 2, 3, 4], 2),
        vec![vec![1, 2], vec![2, 3], vec![3, 4]]
    );
}
#[test]
fn test_windows_of_larger_than_slice() {
    let r: Vec<Vec<i32>> = vec![];
    assert_eq!(windows_of(&[1, 2], 5), r);
}
#[test]
fn test_split_first_word() {
    assert_eq!(
        split_first_word("hello world foo"),
        Some(("hello", "world foo"))
    );
}
#[test]
fn test_split_first_word_single() {
    assert_eq!(split_first_word("hello"), Some(("hello", "")));
}
#[test]
fn test_split_first_word_empty() {
    assert_eq!(split_first_word(""), None);
}
#[test]
fn test_contains_subslice_true() {
    assert!(contains_subslice(&[1, 2, 3, 4, 5], &[2, 3, 4]));
}
#[test]
fn test_contains_subslice_false() {
    assert!(!contains_subslice(&[1, 2, 3], &[2, 4]));
}
#[test]
fn test_contains_subslice_empty_needle() {
    assert!(contains_subslice(&[1, 2, 3], &[]));
}

#[test]
fn test_parse_and_double_valid() {
    assert_eq!(parse_and_double("21"), Some(42));
}
#[test]
fn test_parse_and_double_invalid() {
    assert_eq!(parse_and_double("abc"), None);
}
#[test]
fn test_add_options_both_some() {
    assert_eq!(add_options(Some(3), Some(4)), Some(7));
}
#[test]
fn test_add_options_one_none() {
    assert_eq!(add_options(Some(3), None), None);
}
#[test]
fn test_first_even_squared() {
    assert_eq!(first_even_squared(&[1, 3, 4, 6]), Some(16));
}
#[test]
fn test_first_even_squared_none() {
    assert_eq!(first_even_squared(&[1, 3, 5]), None);
}
#[test]
fn test_lookup_found() {
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    assert_eq!(lookup(&pairs, "b"), Some(2));
}
#[test]
fn test_lookup_not_found() {
    let pairs = vec![("a", 1)];
    assert_eq!(lookup(&pairs, "z"), None);
}
#[test]
fn test_lookup_age_found() {
    let ids = vec![(1, "Alice"), (2, "Bob")];
    let ages = vec![("Alice", 30), ("Bob", 25)];
    assert_eq!(lookup_age(&ids, &ages, 1), Some(30));
}
#[test]
fn test_lookup_age_missing_name() {
    let ids = vec![(1, "Alice")];
    let ages = vec![("Bob", 25)];
    assert_eq!(lookup_age(&ids, &ages, 1), None);
}

#[test]
fn test_describe_option_some() {
    assert_eq!(describe_option(Some(42)), "Value: 42");
}
#[test]
fn test_describe_option_none() {
    assert_eq!(describe_option(None), "No value");
}
#[test]
fn test_is_some_positive_yes() {
    assert!(is_some_positive(Some(5)));
}
#[test]
fn test_is_some_positive_negative() {
    assert!(!is_some_positive(Some(-1)));
}
#[test]
fn test_is_some_positive_none() {
    assert!(!is_some_positive(None));
}
#[test]
fn test_classify_result_ok() {
    assert_eq!(classify_result(Ok(42)), "ok:42");
}
#[test]
fn test_classify_result_err() {
    assert_eq!(classify_result(Err("oops".into())), "err:oops");
}
#[test]
fn test_get_username_logged_in() {
    let user = User::LoggedIn {
        username: "alice".into(),
        role: "admin".into(),
    };
    assert_eq!(get_username(&user), "alice");
}
#[test]
fn test_get_username_anonymous() {
    assert_eq!(get_username(&User::Anonymous), "anonymous");
}
#[test]
fn test_is_admin_true() {
    let user = User::LoggedIn {
        username: "alice".into(),
        role: "admin".into(),
    };
    assert!(is_admin(&user));
}
#[test]
fn test_is_admin_false() {
    let user = User::LoggedIn {
        username: "bob".into(),
        role: "user".into(),
    };
    assert!(!is_admin(&user));
}

#[test]
fn test_flatten_option_some_some() {
    assert_eq!(flatten_option(Some(Some(42))), Some(42));
}
#[test]
fn test_flatten_option_some_none() {
    assert_eq!(flatten_option(Some(None::<i32>)), None);
}
#[test]
fn test_flatten_option_none() {
    assert_eq!(flatten_option(None::<Option<i32>>), None);
}
#[test]
fn test_collect_somes() {
    assert_eq!(
        collect_somes(&[Some(1), None, Some(3), None, Some(5)]),
        vec![1, 3, 5]
    );
}
#[test]
fn test_transpose_some_ok() {
    assert_eq!(
        transpose_option_result::<i32, String>(Some(Ok(42))),
        Ok(Some(42))
    );
}
#[test]
fn test_transpose_some_err() {
    let input: Option<Result<i32, String>> = Some(Err("fail".into()));
    assert_eq!(transpose_option_result(input), Err("fail".into()));
}
#[test]
fn test_transpose_none() {
    let input: Option<Result<i32, String>> = None;
    assert_eq!(transpose_option_result(input), Ok(None));
}
#[test]
fn test_first_parseable() {
    assert_eq!(first_parseable(&["abc", "42", "100"]), Some(42));
}
#[test]
fn test_first_parseable_none() {
    assert_eq!(first_parseable(&["abc", "def"]), None);
}
#[test]
fn test_parse_positive_positive() {
    assert_eq!(parse_positive("42"), Ok(Some(42)));
}
#[test]
fn test_parse_positive_negative() {
    assert_eq!(parse_positive("-5"), Ok(None));
}
#[test]
fn test_parse_positive_invalid() {
    assert!(parse_positive("abc").is_err());
}

#[test]
fn test_to_lowercase_cow_already_lower() {
    let r = to_lowercase_cow("hello");
    assert!(matches!(r, Cow::Borrowed(_)));
    assert_eq!(r, "hello");
}
#[test]
fn test_to_lowercase_cow_has_upper() {
    let r = to_lowercase_cow("Hello");
    assert!(matches!(r, Cow::Owned(_)));
    assert_eq!(r, "hello");
}
#[test]
fn test_remove_spaces_cow_no_spaces() {
    let r = remove_spaces_cow("hello");
    assert!(matches!(r, Cow::Borrowed(_)));
}
#[test]
fn test_remove_spaces_cow_has_spaces() {
    let r = remove_spaces_cow("he llo");
    assert!(matches!(r, Cow::Owned(_)));
    assert_eq!(r, "hello");
}
#[test]
fn test_ensure_period_already_has() {
    let r = ensure_period("hello.");
    assert!(matches!(r, Cow::Borrowed(_)));
}
#[test]
fn test_ensure_period_needs_one() {
    let r = ensure_period("hello");
    assert!(matches!(r, Cow::Owned(_)));
    assert_eq!(r, "hello.");
}
#[test]
fn test_trim_cow_no_whitespace() {
    let r = trim_cow("hello");
    assert!(matches!(r, Cow::Borrowed(_)));
}
#[test]
fn test_trim_cow_with_whitespace() {
    let r = trim_cow("  hello  ");
    assert_eq!(r, "hello");
}
#[test]
fn test_ensure_prefix_already_has() {
    let r = ensure_prefix("https://example.com", "https://");
    assert!(matches!(r, Cow::Borrowed(_)));
}
#[test]
fn test_ensure_prefix_needs_one() {
    let r = ensure_prefix("example.com", "https://");
    assert_eq!(r, "https://example.com");
}

#[test]
fn test_circle_area() {
    let c = Shape::Circle(5.0);
    assert!((c.area() - 78.53981633974483).abs() < 1e-6);
}
#[test]
fn test_rectangle_area() {
    assert!((Shape::Rectangle(3.0, 4.0).area() - 12.0).abs() < 1e-6);
}
#[test]
fn test_triangle_area() {
    assert!((Shape::Triangle(6.0, 4.0).area() - 12.0).abs() < 1e-6);
}
#[test]
fn test_describe_shape() {
    assert_eq!(Shape::Circle(1.0).describe(), "Circle with radius 1");
}
#[test]
fn test_total_area() {
    let shapes = vec![Shape::Rectangle(2.0, 3.0), Shape::Rectangle(4.0, 5.0)];
    assert!((total_area(&shapes) - 26.0).abs() < 1e-6);
}
#[test]
fn test_filter_by_min_area() {
    let shapes = vec![
        Shape::Circle(1.0),
        Shape::Rectangle(10.0, 10.0),
        Shape::Triangle(2.0, 1.0),
    ];
    let big = filter_by_min_area(&shapes, 5.0);
    assert_eq!(big.len(), 1);
}
#[test]
fn test_largest_shape() {
    let shapes = vec![Shape::Circle(1.0), Shape::Rectangle(10.0, 10.0)];
    assert!(matches!(
        largest_shape(&shapes),
        Some(Shape::Rectangle(_, _))
    ));
}
#[test]
fn test_largest_shape_empty() {
    let shapes: Vec<Shape> = vec![];
    assert!(largest_shape(&shapes).is_none());
}

#[test]
fn test_divide_into_chunks() {
    assert_eq!(divide_into_chunks(&[1, 2, 3, 4, 5], 2), vec![vec![1, 2], vec![3, 4], vec![5]]);
    let empty: Vec<Vec<i32>> = vec![];
    assert_eq!(divide_into_chunks(&[1], 0), empty);
}

#[test]
fn test_parse_or_default() {
    assert_eq!(parse_or_default("42", 0), 42);
    assert_eq!(parse_or_default("abc", 99), 99);
}

#[test]
fn test_filter_even() {
    assert_eq!(filter_even(Some(4)), Some(4));
    assert_eq!(filter_even(Some(3)), None);
    assert_eq!(filter_even(None), None);
}

#[test]
fn test_process_user() {
    let u1 = User::LoggedIn { username: "alice".into(), role: "admin".into() };
    assert_eq!(process_user(&u1), "User alice has role admin");
    assert_eq!(process_user(&User::Anonymous), "Guest user");
}
