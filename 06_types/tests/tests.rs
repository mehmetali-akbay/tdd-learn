use types::*;

// ===== Topic 1: JsonValue =====

#[test]
fn json_truthiness_follows_variant_rules() {
    assert!(!JsonValue::Null.is_truthy());
    assert!(JsonValue::Bool(true).is_truthy());
    assert!(!JsonValue::Bool(false).is_truthy());
    assert!(JsonValue::Num(10.0).is_truthy());
    assert!(!JsonValue::Num(0.0).is_truthy());
    assert!(JsonValue::Str("x".to_string()).is_truthy());
    assert!(!JsonValue::Str("".to_string()).is_truthy());
    assert!(JsonValue::Array(vec![JsonValue::Null]).is_truthy());
    assert!(!JsonValue::Array(vec![]).is_truthy());
}

#[test]
fn json_type_name_matches_each_variant() {
    assert_eq!(JsonValue::Null.type_name(), "null");
    assert_eq!(JsonValue::Bool(true).type_name(), "boolean");
    assert_eq!(JsonValue::Num(1.0).type_name(), "number");
    assert_eq!(JsonValue::Str("a".to_string()).type_name(), "string");
    assert_eq!(JsonValue::Array(vec![]).type_name(), "array");
}

#[test]
fn json_display_formats_primitives() {
    assert_eq!(JsonValue::Null.to_display_string(), "null");
    assert_eq!(JsonValue::Bool(false).to_display_string(), "false");
    assert_eq!(JsonValue::Num(42.5).to_display_string(), "42.5");
    assert_eq!(JsonValue::Str("hi".to_string()).to_display_string(), "\"hi\"");
}

#[test]
fn json_display_formats_arrays_recursively() {
    let value = JsonValue::Array(vec![
        JsonValue::Num(1.0),
        JsonValue::Str("two".to_string()),
        JsonValue::Array(vec![JsonValue::Bool(true)]),
    ]);
    assert_eq!(value.to_display_string(), "[1, \"two\", [true]]");
}

#[test]
fn json_helpers_is_null_and_as_num_are_precise() {
    assert!(JsonValue::Null.is_null());
    assert!(!JsonValue::Bool(false).is_null());
    assert_eq!(JsonValue::Num(99.0).as_num(), Some(99.0));
    assert_eq!(JsonValue::Str("99".to_string()).as_num(), None);
}

// ===== Topic 2: TrafficLight =====

#[test]
fn traffic_light_next_transitions_in_expected_order() {
    assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
    assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
    assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
}

#[test]
fn traffic_light_cycles_back_after_three_steps() {
    let light = TrafficLight::Red;
    assert_eq!(light.next().next().next(), TrafficLight::Red);
}

#[test]
fn traffic_light_wait_time_is_state_specific() {
    assert_eq!(TrafficLight::Red.wait_time(), 60);
    assert_eq!(TrafficLight::Yellow.wait_time(), 5);
    assert_eq!(TrafficLight::Green.wait_time(), 45);
}

#[test]
fn traffic_light_stop_and_go_flags_are_consistent() {
    assert!(TrafficLight::Red.is_stop());
    assert!(TrafficLight::Yellow.is_stop());
    assert!(!TrafficLight::Green.is_stop());
    assert!(TrafficLight::Green.can_go());
    assert!(!TrafficLight::Red.can_go());
}

#[test]
fn traffic_light_color_name_returns_lowercase_text() {
    assert_eq!(TrafficLight::Red.color_name(), "red");
    assert_eq!(TrafficLight::Yellow.color_name(), "yellow");
    assert_eq!(TrafficLight::Green.color_name(), "green");
}

// ===== Topic 3: Linked List =====

#[test]
fn list_new_creates_empty_structure() {
    let list = List::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.head(), None);
}

#[test]
fn list_push_places_value_at_front() {
    let list = List::new().push(1).push(2).push(3);
    assert_eq!(list.head(), Some(3));
    assert_eq!(list.len(), 3);
}

#[test]
fn list_contains_checks_entire_recursive_chain() {
    let list = List::new().push(10).push(20).push(30);
    assert!(list.contains(20));
    assert!(!list.contains(999));
}

#[test]
fn list_sum_handles_empty_and_non_empty_lists() {
    assert_eq!(List::new().sum(), 0);
    assert_eq!(List::new().push(1).push(2).push(3).sum(), 6);
}

#[test]
fn list_to_vec_preserves_node_order() {
    let list = List::new().push(1).push(2).push(3);
    assert_eq!(list.to_vec(), vec![3, 2, 1]);
}

#[test]
fn list_from_vec_round_trip_matches_input_order() {
    let list = List::from_vec(&[4, 5, 6]);
    assert_eq!(list.to_vec(), vec![4, 5, 6]);
}

#[test]
fn list_from_vec_of_empty_slice_returns_nil() {
    let list = List::from_vec(&[]);
    assert!(list.is_empty());
    assert_eq!(list, List::Nil);
}

// ===== Topic 4: Expression Tree =====

#[test]
fn expr_eval_returns_number_for_num_variant() {
    assert_eq!(Expr::Num(42.0).eval(), Some(42.0));
}

#[test]
fn expr_eval_handles_add_sub_mul_div_composition() {
    // ((10 + 2) * 3 - 6) / 2 = 15
    let expr = Expr::Div(
        Box::new(Expr::Sub(
            Box::new(Expr::Mul(
                Box::new(Expr::Add(Expr::num(10.0), Expr::num(2.0))),
                Expr::num(3.0),
            )),
            Expr::num(6.0),
        )),
        Expr::num(2.0),
    );
    assert_eq!(expr.eval(), Some(15.0));
}

#[test]
fn expr_eval_negates_inner_expression() {
    let expr = Expr::Neg(Box::new(Expr::Num(5.0)));
    assert_eq!(expr.eval(), Some(-5.0));
}

#[test]
fn expr_eval_returns_none_on_division_by_zero() {
    let expr = Expr::Div(Expr::num(10.0), Expr::num(0.0));
    assert_eq!(expr.eval(), None);
}

#[test]
fn expr_to_string_repr_preserves_tree_shape() {
    let expr = Expr::Add(Expr::num(2.0), Box::new(Expr::Mul(Expr::num(3.0), Expr::num(4.0))));
    assert_eq!(expr.to_string_repr(), "(2 + (3 * 4))");
}

#[test]
fn expr_count_ops_counts_non_num_nodes_only() {
    let expr = Expr::Mul(
        Box::new(Expr::Add(Expr::num(2.0), Expr::num(3.0))),
        Box::new(Expr::Neg(Expr::num(4.0))),
    );
    assert_eq!(expr.count_ops(), 3);
    assert_eq!(Expr::Num(10.0).count_ops(), 0);
}

#[test]
fn expr_contains_div_detects_nested_division() {
    let with_div = Expr::Add(
        Expr::num(1.0),
        Box::new(Expr::Div(Expr::num(10.0), Expr::num(2.0))),
    );
    let without_div = Expr::Mul(Expr::num(2.0), Expr::num(3.0));
    assert!(with_div.contains_div());
    assert!(!without_div.contains_div());
}

// ===== Topic 5: Custom Errors =====

#[test]
fn app_error_message_is_user_friendly() {
    assert_eq!(
        AppError::NotFound("user".to_string()).message(),
        "user not found"
    );
    assert_eq!(
        AppError::InvalidInput("bad".to_string()).message(),
        "Invalid input: bad"
    );
    assert_eq!(AppError::Unauthorized.message(), "Unauthorized access");
    assert_eq!(
        AppError::ParseError("oops".to_string()).message(),
        "Parse error: oops"
    );
}

#[test]
fn app_error_is_user_error_classifies_only_input_and_parse() {
    assert!(AppError::InvalidInput("x".to_string()).is_user_error());
    assert!(AppError::ParseError("x".to_string()).is_user_error());
    assert!(!AppError::NotFound("x".to_string()).is_user_error());
    assert!(!AppError::Unauthorized.is_user_error());
}

#[test]
fn parse_age_accepts_valid_boundaries() {
    assert_eq!(parse_age("0"), Ok(0));
    assert_eq!(parse_age("18"), Ok(18));
    assert_eq!(parse_age("150"), Ok(150));
}

#[test]
fn parse_age_returns_typed_errors_for_invalid_input() {
    assert!(matches!(parse_age("abc"), Err(AppError::ParseError(_))));
    assert!(matches!(parse_age("-1"), Err(AppError::InvalidInput(_))));
    assert!(matches!(parse_age("151"), Err(AppError::InvalidInput(_))));
}

#[test]
fn divide_safe_returns_error_on_zero_divisor() {
    assert_eq!(divide_safe(10.0, 2.0), Ok(5.0));
    assert!(matches!(
        divide_safe(10.0, 0.0),
        Err(AppError::InvalidInput(_))
    ));
}

#[test]
fn lookup_user_returns_known_users_or_not_found_error() {
    assert_eq!(lookup_user(1), Ok("Alice".to_string()));
    assert_eq!(lookup_user(2), Ok("Bob".to_string()));
    assert!(matches!(lookup_user(999), Err(AppError::NotFound(_))));
}

#[test]
fn parse_and_check_adult_chains_parse_and_business_rule() {
    assert_eq!(parse_and_check_adult("25"), Ok(true));
    assert_eq!(parse_and_check_adult("17"), Ok(false));
    assert!(matches!(
        parse_and_check_adult("oops"),
        Err(AppError::ParseError(_))
    ));
}

// ===== Topic 6: Payload =====

#[test]
fn payload_size_is_variant_dependent() {
    assert_eq!(Payload::Text("hello".to_string()).size(), 5);
    assert_eq!(Payload::Binary(vec![1, 2, 3]).size(), 3);
    assert_eq!(Payload::Json(JsonValue::Null).size(), 4);
    assert_eq!(Payload::Empty.size(), 0);
}

#[test]
fn payload_is_text_true_for_text_and_json_only() {
    assert!(Payload::Text("hi".to_string()).is_text());
    assert!(Payload::Json(JsonValue::Num(1.0)).is_text());
    assert!(!Payload::Binary(vec![1]).is_text());
    assert!(!Payload::Empty.is_text());
}

#[test]
fn payload_is_empty_has_variant_specific_rules() {
    assert!(Payload::Empty.is_empty());
    assert!(Payload::Text("".to_string()).is_empty());
    assert!(Payload::Binary(vec![]).is_empty());
    assert!(Payload::Json(JsonValue::Null).is_empty());
    assert!(!Payload::Text("x".to_string()).is_empty());
    assert!(!Payload::Json(JsonValue::Num(0.0)).is_empty());
}

#[test]
fn payload_to_text_formats_each_variant() {
    assert_eq!(Payload::Text("hi".to_string()).to_text(), "hi");
    assert_eq!(Payload::Binary(vec![1, 2, 3]).to_text(), "<3 bytes>");
    assert_eq!(Payload::Json(JsonValue::Bool(true)).to_text(), "true");
    assert_eq!(Payload::Empty.to_text(), "<empty>");
}

#[test]
fn payload_merge_concatenates_text_payloads() {
    let left = Payload::Text("hello".to_string());
    let right = Payload::Text(" world".to_string());
    assert_eq!(left.merge(right), Payload::Text("hello world".to_string()));
}

#[test]
fn payload_merge_concatenates_binary_payloads() {
    let left = Payload::Binary(vec![1, 2]);
    let right = Payload::Binary(vec![3, 4]);
    assert_eq!(left.merge(right), Payload::Binary(vec![1, 2, 3, 4]));
}

#[test]
fn payload_merge_treats_empty_as_identity() {
    let text = Payload::Text("hi".to_string());
    let binary = Payload::Binary(vec![9, 8]);
    assert_eq!(Payload::Empty.clone().merge(text.clone()), text);
    assert_eq!(binary.clone().merge(Payload::Empty), binary);
}
