use types::*;

// ===== Topic 1: JsonValue =====

#[test]
fn test_json_truthy() {
    assert!(!JsonValue::Null.is_truthy());
    assert!(JsonValue::Bool(true).is_truthy());
    assert!(!JsonValue::Bool(false).is_truthy());
    assert!(JsonValue::Num(42.0).is_truthy());
    assert!(!JsonValue::Num(0.0).is_truthy());
    assert!(JsonValue::Str("hello".to_string()).is_truthy());
    assert!(!JsonValue::Str("".to_string()).is_truthy());
    assert!(JsonValue::Array(vec![JsonValue::Num(1.0)]).is_truthy());
    assert!(!JsonValue::Array(vec![]).is_truthy());
}

#[test]
fn test_json_type_name() {
    assert_eq!(JsonValue::Null.type_name(), "null");
    assert_eq!(JsonValue::Bool(true).type_name(), "boolean");
    assert_eq!(JsonValue::Num(1.0).type_name(), "number");
    assert_eq!(JsonValue::Str("hi".to_string()).type_name(), "string");
    assert_eq!(JsonValue::Array(vec![]).type_name(), "array");
}

#[test]
fn test_json_display_string() {
    assert_eq!(JsonValue::Null.to_display_string(), "null");
    assert_eq!(JsonValue::Bool(true).to_display_string(), "true");
    assert_eq!(JsonValue::Num(42.0).to_display_string(), "42");
    assert_eq!(
        JsonValue::Str("hi".to_string()).to_display_string(),
        "\"hi\""
    );
}

#[test]
fn test_json_array_display() {
    let arr = JsonValue::Array(vec![JsonValue::Num(1.0), JsonValue::Str("two".to_string())]);
    assert_eq!(arr.to_display_string(), "[1, \"two\"]");
}

#[test]
fn test_json_is_null() {
    assert!(JsonValue::Null.is_null());
    assert!(!JsonValue::Num(0.0).is_null());
}

#[test]
fn test_json_as_num() {
    assert_eq!(JsonValue::Num(42.0).as_num(), Some(42.0));
    assert_eq!(JsonValue::Str("42".to_string()).as_num(), None);
}

// ===== Topic 2: TrafficLight =====

#[test]
fn test_traffic_light_next() {
    assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
    assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
    assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
}

#[test]
fn test_traffic_light_cycle() {
    // Full cycle: Red -> Green -> Yellow -> Red
    let light = TrafficLight::Red;
    assert_eq!(light.next().next().next(), TrafficLight::Red);
}

#[test]
fn test_traffic_light_wait() {
    assert_eq!(TrafficLight::Red.wait_time(), 60);
    assert_eq!(TrafficLight::Yellow.wait_time(), 5);
    assert_eq!(TrafficLight::Green.wait_time(), 45);
}

#[test]
fn test_traffic_light_stop_go() {
    assert!(TrafficLight::Red.is_stop());
    assert!(TrafficLight::Yellow.is_stop());
    assert!(!TrafficLight::Green.is_stop());
    assert!(TrafficLight::Green.can_go());
    assert!(!TrafficLight::Red.can_go());
}

#[test]
fn test_traffic_light_color_name() {
    assert_eq!(TrafficLight::Red.color_name(), "red");
    assert_eq!(TrafficLight::Green.color_name(), "green");
}

// ===== Topic 3: Linked List =====

#[test]
fn test_list_new_is_empty() {
    let list = List::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_list_push() {
    let list = List::new().push(1).push(2).push(3);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
}

#[test]
fn test_list_head() {
    assert_eq!(List::new().head(), None);
    assert_eq!(List::new().push(1).push(2).head(), Some(2));
}

#[test]
fn test_list_contains() {
    let list = List::new().push(10).push(20).push(30);
    assert!(list.contains(20));
    assert!(!list.contains(99));
}

#[test]
fn test_list_sum() {
    let list = List::new().push(1).push(2).push(3);
    assert_eq!(list.sum(), 6);
    assert_eq!(List::new().sum(), 0);
}

#[test]
fn test_list_to_vec() {
    let list = List::new().push(1).push(2).push(3);
    assert_eq!(list.to_vec(), vec![3, 2, 1]);
}

#[test]
fn test_list_from_vec() {
    let list = List::from_vec(&[1, 2, 3]);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
    assert_eq!(list.len(), 3);
}

// ===== Topic 4: Expression Tree =====

#[test]
fn test_expr_eval_num() {
    assert_eq!(Expr::Num(42.0).eval(), Some(42.0));
}

#[test]
fn test_expr_eval_add_sub() {
    let expr = Expr::Sub(
        Box::new(Expr::Add(Expr::num(10.0), Expr::num(5.0))),
        Expr::num(3.0),
    );
    assert_eq!(expr.eval(), Some(12.0));
}

#[test]
fn test_expr_eval_mul_div() {
    let expr = Expr::Div(
        Box::new(Expr::Mul(Expr::num(6.0), Expr::num(3.0))),
        Expr::num(2.0),
    );
    assert_eq!(expr.eval(), Some(9.0));
}

#[test]
fn test_expr_div_by_zero() {
    let expr = Expr::Div(Expr::num(10.0), Expr::num(0.0));
    assert_eq!(expr.eval(), None);
}

#[test]
fn test_expr_to_string() {
    let expr = Expr::Add(Expr::num(2.0), Expr::num(3.0));
    assert_eq!(expr.to_string_repr(), "(2 + 3)");
}

#[test]
fn test_expr_count_ops() {
    // (2 + 3) * 4 => 2 operations
    let expr = Expr::Mul(
        Box::new(Expr::Add(Expr::num(2.0), Expr::num(3.0))),
        Expr::num(4.0),
    );
    assert_eq!(expr.count_ops(), 2);
    assert_eq!(Expr::Num(42.0).count_ops(), 0);
}

#[test]
fn test_expr_contains_div() {
    let with_div = Expr::Add(
        Expr::num(1.0),
        Box::new(Expr::Div(Expr::num(10.0), Expr::num(2.0))),
    );
    let without_div = Expr::Add(Expr::num(1.0), Expr::num(2.0));
    assert!(with_div.contains_div());
    assert!(!without_div.contains_div());
}

// ===== Topic 5: Custom Errors =====

#[test]
fn test_app_error_message() {
    assert_eq!(
        AppError::NotFound("user".to_string()).message(),
        "user not found"
    );
    assert_eq!(
        AppError::InvalidInput("bad".to_string()).message(),
        "Invalid input: bad"
    );
    assert_eq!(AppError::Unauthorized.message(), "Unauthorized access");
}

#[test]
fn test_app_error_is_user_error() {
    assert!(AppError::InvalidInput("x".to_string()).is_user_error());
    assert!(AppError::ParseError("x".to_string()).is_user_error());
    assert!(!AppError::NotFound("x".to_string()).is_user_error());
    assert!(!AppError::Unauthorized.is_user_error());
}

#[test]
fn test_parse_age() {
    assert_eq!(parse_age("25"), Ok(25));
    assert_eq!(parse_age("0"), Ok(0));
    assert_eq!(parse_age("150"), Ok(150));
    assert!(parse_age("151").is_err());
    assert!(parse_age("-1").is_err());
    assert!(parse_age("abc").is_err());
}

#[test]
fn test_divide_safe() {
    assert_eq!(divide_safe(10.0, 2.0), Ok(5.0));
    assert!(divide_safe(10.0, 0.0).is_err());
}

#[test]
fn test_lookup_user() {
    assert_eq!(lookup_user(1), Ok("Alice".to_string()));
    assert_eq!(lookup_user(2), Ok("Bob".to_string()));
    assert!(lookup_user(99).is_err());
}

#[test]
fn test_parse_and_check_adult() {
    assert_eq!(parse_and_check_adult("25"), Ok(true));
    assert_eq!(parse_and_check_adult("10"), Ok(false));
    assert_eq!(parse_and_check_adult("18"), Ok(true));
    assert!(parse_and_check_adult("abc").is_err());
}

// ===== Topic 6: Payload =====

#[test]
fn test_payload_size() {
    assert_eq!(Payload::Text("hello".to_string()).size(), 5);
    assert_eq!(Payload::Binary(vec![1, 2, 3]).size(), 3);
    assert_eq!(Payload::Empty.size(), 0);
}

#[test]
fn test_payload_is_text() {
    assert!(Payload::Text("hi".to_string()).is_text());
    assert!(Payload::Json(JsonValue::Null).is_text());
    assert!(!Payload::Binary(vec![]).is_text());
    assert!(!Payload::Empty.is_text());
}

#[test]
fn test_payload_is_empty() {
    assert!(Payload::Empty.is_empty());
    assert!(Payload::Text("".to_string()).is_empty());
    assert!(Payload::Binary(vec![]).is_empty());
    assert!(!Payload::Text("hi".to_string()).is_empty());
}

#[test]
fn test_payload_to_text() {
    assert_eq!(Payload::Text("hi".to_string()).to_text(), "hi");
    assert_eq!(Payload::Binary(vec![1, 2, 3]).to_text(), "<3 bytes>");
    assert_eq!(Payload::Empty.to_text(), "<empty>");
}

#[test]
fn test_payload_merge_text() {
    let a = Payload::Text("hello".to_string());
    let b = Payload::Text(" world".to_string());
    assert_eq!(a.merge(b), Payload::Text("hello world".to_string()));
}

#[test]
fn test_payload_merge_binary() {
    let a = Payload::Binary(vec![1, 2]);
    let b = Payload::Binary(vec![3, 4]);
    assert_eq!(a.merge(b), Payload::Binary(vec![1, 2, 3, 4]));
}

#[test]
fn test_payload_merge_empty() {
    let text = Payload::Text("hi".to_string());
    assert_eq!(Payload::Empty.merge(text.clone()), text);
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_clamp() {
    assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
    assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
    assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
}

#[test]
fn test_safe_u32_to_u8() {
    assert_eq!(safe_u32_to_u8(100), Some(100));
    assert_eq!(safe_u32_to_u8(256), None);
}

#[test]
fn test_temp_classify() {
    assert_eq!(temp_classify(32.0), "cold"); // 0°C
    assert_eq!(temp_classify(212.0), "hot"); // 100°C
    assert_eq!(temp_classify(72.0), "comfortable"); // ~22°C
}

#[test]
fn test_map_range() {
    assert!((map_range(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < 0.01);
}
