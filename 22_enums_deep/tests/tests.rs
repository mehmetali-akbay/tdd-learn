use enums_deep::*;
use std::borrow::Cow;

// ============================================
// Topic 1: Slice Basics
// ============================================

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
fn test_sum_range_start_equals_end() {
    assert_eq!(sum_range(&[1, 2, 3], 1, 1), 0);
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
fn test_windows_of_zero() {
    let r: Vec<Vec<i32>> = vec![];
    assert_eq!(windows_of(&[1, 2], 0), r);
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
fn test_divide_into_chunks() {
    assert_eq!(
        divide_into_chunks(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
}

#[test]
fn test_divide_into_chunks_zero() {
    let empty: Vec<Vec<i32>> = vec![];
    assert_eq!(divide_into_chunks(&[1], 0), empty);
}

#[test]
fn test_middle_elements_odd() {
    assert_eq!(middle_elements(&[1, 2, 3, 4, 5]), Some(vec![3]));
}

#[test]
fn test_middle_elements_even() {
    assert_eq!(middle_elements(&[1, 2, 3, 4]), Some(vec![2, 3]));
}

#[test]
fn test_middle_elements_single() {
    assert_eq!(middle_elements(&[42]), Some(vec![42]));
}

#[test]
fn test_middle_elements_empty() {
    assert_eq!(middle_elements(&[]), None);
}

#[test]
fn test_rotate_left() {
    assert_eq!(rotate_left(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
}

#[test]
fn test_rotate_left_zero() {
    assert_eq!(rotate_left(&[1, 2, 3], 0), vec![1, 2, 3]);
}

#[test]
fn test_rotate_left_full() {
    assert_eq!(rotate_left(&[1, 2, 3], 3), vec![1, 2, 3]);
}

#[test]
fn test_rotate_left_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(rotate_left(&[], 2), empty);
}

#[test]
fn test_is_sorted_true() {
    assert!(is_sorted(&[1, 2, 3, 4, 5]));
}

#[test]
fn test_is_sorted_false() {
    assert!(!is_sorted(&[1, 3, 2]));
}

#[test]
fn test_is_sorted_empty() {
    assert!(is_sorted(&[]));
}

#[test]
fn test_is_sorted_equal() {
    assert!(is_sorted(&[3, 3, 3]));
}

#[test]
fn test_dedup_consecutive() {
    assert_eq!(dedup_consecutive(&[1, 1, 2, 3, 3, 3, 2, 2]), vec![1, 2, 3, 2]);
}

#[test]
fn test_dedup_consecutive_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(dedup_consecutive(&[]), empty);
}

#[test]
fn test_dedup_consecutive_no_dups() {
    assert_eq!(dedup_consecutive(&[1, 2, 3]), vec![1, 2, 3]);
}

// ============================================
// Topic 2: Option Combinators
// ============================================

#[test]
fn test_parse_and_double_valid() {
    assert_eq!(parse_and_double("21"), Some(42));
}

#[test]
fn test_parse_and_double_invalid() {
    assert_eq!(parse_and_double("abc"), None);
}

#[test]
fn test_parse_or_default() {
    assert_eq!(parse_or_default("42", 0), 42);
    assert_eq!(parse_or_default("abc", 99), 99);
}

#[test]
fn test_filter_even_some_even() {
    assert_eq!(filter_even(Some(4)), Some(4));
}

#[test]
fn test_filter_even_some_odd() {
    assert_eq!(filter_even(Some(3)), None);
}

#[test]
fn test_filter_even_none() {
    assert_eq!(filter_even(None), None);
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
fn test_add_options_both_none() {
    assert_eq!(add_options(None, None), None);
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
fn test_lookup_age_missing_id() {
    let ids = vec![(1, "Alice")];
    let ages = vec![("Alice", 30)];
    assert_eq!(lookup_age(&ids, &ages, 99), None);
}

#[test]
fn test_option_multiply_nonzero() {
    assert_eq!(option_multiply_nonzero(Some(5), 3), Some(15));
    assert_eq!(option_multiply_nonzero(Some(5), 0), None);
    assert_eq!(option_multiply_nonzero(None, 3), None);
}

#[test]
fn test_option_max() {
    assert_eq!(option_max(Some(3), Some(5)), Some(5));
    assert_eq!(option_max(Some(3), None), Some(3));
    assert_eq!(option_max(None, Some(7)), Some(7));
    assert_eq!(option_max(None, None), None);
}

#[test]
fn test_option_to_owned() {
    assert_eq!(option_to_owned(Some("hello")), Some("hello".to_string()));
    assert_eq!(option_to_owned(None), None);
}

// ============================================
// Topic 3: if let / let...else / matches!
// ============================================

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
fn test_is_some_positive_zero() {
    assert!(!is_some_positive(Some(0)));
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
fn test_is_admin_false_role() {
    let user = User::LoggedIn {
        username: "bob".into(),
        role: "user".into(),
    };
    assert!(!is_admin(&user));
}

#[test]
fn test_is_admin_anonymous() {
    assert!(!is_admin(&User::Anonymous));
}

#[test]
fn test_process_user_logged_in() {
    let u = User::LoggedIn {
        username: "alice".into(),
        role: "admin".into(),
    };
    assert_eq!(process_user(&u), "User alice has role admin");
}

#[test]
fn test_process_user_anonymous() {
    assert_eq!(process_user(&User::Anonymous), "Guest user");
}

#[test]
fn test_is_in_range() {
    assert!(is_in_range(5, 1, 10));
    assert!(is_in_range(1, 1, 10));
    assert!(is_in_range(10, 1, 10));
    assert!(!is_in_range(0, 1, 10));
    assert!(!is_in_range(11, 1, 10));
}

#[test]
fn test_char_type_letter() {
    assert_eq!(char_type('a'), "letter");
    assert_eq!(char_type('Z'), "letter");
}

#[test]
fn test_char_type_digit() {
    assert_eq!(char_type('5'), "digit");
}

#[test]
fn test_char_type_other() {
    assert_eq!(char_type('!'), "other");
    assert_eq!(char_type(' '), "other");
}

// ============================================
// Topic 4: Nested Enums & Flattening
// ============================================

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
fn test_collect_somes_all_none() {
    let items = vec![None, None, None];
    let empty: Vec<i32> = vec![];
    assert_eq!(collect_somes(&items), empty);
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
fn test_first_parseable_found() {
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
fn test_parse_positive_zero() {
    assert_eq!(parse_positive("0"), Ok(None));
}

#[test]
fn test_parse_positive_invalid() {
    assert!(parse_positive("abc").is_err());
}

#[test]
fn test_sum_parseable() {
    assert_eq!(sum_parseable(&["1", "abc", "3", "def", "5"]), 9);
}

#[test]
fn test_sum_parseable_none() {
    assert_eq!(sum_parseable(&["abc", "def"]), 0);
}

#[test]
fn test_partition_options() {
    let items = vec![Some(1), None, Some(3), None, None, Some(5)];
    let (somes, nones) = partition_options(&items);
    assert_eq!(somes, vec![1, 3, 5]);
    assert_eq!(nones, 3);
}

#[test]
fn test_partition_options_all_some() {
    let items = vec![Some(1), Some(2)];
    let (somes, nones) = partition_options(&items);
    assert_eq!(somes, vec![1, 2]);
    assert_eq!(nones, 0);
}

#[test]
fn test_parse_double_positive() {
    assert_eq!(parse_double_positive("5"), Some(10));
    assert_eq!(parse_double_positive("-3"), None); // -6 not positive
    assert_eq!(parse_double_positive("abc"), None);
    assert_eq!(parse_double_positive("0"), None); // 0 not positive
}

// ============================================
// Topic 5: Cow<T>
// ============================================

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
    assert!(matches!(r, Cow::Owned(_)));
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
fn test_ensure_suffix_already_has() {
    let r = ensure_suffix("file.rs", ".rs");
    assert!(matches!(r, Cow::Borrowed(_)));
}

#[test]
fn test_ensure_suffix_needs_one() {
    let r = ensure_suffix("file", ".rs");
    assert!(matches!(r, Cow::Owned(_)));
    assert_eq!(r, "file.rs");
}

#[test]
fn test_replace_cow_has_match() {
    let r = replace_cow("hello world", "world", "rust");
    assert!(matches!(r, Cow::Owned(_)));
    assert_eq!(r, "hello rust");
}

#[test]
fn test_replace_cow_no_match() {
    let r = replace_cow("hello", "xyz", "abc");
    assert!(matches!(r, Cow::Borrowed(_)));
}

#[test]
fn test_truncate_cow_short_enough() {
    let r = truncate_cow("hi", 10);
    assert!(matches!(r, Cow::Borrowed(_)));
    assert_eq!(r, "hi");
}

#[test]
fn test_truncate_cow_needs_truncation() {
    let r = truncate_cow("hello world", 5);
    assert!(matches!(r, Cow::Owned(_)));
    assert_eq!(r, "hello");
}

// ============================================
// Topic 6: Enum Dispatch — Shape
// ============================================

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
fn test_describe_circle() {
    assert_eq!(Shape::Circle(1.0).describe(), "Circle with radius 1");
}

#[test]
fn test_describe_rectangle() {
    assert_eq!(Shape::Rectangle(3.0, 4.0).describe(), "Rectangle 3x4");
}

#[test]
fn test_perimeter_circle() {
    let p = Shape::Circle(1.0).perimeter();
    assert!((p - 2.0 * std::f64::consts::PI).abs() < 1e-6);
}

#[test]
fn test_perimeter_rectangle() {
    assert!((Shape::Rectangle(3.0, 4.0).perimeter() - 14.0).abs() < 1e-6);
}

#[test]
fn test_is_circle() {
    assert!(Shape::Circle(1.0).is_circle());
    assert!(!Shape::Rectangle(1.0, 2.0).is_circle());
}

#[test]
fn test_scale_shape() {
    let s = Shape::Rectangle(2.0, 3.0).scale(2.0);
    assert_eq!(s, Shape::Rectangle(4.0, 6.0));
}

#[test]
fn test_scale_circle() {
    let s = Shape::Circle(5.0).scale(0.5);
    assert_eq!(s, Shape::Circle(2.5));
}

#[test]
fn test_total_area() {
    let shapes = vec![Shape::Rectangle(2.0, 3.0), Shape::Rectangle(4.0, 5.0)];
    assert!((total_area(&shapes) - 26.0).abs() < 1e-6);
}

#[test]
fn test_total_area_empty() {
    assert_eq!(total_area(&[]), 0.0);
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
fn test_count_circles() {
    let shapes = vec![
        Shape::Circle(1.0),
        Shape::Rectangle(2.0, 3.0),
        Shape::Circle(5.0),
    ];
    assert_eq!(count_circles(&shapes), 2);
}

#[test]
fn test_count_circles_none() {
    let shapes = vec![Shape::Rectangle(2.0, 3.0)];
    assert_eq!(count_circles(&shapes), 0);
}

#[test]
fn test_sort_by_area() {
    let shapes = vec![
        Shape::Rectangle(10.0, 10.0), // 100
        Shape::Rectangle(1.0, 1.0),   // 1
        Shape::Rectangle(5.0, 5.0),   // 25
    ];
    let sorted = sort_by_area(&shapes);
    assert!((sorted[0].area() - 1.0).abs() < 1e-6);
    assert!((sorted[1].area() - 25.0).abs() < 1e-6);
    assert!((sorted[2].area() - 100.0).abs() < 1e-6);
}

// ============================================
// Topic 7: Rich Enums with Data — Expr
// ============================================

#[test]
fn test_expr_num() {
    assert!((Expr::num(42.0).eval() - 42.0).abs() < 1e-10);
}

#[test]
fn test_expr_add() {
    let e = Expr::add(Expr::num(2.0), Expr::num(3.0));
    assert!((e.eval() - 5.0).abs() < 1e-10);
}

#[test]
fn test_expr_mul() {
    let e = Expr::mul(Expr::num(4.0), Expr::num(5.0));
    assert!((e.eval() - 20.0).abs() < 1e-10);
}

#[test]
fn test_expr_neg() {
    let e = Expr::neg(Expr::num(7.0));
    assert!((e.eval() - (-7.0)).abs() < 1e-10);
}

#[test]
fn test_expr_complex() {
    // (2 + 3) * -4 = -20
    let e = Expr::mul(
        Expr::add(Expr::num(2.0), Expr::num(3.0)),
        Expr::neg(Expr::num(4.0)),
    );
    assert!((e.eval() - (-20.0)).abs() < 1e-10);
}

#[test]
fn test_expr_node_count() {
    let e = Expr::add(Expr::num(1.0), Expr::mul(Expr::num(2.0), Expr::num(3.0)));
    assert_eq!(e.node_count(), 5); // add, num, mul, num, num
}

#[test]
fn test_expr_depth() {
    let e = Expr::num(1.0);
    assert_eq!(e.depth(), 1);
    let e2 = Expr::add(Expr::num(1.0), Expr::num(2.0));
    assert_eq!(e2.depth(), 2);
    let e3 = Expr::add(Expr::num(1.0), Expr::neg(Expr::num(2.0)));
    assert_eq!(e3.depth(), 3);
}

#[test]
fn test_expr_has_negation() {
    let e = Expr::add(Expr::num(1.0), Expr::num(2.0));
    assert!(!e.has_negation());
    let e2 = Expr::neg(Expr::num(1.0));
    assert!(e2.has_negation());
    let e3 = Expr::add(Expr::num(1.0), Expr::neg(Expr::num(2.0)));
    assert!(e3.has_negation());
}

// ============================================
// Topic 8: State Machines — TrafficLight & OrderStatus
// ============================================

#[test]
fn test_traffic_light_next() {
    assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
    assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
    assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
}

#[test]
fn test_traffic_light_duration() {
    assert_eq!(TrafficLight::Red.duration(), 60);
    assert_eq!(TrafficLight::Yellow.duration(), 5);
    assert_eq!(TrafficLight::Green.duration(), 45);
}

#[test]
fn test_traffic_light_can_go() {
    assert!(!TrafficLight::Red.can_go());
    assert!(!TrafficLight::Yellow.can_go());
    assert!(TrafficLight::Green.can_go());
}

#[test]
fn test_traffic_light_color() {
    assert_eq!(TrafficLight::Red.color(), "red");
    assert_eq!(TrafficLight::Yellow.color(), "yellow");
    assert_eq!(TrafficLight::Green.color(), "green");
}

#[test]
fn test_advance_n() {
    // Red -> Green -> Yellow -> Red (3 steps = full cycle)
    assert_eq!(advance_n(&TrafficLight::Red, 3), TrafficLight::Red);
    assert_eq!(advance_n(&TrafficLight::Red, 1), TrafficLight::Green);
    assert_eq!(advance_n(&TrafficLight::Red, 0), TrafficLight::Red);
}

#[test]
fn test_order_status_display() {
    assert_eq!(OrderStatus::Pending.display(), "Pending");
    assert_eq!(OrderStatus::Confirmed.display(), "Confirmed");
    assert_eq!(
        OrderStatus::Shipped {
            tracking: "XYZ123".into()
        }
        .display(),
        "Shipped (XYZ123)"
    );
    assert_eq!(OrderStatus::Delivered.display(), "Delivered");
    assert_eq!(
        OrderStatus::Cancelled {
            reason: "changed mind".into()
        }
        .display(),
        "Cancelled: changed mind"
    );
}

#[test]
fn test_order_can_cancel() {
    assert!(OrderStatus::Pending.can_cancel());
    assert!(OrderStatus::Confirmed.can_cancel());
    assert!(!OrderStatus::Shipped {
        tracking: "X".into()
    }
    .can_cancel());
    assert!(!OrderStatus::Delivered.can_cancel());
}

#[test]
fn test_order_is_final() {
    assert!(!OrderStatus::Pending.is_final());
    assert!(!OrderStatus::Confirmed.is_final());
    assert!(OrderStatus::Delivered.is_final());
    assert!(OrderStatus::Cancelled {
        reason: "x".into()
    }
    .is_final());
}

#[test]
fn test_order_full_workflow() {
    let order = OrderStatus::Pending;
    let order = order.confirm();
    assert_eq!(order, OrderStatus::Confirmed);
    let order = order.ship("TRACK123");
    assert!(matches!(order, OrderStatus::Shipped { .. }));
    let order = order.deliver();
    assert_eq!(order, OrderStatus::Delivered);
}

#[test]
fn test_order_cancel_workflow() {
    let order = OrderStatus::Pending;
    let order = order.cancel("no longer needed");
    assert!(matches!(order, OrderStatus::Cancelled { .. }));
}

#[test]
fn test_order_invalid_transition() {
    // Can't ship from Pending
    let order = OrderStatus::Pending.ship("X");
    assert_eq!(order, OrderStatus::Pending);
    // Can't deliver from Confirmed
    let order = OrderStatus::Confirmed.deliver();
    assert_eq!(order, OrderStatus::Confirmed);
    // Can't cancel from Delivered
    let order = OrderStatus::Delivered.cancel("too late");
    assert_eq!(order, OrderStatus::Delivered);
}

// ============================================
// Topic 9: Result Combinators
// ============================================

#[test]
fn test_parse_and_add_success() {
    assert_eq!(parse_and_add("3", "4"), Ok(7));
}

#[test]
fn test_parse_and_add_first_err() {
    assert!(parse_and_add("abc", "4").is_err());
}

#[test]
fn test_parse_and_add_second_err() {
    assert!(parse_and_add("3", "xyz").is_err());
}

#[test]
fn test_safe_divide_ok() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
}

#[test]
fn test_safe_divide_by_zero() {
    assert!(safe_divide(1.0, 0.0).is_err());
}

#[test]
fn test_parse_validate_double_success() {
    assert_eq!(parse_validate_double("5"), Ok(10));
}

#[test]
fn test_parse_validate_double_not_positive() {
    assert_eq!(
        parse_validate_double("-3"),
        Err("not positive".to_string())
    );
}

#[test]
fn test_parse_validate_double_invalid() {
    assert!(parse_validate_double("abc").is_err());
}

#[test]
fn test_parse_all_success() {
    assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
}

#[test]
fn test_parse_all_with_errors() {
    let result = parse_all(&["1", "abc", "3", "def"]);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
}

#[test]
fn test_parse_with_fallback_primary() {
    assert_eq!(parse_with_fallback("42", "99"), Ok(42));
}

#[test]
fn test_parse_with_fallback_secondary() {
    assert_eq!(parse_with_fallback("abc", "99"), Ok(99));
}

#[test]
fn test_parse_with_fallback_both_fail() {
    assert!(parse_with_fallback("abc", "xyz").is_err());
}

#[test]
fn test_transform_result_ok() {
    assert_eq!(transform_result(Ok(42)), Ok("ok:42".to_string()));
}

#[test]
fn test_transform_result_err() {
    assert_eq!(transform_result(Err(5)), Err("err:5".to_string()));
}

// ============================================
// Topic 10: Enum-based Error Types
// ============================================

#[test]
fn test_app_parse_success() {
    assert_eq!(app_parse("42"), Ok(42));
}

#[test]
fn test_app_parse_failure() {
    assert!(matches!(app_parse("abc"), Err(AppError::ParseError(_))));
}

#[test]
fn test_app_parse_in_range_valid() {
    assert_eq!(app_parse_in_range("5", 1, 10), Ok(5));
}

#[test]
fn test_app_parse_in_range_out_of_range() {
    assert!(matches!(
        app_parse_in_range("20", 1, 10),
        Err(AppError::ValidationError(_))
    ));
}

#[test]
fn test_app_parse_in_range_parse_error() {
    assert!(matches!(
        app_parse_in_range("abc", 1, 10),
        Err(AppError::ParseError(_))
    ));
}

#[test]
fn test_app_lookup_found() {
    let items = vec![("a", 1), ("b", 2)];
    assert_eq!(app_lookup(&items, "b"), Ok(2));
}

#[test]
fn test_app_lookup_not_found() {
    let items = vec![("a", 1)];
    assert!(matches!(
        app_lookup(&items, "z"),
        Err(AppError::NotFound(_))
    ));
}

#[test]
fn test_error_kind() {
    assert_eq!(error_kind(&AppError::ParseError("x".into())), "parse");
    assert_eq!(
        error_kind(&AppError::ValidationError("x".into())),
        "validation"
    );
    assert_eq!(error_kind(&AppError::NotFound("x".into())), "not_found");
}

#[test]
fn test_is_recoverable() {
    assert!(is_recoverable(&AppError::ParseError("x".into())));
    assert!(is_recoverable(&AppError::ValidationError("x".into())));
    assert!(!is_recoverable(&AppError::NotFound("x".into())));
}

#[test]
fn test_app_error_display() {
    let e = AppError::ParseError("bad input".into());
    assert_eq!(format!("{}", e), "Parse error: bad input");
    let e = AppError::NotFound("key".into());
    assert_eq!(format!("{}", e), "Not found: key");
}
