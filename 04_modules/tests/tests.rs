use modules_intro::*;

// =============================================================
// Topic 1: Module Basics — Accessing items via module paths
// =============================================================

#[test]
fn test_math_add() {
    assert_eq!(math::add(2, 3), 5);
}

#[test]
fn test_math_subtract() {
    assert_eq!(math::subtract(10, 4), 6);
}

#[test]
fn test_math_multiply() {
    assert_eq!(math::multiply(3, 7), 21);
}

#[test]
fn test_math_safe_divide() {
    assert_eq!(math::safe_divide(10, 3), 3);
    assert_eq!(math::safe_divide(15, 5), 3);
}

#[test]
fn test_math_safe_divide_by_zero() {
    // Division by zero returns 0 instead of panicking.
    assert_eq!(math::safe_divide(10, 0), 0);
}

#[test]
fn test_math_negative_numbers() {
    assert_eq!(math::add(-5, 3), -2);
    assert_eq!(math::subtract(-1, -1), 0);
    assert_eq!(math::multiply(-2, 4), -8);
}

// =============================================================
// Topic 2: Visibility — pub and private items
// =============================================================

#[test]
fn test_hello_greeting() {
    assert_eq!(greetings::hello("Alice"), "Hello, Alice!");
}

#[test]
fn test_goodbye_greeting() {
    assert_eq!(greetings::goodbye("Bob"), "Goodbye, Bob!");
}

#[test]
fn test_formal_hello_capitalizes() {
    // `formal_hello` internally uses a private helper to capitalize.
    assert_eq!(greetings::formal_hello("alice"), "Good day, Alice.");
}

#[test]
fn test_formal_hello_already_capitalized() {
    assert_eq!(greetings::formal_hello("Bob"), "Good day, Bob.");
}

#[test]
fn test_formal_hello_empty_string() {
    assert_eq!(greetings::formal_hello(""), "Good day, .");
}

// Note: greetings::capitalize_first is private — we can't call it
// directly from here. This is intentional! Private helpers stay hidden.

// =============================================================
// Topic 3: Nested Modules and `super`
// =============================================================

#[test]
fn test_dog_speak() {
    assert_eq!(animals::dog::speak(), "The dog says woof!");
}

#[test]
fn test_cat_speak() {
    assert_eq!(animals::cat::speak(), "The cat says meow!");
}

#[test]
fn test_bird_speak() {
    assert_eq!(animals::bird::speak(), "The bird says tweet!");
}

#[test]
fn test_animal_names() {
    assert_eq!(animals::dog::name(), "Buddy");
    assert_eq!(animals::cat::name(), "Whiskers");
    assert_eq!(animals::bird::name(), "Tweety");
}

// Note: animals::describe is private — the nested modules use
// it via `super::describe`, but we can't call it from here.

// =============================================================
// Topic 4: Struct and Enum Visibility
// =============================================================

#[test]
fn test_create_book() {
    let book = library::Book::new("Rust in Action", "Tim McNamara", "978-1617294556");
    // Public fields are directly accessible:
    assert_eq!(book.title, "Rust in Action");
    assert_eq!(book.author, "Tim McNamara");
}

#[test]
fn test_book_isbn_accessor() {
    // `isbn` is private, so we use the accessor method.
    let book = library::Book::new("The Rust Book", "Steve Klabnik", "978-1718500440");
    assert_eq!(book.isbn(), "978-1718500440");
}

#[test]
fn test_book_checkout_and_return() {
    let mut book = library::Book::new("Dune", "Frank Herbert", "978-0441013593");
    assert!(book.is_available());

    book.check_out();
    assert!(!book.is_available());

    book.return_book();
    assert!(book.is_available());
}

#[test]
fn test_book_public_fields_are_mutable() {
    // Public fields can be modified directly.
    let mut book = library::Book::new("Old Title", "Old Author", "000");
    book.title = "New Title".to_string();
    book.author = "New Author".to_string();
    assert_eq!(book.title, "New Title");
    assert_eq!(book.author, "New Author");
}

#[test]
fn test_genre_enum_all_variants_public() {
    // All variants of a pub enum are automatically public.
    let fiction = library::Genre::Fiction;
    let non_fiction = library::Genre::NonFiction;
    let science = library::Genre::Science;
    let history = library::Genre::History;

    assert_eq!(fiction.description(), "Fiction: novels, stories, and imagination");
    assert_eq!(
        non_fiction.description(),
        "Non-Fiction: facts, essays, and real events"
    );
    assert_eq!(science.description(), "Science: exploration and discovery");
    assert_eq!(history.description(), "History: lessons from the past");
}

// =============================================================
// Topic 5: The `use` Keyword and Aliases
// =============================================================

#[test]
fn test_colors_full_path() {
    // Using the full module path.
    assert_eq!(colors::red(), (255, 0, 0));
    assert_eq!(colors::green(), (0, 255, 0));
    assert_eq!(colors::blue(), (0, 0, 255));
}

#[test]
fn test_colors_with_use() {
    // Bring specific items into scope with `use`.
    use modules_intro::colors::{red, green, blue};
    assert_eq!(red(), (255, 0, 0));
    assert_eq!(green(), (0, 255, 0));
    assert_eq!(blue(), (0, 0, 255));
}

#[test]
fn test_color_alias_with_as() {
    // Use `as` to create an alias for an imported item.
    use modules_intro::colors::red as crimson;
    assert_eq!(crimson(), (255, 0, 0));
}

#[test]
fn test_color_mix() {
    let r = colors::red();
    let b = colors::blue();
    let mixed = colors::mix(r, b);
    // (255+0)/2=127, (0+0)/2=0, (0+255)/2=127
    assert_eq!(mixed, (127, 0, 127));
}

#[test]
fn test_color_mix_same() {
    let g = colors::green();
    let mixed = colors::mix(g, g);
    assert_eq!(mixed, (0, 255, 0));
}

#[test]
fn test_color_to_hex() {
    assert_eq!(colors::to_hex((255, 0, 0)), "#FF0000");
    assert_eq!(colors::to_hex((0, 255, 0)), "#00FF00");
    assert_eq!(colors::to_hex((0, 0, 0)), "#000000");
}

#[test]
fn test_color_palettes() {
    // Accessing nested module items.
    let sunset = colors::palettes::sunset();
    assert_eq!(sunset.len(), 3);
    assert_eq!(sunset[0], (255, 94, 77));

    let ocean = colors::palettes::ocean();
    assert_eq!(ocean.len(), 3);
}

#[test]
fn test_nested_use_statement() {
    // Nested `use` to bring multiple items from the same module.
    use modules_intro::colors::palettes::{ocean, sunset};
    assert!(!sunset().is_empty());
    assert!(!ocean().is_empty());
}

// =============================================================
// Topic 6: Re-exports with `pub use`
// =============================================================

#[test]
fn test_reexported_instruments() {
    // Thanks to `pub use`, we access these directly from `music`
    // instead of `music::instruments::guitar()` (which wouldn't
    // work anyway since `instruments` is private).
    assert_eq!(music::guitar(), "🎸 Guitar");
    assert_eq!(music::piano(), "🎹 Piano");
    assert_eq!(music::drums(), "🥁 Drums");
}

#[test]
fn test_reexported_genres() {
    assert_eq!(music::rock(), "Rock");
    assert_eq!(music::jazz(), "Jazz");
    assert_eq!(music::classical(), "Classical");
}

#[test]
fn test_describe_style() {
    let style = music::describe_style(music::guitar(), music::rock());
    assert_eq!(style, "🎸 Guitar playing Rock");
}

#[test]
fn test_describe_multiple_styles() {
    assert_eq!(
        music::describe_style(music::piano(), music::jazz()),
        "🎹 Piano playing Jazz"
    );
    assert_eq!(
        music::describe_style(music::drums(), music::classical()),
        "🥁 Drums playing Classical"
    );
}

// =============================================================
// Topic 7: Fine-grained Visibility — pub(crate), pub(super)
// =============================================================
// Note: `pub(crate)` items are NOT accessible from integration tests
// because tests/ are separate crates. We test via the public wrappers.

#[test]
fn test_config_summary() {
    let summary = config::summary();
    assert!(summary.contains("max_retries=3"));
    assert!(summary.contains("timeout=30s"));
    assert!(summary.contains("db_pool=5"));
}

#[test]
fn test_config_full_summary() {
    let summary = config::full_summary();
    assert!(summary.contains("retries=3"));
    assert!(summary.contains("timeout=30s"));
    assert!(summary.contains("pool=5"));
    assert!(summary.contains("postgres://localhost:5432/mydb"));
}