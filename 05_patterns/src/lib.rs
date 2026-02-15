// ============================================
// Topic 1: Match Basics
// Learn: Pattern matching on numbers, chars
// ============================================

/// Describe a number: "zero", "positive", or "negative"
pub fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        n if n > 0 => "positive",
        _ => "negative",
    }
}

/// Convert a grade number to a letter
/// 90-100 => "A", 80-89 => "B", 70-79 => "C", 60-69 => "D", below 60 => "F"
pub fn grade_to_letter(grade: u32) -> &'static str {
    match grade {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

/// Classify a character: "vowel", "consonant", "digit", or "other"
pub fn classify_char(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => "vowel",
        'a'..='z' | 'A'..='Z' => "consonant",
        '0'..='9' => "digit",
        _ => "other",
    }
}

/// Return the day name for a number (1=Monday, 7=Sunday)
/// Return "invalid" for anything else
pub fn day_name(day: u32) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "invalid",
    }
}

/// FizzBuzz: divisible by 15 => "FizzBuzz", by 3 => "Fizz", by 5 => "Buzz", else the number
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

// ============================================
// Topic 2: Enums with Match
// Learn: Defining enums, matching on variants
// ============================================

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

/// Return the hex string for a color
/// Red => "#FF0000", Green => "#00FF00", Blue => "#0000FF", Custom(r,g,b) => "#RRGGBB"
pub fn color_to_hex(color: &Color) -> String {
    match color {
        Color::Red => "#FF0000".to_string(),
        Color::Green => "#00FF00".to_string(),
        Color::Blue => "#0000FF".to_string(),
        Color::Custom(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
    }
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

/// Calculate the area of a shape
pub fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(b, h) => 0.5 * b * h,
    }
}

/// Return a description of the shape
pub fn describe_shape(shape: &Shape) -> String {
    match shape {
        Shape::Circle(r) => format!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => format!("Rectangle {}x{}", w, h),
        Shape::Triangle(b, h) => format!("Triangle with base {} and height {}", b, h),
    }
}

#[derive(Debug, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/// Return the value of a coin in cents
pub fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/// Calculate the total value of a collection of coins
pub fn total_value(coins: &[Coin]) -> u32 {
    coins.iter().map(coin_value).sum()
}

// ============================================
// Topic 3: Option Matching
// Learn: Working with Option<T>
// ============================================

/// Divide two numbers, return None if divisor is zero
pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Get the first element of a slice, or return a default
pub fn first_or_default(v: &[i32], default: i32) -> i32 {
    v.first().copied().unwrap_or(default)
}

/// Double the value inside an Option, return None if None
pub fn double_option(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

/// Chain: parse a string to i32, then double it
pub fn parse_and_double(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().map(|x| x * 2)
}

/// Return the length of the string inside an Option, or 0 if None
pub fn option_string_length(opt: Option<&str>) -> usize {
    match opt {
        Some(s) => s.len(),
        None => 0,
    }
}

/// Find the first even number in a slice
pub fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

// ============================================
// Topic 4: If Let, While Let, Matches!
// Learn: Concise pattern matching alternatives
// ============================================

/// Extract the value from an Option and format it, or return "nothing"
pub fn describe_option(opt: Option<i32>) -> String {
    if let Some(value) = opt {
        format!("Got: {}", value)
    } else {
        "nothing".to_string()
    }
}

/// Count how many items can be popped from a vec (using while let)
pub fn count_items(mut v: Vec<i32>) -> usize {
    let mut count = 0;
    while v.pop().is_some() {
        count += 1;
    }
    count
}

/// Process a Result: return the value as string, or the error message
pub fn result_to_string(result: Result<i32, String>) -> String {
    if let Ok(value) = result {
        format!("Success: {}", value)
    } else if let Err(e) = result {
        format!("Error: {}", e)
    } else {
        unreachable!()
    }
}

/// Check if a number is in range 1..=10 using matches! macro
pub fn is_in_range(n: i32) -> bool {
    matches!(n, 1..=10)
}

/// Check if an Option contains an even number
pub fn is_even_option(opt: Option<i32>) -> bool {
    matches!(opt, Some(x) if x % 2 == 0)
}

// ============================================
// Topic 5: Destructuring
// Learn: Breaking apart tuples, structs
// ============================================

/// Destructure a tuple and return the sum
pub fn tuple_sum(pair: (i32, i32)) -> i32 {
    let (a, b) = pair;
    a + b
}

/// Destructure a triple and return the largest
pub fn triple_max(triple: (i32, i32, i32)) -> i32 {
    let (a, b, c) = triple;
    a.max(b).max(c)
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Calculate distance from origin using destructuring
pub fn distance_from_origin(point: &Point) -> f64 {
    let Point { x, y } = point;
    (x * x + y * y).sqrt()
}

/// Destructure a nested tuple ((a, b), c) and return a + b + c
pub fn nested_sum(nested: ((i32, i32), i32)) -> i32 {
    let ((a, b), c) = nested;
    a + b + c
}

/// Swap x and y coordinates of a Point
pub fn swap_coordinates(point: &Point) -> Point {
    let Point { x, y } = point;
    Point { x: *y, y: *x }
}

/// Compute area of a rectangle given as (width, height) — destructure in params
pub fn rect_area((width, height): (f64, f64)) -> f64 {
    width * height
}

// ============================================
// Topic 6: Advanced Pattern Challenges
// Learn: Recursive enums, state machines, complex matching
// ============================================

#[derive(Debug, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

/// Convert any temperature to Celsius
pub fn to_celsius(temp: &Temperature) -> f64 {
    match temp {
        Temperature::Celsius(c) => *c,
        Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
        Temperature::Kelvin(k) => k - 273.15,
    }
}

/// Describe a temperature: "freezing" (<0°C), "cold" (0-15), "comfortable" (15-25), "hot" (>25)
pub fn describe_temperature(temp: &Temperature) -> &'static str {
    let celsius = to_celsius(temp);
    match celsius {
        c if c < 0.0 => "freezing",
        c if c < 15.0 => "cold",
        c if c < 25.0 => "comfortable",
        _ => "hot",
    }
}

/// A recursive expression tree
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

/// Evaluate a math expression tree recursively
pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(e) => -eval(e),
    }
}

/// Pretty-print an expression tree
pub fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Num(n) => format!("{}", n),
        Expr::Add(a, b) => format!("({} + {})", expr_to_string(a), expr_to_string(b)),
        Expr::Mul(a, b) => format!("({} * {})", expr_to_string(a), expr_to_string(b)),
        Expr::Neg(e) => format!("(-{})", expr_to_string(e)),
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(Color),
}

/// Parse a command string into a Command enum
/// "quit" => Quit, "echo hello world" => Echo("hello world"),
/// "move 10 20" => Move { x: 10, y: 20 }, "color red" => ChangeColor(Red)
pub fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    match parts.as_slice() {
        ["quit"] => Some(Command::Quit),
        ["echo", msg] => Some(Command::Echo(msg.to_string())),
        ["move", coords] => {
            let nums: Vec<&str> = coords.split_whitespace().collect();
            if nums.len() == 2 {
                if let (Ok(x), Ok(y)) = (nums[0].parse(), nums[1].parse()) {
                    return Some(Command::Move { x, y });
                }
            }
            None
        }
        ["color", "red"] => Some(Command::ChangeColor(Color::Red)),
        ["color", "green"] => Some(Command::ChangeColor(Color::Green)),
        ["color", "blue"] => Some(Command::ChangeColor(Color::Blue)),
        _ => None,
    }
}

/// A recursive binary tree
#[derive(Debug, PartialEq)]
pub enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),
}

/// Sum all leaf values in a binary tree
pub fn tree_sum(tree: &Tree) -> i32 {
    match tree {
        Tree::Leaf(v) => *v,
        Tree::Node(left, right) => tree_sum(left) + tree_sum(right),
    }
}

/// Count the number of leaves in a tree
pub fn tree_leaf_count(tree: &Tree) -> usize {
    match tree {
        Tree::Leaf(_) => 1,
        Tree::Node(left, right) => tree_leaf_count(left) + tree_leaf_count(right),
    }
}

/// Find the depth of a tree (longest path from root to leaf)
pub fn tree_depth(tree: &Tree) -> usize {
    match tree {
        Tree::Leaf(_) => 0,
        Tree::Node(left, right) => 1 + tree_depth(left).max(tree_depth(right)),
    }
}

/// Flatten nested Options: Option<Option<T>> => Option<T>
pub fn flatten_option(opt: Option<Option<i32>>) -> Option<i32> {
    match opt {
        Some(Some(x)) => Some(x),
        _ => None,
    }
}

/// Classify a list of Results: count (successes, failures)
pub fn count_results(results: &[Result<i32, String>]) -> (usize, usize) {
    let successes = results.iter().filter(|r| r.is_ok()).count();
    let failures = results.len() - successes;
    (successes, failures)
}

/// Collect all Ok values from a list of Results, discarding errors
pub fn collect_successes(results: Vec<Result<i32, String>>) -> Vec<i32> {
    results.into_iter().filter_map(|r| r.ok()).collect()
}

/// Apply a function to the value inside an Option, or return default
pub fn map_or_default(opt: Option<i32>, f: fn(i32) -> i32, default: i32) -> i32 {
    match opt {
        Some(x) => f(x),
        None => default,
    }
}

// ============================================
// Topic 7: Extra Practice
// Learn: More pattern matching and destructuring exercises
// ============================================

/// Describe a point's quadrant using pattern matching on (x, y) signs.
pub fn quadrant(x: f64, y: f64) -> &'static str {
    match (x > 0.0, y > 0.0, x == 0.0, y == 0.0) {
        (_, _, true, true) => "origin",
        (_, _, true, _) => "y-axis",
        (_, _, _, true) => "x-axis",
        (true, true, _, _) => "Q1",
        (false, true, _, _) => "Q2",
        (false, false, _, _) => "Q3",
        (true, false, _, _) => "Q4",
    }
}

/// Classify a character as vowel, consonant, digit, or other.
pub fn classify_ascii(c: char) -> &'static str {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'a'..='z' => "consonant",
        '0'..='9' => "digit",
        _ => "other",
    }
}

/// Parse a simple command string: "quit" -> Some(("quit","")), "" -> None
pub fn parse_cmd_str(input: &str) -> Option<(&str, &str)> {
    match input.split_once(' ') {
        Some((cmd, arg)) => Some((cmd, arg)),
        None if !input.is_empty() => Some((input, "")),
        _ => None,
    }
}

/// Nested pattern: extract value from Option<Option<T>>
pub fn flatten_nested_option<T>(opt: Option<Option<T>>) -> Option<T> {
    match opt {
        Some(Some(v)) => Some(v),
        _ => None,
    }
}

/// Return the sum if both are Some and positive.
pub fn sum_if_both_positive(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    if let (Some(x), Some(y)) = (a, b) {
        if x > 0 && y > 0 { return Some(x + y); }
    }
    None
}
