use traits::*;

// ===== Topic 1: Basic Traits — Describable =====

#[test]
fn test_pet_describe() {
    let pet = Pet::new("Buddy", "dog");
    assert_eq!(pet.describe(), "Buddy the dog");
}

#[test]
fn test_car_describe() {
    let car = Car::new("Toyota", "Corolla", 2024);
    assert_eq!(car.describe(), "2024 Toyota Corolla");
}

#[test]
fn test_book_describe() {
    let book = Book::new("Rust in Action", "Tim McNamara", 456);
    assert_eq!(
        book.describe(),
        "\"Rust in Action\" by Tim McNamara (456 pages)"
    );
}

#[test]
fn test_print_description() {
    let pet = Pet::new("Luna", "cat");
    assert_eq!(print_description(&pet), "Luna the cat");

    let car = Car::new("Honda", "Civic", 2023);
    assert_eq!(print_description(&car), "2023 Honda Civic");
}

// ===== Topic 2: Display & Debug =====

#[test]
fn test_coord_display() {
    let c = Coord::new(3.0, 4.0);
    assert_eq!(format!("{}", c), "(3, 4)");
}

#[test]
fn test_coord_debug() {
    let c = Coord::new(1.0, 2.0);
    // Debug format should work due to derive
    let debug = format!("{:?}", c);
    assert!(debug.contains("Coord"));
}

#[test]
fn test_temperature_display() {
    assert_eq!(format!("{}", Temperature::celsius(100.0)), "100°C");
    assert_eq!(format!("{}", Temperature::fahrenheit(212.0)), "212°F");
}

// ===== Topic 3: Default Trait =====

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert!(!config.debug);
    assert_eq!(config.max_connections, 100);
}

#[test]
fn test_config_builder() {
    let config = Config::default()
        .with_host("0.0.0.0")
        .with_port(3000)
        .with_debug(true);
    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 3000);
    assert!(config.debug);
    assert_eq!(config.max_connections, 100); // unchanged
}

#[test]
fn test_game_settings_default() {
    let settings = GameSettings::default();
    assert_eq!(settings.difficulty, "normal");
    assert!(settings.sound);
    assert!(!settings.fullscreen);
}

// ===== Topic 4: From / Into =====

#[test]
fn test_celsius_to_fahrenheit() {
    let c = Celsius(100.0);
    let f: Fahrenheit = c.into();
    assert!((f.0 - 212.0).abs() < 0.01);
}

#[test]
fn test_fahrenheit_to_celsius() {
    let f = Fahrenheit(32.0);
    let c: Celsius = f.into();
    assert!((c.0 - 0.0).abs() < 0.01);
}

#[test]
fn test_slug_from_str() {
    let slug = Slug::from("Hello World");
    assert_eq!(slug.0, "hello-world");

    let slug2 = Slug::from("  Rust Is Great  ");
    assert_eq!(slug2.0, "rust-is-great");
}

#[test]
fn test_slug_display() {
    let slug = Slug::from("Hello World");
    assert_eq!(format!("{}", slug), "hello-world");
}

#[test]
fn test_rgb_from_tuple() {
    let color: Rgb = (255, 128, 0).into();
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 0);
}

#[test]
fn test_rgb_to_tuple() {
    let color = Rgb {
        r: 10,
        g: 20,
        b: 30,
    };
    let tuple: (u8, u8, u8) = color.into();
    assert_eq!(tuple, (10, 20, 30));
}

// ===== Topic 5: Trait Bounds =====

#[test]
fn test_format_all() {
    assert_eq!(format_all(&[1, 2, 3]), "1, 2, 3");
    assert_eq!(format_all(&["a", "b", "c"]), "a, b, c");
    assert_eq!(format_all::<i32>(&[]), "");
}

#[test]
fn test_largest() {
    assert_eq!(largest(&[1, 5, 3, 9, 2]), Some(9));
    assert_eq!(largest(&[3.15, 2.72, 1.42]), Some(3.15));
    assert_eq!(largest::<i32>(&[]), None);
}

#[test]
fn test_find_match() {
    assert_eq!(find_match(&[10, 20, 30], &20), Some(1));
    assert_eq!(find_match(&["a", "b", "c"], &"c"), Some(2));
    assert_eq!(find_match(&[1, 2, 3], &99), None);
}

#[test]
fn test_count_where() {
    fn is_positive(x: &i32) -> bool {
        *x > 0
    }
    assert_eq!(count_where(&[1, -2, 3, -4, 5], is_positive), 3);
    assert_eq!(count_where(&[-1, -2], is_positive), 0);
}

#[test]
fn test_all_match() {
    fn is_positive(x: &i32) -> bool {
        *x > 0
    }
    assert!(all_match(&[1, 2, 3], is_positive));
    assert!(!all_match(&[1, -2, 3], is_positive));
    assert!(all_match::<i32>(&[], is_positive));
}

// ===== Topic 6: Advanced — Trait Objects & Default Methods =====

#[test]
fn test_area_measurable() {
    let area = Area(25.0);
    assert_eq!(area.measure(), 25.0);
    assert_eq!(area.unit(), "m²");
    assert_eq!(area.display(), "25.0 m²");
}

#[test]
fn test_distance_measurable() {
    let dist = Distance(100.0);
    assert_eq!(dist.measure(), 100.0);
    assert_eq!(dist.unit(), "m");
    assert_eq!(dist.display(), "100.0 m");
}

#[test]
fn test_duration_measurable() {
    let dur = Duration(30.0);
    assert_eq!(dur.display(), "30.0 s");
}

#[test]
fn test_is_greater_than() {
    let big = Area(100.0);
    let small = Distance(50.0);
    assert!(big.is_greater_than(&small));
    assert!(!small.is_greater_than(&big));
}

#[test]
fn test_total_measurement() {
    let items: Vec<&dyn Measurable> = vec![&Area(10.0), &Distance(20.0), &Duration(30.0)];
    assert!((total_measurement(&items) - 60.0).abs() < 0.001);
}

#[test]
fn test_max_measurement() {
    let items: Vec<&dyn Measurable> = vec![&Area(10.0), &Distance(50.0), &Duration(30.0)];
    assert_eq!(max_measurement(&items), Some(50.0));
    assert_eq!(max_measurement(&[]), None);
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_article_taggable() {
    let a = Article { title: "Rust".into(), article_tags: vec!["rust".into(), "lang".into()] };
    assert!(a.has_tag("rust"));
    assert!(!a.has_tag("python"));
}

#[test]
fn test_photo_taggable() {
    let p = Photo { filename: "pic.jpg".into(), photo_tags: vec!["nature".into()] };
    assert_eq!(p.tags(), vec!["nature"]);
}

#[test]
fn test_all_tags() {
    let a = Article { title: "T".into(), article_tags: vec!["rust".into(), "code".into()] };
    let p = Photo { filename: "f".into(), photo_tags: vec!["code".into(), "art".into()] };
    let items: Vec<&dyn Taggable> = vec![&a, &p];
    let tags = all_tags(&items);
    assert_eq!(tags.len(), 3); // rust, code, art (code deduped)
}

#[test]
fn test_filter_by_tag() {
    let a = Article { title: "T".into(), article_tags: vec!["rust".into()] };
    let p = Photo { filename: "f".into(), photo_tags: vec!["art".into()] };
    let items: Vec<&dyn Taggable> = vec![&a, &p];
    assert_eq!(filter_by_tag(&items, "rust").len(), 1);
    assert_eq!(filter_by_tag(&items, "missing").len(), 0);
}

