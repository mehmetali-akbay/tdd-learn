use results::*;

// ===== Topic 1: Result Basics =====

#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), Ok(5));
    assert_eq!(divide(7, 3), Ok(2)); // integer division
    assert!(divide(10, 0).is_err());
}

#[test]
fn test_parse_number() {
    assert_eq!(parse_number("42"), Ok(42));
    assert_eq!(parse_number("-5"), Ok(-5));
    assert!(parse_number("abc").is_err());
}

#[test]
fn test_get_at() {
    assert_eq!(get_at(&[10, 20, 30], 1), Ok(20));
    assert!(get_at(&[10, 20, 30], 5).is_err());
}

#[test]
fn test_double_result() {
    assert_eq!(double_result(Ok(5)), Ok(10));
    assert_eq!(
        double_result(Err("err".to_string())),
        Err("err".to_string())
    );
}

#[test]
fn test_parse_and_double() {
    assert_eq!(parse_and_double("5"), Ok(10));
    assert!(parse_and_double("abc").is_err());
}

#[test]
fn test_unwrap_or_default() {
    assert_eq!(unwrap_or_default(Ok(42), 0), 42);
    assert_eq!(unwrap_or_default(Err("err".to_string()), 99), 99);
}

// ===== Topic 2: The ? Operator =====

#[test]
fn test_add_strings() {
    assert_eq!(add_strings("3", "4"), Ok(7));
    assert!(add_strings("abc", "4").is_err());
    assert!(add_strings("3", "xyz").is_err());
}

#[test]
fn test_parse_and_divide() {
    assert_eq!(parse_and_divide("10", "2"), Ok(5));
    assert!(parse_and_divide("10", "0").is_err());
    assert!(parse_and_divide("abc", "2").is_err());
}

#[test]
fn test_average_of_csv() {
    assert_eq!(average_of_csv("1,2,3"), Ok(2.0));
    assert_eq!(average_of_csv("10"), Ok(10.0));
    assert!(average_of_csv("").is_err());
    assert!(average_of_csv("1,abc,3").is_err());
}

#[test]
fn test_first_positive_from_csv() {
    assert_eq!(first_positive_from_csv("-1,-2,3,4"), Ok(3));
    assert!(first_positive_from_csv("-1,-2,-3").is_err());
    assert!(first_positive_from_csv("").is_err());
    assert!(first_positive_from_csv("abc").is_err());
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
}

#[test]
fn test_validate_username() {
    assert_eq!(validate_username("alice"), Ok(()));
    assert_eq!(validate_username("user_123"), Ok(()));
    assert_eq!(validate_username(""), Err(ValidationError::Empty));
    assert_eq!(validate_username("ab"), Err(ValidationError::TooShort(2)));
    assert_eq!(
        validate_username("user@name"),
        Err(ValidationError::InvalidChar('@'))
    );
}

#[test]
fn test_validate_username_too_long() {
    let long_name = "a".repeat(21);
    assert_eq!(
        validate_username(&long_name),
        Err(ValidationError::TooLong(21))
    );
}

#[test]
fn test_validate_password() {
    assert_eq!(validate_password("Passw0rd"), Ok(()));
    assert_eq!(validate_password(""), Err(ValidationError::Empty));
    assert!(validate_password("short").is_err()); // too short
    assert!(validate_password("alllowercase1").is_err()); // no uppercase
}

// ===== Topic 4: Error Conversion =====

#[test]
fn test_register_user_ok() {
    let result = register_user("alice", "25");
    assert_eq!(result, Ok("Welcome alice, age 25".to_string()));
}

#[test]
fn test_register_user_bad_username() {
    let result = register_user("ab", "25");
    assert!(matches!(result, Err(AppError::Validation(_))));
}

#[test]
fn test_register_user_bad_age() {
    let result = register_user("alice", "abc");
    assert!(matches!(result, Err(AppError::Parse(_))));
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

// ===== Topic 5: Collecting Results =====

#[test]
fn test_parse_all_strict() {
    assert_eq!(parse_all_strict(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    assert!(parse_all_strict(&["1", "abc", "3"]).is_err());
}

#[test]
fn test_parse_all_lenient() {
    assert_eq!(parse_all_lenient(&["1", "abc", "3", "xyz"]), vec![1, 3]);
    assert_eq!(parse_all_lenient(&["abc"]), vec![]);
}

#[test]
fn test_partition_results() {
    let (ok, err) = partition_results(&["1", "abc", "3"]);
    assert_eq!(ok, vec![1, 3]);
    assert_eq!(err.len(), 1);
}

#[test]
fn test_sum_with_error_count() {
    assert_eq!(sum_with_error_count(&["1", "abc", "3", "xyz"]), (4, 2));
    assert_eq!(sum_with_error_count(&["10", "20"]), (30, 0));
}

// ===== Topic 6: Advanced — Result Combinators =====

#[test]
fn test_parse_validate_double() {
    assert_eq!(parse_validate_double("50"), Ok(100));
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
    assert!(safe_compute("10", "0").is_err());
    assert!(safe_compute("abc", "2").is_err());
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_parse_and_add() {
    assert_eq!(parse_and_add("3.5", "2.5"), Ok(6.0));
    assert!(parse_and_add("abc", "2").is_err());
}

#[test]
fn test_validate_email_simple() {
    assert!(validate_email_simple("user@example.com").is_ok());
    assert!(validate_email_simple("noat.com").is_err());
    assert!(validate_email_simple("user@nodot").is_err());
}

#[test]
fn test_safe_divide_f64() {
    assert_eq!(safe_divide_f64("10", "4"), Ok(2.5));
    assert!(safe_divide_f64("10", "0").is_err());
    assert!(safe_divide_f64("abc", "4").is_err());
}

#[test]
fn test_parse_all_ints() {
    assert_eq!(parse_all_ints(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    assert!(parse_all_ints(&["1", "abc"]).is_err());
}

