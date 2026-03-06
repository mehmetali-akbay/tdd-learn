// ============================================
// Topic 1: Match Basics
// Learn: Pattern matching on numbers, chars
// ============================================
// Guidance:
// - Prefer a single `match` expression where possible.
// - Use ranges for classification instead of long if/else chains.

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
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'a'..='z' => "consonant",
        a if a.is_digit(35) => "digit",
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
        Shape::Circle(a) => a * a * std::f64::consts::PI,
        Shape::Rectangle(a, b) => a * b,
        Shape::Triangle(a, b) => a * b * 0.5,
    }
}

/// Return a description of the shape
pub fn describe_shape(shape: &Shape) -> String {
    match shape {
        Shape::Circle(a) => format!("Circle with radius {a}"),
        Shape::Rectangle(a, b) => format!("Rectangle {a}x{b}"),
        Shape::Triangle(a, b) => format!("Triangle with base {a} and height {b}"),
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

/// A Chapter-6-style message enum with mixed variant shapes.
#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    /// Simulate handling a message as a readable string.
    pub fn call(&self) -> String {
        match self {
            Message::Quit => "Quit".to_string(),
            Message::Move { x, y } => format!("Move to ({x}, {y})"),
            Message::Write(w) => format!("Text: {w}"),
            Message::ChangeColor(a, b, c) => format!("Color: ({a}, {b}, {c})"),
        }
    }
}

// ============================================
// Topic 3: Option Matching
// Learn: Working with Option<T>
// ============================================
// Guidance:
// - Solve with explicit pattern matching first (`match` / `if let`).
// - Refactor to combinators only after tests pass.

/// Divide two numbers, return None if divisor is zero
pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        return Some(a / b);
    } else {
        return None;
    }
}

/// Get the first element of a slice, or return a default
pub fn first_or_default(v: &[i32], default: i32) -> i32 {
    v.first().copied().unwrap_or(default)
}

/// Double the value inside an Option, return None if None
pub fn double_option(opt: Option<i32>) -> Option<i32> {
    opt.map(|e| e * 2)
}

/// Chain: parse a string to i32, then double it
pub fn parse_and_double(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().map(|e| e * 2)
}

/// Return the length of the string inside an Option, or 0 if None
pub fn option_string_length(opt: Option<&str>) -> usize {
    match opt {
        Some(val) => val.chars().count(),
        None => 0,
    }
}

/// Find the first even number in a slice
pub fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&e| e % 2 == 0).copied()
}

/// Book-style exercise: add one to an optional number.
pub fn plus_one(opt: Option<i32>) -> Option<i32> {
    match opt {
        Some(val) => Some(val + 1),
        None => None,
    }
}

// ============================================
// Topic 4: If Let, While Let, Matches!
// Learn: Concise pattern matching alternatives
// ============================================

/// Extract the value from an Option and format it, or return "nothing"
pub fn describe_option(opt: Option<i32>) -> String {
    match opt {
        Some(i) => format!("Got: {i}"),
        None => "nothing".to_string(),
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
    match result {
        Ok(val) => format!("Success: {val}"),
        Err(msg) => format!("Error: {msg}"),
    }
}

/// Check if a number is in range 1..=10 using matches! macro
pub fn is_in_range(n: i32) -> bool {
    matches!(n, 1..=10)
}

/// Check if an Option contains an even number
pub fn is_even_option(opt: Option<i32>) -> bool {
    matches!(opt, Some(x) if x%2==0)
}

/// U.S. state payload used by quarter variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

/// Coin example that carries extra data in one variant.
#[derive(Debug, PartialEq)]
pub enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/// Extract quarter state using `if let`, otherwise return None.
pub fn quarter_state(coin: &CoinWithState) -> Option<UsState> {
    if let CoinWithState::Quarter(state) = coin {
        Some(*state)
    } else {
        None
    }
}

/// Parse exactly two integers from whitespace-separated input using `let...else`.
pub fn parse_pair(input: &str) -> Option<(i32, i32)> {
    let mut it = input.split_whitespace();
    let (Some(first), Some(second), None) = (it.next(), it.next(), it.next()) else {
        return None;
    };
    let Ok(first_cnv) = first.parse::<i32>() else {
        return None;
    };
    let Ok(second_cnv) = second.parse::<i32>() else {
        return None;
    };
    Some((first_cnv, second_cnv))
}

/// Demonstrates that non-exhaustive `match` expressions do not compile.
///
/// ```compile_fail
/// enum Coin {
///     Penny,
///     Nickel,
/// }
///
/// fn cents(c: Coin) -> u8 {
///     match c {
///         Coin::Penny => 1,
///     }
/// }
/// ```
pub fn exhaustiveness_demo() -> &'static str {
    todo!()
}

// ============================================
// Topic 5: Destructuring
// Learn: Breaking apart tuples, structs
// ============================================

/// Destructure a tuple and return the sum
pub fn tuple_sum(pair: (i32, i32)) -> i32 {
    let (val, val2) = pair;
    val + val2
}

/// Destructure a triple and return the largest
pub fn triple_max(triple: (i32, i32, i32)) -> i32 {
    let (v1, v2, v3) = triple;
    v1.max(v2).max(v3)
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
// Guidance:
// - Keep recursive cases tiny and delegate to helper calls.
// - Ensure every enum variant has an explicit behavior path.

#[derive(Debug, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

/// Convert any temperature to Celsius
pub fn to_celsius(temp: &Temperature) -> f64 {
    match temp {
        Temperature::Celsius(val) => *val,
        Temperature::Fahrenheit(val) => (*val - 32.0) * (5.0 / 9.0),
        Temperature::Kelvin(val) => val - 273.15,
    }
}

/// Describe a temperature: "freezing" (<0°C), "cold" (0-15), "comfortable" (15-25), "hot" (>25)
pub fn describe_temperature(temp: &Temperature) -> &'static str {
    let celsius = to_celsius(temp);
    match celsius {
        c if c <0.0 => "freezing",
        c if c <15.0 => "cold",
        c if c < 25.0 => "comfortable",
        _ => "hot"
    }

}
