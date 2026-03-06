use results::*;

// ===== Topic 1: Result Basics =====

#[test]
fn test_divide_ok() {
    assert_eq!(divide(10, 2), Ok(5));
    assert_eq!(divide(7, 3), Ok(2)); // integer division
    assert_eq!(divide(-10, 2), Ok(-5));
    assert_eq!(divide(0, 5), Ok(0));
}

#[test]
fn test_divide_by_zero() {
    assert_eq!(divide(10, 0), Err("division by zero".to_string()));
    assert_eq!(divide(0, 0), Err("division by zero".to_string()));
}

#[test]
fn test_parse_number_valid() {
    assert_eq!(parse_number("42"), Ok(42));
    assert_eq!(parse_number("-5"), Ok(-5));
    assert_eq!(parse_number("0"), Ok(0));
}

#[test]
fn test_parse_number_invalid() {
    assert!(parse_number("abc").is_err());
    assert!(parse_number("").is_err());
    assert!(parse_number("12.5").is_err());
}

#[test]
fn test_get_at_valid() {
    assert_eq!(get_at(&[10, 20, 30], 0), Ok(10));
    assert_eq!(get_at(&[10, 20, 30], 1), Ok(20));
    assert_eq!(get_at(&[10, 20, 30], 2), Ok(30));
}

#[test]
fn test_get_at_out_of_bounds() {
    assert!(get_at(&[10, 20, 30], 5).is_err());
    assert!(get_at(&[], 0).is_err());
}

#[test]
fn test_double_result() {
    assert_eq!(double_result(Ok(5)), Ok(10));
    assert_eq!(double_result(Ok(-3)), Ok(-6));
    assert_eq!(double_result(Ok(0)), Ok(0));
    assert_eq!(
        double_result(Err("err".to_string())),
        Err("err".to_string())
    );
}

#[test]
fn test_parse_and_double() {
    assert_eq!(parse_and_double("5"), Ok(10));
    assert_eq!(parse_and_double("-4"), Ok(-8));
    assert!(parse_and_double("abc").is_err());
}

#[test]
fn test_unwrap_or_default() {
    assert_eq!(unwrap_or_default(Ok(42), 0), 42);
    assert_eq!(unwrap_or_default(Err("err".to_string()), 99), 99);
}

#[test]
fn test_find_first_even() {
    assert_eq!(find_first_even(&[1, 3, 4, 6]), Ok(4));
    assert_eq!(find_first_even(&[2, 4, 6]), Ok(2));
    assert_eq!(find_first_even(&[-3, -2, 1]), Ok(-2));
    assert!(find_first_even(&[1, 3, 5, 7]).is_err());
    assert!(find_first_even(&[]).is_err());
}

#[test]
fn test_lookup_grade() {
    assert_eq!(lookup_grade(95), Ok('A'));
    assert_eq!(lookup_grade(100), Ok('A'));
    assert_eq!(lookup_grade(90), Ok('A'));
    assert_eq!(lookup_grade(85), Ok('B'));
    assert_eq!(lookup_grade(75), Ok('C'));
    assert_eq!(lookup_grade(65), Ok('D'));
    assert_eq!(lookup_grade(50), Ok('F'));
    assert_eq!(lookup_grade(0), Ok('F'));
    assert!(lookup_grade(101).is_err());
    assert!(lookup_grade(200).is_err());
}

// ===== Topic 2: The ? Operator =====

#[test]
fn test_add_strings() {
    assert_eq!(add_strings("3", "4"), Ok(7));
    assert_eq!(add_strings("-1", "5"), Ok(4));
    assert!(add_strings("abc", "4").is_err());
    assert!(add_strings("3", "xyz").is_err());
}

#[test]
fn test_parse_and_divide() {
    assert_eq!(parse_and_divide("10", "2"), Ok(5));
    assert_eq!(parse_and_divide("10", "3"), Ok(3));
    assert!(parse_and_divide("10", "0").is_err());
    assert!(parse_and_divide("abc", "2").is_err());
}

#[test]
fn test_average_of_csv() {
    assert_eq!(average_of_csv("1,2,3"), Ok(2.0));
    assert_eq!(average_of_csv("10"), Ok(10.0));
    assert_eq!(average_of_csv("1, 2, 3"), Ok(2.0)); // handles whitespace
    assert!(average_of_csv("").is_err());
    assert!(average_of_csv("1,abc,3").is_err());
}

#[test]
fn test_first_positive_from_csv() {
    assert_eq!(first_positive_from_csv("-1,-2,3,4"), Ok(3));
    assert_eq!(first_positive_from_csv("5"), Ok(5));
    assert!(first_positive_from_csv("-1,-2,-3").is_err());
    assert!(first_positive_from_csv("").is_err());
    assert!(first_positive_from_csv("abc").is_err());
}

#[test]
fn test_parse_point() {
    assert_eq!(parse_point("3.5,2.1"), Ok((3.5, 2.1)));
    assert_eq!(parse_point("0,0"), Ok((0.0, 0.0)));
    assert_eq!(parse_point(" 1.5 , -2.5 "), Ok((1.5, -2.5)));
    assert!(parse_point("1,2,3").is_err()); // too many parts
    assert!(parse_point("abc,2").is_err());
    assert!(parse_point("1").is_err()); // too few parts
}

#[test]
fn test_sum_positive_from_csv() {
    assert_eq!(sum_positive_from_csv("1,-2,3,-4,5"), Ok(9));
    assert_eq!(sum_positive_from_csv("-1,-2,-3"), Ok(0));
    assert_eq!(sum_positive_from_csv("10"), Ok(10));
    assert!(sum_positive_from_csv("").is_err());
    assert!(sum_positive_from_csv("1,abc").is_err());
}

#[test]
fn test_parse_int_range() {
    assert_eq!(parse_int_range("1..5"), Ok(vec![1, 2, 3, 4]));
    assert_eq!(parse_int_range("3..3"), Ok(vec![])); // empty range
    assert_eq!(parse_int_range("-2..2"), Ok(vec![-2, -1, 0, 1]));
    assert!(parse_int_range("5..2").is_err()); // start > end
    assert!(parse_int_range("abc..5").is_err());
    assert!(parse_int_range("1..2..3").is_err()); // too many parts
}

// ===== Topic 3: Custom Error Types =====

#[test]
fn test_validation_error_display() {
    assert_eq!(format!("{}", ValidationError::Empty), "cannot be empty");
    assert_eq!(
        format!("{}", ValidationError::TooShort(2)),
        "too short: 2 chars"
    );
    assert_eq!(
        format!("{}", ValidationError::TooLong(25)),
        "too long: 25 chars"
    );
    assert_eq!(
        format!("{}", ValidationError::InvalidChar('@')),
        "invalid character: '@'"
    );
    assert_eq!(
        format!("{}", ValidationError::MissingUppercase),
        "missing uppercase letter"
    );
    assert_eq!(
        format!("{}", ValidationError::MissingLowercase),
        "missing lowercase letter"
    );
    assert_eq!(
        format!("{}", ValidationError::MissingDigit),
        "missing digit"
    );
    assert_eq!(
        format!("{}", ValidationError::InvalidFormat("bad".to_string())),
        "invalid format: bad"
    );
}

#[test]
fn test_validation_error_is_std_error() {
    // ValidationError implements std::error::Error
    let err: Box<dyn std::error::Error> = Box::new(ValidationError::Empty);
    assert_eq!(format!("{}", err), "cannot be empty");
}

#[test]
fn test_validate_username_ok() {
    assert_eq!(validate_username("alice"), Ok(()));
    assert_eq!(validate_username("user_123"), Ok(()));
    assert_eq!(validate_username("abc"), Ok(())); // exactly 3 chars
    assert_eq!(validate_username(&"a".repeat(20)), Ok(())); // exactly 20 chars
}

#[test]
fn test_validate_username_errors() {
    assert_eq!(validate_username(""), Err(ValidationError::Empty));
    assert_eq!(validate_username("ab"), Err(ValidationError::TooShort(2)));
    assert_eq!(
        validate_username(&"a".repeat(21)),
        Err(ValidationError::TooLong(21))
    );
    assert_eq!(
        validate_username("user@name"),
        Err(ValidationError::InvalidChar('@'))
    );
    assert_eq!(
        validate_username("has space"),
        Err(ValidationError::InvalidChar(' '))
    );
}

#[test]
fn test_validate_password_ok() {
    assert_eq!(validate_password("Passw0rd"), Ok(()));
    assert_eq!(validate_password("MyP4ssword"), Ok(()));
    assert_eq!(validate_password("Ab1defgh"), Ok(())); // exactly 8 chars
}

#[test]
fn test_validate_password_errors() {
    assert_eq!(validate_password(""), Err(ValidationError::Empty));
    assert_eq!(validate_password("Aa1"), Err(ValidationError::TooShort(3)));
    assert_eq!(
        validate_password("alllower1"),
        Err(ValidationError::MissingUppercase)
    );
    assert_eq!(
        validate_password("ALLUPPER1"),
        Err(ValidationError::MissingLowercase)
    );
    assert_eq!(
        validate_password("NoDigitsHere"),
        Err(ValidationError::MissingDigit)
    );
}

#[test]
fn test_validate_email_ok() {
    assert_eq!(validate_email("user@example.com"), Ok(()));
    assert_eq!(validate_email("a@b.c"), Ok(()));
    assert_eq!(validate_email("test.name@domain.org"), Ok(()));
}

#[test]
fn test_validate_email_errors() {
    assert!(matches!(validate_email(""), Err(ValidationError::Empty)));
    assert!(matches!(
        validate_email("noatsign.com"),
        Err(ValidationError::InvalidFormat(_))
    ));
    assert!(matches!(
        validate_email("@domain.com"),
        Err(ValidationError::InvalidFormat(_))
    ));
    assert!(matches!(
        validate_email("user@nodot"),
        Err(ValidationError::InvalidFormat(_))
    ));
    assert!(matches!(
        validate_email("a@@b.com"),
        Err(ValidationError::InvalidFormat(_))
    ));
}

#[test]
fn test_validated_name_ok() {
    let name = ValidatedName::new("alice").unwrap();
    assert_eq!(name.value(), "alice");
}

#[test]
fn test_validated_name_preserves_value() {
    let name = ValidatedName::new("user_123").unwrap();
    assert_eq!(name.value(), "user_123");
}

#[test]
fn test_validated_name_errors() {
    assert!(ValidatedName::new("").is_err());
    assert!(ValidatedName::new("ab").is_err());
    assert!(ValidatedName::new("bad@name").is_err());
    assert!(ValidatedName::new(&"x".repeat(21)).is_err());
}

#[test]
fn test_validate_registration_ok() {
    assert_eq!(
        validate_registration("alice", "Passw0rd", "alice@example.com"),
        Ok(())
    );
}

#[test]
fn test_validate_registration_errors() {
    // bad username
    assert!(validate_registration("", "Passw0rd", "a@b.com").is_err());
    // bad password
    assert!(validate_registration("alice", "short", "a@b.com").is_err());
    // bad email
    assert!(validate_registration("alice", "Passw0rd", "bad").is_err());
}

// ===== Topic 4: Error Conversion =====

#[test]
fn test_app_error_display() {
    assert_eq!(
        format!("{}", AppError::NotFound("user".to_string())),
        "user not found"
    );
    assert!(format!("{}", AppError::Parse("bad".to_string())).contains("Parse error"));
    assert!(format!("{}", AppError::Custom("oops".to_string())).contains("oops"));
}

#[test]
fn test_app_error_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(AppError::Custom("oops".to_string()));
    assert_eq!(format!("{}", err), "oops");
}

#[test]
fn test_register_user_ok() {
    assert_eq!(
        register_user("alice", "25"),
        Ok("Welcome alice, age 25".to_string())
    );
}

#[test]
fn test_register_user_bad_username() {
    assert!(matches!(register_user("ab", "25"), Err(AppError::Validation(_))));
}

#[test]
fn test_register_user_bad_age() {
    assert!(matches!(register_user("alice", "abc"), Err(AppError::Parse(_))));
}

#[test]
fn test_parse_config_line() {
    assert_eq!(
        parse_config_line("port=8080"),
        Ok(("port".to_string(), 8080))
    );
    assert_eq!(
        parse_config_line("timeout = 30"),
        Ok(("timeout".to_string(), 30))
    );
    assert!(parse_config_line("no_equals").is_err());
    assert!(parse_config_line("key=abc").is_err());
}

#[test]
fn test_create_user_profile_ok() {
    let profile = create_user_profile("alice", "25", "alice@example.com").unwrap();
    assert_eq!(profile.username, "alice");
    assert_eq!(profile.age, 25);
    assert_eq!(profile.email, "alice@example.com");
}

#[test]
fn test_create_user_profile_errors() {
    // bad username → Validation error
    assert!(matches!(
        create_user_profile("ab", "25", "a@b.com"),
        Err(AppError::Validation(_))
    ));
    // bad age string → Parse error
    assert!(matches!(
        create_user_profile("alice", "abc", "a@b.com"),
        Err(AppError::Parse(_))
    ));
    // bad email → Validation error
    assert!(matches!(
        create_user_profile("alice", "25", "bad"),
        Err(AppError::Validation(_))
    ));
    // negative age → Custom error
    assert!(matches!(
        create_user_profile("alice", "-5", "a@b.com"),
        Err(AppError::Custom(_))
    ));
}

#[test]
fn test_user_profile_summary() {
    let profile = create_user_profile("alice", "25", "alice@example.com").unwrap();
    assert_eq!(profile.summary(), "alice (age 25): alice@example.com");
}

#[test]
fn test_parse_config_map() {
    let lines = &["port=8080", "timeout=30", "retries=3"];
    let map = parse_config_map(lines).unwrap();
    assert_eq!(map.get("port"), Some(&8080));
    assert_eq!(map.get("timeout"), Some(&30));
    assert_eq!(map.get("retries"), Some(&3));
    assert_eq!(map.len(), 3);
}

#[test]
fn test_parse_config_map_error() {
    let lines = &["port=8080", "bad_line"];
    assert!(parse_config_map(lines).is_err());
}

#[test]
fn test_parse_config_map_empty() {
    let lines: &[&str] = &[];
    let map = parse_config_map(lines).unwrap();
    assert!(map.is_empty());
}

#[test]
fn test_find_user_summary() {
    let users = vec![
        create_user_profile("alice", "25", "alice@example.com").unwrap(),
        create_user_profile("bob_1", "30", "bob@example.com").unwrap(),
    ];
    let summary = find_user_summary(&users, "alice").unwrap();
    assert!(summary.contains("alice"));
    assert!(summary.contains("25"));
}

#[test]
fn test_find_user_summary_not_found() {
    let users = vec![
        create_user_profile("alice", "25", "alice@example.com").unwrap(),
    ];
    assert!(matches!(
        find_user_summary(&users, "charlie"),
        Err(AppError::NotFound(_))
    ));
}

// ===== Topic 5: Collecting Results =====

#[test]
fn test_parse_all_strict() {
    assert_eq!(parse_all_strict(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    assert!(parse_all_strict(&["1", "abc", "3"]).is_err());
    assert_eq!(parse_all_strict(&[]), Ok(vec![]));
}

#[test]
fn test_parse_all_lenient() {
    assert_eq!(parse_all_lenient(&["1", "abc", "3", "xyz"]), vec![1, 3]);
    assert_eq!(parse_all_lenient(&["abc"]), vec![]);
    assert_eq!(parse_all_lenient(&[]), vec![]);
}

#[test]
fn test_partition_results() {
    let (ok, err) = partition_results(&["1", "abc", "3"]);
    assert_eq!(ok, vec![1, 3]);
    assert_eq!(err.len(), 1);
}

#[test]
fn test_partition_results_all_valid() {
    let (ok, err) = partition_results(&["1", "2", "3"]);
    assert_eq!(ok, vec![1, 2, 3]);
    assert!(err.is_empty());
}

#[test]
fn test_sum_with_error_count() {
    assert_eq!(sum_with_error_count(&["1", "abc", "3", "xyz"]), (4, 2));
    assert_eq!(sum_with_error_count(&["10", "20"]), (30, 0));
    assert_eq!(sum_with_error_count(&[]), (0, 0));
}

#[test]
fn test_filter_valid_in_range() {
    let (valid, rejected) = filter_valid_in_range(&["5", "abc", "15", "3", "25"], 1, 10);
    assert_eq!(valid, vec![5, 3]);
    assert_eq!(rejected.len(), 3); // "abc" not a number, 15 out of range, 25 out of range
}

#[test]
fn test_filter_valid_in_range_all_valid() {
    let (valid, rejected) = filter_valid_in_range(&["1", "5", "10"], 1, 10);
    assert_eq!(valid, vec![1, 5, 10]);
    assert!(rejected.is_empty());
}

#[test]
fn test_filter_valid_in_range_none_valid() {
    let (valid, rejected) = filter_valid_in_range(&["abc", "50"], 1, 10);
    assert!(valid.is_empty());
    assert_eq!(rejected.len(), 2);
}

#[test]
fn test_parse_matrix_row() {
    assert_eq!(parse_matrix_row("1.0 2.5 3.0"), Ok(vec![1.0, 2.5, 3.0]));
    assert_eq!(parse_matrix_row("42"), Ok(vec![42.0]));
    assert!(parse_matrix_row("1.0 abc 3.0").is_err());
    assert!(parse_matrix_row("").is_err());
    assert!(parse_matrix_row("   ").is_err());
}

#[test]
fn test_validate_usernames() {
    let (valid, invalid) = validate_usernames(&["alice", "ab", "bob_1", "", "user@bad"]);
    assert_eq!(valid, vec!["alice".to_string(), "bob_1".to_string()]);
    assert_eq!(invalid.len(), 3);
    assert_eq!(invalid[0].0, "ab");
    assert_eq!(invalid[1].0, "");
    assert_eq!(invalid[2].0, "user@bad");
}

#[test]
fn test_validate_usernames_all_valid() {
    let (valid, invalid) = validate_usernames(&["alice", "bob_1", "charlie"]);
    assert_eq!(valid.len(), 3);
    assert!(invalid.is_empty());
}

// ===== Topic 6: Advanced — Result Combinators & Patterns =====

#[test]
fn test_parse_validate_double() {
    assert_eq!(parse_validate_double("50"), Ok(100));
    assert_eq!(parse_validate_double("1"), Ok(2));
    assert_eq!(parse_validate_double("100"), Ok(200));
    assert!(parse_validate_double("abc").is_err()); // parse error
    assert!(parse_validate_double("200").is_err()); // out of range
    assert!(parse_validate_double("0").is_err()); // out of range
}

#[test]
fn test_parse_with_fallback() {
    assert_eq!(parse_with_fallback("42", "99"), Ok(42));
    assert_eq!(parse_with_fallback("abc", "99"), Ok(99));
    assert!(parse_with_fallback("abc", "def").is_err());
}

#[test]
fn test_standardize_error() {
    assert_eq!(standardize_error(Ok(42)), Ok(42));
    assert_eq!(
        standardize_error(Err("bad input".to_string())),
        Err("[ERROR] bad input".to_string())
    );
}

#[test]
fn test_safe_compute() {
    assert_eq!(safe_compute("10", "2"), Ok(5.0));
    assert_eq!(safe_compute("7", "2"), Ok(3.5));
    assert!(safe_compute("10", "0").is_err());
    assert!(safe_compute("abc", "2").is_err());
    assert!(safe_compute("10", "xyz").is_err());
}

#[test]
fn test_bounded_i32_ok() {
    let b = BoundedI32::new(5, 1, 10).unwrap();
    assert_eq!(b.value(), 5);
    assert_eq!(b.min(), 1);
    assert_eq!(b.max(), 10);
}

#[test]
fn test_bounded_i32_at_boundaries() {
    assert!(BoundedI32::new(1, 1, 10).is_ok());
    assert!(BoundedI32::new(10, 1, 10).is_ok());
}

#[test]
fn test_bounded_i32_out_of_range() {
    assert!(BoundedI32::new(0, 1, 10).is_err());
    assert!(BoundedI32::new(11, 1, 10).is_err());
    assert!(BoundedI32::new(-5, 0, 100).is_err());
}

#[test]
fn test_bounded_i32_invalid_bounds() {
    assert!(BoundedI32::new(5, 10, 1).is_err()); // min > max
}

#[test]
fn test_bounded_i32_negative_range() {
    let b = BoundedI32::new(-5, -10, -1).unwrap();
    assert_eq!(b.value(), -5);
    assert_eq!(b.min(), -10);
    assert_eq!(b.max(), -1);
}

#[test]
fn test_bounded_i32_display() {
    let b = BoundedI32::new(5, 1, 10).unwrap();
    assert_eq!(format!("{}", b), "5(1..=10)");
}

#[test]
fn test_option_to_result() {
    assert_eq!(option_to_result(Some(42), "not found"), Ok(42));
    assert_eq!(
        option_to_result(None, "not found"),
        Err("not found".to_string())
    );
}

#[test]
fn test_result_to_option() {
    assert_eq!(result_to_option(Ok(42)), Some(42));
    assert_eq!(result_to_option(Err("err".to_string())), None);
}

#[test]
fn test_first_valid_parse() {
    assert_eq!(first_valid_parse(&["abc", "def", "42", "7"]), Ok(42));
    assert_eq!(first_valid_parse(&["5"]), Ok(5));
    assert!(first_valid_parse(&["abc", "def"]).is_err());
    assert!(first_valid_parse(&[]).is_err());
}

#[test]
fn test_apply_operation() {
    assert_eq!(apply_operation("5", "double"), Ok(10));
    assert_eq!(apply_operation("5", "negate"), Ok(-5));
    assert_eq!(apply_operation("-3", "abs"), Ok(3));
    assert_eq!(apply_operation("4", "square"), Ok(16));
    assert!(apply_operation("5", "unknown").is_err());
    assert!(apply_operation("abc", "double").is_err());
}

#[test]
fn test_apply_operation_edge_cases() {
    assert_eq!(apply_operation("0", "double"), Ok(0));
    assert_eq!(apply_operation("0", "negate"), Ok(0));
    assert_eq!(apply_operation("0", "abs"), Ok(0));
    assert_eq!(apply_operation("-7", "double"), Ok(-14));
    assert_eq!(apply_operation("-7", "negate"), Ok(7));
}