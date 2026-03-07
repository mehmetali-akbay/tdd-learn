use modules_intro::*;

// =============================================================
// Topics 1–4: Tests are provided — implement the code in lib.rs
// =============================================================

// =============================================================
// Topic 1: Module Basics — Accessing items via module paths
// Reinforces: if/else, match, Option<T>
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
    assert_eq!(math::safe_divide(10, 0), 0);
}

#[test]
fn test_math_negative_numbers() {
    assert_eq!(math::add(-5, 3), -2);
    assert_eq!(math::subtract(-1, -1), 0);
    assert_eq!(math::multiply(-2, 4), -8);
}

#[test]
fn test_math_clamp_within_range() {
    assert_eq!(math::clamp(5, 0, 10), 5);
}

#[test]
fn test_math_clamp_below_min() {
    assert_eq!(math::clamp(-5, 0, 10), 0);
}

#[test]
fn test_math_clamp_above_max() {
    assert_eq!(math::clamp(15, 0, 10), 10);
}

#[test]
fn test_math_clamp_at_boundaries() {
    assert_eq!(math::clamp(0, 0, 10), 0);
    assert_eq!(math::clamp(10, 0, 10), 10);
}

#[test]
fn test_math_eval_add() {
    assert_eq!(math::eval("+", 3, 4), Some(7));
}

#[test]
fn test_math_eval_subtract() {
    assert_eq!(math::eval("-", 10, 3), Some(7));
}

#[test]
fn test_math_eval_multiply() {
    assert_eq!(math::eval("*", 5, 6), Some(30));
}

#[test]
fn test_math_eval_divide() {
    assert_eq!(math::eval("/", 10, 3), Some(3));
}

#[test]
fn test_math_eval_divide_by_zero() {
    assert_eq!(math::eval("/", 10, 0), None);
}

#[test]
fn test_math_eval_unknown_operator() {
    assert_eq!(math::eval("%", 10, 3), None);
    assert_eq!(math::eval("pow", 2, 3), None);
}

// =============================================================
// Topic 2: Visibility — pub and private items
// Reinforces: slices (&[&str]), Vec<String>, Option<T>
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

#[test]
fn test_greet_all_multiple() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let greetings = greetings::greet_all(&names);
    assert_eq!(greetings.len(), 3);
    assert_eq!(greetings[0], "Hello, Alice!");
    assert_eq!(greetings[1], "Hello, Bob!");
    assert_eq!(greetings[2], "Hello, Charlie!");
}

#[test]
fn test_greet_all_empty_slice() {
    let greetings = greetings::greet_all(&[]);
    assert!(greetings.is_empty());
}

#[test]
fn test_greet_all_single() {
    let greetings = greetings::greet_all(&["World"]);
    assert_eq!(greetings, vec!["Hello, World!"]);
}

#[test]
fn test_greet_option_some() {
    assert_eq!(greetings::greet_option(Some("Alice")), "Hello, Alice!");
}

#[test]
fn test_greet_option_none() {
    assert_eq!(greetings::greet_option(None), "Hello, stranger!");
}

// =============================================================
// Topic 3: Nested Modules and `super`
// Reinforces: Vec<String>, match, Option<T>
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

#[test]
fn test_all_sounds() {
    let sounds = animals::all_sounds();
    assert_eq!(sounds.len(), 3);
    assert!(sounds.contains(&"The dog says woof!".to_string()));
    assert!(sounds.contains(&"The cat says meow!".to_string()));
    assert!(sounds.contains(&"The bird says tweet!".to_string()));
}

#[test]
fn test_find_by_name_found() {
    assert_eq!(
        animals::find_by_name("Buddy"),
        Some("The dog says woof!".to_string())
    );
    assert_eq!(
        animals::find_by_name("Whiskers"),
        Some("The cat says meow!".to_string())
    );
}

#[test]
fn test_find_by_name_case_insensitive() {
    assert_eq!(
        animals::find_by_name("buddy"),
        Some("The dog says woof!".to_string())
    );
    assert_eq!(
        animals::find_by_name("TWEETY"),
        Some("The bird says tweet!".to_string())
    );
}

#[test]
fn test_find_by_name_not_found() {
    assert_eq!(animals::find_by_name("Rex"), None);
    assert_eq!(animals::find_by_name(""), None);
}

// =============================================================
// Topic 4: Struct and Enum Visibility
// Reinforces: ownership (Vec<Book>), &self/&mut self, Option<&T>,
//             iterators, if let
// =============================================================

#[test]
fn test_create_book() {
    let book = library::Book::new("Rust in Action", "Tim McNamara", "978-1617294556");
    assert_eq!(book.title, "Rust in Action");
    assert_eq!(book.author, "Tim McNamara");
}

#[test]
fn test_book_isbn_accessor() {
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
    let mut book = library::Book::new("Old Title", "Old Author", "000");
    book.title = "New Title".to_string();
    book.author = "New Author".to_string();
    assert_eq!(book.title, "New Title");
    assert_eq!(book.author, "New Author");
}

#[test]
fn test_genre_enum_all_variants_public() {
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

#[test]
fn test_book_summary_available() {
    let book = library::Book::new("Dune", "Frank Herbert", "978-0441013593");
    assert_eq!(
        book.summary(),
        "Dune by Frank Herbert [ISBN: 978-0441013593] - Available"
    );
}

#[test]
fn test_book_summary_checked_out() {
    let mut book = library::Book::new("Dune", "Frank Herbert", "978-0441013593");
    book.check_out();
    assert_eq!(
        book.summary(),
        "Dune by Frank Herbert [ISBN: 978-0441013593] - Checked out"
    );
}

#[test]
fn test_shelf_new_is_empty() {
    let shelf = library::Shelf::new();
    assert!(shelf.is_empty());
    assert_eq!(shelf.len(), 0);
}

#[test]
fn test_shelf_add_books() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("Book A", "Author A", "001"));
    shelf.add(library::Book::new("Book B", "Author B", "002"));
    assert_eq!(shelf.len(), 2);
    assert!(!shelf.is_empty());
}

#[test]
fn test_shelf_find_by_title_found() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("Rust Book", "Klabnik", "111"));
    shelf.add(library::Book::new("Dune", "Herbert", "222"));

    let found = shelf.find_by_title("Dune");
    assert!(found.is_some());
    let book = found.unwrap();
    assert_eq!(book.title, "Dune");
    assert_eq!(book.author, "Herbert");
}

#[test]
fn test_shelf_find_by_title_not_found() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("Rust Book", "Klabnik", "111"));
    assert!(shelf.find_by_title("Unknown").is_none());
}

#[test]
fn test_shelf_available_count() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("A", "X", "1"));
    shelf.add(library::Book::new("B", "Y", "2"));
    shelf.add(library::Book::new("C", "Z", "3"));
    assert_eq!(shelf.available_count(), 3);

    shelf.check_out_by_title("B");
    assert_eq!(shelf.available_count(), 2);
}

#[test]
fn test_shelf_titles() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("Alpha", "A", "1"));
    shelf.add(library::Book::new("Beta", "B", "2"));
    shelf.add(library::Book::new("Gamma", "C", "3"));

    let titles = shelf.titles();
    assert_eq!(titles, vec!["Alpha", "Beta", "Gamma"]);
}

#[test]
fn test_shelf_check_out_by_title_success() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("Dune", "Herbert", "1"));
    assert!(shelf.check_out_by_title("Dune"));
    // Verify it's now checked out:
    let book = shelf.find_by_title("Dune").unwrap();
    assert!(!book.is_available());
}

#[test]
fn test_shelf_check_out_by_title_not_found() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("Dune", "Herbert", "1"));
    assert!(!shelf.check_out_by_title("Unknown"));
}

#[test]
fn test_shelf_check_out_already_checked_out() {
    let mut shelf = library::Shelf::new();
    shelf.add(library::Book::new("Dune", "Herbert", "1"));
    assert!(shelf.check_out_by_title("Dune"));  // first time: success
    assert!(!shelf.check_out_by_title("Dune")); // second time: already out
}

// =============================================================
// ⬇ Topics 5–7: Write the TESTS below! Code is in lib.rs ⬇
// =============================================================
// The module code is already implemented. Your task is to write
// tests that exercise module paths, `use`, `as`, `pub use`,
// and visibility rules.
//
// Each test has a comment explaining what to test and what
// module concept it exercises. Replace todo!() with real asserts.
// =============================================================

// =============================================================
// Topic 5: The `use` Keyword and Aliases
// =============================================================
// Learning goals:
//   - Access items via full module paths (colors::red())
//   - Bring items into scope with `use`
//   - Rename imports with `as`
//   - Use nested imports (use colors::palettes::{sunset, ocean})
//   - Call functions within submodules (colors::palettes::average)

#[test]
fn test_colors_full_path() {
    // TODO: Call colors::red(), colors::green(), colors::blue()
    // using full module paths and assert their RGB values.
    // red=(255,0,0), green=(0,255,0), blue=(0,0,255)
    todo!()
}

#[test]
fn test_colors_with_use() {
    // TODO: Add a `use` statement to bring red, green, blue into scope
    // WITHOUT the module prefix. Then call them directly.
    // Hint: use modules_intro::colors::{red, green, blue};
    todo!()
}

#[test]
fn test_color_alias_with_as() {
    // TODO: Use `as` to rename colors::red to `crimson`.
    // Then call crimson() and assert it returns (255, 0, 0).
    // Hint: use modules_intro::colors::red as crimson;
    todo!()
}

#[test]
fn test_color_mix() {
    // TODO: Mix red and blue using colors::mix().
    // Average of (255,0,0) and (0,0,255) is (127,0,127).
    // Practice: use full paths or bring into scope with `use`.
    todo!()
}

#[test]
fn test_color_mix_same() {
    // TODO: Mix green with itself.
    // Mixing (0,255,0) with (0,255,0) should give (0,255,0).
    todo!()
}

#[test]
fn test_color_to_hex() {
    // TODO: Test colors::to_hex() with multiple colors:
    // (255,0,0) => "#FF0000", (0,255,0) => "#00FF00",
    // (0,0,0) => "#000000", (255,255,255) => "#FFFFFF"
    todo!()
}

#[test]
fn test_color_brightness_white() {
    // TODO: Test brightness of white (255,255,255).
    // Formula: (255*299 + 255*587 + 255*114) / 1000 = 255
    todo!()
}

#[test]
fn test_color_brightness_black() {
    // TODO: Test brightness of black (0,0,0) is 0.
    todo!()
}

#[test]
fn test_color_brightness_red() {
    // TODO: Test brightness of red (255,0,0).
    // (255*299 + 0 + 0) / 1000 = 76
    todo!()
}

#[test]
fn test_color_is_dark() {
    // TODO: Test is_dark for various colors:
    // black (0,0,0) → dark, red (255,0,0) → dark (brightness 76),
    // white (255,255,255) → NOT dark, green (0,255,0) → NOT dark
    todo!()
}

#[test]
fn test_color_from_name_known() {
    // TODO: Test from_name for "red", "green", "blue", "white", "black".
    // Each returns Some((r,g,b)).
    todo!()
}

#[test]
fn test_color_from_name_case_insensitive() {
    // TODO: Test that from_name("RED") and from_name("Blue") work.
    // The function is case-insensitive.
    todo!()
}

#[test]
fn test_color_from_name_unknown() {
    // TODO: Test from_name("purple") and from_name("") return None.
    todo!()
}

#[test]
fn test_color_palettes() {
    // TODO: Call colors::palettes::sunset() and colors::palettes::ocean().
    // Assert sunset has 3 colors, first is (255,94,77).
    // Assert ocean has 3 colors.
    // This exercises nested module paths!
    todo!()
}

#[test]
fn test_nested_use_statement() {
    // TODO: Use a nested `use` to bring sunset and ocean into scope:
    //   use modules_intro::colors::palettes::{sunset, ocean};
    // Then call them directly without the full path.
    todo!()
}

#[test]
fn test_palette_average_sunset() {
    // TODO: Get the sunset palette, compute its average.
    // (255+255+255)/3=255, (94+154+206)/3=151, (77+0+0)/3=25
    // Assert average is (255, 151, 25).
    todo!()
}

#[test]
fn test_palette_average_empty() {
    // TODO: Average of an empty palette should be (0,0,0).
    todo!()
}

#[test]
fn test_palette_average_single() {
    // TODO: Average of a single-element palette [(100,150,200)]
    // should be (100, 150, 200).
    todo!()
}

// =============================================================
// Topic 6: Re-exports with `pub use`
// =============================================================
// Learning goals:
//   - Access re-exported items at the shorter path (music::guitar)
//   - Understand that private submodules are NOT directly accessible
//   - Use the re-exported Playlist struct
//   - Understand pub use creates a "public API" from private internals

#[test]
fn test_reexported_instruments() {
    // TODO: Call music::guitar(), music::piano(), music::drums()
    // These are re-exported from the PRIVATE music::instruments module.
    // Assert: guitar="🎸 Guitar", piano="🎹 Piano", drums="🥁 Drums"
    //
    // Note: music::instruments::guitar() would NOT compile because
    // `instruments` is a private module! pub use makes it available
    // at the music:: level instead.
    todo!()
}

#[test]
fn test_reexported_genres() {
    // TODO: Call music::rock(), music::jazz(), music::classical()
    // These are re-exported from the private music::genres module.
    // Assert their return values.
    todo!()
}

#[test]
fn test_describe_style() {
    // TODO: Call music::describe_style(music::guitar(), music::rock())
    // Assert it returns "🎸 Guitar playing Rock"
    // Notice how you compose re-exported functions!
    todo!()
}

#[test]
fn test_describe_multiple_styles() {
    // TODO: Test describe_style with piano+jazz and drums+classical.
    // "🎹 Piano playing Jazz" and "🥁 Drums playing Classical"
    todo!()
}

#[test]
fn test_playlist_new_is_empty() {
    // TODO: Create music::Playlist (re-exported from private collections).
    // Assert its name, that it's empty, and len is 0.
    // Hint: let pl = music::Playlist::new("My Playlist");
    todo!()
}

#[test]
fn test_playlist_add_songs() {
    // TODO: Create a playlist, add two songs, assert len == 2.
    todo!()
}

#[test]
fn test_playlist_contains() {
    // TODO: Create a playlist, add "Yesterday" and "Imagine".
    // Assert contains("Yesterday") is true.
    // Assert contains("Stairway to Heaven") is false.
    todo!()
}

#[test]
fn test_playlist_songs_returns_slice() {
    // TODO: Create a playlist, add songs, call .songs().
    // Assert it returns a &[String] with the correct songs.
    todo!()
}

#[test]
fn test_playlist_describe_empty() {
    // TODO: Create an empty playlist named "Empty".
    // Assert describe returns "Playlist 'Empty': (empty)".
    todo!()
}

#[test]
fn test_playlist_describe_with_songs() {
    // TODO: Create playlist "Jazz Night", add "So What" and "Take Five".
    // Assert describe returns "Playlist 'Jazz Night': So What, Take Five".
    todo!()
}

// =============================================================
// Topic 7: Fine-grained Visibility — pub(crate), pub(super)
// =============================================================
// Learning goals:
//   - pub(crate) items ARE accessible from within the crate
//     (unit tests in lib.rs) but NOT from integration tests here
//   - Public wrapper functions (summary, full_summary, custom_summary)
//     expose the behavior without exposing the internals
//   - The unit tests in lib.rs (#[cfg(test)] mod tests) are where
//     you test pub(crate) items directly
//
// For integration tests, you can only test the public API:

#[test]
fn test_config_summary() {
    // TODO: Call config::summary() and verify it contains:
    // "max_retries=3", "timeout=30s", "db_pool=5"
    // Hint: use .contains() to check substrings
    todo!()
}

#[test]
fn test_config_full_summary() {
    // TODO: Call config::full_summary() and verify it contains:
    // "retries=3", "timeout=30s", "pool=5", "postgres://localhost:5432/mydb"
    todo!()
}

#[test]
fn test_config_custom_summary() {
    // TODO: Call config::custom_summary(10, 60) and verify:
    // "max_retries=10", "timeout=60s", "db_pool=5" (pool stays default)
    todo!()
}

// NOTE: You CANNOT do the following from an integration test:
//   let cfg = config::AppConfig::default_config();  // pub(crate) — won't compile!
//   config::database::pool_size();                   // pub(crate) — won't compile!
//   config::database::connection_string();           // pub(super) — won't compile!
// Go to lib.rs and write the unit tests in #[cfg(test)] mod tests instead!
