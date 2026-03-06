use std::collections::HashMap;
use traits::*;

// ============================================
// Topic 1: Basic Traits — Defining & Implementing
// ============================================

#[test]
fn test_pet_describe() {
    let p = Pet::new("Buddy", "dog");
    assert_eq!(p.describe(), "Buddy the dog");
}

#[test]
fn test_car_describe() {
    let c = Car::new("Toyota", "Camry", 2023);
    assert_eq!(c.describe(), "2023 Toyota Camry");
}

#[test]
fn test_car_is_vintage() {
    assert!(Car::new("Ford", "Mustang", 1967).is_vintage());
    assert!(!Car::new("Tesla", "Model S", 2023).is_vintage());
}

#[test]
fn test_book_describe() {
    let b = Book::new("Rust Book", "Steve", 560);
    assert_eq!(b.describe(), "\"Rust Book\" by Steve (560 pages)");
}

#[test]
fn test_print_description_trait_object() {
    let pet = Pet::new("Luna", "cat");
    let car = Car::new("Honda", "Civic", 2020);
    assert_eq!(print_description(&pet), "Luna the cat");
    assert_eq!(print_description(&car), "2020 Honda Civic");
}

#[test]
fn test_describe_all() {
    let pet = Pet::new("Max", "dog");
    let car = Car::new("BMW", "M3", 2021);
    let book = Book::new("Dune", "Herbert", 412);
    let items: Vec<&dyn Describable> = vec![&pet, &car, &book];
    let descriptions = describe_all(&items);
    assert_eq!(descriptions.len(), 3);
    assert_eq!(descriptions[0], "Max the dog");
    assert_eq!(descriptions[1], "2021 BMW M3");
    assert_eq!(descriptions[2], "\"Dune\" by Herbert (412 pages)");
}

#[test]
fn test_describe_all_empty() {
    let items: Vec<&dyn Describable> = vec![];
    assert!(describe_all(&items).is_empty());
}

#[test]
fn test_longest_description() {
    let pet = Pet::new("Bo", "cat");
    let book = Book::new("The Lord of the Rings", "Tolkien", 1178);
    let items: Vec<&dyn Describable> = vec![&pet, &book];
    let longest = longest_description(&items);
    assert!(longest.is_some());
    assert_eq!(
        longest.unwrap(),
        "\"The Lord of the Rings\" by Tolkien (1178 pages)"
    );
}

#[test]
fn test_longest_description_empty() {
    let items: Vec<&dyn Describable> = vec![];
    assert!(longest_description(&items).is_none());
}

// ============================================
// Topic 2: Display & Debug
// ============================================

#[test]
fn test_coord_display() {
    let c = Coord::new(3.5, -2.1);
    assert_eq!(format!("{}", c), "(3.5, -2.1)");
}

#[test]
fn test_coord_distance_to() {
    let a = Coord::new(0.0, 0.0);
    let b = Coord::new(3.0, 4.0);
    assert!((a.distance_to(&b) - 5.0).abs() < 1e-9);
}

#[test]
fn test_coord_distance_to_self() {
    let a = Coord::new(7.0, 3.0);
    assert!((a.distance_to(&a) - 0.0).abs() < 1e-9);
}

#[test]
fn test_temperature_display_celsius() {
    let t = Temperature::celsius(36.6);
    assert_eq!(format!("{}", t), "36.6°C");
}

#[test]
fn test_temperature_display_fahrenheit() {
    let t = Temperature::fahrenheit(98.6);
    assert_eq!(format!("{}", t), "98.6°F");
}

#[test]
fn test_temperature_to_celsius() {
    let t = Temperature::fahrenheit(212.0).to_celsius();
    assert!((t.value - 100.0).abs() < 0.1);
    assert_eq!(t.unit, TempUnit::Celsius);
}

#[test]
fn test_temperature_to_fahrenheit() {
    let t = Temperature::celsius(0.0).to_fahrenheit();
    assert!((t.value - 32.0).abs() < 0.1);
    assert_eq!(t.unit, TempUnit::Fahrenheit);
}

#[test]
fn test_temperature_celsius_to_celsius_unchanged() {
    let t = Temperature::celsius(25.0);
    let t2 = t.to_celsius();
    assert!((t2.value - 25.0).abs() < 1e-9);
}

#[test]
fn test_color_display_hex() {
    let c = Color::new(255, 128, 0);
    assert_eq!(format!("{}", c), "#FF8000");
}

#[test]
fn test_color_display_black() {
    let c = Color::new(0, 0, 0);
    assert_eq!(format!("{}", c), "#000000");
}

#[test]
fn test_color_is_grayscale() {
    assert!(Color::new(128, 128, 128).is_grayscale());
    assert!(Color::new(0, 0, 0).is_grayscale());
    assert!(!Color::new(255, 128, 0).is_grayscale());
}

#[test]
fn test_color_brightness() {
    let white = Color::new(255, 255, 255);
    assert!((white.brightness() - 255.0).abs() < 1.0);
    let black = Color::new(0, 0, 0);
    assert!((black.brightness() - 0.0).abs() < 1e-9);
}

// ============================================
// Topic 3: Default Trait & Builder Pattern
// ============================================

#[test]
fn test_config_defaults() {
    let c = Config::default();
    assert_eq!(c.host, "localhost");
    assert_eq!(c.port, 8080);
    assert!(!c.debug);
    assert_eq!(c.max_connections, 100);
}

#[test]
fn test_config_builder_chain() {
    let c = Config::default()
        .with_host("0.0.0.0")
        .with_port(3000)
        .with_debug(true)
        .with_max_connections(500);
    assert_eq!(c.host, "0.0.0.0");
    assert_eq!(c.port, 3000);
    assert!(c.debug);
    assert_eq!(c.max_connections, 500);
}

#[test]
fn test_config_summary() {
    let c = Config::default().with_debug(true).with_port(3000);
    assert_eq!(c.summary(), "localhost:3000 (debug, max_conn=100)");
}

#[test]
fn test_config_summary_release() {
    let c = Config::default();
    assert_eq!(c.summary(), "localhost:8080 (release, max_conn=100)");
}

#[test]
fn test_config_is_default() {
    assert!(Config::default().is_default());
    assert!(!Config::default().with_port(9090).is_default());
}

#[test]
fn test_game_settings_default() {
    let g = GameSettings::default();
    assert_eq!(g.difficulty, "normal");
    assert!(g.sound);
    assert!(!g.fullscreen);
}

#[test]
fn test_game_settings_builder() {
    let g = GameSettings::default()
        .with_difficulty("hard")
        .with_sound(false)
        .with_fullscreen(true);
    assert_eq!(g.difficulty, "hard");
    assert!(!g.sound);
    assert!(g.fullscreen);
}

// ============================================
// Topic 4: Operator Overloading
// ============================================

#[test]
fn test_vec2d_add() {
    let a = Vec2d::new(1.0, 2.0);
    let b = Vec2d::new(3.0, 4.0);
    assert_eq!(a + b, Vec2d::new(4.0, 6.0));
}

#[test]
fn test_vec2d_sub() {
    let a = Vec2d::new(5.0, 7.0);
    let b = Vec2d::new(3.0, 2.0);
    assert_eq!(a - b, Vec2d::new(2.0, 5.0));
}

#[test]
fn test_vec2d_neg() {
    let a = Vec2d::new(3.0, -4.0);
    assert_eq!(-a, Vec2d::new(-3.0, 4.0));
}

#[test]
fn test_vec2d_magnitude() {
    let v = Vec2d::new(3.0, 4.0);
    assert!((v.magnitude() - 5.0).abs() < 1e-9);
}

#[test]
fn test_vec2d_dot_product() {
    let a = Vec2d::new(1.0, 2.0);
    let b = Vec2d::new(3.0, 4.0);
    assert!((a.dot(&b) - 11.0).abs() < 1e-9);
}

#[test]
fn test_vec2d_display() {
    let v = Vec2d::new(1.5, -2.3);
    assert_eq!(format!("{}", v), "(1.5, -2.3)");
}

#[test]
fn test_money_new_and_parts() {
    let m = Money::new(5, 75);
    assert_eq!(m.dollars(), 5);
    assert_eq!(m.cents(), 75);
}

#[test]
fn test_money_add() {
    let a = Money::new(3, 50);
    let b = Money::new(2, 75);
    let sum = a + b;
    assert_eq!(sum.dollars(), 6);
    assert_eq!(sum.cents(), 25);
}

#[test]
fn test_money_sub() {
    let a = Money::new(10, 0);
    let b = Money::new(3, 50);
    let diff = a - b;
    assert_eq!(diff.dollars(), 6);
    assert_eq!(diff.cents(), 50);
}

#[test]
fn test_money_comparison() {
    assert!(Money::new(10, 0) > Money::new(5, 99));
    assert!(Money::new(5, 50) == Money::new(5, 50));
    assert!(Money::new(0, 1) < Money::new(0, 2));
}

#[test]
fn test_money_display() {
    assert_eq!(format!("{}", Money::new(42, 7)), "$42.07");
    assert_eq!(format!("{}", Money::new(0, 0)), "$0.00");
}

#[test]
fn test_money_display_negative() {
    let m = Money::new(3, 0) - Money::new(5, 0);
    assert_eq!(format!("{}", m), "-$2.00");
}

// ============================================
// Topic 5: From / Into Conversions
// ============================================

#[test]
fn test_celsius_to_fahrenheit() {
    let f: Fahrenheit = Celsius(100.0).into();
    assert!((f.0 - 212.0).abs() < 0.1);
}

#[test]
fn test_fahrenheit_to_celsius() {
    let c: Celsius = Fahrenheit(32.0).into();
    assert!((c.0 - 0.0).abs() < 0.1);
}

#[test]
fn test_slug_from_str() {
    let s: Slug = "Hello World".into();
    assert_eq!(s.0, "hello-world");
}

#[test]
fn test_slug_from_str_with_spaces_and_case() {
    let s = Slug::from("  Rust Is Great  ");
    assert_eq!(format!("{}", s), "rust-is-great");
}

#[test]
fn test_rgb_from_tuple() {
    let c: Rgb = (255, 128, 0).into();
    assert_eq!(c, Rgb { r: 255, g: 128, b: 0 });
}

#[test]
fn test_rgb_to_tuple() {
    let t: (u8, u8, u8) = Rgb { r: 10, g: 20, b: 30 }.into();
    assert_eq!(t, (10, 20, 30));
}

#[test]
fn test_coord_from_array() {
    let c: Coord = [3.0, 4.0].into();
    assert_eq!(c, Coord::new(3.0, 4.0));
}

#[test]
fn test_book_from_tuple() {
    let b: Book = ("1984", "Orwell", 328).into();
    assert_eq!(b.title, "1984");
    assert_eq!(b.author, "Orwell");
    assert_eq!(b.pages, 328);
}

#[test]
fn test_stats_from_slice() {
    let data = [10, 20, 30, 40, 50];
    let s = Stats::from(data.as_slice());
    assert_eq!(s.count, 5);
    assert_eq!(s.sum, 150);
    assert_eq!(s.min, 10);
    assert_eq!(s.max, 50);
    assert!((s.average() - 30.0).abs() < 1e-9);
}

#[test]
fn test_stats_from_empty_slice() {
    let data: [i32; 0] = [];
    let s = Stats::from(data.as_slice());
    assert_eq!(s.count, 0);
    assert!((s.average() - 0.0).abs() < 1e-9);
}

#[test]
fn test_stats_from_single() {
    let data = [42];
    let s = Stats::from(data.as_slice());
    assert_eq!(s.count, 1);
    assert_eq!(s.min, 42);
    assert_eq!(s.max, 42);
    assert_eq!(s.sum, 42);
}

#[test]
fn test_priority_from_str() {
    assert_eq!(Priority::from("low"), Priority::Low);
    assert_eq!(Priority::from("HIGH"), Priority::High);
    assert_eq!(Priority::from("crit"), Priority::Critical);
    assert_eq!(Priority::from("unknown"), Priority::Medium); // default
}

#[test]
fn test_priority_display() {
    assert_eq!(format!("{}", Priority::Critical), "critical");
    assert_eq!(format!("{}", Priority::Low), "low");
}

// ============================================
// Topic 6: Trait Bounds & Generic Functions
// ============================================

#[test]
fn test_format_all_ints() {
    assert_eq!(format_all(&[1, 2, 3]), "1, 2, 3");
}

#[test]
fn test_format_all_strings() {
    assert_eq!(
        format_all(&["hello".to_string(), "world".to_string()]),
        "hello, world"
    );
}

#[test]
fn test_format_all_empty() {
    let items: Vec<i32> = vec![];
    assert_eq!(format_all(&items), "");
}

#[test]
fn test_largest() {
    assert_eq!(largest(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
    assert_eq!(largest(&['a', 'z', 'm']), Some('z'));
}

#[test]
fn test_largest_empty() {
    let items: Vec<i32> = vec![];
    assert_eq!(largest(&items), None);
}

#[test]
fn test_smallest() {
    assert_eq!(smallest(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(1));
    assert_eq!(smallest(&['a', 'z', 'm']), Some('a'));
}

#[test]
fn test_smallest_empty() {
    let items: Vec<i32> = vec![];
    assert_eq!(smallest(&items), None);
}

#[test]
fn test_find_match() {
    assert_eq!(find_match(&[10, 20, 30], &20), Some(1));
    assert_eq!(find_match(&[10, 20, 30], &99), None);
}

#[test]
fn test_find_match_strings() {
    let words = vec!["alpha", "beta", "gamma"];
    assert_eq!(find_match(&words, &"beta"), Some(1));
}

#[test]
fn test_count_where() {
    assert_eq!(count_where(&[1, 2, 3, 4, 5, 6], |x| x % 2 == 0), 3);
}

#[test]
fn test_all_match() {
    assert!(all_match(&[2, 4, 6, 8], |x| x % 2 == 0));
    assert!(!all_match(&[2, 3, 6, 8], |x| x % 2 == 0));
}

#[test]
fn test_any_match() {
    assert!(any_match(&[1, 3, 5, 6], |x| x % 2 == 0));
    assert!(!any_match(&[1, 3, 5, 7], |x| x % 2 == 0));
}

#[test]
fn test_sort_and_dedup() {
    assert_eq!(sort_and_dedup(&[3, 1, 4, 1, 5, 9, 2, 6, 5]), vec![1, 2, 3, 4, 5, 6, 9]);
}

#[test]
fn test_sort_and_dedup_strings() {
    let items = vec!["banana", "apple", "banana", "cherry"];
    assert_eq!(sort_and_dedup(&items), vec!["apple", "banana", "cherry"]);
}

#[test]
fn test_unique_ordered() {
    assert_eq!(unique_ordered(&[3, 1, 4, 1, 5, 3, 2]), vec![3, 1, 4, 5, 2]);
}

#[test]
fn test_unique_ordered_strings() {
    let items = vec!["foo", "bar", "foo", "baz", "bar"];
    assert_eq!(unique_ordered(&items), vec!["foo", "bar", "baz"]);
}

#[test]
fn test_frequency_map() {
    let items = vec!["a", "b", "a", "c", "b", "a"];
    let freq = frequency_map(&items);
    assert_eq!(freq.get("a"), Some(&3));
    assert_eq!(freq.get("b"), Some(&2));
    assert_eq!(freq.get("c"), Some(&1));
}

#[test]
fn test_frequency_map_ints() {
    let items = vec![1, 2, 2, 3, 3, 3];
    let freq: HashMap<i32, usize> = frequency_map(&items);
    assert_eq!(freq[&1], 1);
    assert_eq!(freq[&2], 2);
    assert_eq!(freq[&3], 3);
}

// ============================================
// Topic 7: Default Methods, Trait Objects & impl Trait
// ============================================

#[test]
fn test_measurable_display() {
    let a = Area(50.0);
    assert_eq!(a.display(), "50.0 m²");
}

#[test]
fn test_measurable_display_distance() {
    let d = Distance(100.5);
    assert_eq!(d.display(), "100.5 m");
}

#[test]
fn test_measurable_is_greater_than() {
    let a = Area(100.0);
    let d = Distance(50.0);
    assert!(a.is_greater_than(&d));
    assert!(!d.is_greater_than(&a));
}

#[test]
fn test_total_measurement() {
    let a = Area(10.0);
    let b = Area(20.0);
    let c = Area(30.0);
    let items: Vec<&dyn Measurable> = vec![&a, &b, &c];
    assert!((total_measurement(&items) - 60.0).abs() < 1e-9);
}

#[test]
fn test_max_measurement() {
    let a = Distance(10.0);
    let b = Distance(50.0);
    let c = Distance(30.0);
    let items: Vec<&dyn Measurable> = vec![&a, &b, &c];
    assert_eq!(max_measurement(&items), Some(50.0));
}

#[test]
fn test_max_measurement_empty() {
    let items: Vec<&dyn Measurable> = vec![];
    assert_eq!(max_measurement(&items), None);
}

#[test]
fn test_news_article_summarize_overrides_default() {
    let article = NewsArticle::new("Rust 2024!", "Alice", "SF");
    assert_eq!(article.summarize(), "Rust 2024!, by Alice (SF)");
    assert_eq!(article.summarize_author(), "Alice");
}

#[test]
fn test_social_post_uses_default_summarize() {
    let post = SocialPost::new("rustfan", "I love Rust!");
    assert_eq!(post.summarize_author(), "@rustfan");
    assert_eq!(post.summarize(), "(Read more from @rustfan...)");
}

#[test]
fn test_collect_summaries() {
    let article = NewsArticle::new("Big News", "Bob", "NYC");
    let post = SocialPost::new("dev", "Hello world");
    let items: Vec<&dyn Summarizable> = vec![&article, &post];
    let summaries = collect_summaries(&items);
    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0], "Big News, by Bob (NYC)");
    assert_eq!(summaries[1], "(Read more from @dev...)");
}

#[test]
fn test_make_social_post_returns_impl_summarizable() {
    let post = make_social_post("alice", "Testing impl Trait");
    assert_eq!(post.summarize_author(), "@alice");
    assert_eq!(post.summarize(), "(Read more from @alice...)");
}

#[test]
fn test_taggable_article() {
    let a = Article {
        title: "Rust Tips".into(),
        article_tags: vec!["rust".into(), "programming".into()],
    };
    assert!(a.has_tag("rust"));
    assert!(!a.has_tag("python"));
    assert_eq!(a.tag_count(), 2);
}

#[test]
fn test_taggable_photo() {
    let p = Photo {
        filename: "sunset.jpg".into(),
        photo_tags: vec!["nature".into(), "sunset".into(), "travel".into()],
    };
    assert!(p.has_tag("sunset"));
    assert_eq!(p.tag_count(), 3);
}

#[test]
fn test_all_tags() {
    let a = Article {
        title: "Post".into(),
        article_tags: vec!["rust".into(), "code".into()],
    };
    let p = Photo {
        filename: "pic.jpg".into(),
        photo_tags: vec!["code".into(), "art".into()],
    };
    let items: Vec<&dyn Taggable> = vec![&a, &p];
    let tags = all_tags(&items);
    assert_eq!(tags.len(), 3); // "rust", "code", "art" (deduped)
    assert!(tags.contains(&"rust".to_string()));
    assert!(tags.contains(&"code".to_string()));
    assert!(tags.contains(&"art".to_string()));
}

#[test]
fn test_filter_by_tag() {
    let a = Article {
        title: "A".into(),
        article_tags: vec!["rust".into()],
    };
    let p = Photo {
        filename: "B".into(),
        photo_tags: vec!["travel".into()],
    };
    let items: Vec<&dyn Taggable> = vec![&a, &p];
    let filtered = filter_by_tag(&items, "rust");
    assert_eq!(filtered.len(), 1);
}

#[test]
fn test_count_with_tag() {
    let a = Article {
        title: "A".into(),
        article_tags: vec!["rust".into(), "code".into()],
    };
    let b = Article {
        title: "B".into(),
        article_tags: vec!["code".into()],
    };
    let p = Photo {
        filename: "C".into(),
        photo_tags: vec!["art".into()],
    };
    let items: Vec<&dyn Taggable> = vec![&a, &b, &p];
    assert_eq!(count_with_tag(&items, "code"), 2);
    assert_eq!(count_with_tag(&items, "art"), 1);
    assert_eq!(count_with_tag(&items, "nope"), 0);
}