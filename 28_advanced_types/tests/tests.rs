use advanced_types::*;

#[test]
fn test_find_item_found() {
    assert_eq!(find_item(&[("apple", 3), ("banana", 5)], "apple"), Ok(3));
}
#[test]
fn test_find_item_not_found() {
    assert!(matches!(
        find_item(&[("apple", 3)], "grape"),
        Err(AppError::NotFound(_))
    ));
}
#[test]
fn test_parse_age_valid() {
    assert_eq!(parse_age("25"), Ok(25));
}
#[test]
fn test_parse_age_invalid() {
    assert!(matches!(parse_age("abc"), Err(AppError::InvalidInput(_))));
}
#[test]
fn test_authorized_ok() {
    assert!(authorized_action(true, "delete").is_ok());
}
#[test]
fn test_authorized_fail() {
    assert_eq!(
        authorized_action(false, "delete"),
        Err(AppError::Unauthorized)
    );
}
#[test]
fn test_app_error_display() {
    assert!(format!("{}", AppError::NotFound("item".into())).contains("item"));
}
#[test]
fn test_non_empty_string_valid() {
    let s = NonEmptyString::new("hello").unwrap();
    assert_eq!(s.as_str(), "hello");
    assert_eq!(s.len(), 5);
}
#[test]
fn test_non_empty_string_empty() {
    assert!(NonEmptyString::new("").is_err());
}
#[test]
fn test_percentage_valid() {
    let p = Percentage::new(75.0).unwrap();
    assert!((p.value() - 75.0).abs() < 1e-6);
    assert!((p.as_ratio() - 0.75).abs() < 1e-6);
}
#[test]
fn test_percentage_invalid() {
    assert!(Percentage::new(101.0).is_err());
    assert!(Percentage::new(-1.0).is_err());
}
#[test]
fn test_port_valid() {
    let p = Port::new(8080).unwrap();
    assert_eq!(p.value(), 8080);
    assert!(!p.is_privileged());
}
#[test]
fn test_port_privileged() {
    assert!(Port::new(80).unwrap().is_privileged());
}
#[test]
fn test_port_invalid() {
    assert!(Port::new(0).is_err());
}
#[test]
fn test_parse_or_die_valid() {
    assert_eq!(parse_or_die("42"), 42);
}
#[test]
#[should_panic]
fn test_parse_or_die_invalid() {
    parse_or_die("nope");
}
#[test]
fn test_unwrap_or_die_ok() {
    assert_eq!(unwrap_or_die(Ok::<i32, String>(42)), 42);
}
#[test]
#[should_panic]
fn test_unwrap_or_die_err() {
    unwrap_or_die(Err::<i32, String>("fail".into()));
}
#[test]
fn test_parse_all_or_skip() {
    assert_eq!(
        parse_all_or_skip(&["1", "abc", "3", "def", "5"]),
        vec![1, 3, 5]
    );
}
#[test]
fn test_print_it_str() {
    assert_eq!(print_it("hello"), "hello");
}
#[test]
fn test_print_it_number() {
    assert_eq!(print_it(&42), "42");
}
#[test]
fn test_str_len() {
    assert_eq!(str_len("hello"), 5);
    assert_eq!(str_len(""), 0);
}
#[test]
fn test_ref_display() {
    assert_eq!(Ref::new("hello").display(), "hello");
}
#[test]
fn test_ref_display_number() {
    let v = 42;
    assert_eq!(Ref::new(&v).display(), "42");
}
#[test]
fn test_map_with_fn() {
    fn double(x: i32) -> i32 {
        x * 2
    }
    assert_eq!(map_with_fn(&[1, 2, 3], double), vec![2, 4, 6]);
}
#[test]
fn test_get_operation_add() {
    assert_eq!(get_operation("add").unwrap()(3, 4), 7);
}
#[test]
fn test_get_operation_mul() {
    assert_eq!(get_operation("mul").unwrap()(3, 4), 12);
}
#[test]
fn test_get_operation_unknown() {
    assert!(get_operation("pow").is_none());
}
#[test]
fn test_apply_pipeline() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn double(x: i32) -> i32 {
        x * 2
    }
    assert_eq!(apply_pipeline(5, &[add_one, double]), 12);
}
#[test]
fn test_transform_strings() {
    fn to_upper(s: &str) -> String {
        s.to_uppercase()
    }
    assert_eq!(
        transform_strings(&["hello", "world"], to_upper),
        vec!["HELLO", "WORLD"]
    );
}
#[test]
fn test_builder_complete() {
    let r = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .build();
    assert_eq!(r.url, "https://example.com");
    assert_eq!(r.method, "GET");
    assert_eq!(r.body, None);
}
#[test]
fn test_builder_with_body() {
    let r = RequestBuilder::new()
        .method("POST")
        .url("https://api.test")
        .body("data")
        .build();
    assert_eq!(r.method, "POST");
    assert_eq!(r.body, Some("data".into()));
}
#[test]
fn test_builder_order_independent() {
    let r1 = RequestBuilder::new()
        .url("https://test.com")
        .method("GET")
        .build();
    let r2 = RequestBuilder::new()
        .method("GET")
        .url("https://test.com")
        .build();
    assert_eq!(r1, r2);
}
