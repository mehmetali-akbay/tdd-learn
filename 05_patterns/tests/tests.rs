use patterns::*;

// ===== Topic 1: Match Basics =====

#[test]
fn test_describe_number() {
    assert_eq!(describe_number(0), "zero");
    assert_eq!(describe_number(42), "positive");
    assert_eq!(describe_number(-5), "negative");
}

#[test]
fn test_grade_to_letter() {
    assert_eq!(grade_to_letter(95), "A");
    assert_eq!(grade_to_letter(85), "B");
    assert_eq!(grade_to_letter(75), "C");
    assert_eq!(grade_to_letter(65), "D");
    assert_eq!(grade_to_letter(55), "F");
    assert_eq!(grade_to_letter(100), "A");
    assert_eq!(grade_to_letter(0), "F");
}

#[test]
fn test_classify_char() {
    assert_eq!(classify_char('a'), "vowel");
    assert_eq!(classify_char('E'), "vowel");
    assert_eq!(classify_char('b'), "consonant");
    assert_eq!(classify_char('Z'), "consonant");
    assert_eq!(classify_char('5'), "digit");
    assert_eq!(classify_char('!'), "other");
}

#[test]
fn test_day_name() {
    assert_eq!(day_name(1), "Monday");
    assert_eq!(day_name(5), "Friday");
    assert_eq!(day_name(7), "Sunday");
    assert_eq!(day_name(0), "invalid");
    assert_eq!(day_name(8), "invalid");
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    assert_eq!(fizzbuzz(7), "7");
}

// ===== Topic 2: Enums with Match =====

#[test]
fn test_color_to_hex() {
    assert_eq!(color_to_hex(&Color::Red), "#FF0000");
    assert_eq!(color_to_hex(&Color::Green), "#00FF00");
    assert_eq!(color_to_hex(&Color::Blue), "#0000FF");
    assert_eq!(color_to_hex(&Color::Custom(255, 128, 0)), "#FF8000");
    assert_eq!(color_to_hex(&Color::Custom(0, 0, 0)), "#000000");
}

#[test]
fn test_area_circle() {
    let circle = Shape::Circle(1.0);
    let result = area(&circle);
    assert!((result - std::f64::consts::PI).abs() < 0.001);
}

#[test]
fn test_area_rectangle() {
    assert_eq!(area(&Shape::Rectangle(3.0, 4.0)), 12.0);
    assert_eq!(area(&Shape::Rectangle(5.0, 2.0)), 10.0);
}

#[test]
fn test_area_triangle() {
    assert_eq!(area(&Shape::Triangle(6.0, 4.0)), 12.0);
}

#[test]
fn test_describe_shape() {
    assert_eq!(describe_shape(&Shape::Circle(5.0)), "Circle with radius 5");
    assert_eq!(describe_shape(&Shape::Rectangle(3.0, 4.0)), "Rectangle 3x4");
    assert_eq!(
        describe_shape(&Shape::Triangle(3.0, 4.0)),
        "Triangle with base 3 and height 4"
    );
}

#[test]
fn test_coin_value() {
    assert_eq!(coin_value(&Coin::Penny), 1);
    assert_eq!(coin_value(&Coin::Nickel), 5);
    assert_eq!(coin_value(&Coin::Dime), 10);
    assert_eq!(coin_value(&Coin::Quarter), 25);
}

#[test]
fn test_total_value() {
    let coins = vec![Coin::Quarter, Coin::Dime, Coin::Penny];
    assert_eq!(total_value(&coins), 36);
    assert_eq!(total_value(&[]), 0);
    let all_pennies = vec![Coin::Penny, Coin::Penny, Coin::Penny];
    assert_eq!(total_value(&all_pennies), 3);
}

// ===== Topic 3: Option Matching =====

#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
    assert_eq!(safe_divide(10.0, 0.0), None);
    assert_eq!(safe_divide(0.0, 5.0), Some(0.0));
}

#[test]
fn test_first_or_default() {
    assert_eq!(first_or_default(&[10, 20, 30], 0), 10);
    assert_eq!(first_or_default(&[], 42), 42);
}

#[test]
fn test_double_option() {
    assert_eq!(double_option(Some(5)), Some(10));
    assert_eq!(double_option(None), None);
    assert_eq!(double_option(Some(0)), Some(0));
}

#[test]
fn test_parse_and_double() {
    assert_eq!(parse_and_double("5"), Some(10));
    assert_eq!(parse_and_double("abc"), None);
    assert_eq!(parse_and_double(""), None);
}

#[test]
fn test_option_string_length() {
    assert_eq!(option_string_length(Some("hello")), 5);
    assert_eq!(option_string_length(None), 0);
    assert_eq!(option_string_length(Some("")), 0);
}

#[test]
fn test_first_even() {
    assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    assert_eq!(first_even(&[1, 3, 5]), None);
    assert_eq!(first_even(&[2, 4, 6]), Some(2));
    assert_eq!(first_even(&[]), None);
}

// ===== Topic 4: If Let, While Let, Matches! =====

#[test]
fn test_describe_option() {
    assert_eq!(describe_option(Some(42)), "Got: 42");
    assert_eq!(describe_option(None), "nothing");
}

#[test]
fn test_count_items() {
    assert_eq!(count_items(vec![1, 2, 3]), 3);
    assert_eq!(count_items(vec![]), 0);
    assert_eq!(count_items(vec![99]), 1);
}

#[test]
fn test_result_to_string() {
    assert_eq!(result_to_string(Ok(42)), "Success: 42");
    assert_eq!(
        result_to_string(Err("not found".to_string())),
        "Error: not found"
    );
}

#[test]
fn test_is_in_range() {
    assert!(is_in_range(1));
    assert!(is_in_range(5));
    assert!(is_in_range(10));
    assert!(!is_in_range(0));
    assert!(!is_in_range(11));
    assert!(!is_in_range(-1));
}

#[test]
fn test_is_even_option() {
    assert!(is_even_option(Some(4)));
    assert!(!is_even_option(Some(3)));
    assert!(!is_even_option(None));
}

// ===== Topic 5: Destructuring =====

#[test]
fn test_tuple_sum() {
    assert_eq!(tuple_sum((3, 4)), 7);
    assert_eq!(tuple_sum((0, 0)), 0);
    assert_eq!(tuple_sum((-1, 1)), 0);
}

#[test]
fn test_triple_max() {
    assert_eq!(triple_max((1, 2, 3)), 3);
    assert_eq!(triple_max((3, 1, 2)), 3);
    assert_eq!(triple_max((-1, -2, -3)), -1);
}

#[test]
fn test_distance_from_origin() {
    let p = Point { x: 3.0, y: 4.0 };
    assert!((distance_from_origin(&p) - 5.0).abs() < 0.001);

    let origin = Point { x: 0.0, y: 0.0 };
    assert_eq!(distance_from_origin(&origin), 0.0);
}

#[test]
fn test_nested_sum() {
    assert_eq!(nested_sum(((1, 2), 3)), 6);
    assert_eq!(nested_sum(((10, 20), 30)), 60);
}

#[test]
fn test_swap_coordinates() {
    let p = Point { x: 3.0, y: 7.0 };
    let swapped = swap_coordinates(&p);
    assert_eq!(swapped, Point { x: 7.0, y: 3.0 });
}

#[test]
fn test_rect_area() {
    assert_eq!(rect_area((5.0, 3.0)), 15.0);
    assert_eq!(rect_area((0.0, 10.0)), 0.0);
}

// ===== Topic 6: Advanced Pattern Challenges =====

#[test]
fn test_to_celsius() {
    assert!((to_celsius(&Temperature::Celsius(100.0)) - 100.0).abs() < 0.01);
    assert!((to_celsius(&Temperature::Fahrenheit(32.0)) - 0.0).abs() < 0.01);
    assert!((to_celsius(&Temperature::Fahrenheit(212.0)) - 100.0).abs() < 0.01);
    assert!((to_celsius(&Temperature::Kelvin(273.15)) - 0.0).abs() < 0.01);
    assert!((to_celsius(&Temperature::Kelvin(373.15)) - 100.0).abs() < 0.01);
}

#[test]
fn test_describe_temperature() {
    assert_eq!(
        describe_temperature(&Temperature::Celsius(-10.0)),
        "freezing"
    );
    assert_eq!(describe_temperature(&Temperature::Celsius(5.0)), "cold");
    assert_eq!(
        describe_temperature(&Temperature::Celsius(20.0)),
        "comfortable"
    );
    assert_eq!(describe_temperature(&Temperature::Celsius(35.0)), "hot");
    assert_eq!(describe_temperature(&Temperature::Fahrenheit(32.0)), "cold"); // 0°C
    assert_eq!(describe_temperature(&Temperature::Kelvin(300.0)), "hot"); // ~26.85°C
}

#[test]
fn test_eval_simple() {
    assert_eq!(eval(&Expr::Num(42.0)), 42.0);
}

#[test]
fn test_eval_add() {
    let expr = Expr::Add(Box::new(Expr::Num(2.0)), Box::new(Expr::Num(3.0)));
    assert_eq!(eval(&expr), 5.0);
}

#[test]
fn test_eval_complex() {
    // (2 + 3) * 4 = 20
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(2.0)),
            Box::new(Expr::Num(3.0)),
        )),
        Box::new(Expr::Num(4.0)),
    );
    assert_eq!(eval(&expr), 20.0);
}

#[test]
fn test_eval_neg() {
    let expr = Expr::Neg(Box::new(Expr::Num(5.0)));
    assert_eq!(eval(&expr), -5.0);
}

#[test]
fn test_expr_to_string() {
    let expr = Expr::Add(
        Box::new(Expr::Num(2.0)),
        Box::new(Expr::Mul(
            Box::new(Expr::Num(3.0)),
            Box::new(Expr::Num(4.0)),
        )),
    );
    assert_eq!(expr_to_string(&expr), "(2 + (3 * 4))");
}

#[test]
fn test_parse_command() {
    assert_eq!(parse_command("quit"), Some(Command::Quit));
    assert_eq!(
        parse_command("echo hello world"),
        Some(Command::Echo("hello world".to_string()))
    );
    assert_eq!(
        parse_command("move 10 20"),
        Some(Command::Move { x: 10, y: 20 })
    );
    assert_eq!(
        parse_command("color red"),
        Some(Command::ChangeColor(Color::Red))
    );
    assert_eq!(
        parse_command("color green"),
        Some(Command::ChangeColor(Color::Green))
    );
    assert_eq!(parse_command("invalid"), None);
    assert_eq!(parse_command("move abc def"), None);
}

#[test]
fn test_tree_sum() {
    //       node
    //      /    \
    //    node    3
    //   /    \
    //  1      2
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)),
    );
    assert_eq!(tree_sum(&tree), 6);
    assert_eq!(tree_sum(&Tree::Leaf(42)), 42);
}

#[test]
fn test_tree_leaf_count() {
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)),
    );
    assert_eq!(tree_leaf_count(&tree), 3);
    assert_eq!(tree_leaf_count(&Tree::Leaf(1)), 1);
}

#[test]
fn test_tree_depth() {
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)),
    );
    assert_eq!(tree_depth(&tree), 2);
    assert_eq!(tree_depth(&Tree::Leaf(1)), 0);
}

#[test]
fn test_flatten_option() {
    assert_eq!(flatten_option(Some(Some(42))), Some(42));
    assert_eq!(flatten_option(Some(None)), None);
    assert_eq!(flatten_option(None), None);
}

#[test]
fn test_count_results() {
    let results: Vec<Result<i32, String>> = vec![
        Ok(1),
        Err("err".to_string()),
        Ok(3),
        Err("fail".to_string()),
        Ok(5),
    ];
    assert_eq!(count_results(&results), (3, 2));
    assert_eq!(count_results(&[]), (0, 0));
}

#[test]
fn test_collect_successes() {
    let results: Vec<Result<i32, String>> = vec![
        Ok(1),
        Err("err".to_string()),
        Ok(3),
        Err("fail".to_string()),
        Ok(5),
    ];
    assert_eq!(collect_successes(results), vec![1, 3, 5]);
}

#[test]
fn test_map_or_default() {
    fn double(x: i32) -> i32 {
        x * 2
    }
    assert_eq!(map_or_default(Some(5), double, 0), 10);
    assert_eq!(map_or_default(None, double, 42), 42);
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_quadrant() {
    assert_eq!(quadrant(1.0, 1.0), "Q1");
    assert_eq!(quadrant(-1.0, 1.0), "Q2");
    assert_eq!(quadrant(-1.0, -1.0), "Q3");
    assert_eq!(quadrant(0.0, 0.0), "origin");
}

#[test]
fn test_classify_ascii() {
    assert_eq!(classify_ascii('a'), "vowel");
    assert_eq!(classify_ascii('b'), "consonant");
    assert_eq!(classify_ascii('5'), "digit");
    assert_eq!(classify_ascii('!'), "other");
}

#[test]
fn test_parse_cmd_str() {
    assert_eq!(parse_cmd_str("say hello"), Some(("say", "hello")));
    assert_eq!(parse_cmd_str("quit"), Some(("quit", "")));
    assert_eq!(parse_cmd_str(""), None);
}

#[test]
fn test_flatten_nested_option() {
    assert_eq!(flatten_nested_option(Some(Some(42))), Some(42));
    assert_eq!(flatten_nested_option(Some(None::<i32>)), None);
    assert_eq!(flatten_nested_option(None::<Option<i32>>), None);
}

#[test]
fn test_sum_if_both_positive() {
    assert_eq!(sum_if_both_positive(Some(3), Some(4)), Some(7));
    assert_eq!(sum_if_both_positive(Some(-1), Some(4)), None);
    assert_eq!(sum_if_both_positive(None, Some(4)), None);
}
