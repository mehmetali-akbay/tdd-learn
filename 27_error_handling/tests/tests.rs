use error_handling::*;

// ===== Topic 1: thiserror =====

#[test]
fn test_validate_username_empty() {
    let err = validate_username("").unwrap_err();
    assert!(err.to_string().contains("required"));
}

#[test]
fn test_validate_username_too_long() {
    let long = "a".repeat(25);
    let err = validate_username(&long).unwrap_err();
    assert!(err.to_string().contains("at most"));
}

#[test]
fn test_validate_username_ok() {
    assert!(validate_username("alice").is_ok());
}

#[test]
fn test_validate_email_invalid() {
    let err = validate_email("not-email").unwrap_err();
    assert!(err.to_string().contains("invalid email"));
}

#[test]
fn test_validate_email_ok() {
    assert!(validate_email("a@b.com").is_ok());
}

#[test]
fn test_parse_error_display() {
    let err = ParseError {
        line: 5,
        col: 10,
        message: "unexpected token".into(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("line 5"));
    assert!(msg.contains("column 10"));
}

#[test]
fn test_parse_line_ok() {
    assert_eq!(parse_line("42", 1).unwrap(), 42);
}

#[test]
fn test_parse_line_err() {
    assert!(parse_line("abc", 1).is_err());
}

// ===== Topic 2: anyhow =====

#[test]
fn test_process_config() {
    let result = process_config("key1=value1\nkey2=value2").unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], ("key1".into(), "value1".into()));
}

#[test]
fn test_process_config_invalid() {
    assert!(process_config("bad line").is_err());
}

#[test]
fn test_get_config_value() {
    let config = vec![("port".into(), "8080".into())];
    assert_eq!(get_config_value(&config, "port").unwrap(), "8080");
}

#[test]
fn test_get_config_value_missing() {
    let config: Vec<(String, String)> = vec![];
    let err = get_config_value(&config, "missing").unwrap_err();
    assert!(err.to_string().contains("missing"));
}

#[test]
fn test_parse_validate_port_ok() {
    assert_eq!(parse_and_validate_port("8080").unwrap(), 8080);
}

#[test]
fn test_parse_validate_port_invalid() {
    assert!(parse_and_validate_port("99999").is_err());
}

// ===== Topic 3: Error Composition =====

#[test]
fn test_db_lookup_found() {
    assert!(db_lookup(1).is_ok());
}

#[test]
fn test_db_lookup_not_found() {
    let err = db_lookup(999).unwrap_err();
    assert!(matches!(err, ServiceError::NotFound(_)));
}

#[test]
fn test_network_fetch() {
    assert!(network_fetch("https://valid.com").is_ok());
}

#[test]
fn test_network_fetch_fail() {
    let err = network_fetch("").unwrap_err();
    assert!(matches!(err, ServiceError::Network(_)));
}

#[test]
fn test_service_error_from_parse() {
    let parse_err: Result<i32, _> = "abc".parse::<i32>();
    let service_err: ServiceError = parse_err.unwrap_err().into();
    assert!(matches!(service_err, ServiceError::Parse(_)));
}

// ===== Topic 4: Error Hierarchies =====

#[test]
fn test_store_get_found() {
    let mut store = Store::new(10);
    store.set("key".into(), "value".into()).unwrap();
    assert_eq!(store.get("key").unwrap(), "value");
}

#[test]
fn test_store_get_not_found() {
    let store = Store::new(10);
    assert!(matches!(store.get("x"), Err(StorageError::KeyNotFound(_))));
}

#[test]
fn test_store_full() {
    let mut store = Store::new(1);
    store.set("a".into(), "1".into()).unwrap();
    assert!(matches!(
        store.set("b".into(), "2".into()),
        Err(StorageError::Full)
    ));
}

#[test]
fn test_app_get_unauthorized() {
    let store = Store::new(10);
    assert!(matches!(
        app_get(&store, "key", false),
        Err(AppError::Unauthorized)
    ));
}

#[test]
fn test_app_get_not_found() {
    let store = Store::new(10);
    assert!(matches!(
        app_get(&store, "key", true),
        Err(AppError::Storage(_))
    ));
}

// ===== Topic 5: panic vs Result =====

#[test]
fn test_assert_valid_index_ok() {
    assert_valid_index(2, 5);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_assert_valid_index_panic() {
    assert_valid_index(5, 3);
}

#[test]
fn test_catch_panic_ok() {
    let result = catch_panic(|| 42);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_catch_panic_err() {
    let result = catch_panic(|| panic!("oops"));
    assert!(result.is_err());
}

#[test]
fn test_safe_divide_ok() {
    assert_eq!(safe_divide(10, 3), Ok(3));
}

#[test]
fn test_safe_divide_zero() {
    assert!(safe_divide(10, 0).is_err());
}

// ===== Topic 6: Error Chains =====

#[test]
fn test_error_chain_length() {
    let err = create_error_chain();
    let chain = error_chain(&err);
    assert_eq!(chain.len(), 3);
}

#[test]
fn test_error_chain_messages() {
    let err = create_error_chain();
    let chain = error_chain(&err);
    assert!(chain[0].contains("high-level"));
    assert!(chain[1].contains("mid-level"));
    assert!(chain[2].contains("low-level"));
}

#[test]
fn test_format_error_chain() {
    let err = create_error_chain();
    let formatted = format_error_chain(&err);
    assert!(formatted.contains("high-level"));
    assert!(formatted.contains("mid-level"));
    assert!(formatted.contains("low-level"));
}

#[test]
fn test_anyhow_backtrace() {
    let err = anyhow_with_backtrace().unwrap_err();
    let bt = extract_backtrace(&err).unwrap();
    // In debug builds with RUST_BACKTRACE=1 (or implicitly in anyhow), 
    // it will contain a backtrace string. We just ensure it's not empty.
    assert!(!bt.is_empty());
}
