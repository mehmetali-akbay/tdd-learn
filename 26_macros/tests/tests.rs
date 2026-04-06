use macros::*;

// ============================================
// Easy: Foundation Macros
// ============================================

#[test]
fn test_easy_hashmap_empty() {
    let map: std::collections::HashMap<&str, i32> = hashmap!();
    assert!(map.is_empty());
}

#[test]
fn test_easy_hashmap_entries() {
    let map: std::collections::HashMap<&str, i32> = hashmap! {
        "a" => 1,
        "b" => 2,
        "c" => 3,
    };
    assert_eq!(map.len(), 3);
    assert_eq!(map["a"], 1);
    assert_eq!(map["c"], 3);
}

#[test]
fn test_easy_vec_of() {
    let v: Vec<i32> = vec_of![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
    let empty: Vec<i32> = vec_of![];
    assert!(empty.is_empty());
}

#[test]
fn test_easy_min_max() {
    assert_eq!(min!(5, 2, 8, 1, 3), 1);
    assert_eq!(max!(5, 2, 8, 1, 3), 8);
    assert_eq!(min!(42), 42);
    assert_eq!(max!(42), 42);
}

#[test]
fn test_easy_repeat_vec() {
    let repeated: Vec<i32> = repeat_vec![7; 3];
    assert_eq!(repeated, vec![7, 7, 7]);
    let empty: Vec<i32> = repeat_vec![5; 0];
    assert!(empty.is_empty());
}

#[test]
fn test_easy_count() {
    assert_eq!(count!(), 0);
    assert_eq!(count!(1), 1);
    assert_eq!(count!(1, 2, 3, 4, 5), 5);
}

#[test]
fn test_easy_sum() {
    assert_eq!(sum!(), 0);
    assert_eq!(sum!(1, 2, 3), 6);
    assert_eq!(sum!(10, 20, 30, 40), 100);
}

#[test]
fn test_easy_avg() {
    let result: f64 = avg!(2, 4, 6);
    assert!((result - 4.0).abs() < 0.001);
    let result2: f64 = avg!(10);
    assert!((result2 - 10.0).abs() < 0.001);
}

#[test]
fn test_easy_newtype() {
    newtype!(Meters, f64);
    let m = Meters::new(5.0);
    assert_eq!(*m.value(), 5.0);
    assert_eq!(m, Meters(5.0));
}

// ============================================
// Medium: Multi-arm + Item-generation Macros
// ============================================

#[test]
fn test_medium_when() {
    assert_eq!(when!(true => 42), Some(42));
    assert_eq!(when!(false => 42), None);
    assert_eq!(when!(true => "yes", else => "no"), "yes");
    assert_eq!(when!(false => "yes", else => "no"), "no");
}

#[test]
fn test_medium_str_join() {
    assert_eq!(str_join!("hello", " ", "world"), "hello world");
    assert_eq!(str_join!(), "");
}

#[test]
fn test_medium_str_join_sep() {
    let result = str_join!(", "; "a", "b", "c");
    assert_eq!(result, "a, b, c");
}

#[test]
fn test_medium_assert_between() {
    assert_between!(5, 1, 10);
    assert_between!(1, 1, 10);
    assert_between!(10, 1, 10);
}

#[test]
fn test_medium_coalesce() {
    let a: Option<i32> = None;
    let b: Option<i32> = None;
    let c = Some(42);
    assert_eq!(coalesce!(a, b, c), Some(42));
    assert_eq!(coalesce!(None::<i32>, None::<i32>), None::<i32>);
}

#[test]
fn test_medium_string_enum() {
    string_enum!(Color {
        Red => "red",
        Green => "green",
        Blue => "blue",
    });

    let c = Color::Red;
    assert_eq!(format!("{}", c), "red");
    assert_eq!(Color::from_str_custom("green"), Some(Color::Green));
    assert_eq!(Color::from_str_custom("purple"), None);
    assert_eq!(Color::variants(), vec!["red", "green", "blue"]);
}

#[test]
fn test_medium_builder() {
    builder!(ServerConfig {
        host: String = "localhost".to_string(),
        port: u16 = 8080,
        debug: bool = false,
    });

    let config = ServerConfig::builder()
        .host("0.0.0.0".to_string())
        .port(3000)
        .debug(true);

    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 3000);
    assert!(config.debug);
}

#[test]
fn test_medium_builder_defaults() {
    builder!(AppConfig {
        name: String = "app".to_string(),
        verbose: bool = false,
    });

    let config = AppConfig::builder();
    assert_eq!(config.name, "app");
    assert!(!config.verbose);
}

// ============================================
// Hard: Utility + Recursive Macros
// ============================================

#[test]
fn test_hard_timed() {
    let (result, elapsed) = timed!({
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        sum
    });
    assert_eq!(result, 499500);
    assert!(elapsed < 1_000_000);
}

#[test]
fn test_hard_retry_success_after_failures() {
    let mut attempts = 0;
    let result: Result<i32, &str> = retry!(5, {
        attempts += 1;
        if attempts < 3 {
            Err("not yet")
        } else {
            Ok(99)
        }
    });

    assert_eq!(result, Ok(99));
    assert_eq!(attempts, 3);
}

#[test]
fn test_hard_retry_failure_returns_last_error() {
    let mut attempts = 0;
    let result: Result<i32, &str> = retry!(3, {
        attempts += 1;
        Err("still failing")
    });

    assert_eq!(result, Err("still failing"));
    assert_eq!(attempts, 3);
}

#[test]
fn test_hard_match_or() {
    let x = 42;
    let result = match_or!(x, "other",
        1 => "one",
        42 => "answer",
    );
    assert_eq!(result, "answer");

    let y = 99;
    let result2 = match_or!(y, "unknown",
        1 => "one",
        2 => "two",
    );
    assert_eq!(result2, "unknown");
}

#[test]
fn test_hard_dbg_expr() {
    let value = dbg_expr!(2 + 3);
    assert_eq!(value, 5);
}

#[test]
fn test_hard_reverse_vec() {
    assert_eq!(reverse_vec![1, 2, 3], vec![3, 2, 1]);
    let empty: Vec<i32> = reverse_vec![];
    assert!(empty.is_empty());
}

#[test]
fn test_hard_nest() {
    assert_eq!(nest!(), ());
    assert_eq!(nest!(1), (1, ()));
    assert_eq!(nest!(1, 2), (1, (2, ())));
    assert_eq!(nest!(1, 2, 3), (1, (2, (3, ()))));
}

#[test]
fn test_hard_chain_calls() {
    let s = " Hello World ".to_string();
    let result = chain_calls!(s, trim(), to_uppercase());
    assert_eq!(result, "HELLO WORLD");
}

#[test]
fn test_hard_const_add() {
    const RESULT: i32 = const_add!(10, 20);
    assert_eq!(RESULT, 30);
}
