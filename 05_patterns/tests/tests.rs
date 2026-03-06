use patterns::*;

const EPS: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPS,
        "expected {expected}, got {actual}"
    );
}

// ===== Topic 1: Match Basics =====

#[test]
fn describe_number_distinguishes_signs() {
    assert_eq!(describe_number(0), "zero");
    assert_eq!(describe_number(17), "positive");
    assert_eq!(describe_number(-2), "negative");
}

#[test]
fn grade_to_letter_handles_boundaries() {
    assert_eq!(grade_to_letter(100), "A");
    assert_eq!(grade_to_letter(90), "A");
    assert_eq!(grade_to_letter(89), "B");
    assert_eq!(grade_to_letter(79), "C");
    assert_eq!(grade_to_letter(69), "D");
    assert_eq!(grade_to_letter(59), "F");
    assert_eq!(grade_to_letter(101), "F");
}

#[test]
fn classify_char_recognizes_vowel_consonant_digit_other() {
    assert_eq!(classify_char('a'), "vowel");
    assert_eq!(classify_char('E'), "vowel");
    assert_eq!(classify_char('z'), "consonant");
    assert_eq!(classify_char('5'), "digit");
    assert_eq!(classify_char('?'), "other");
}

#[test]
fn day_name_maps_1_to_7() {
    assert_eq!(day_name(1), "Monday");
    assert_eq!(day_name(4), "Thursday");
    assert_eq!(day_name(7), "Sunday");
}

#[test]
fn day_name_rejects_invalid_numbers() {
    assert_eq!(day_name(0), "invalid");
    assert_eq!(day_name(8), "invalid");
}

#[test]
fn fizzbuzz_prioritizes_fifteen_before_three_and_five() {
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(16), "16");
}

// ===== Topic 2: Enums with Match =====

#[test]
fn color_to_hex_supports_named_variants() {
    assert_eq!(color_to_hex(&Color::Red), "#FF0000");
    assert_eq!(color_to_hex(&Color::Green), "#00FF00");
    assert_eq!(color_to_hex(&Color::Blue), "#0000FF");
}

#[test]
fn color_to_hex_supports_custom_payload() {
    assert_eq!(color_to_hex(&Color::Custom(255, 128, 0)), "#FF8000");
    assert_eq!(color_to_hex(&Color::Custom(0, 0, 0)), "#000000");
}

#[test]
fn area_matches_each_shape_variant() {
    assert_close(area(&Shape::Circle(2.0)), 4.0 * std::f64::consts::PI);
    assert_close(area(&Shape::Rectangle(3.0, 4.0)), 12.0);
    assert_close(area(&Shape::Triangle(6.0, 4.0)), 12.0);
}

#[test]
fn describe_shape_includes_payload_values() {
    assert_eq!(describe_shape(&Shape::Circle(5.0)), "Circle with radius 5");
    assert_eq!(describe_shape(&Shape::Rectangle(3.0, 4.0)), "Rectangle 3x4");
    assert_eq!(
        describe_shape(&Shape::Triangle(3.0, 4.0)),
        "Triangle with base 3 and height 4"
    );
}

#[test]
fn coin_value_returns_cent_values() {
    assert_eq!(coin_value(&Coin::Penny), 1);
    assert_eq!(coin_value(&Coin::Nickel), 5);
    assert_eq!(coin_value(&Coin::Dime), 10);
    assert_eq!(coin_value(&Coin::Quarter), 25);
}

#[test]
fn total_value_sums_all_coins() {
    let coins = vec![Coin::Quarter, Coin::Dime, Coin::Penny, Coin::Penny];
    assert_eq!(total_value(&coins), 37);
    assert_eq!(total_value(&[]), 0);
}

#[test]
fn message_call_handles_each_variant() {
    assert_eq!(Message::Quit.call(), "Quit");
    assert_eq!(Message::Move { x: 10, y: -5 }.call(), "Move to (10, -5)");
    assert_eq!(Message::Write("hello".to_string()).call(), "Text: hello");
    assert_eq!(Message::ChangeColor(1, 2, 3).call(), "Color: (1, 2, 3)");
}

// ===== Topic 3: Option Matching =====

#[test]
fn safe_divide_returns_none_when_divisor_is_zero() {
    assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
    assert_eq!(safe_divide(10.0, 0.0), None);
}

#[test]
fn first_or_default_uses_fallback_for_empty_slice() {
    assert_eq!(first_or_default(&[9, 8, 7], 0), 9);
    assert_eq!(first_or_default(&[], 42), 42);
}

#[test]
fn double_option_maps_some_and_preserves_none() {
    assert_eq!(double_option(Some(5)), Some(10));
    assert_eq!(double_option(Some(0)), Some(0));
    assert_eq!(double_option(None), None);
}

#[test]
fn parse_and_double_requires_valid_integer() {
    assert_eq!(parse_and_double("12"), Some(24));
    assert_eq!(parse_and_double("-7"), Some(-14));
    assert_eq!(parse_and_double("abc"), None);
}

#[test]
fn option_string_length_defaults_to_zero() {
    assert_eq!(option_string_length(Some("hello")), 5);
    assert_eq!(option_string_length(Some("")), 0);
    assert_eq!(option_string_length(None), 0);
}

#[test]
fn first_even_returns_first_match_only() {
    assert_eq!(first_even(&[1, 3, 8, 10]), Some(8));
    assert_eq!(first_even(&[1, 3, 5]), None);
    assert_eq!(first_even(&[]), None);
}

#[test]
fn plus_one_follows_book_style_option_matching() {
    assert_eq!(plus_one(Some(5)), Some(6));
    assert_eq!(plus_one(Some(-1)), Some(0));
    assert_eq!(plus_one(None), None);
}

// ===== Topic 4: If Let, While Let, Matches! =====

#[test]
fn describe_option_formats_some_and_none() {
    assert_eq!(describe_option(Some(42)), "Got: 42");
    assert_eq!(describe_option(None), "nothing");
}

#[test]
fn count_items_counts_until_pop_returns_none() {
    assert_eq!(count_items(vec![1, 2, 3]), 3);
    assert_eq!(count_items(vec![]), 0);
}

#[test]
fn result_to_string_formats_ok_and_err_paths() {
    assert_eq!(result_to_string(Ok(7)), "Success: 7");
    assert_eq!(
        result_to_string(Err("bad input".to_string())),
        "Error: bad input"
    );
}

#[test]
fn is_in_range_is_inclusive_between_1_and_10() {
    assert!(is_in_range(1));
    assert!(is_in_range(10));
    assert!(!is_in_range(0));
    assert!(!is_in_range(11));
}

#[test]
fn is_even_option_requires_some_even_value() {
    assert!(is_even_option(Some(4)));
    assert!(!is_even_option(Some(3)));
    assert!(!is_even_option(None));
}

#[test]
fn quarter_state_extracts_payload_with_if_let() {
    assert_eq!(
        quarter_state(&CoinWithState::Quarter(UsState::Alaska)),
        Some(UsState::Alaska)
    );
    assert_eq!(quarter_state(&CoinWithState::Penny), None);
}

#[test]
fn parse_pair_uses_let_else_for_happy_path_and_early_returns() {
    assert_eq!(parse_pair("10 20"), Some((10, 20)));
    assert_eq!(parse_pair("  -3   7 "), Some((-3, 7)));
    assert_eq!(parse_pair("10"), None);
    assert_eq!(parse_pair("10 20 30"), None);
    assert_eq!(parse_pair("ten 20"), None);
}

// ===== Topic 5: Destructuring =====

#[test]
fn tuple_sum_adds_pair_values() {
    assert_eq!(tuple_sum((3, 4)), 7);
    assert_eq!(tuple_sum((-1, 1)), 0);
}

#[test]
fn triple_max_finds_largest_member() {
    assert_eq!(triple_max((1, 9, 3)), 9);
    assert_eq!(triple_max((-10, -3, -7)), -3);
}

#[test]
fn distance_from_origin_uses_struct_destructuring() {
    let p = Point { x: 3.0, y: 4.0 };
    assert_close(distance_from_origin(&p), 5.0);
}

#[test]
fn nested_sum_destructures_nested_tuple() {
    assert_eq!(nested_sum(((1, 2), 3)), 6);
    assert_eq!(nested_sum(((10, -5), 7)), 12);
}

#[test]
fn swap_coordinates_returns_new_point() {
    let p = Point { x: 2.5, y: -9.0 };
    assert_eq!(swap_coordinates(&p), Point { x: -9.0, y: 2.5 });
    assert_eq!(p, Point { x: 2.5, y: -9.0 });
}

#[test]
fn rect_area_destructures_in_parameter_list() {
    assert_close(rect_area((5.0, 3.0)), 15.0);
    assert_close(rect_area((0.0, 9.0)), 0.0);
}

// ===== Topic 6: Advanced Pattern Challenges =====

#[test]
fn to_celsius_converts_all_temperature_variants() {
    assert_close(to_celsius(&Temperature::Celsius(12.0)), 12.0);
    assert_close(to_celsius(&Temperature::Fahrenheit(32.0)), 0.0);
    assert_close(to_celsius(&Temperature::Kelvin(273.15)), 0.0);
}

#[test]
fn describe_temperature_uses_celsius_buckets() {
    assert_eq!(
        describe_temperature(&Temperature::Celsius(-0.1)),
        "freezing"
    );
    assert_eq!(describe_temperature(&Temperature::Celsius(10.0)), "cold");
    assert_eq!(
        describe_temperature(&Temperature::Celsius(20.0)),
        "comfortable"
    );
    assert_eq!(describe_temperature(&Temperature::Celsius(30.0)), "hot");
}
