use modules_intro::*;

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
// Topic 5: The `use` Keyword and Aliases
// Reinforces: match, Option<T>, arithmetic, type casting
// =============================================================

#[test]
fn test_colors_full_path() {
    assert_eq!(colors::red(), (255, 0, 0));
    assert_eq!(colors::green(), (0, 255, 0));
    assert_eq!(colors::blue(), (0, 0, 255));
}

#[test]
fn test_colors_with_use() {
    use modules_intro::colors::{red, green, blue};
    assert_eq!(red(), (255, 0, 0));
    assert_eq!(green(), (0, 255, 0));
    assert_eq!(blue(), (0, 0, 255));
}

#[test]
fn test_color_alias_with_as() {
    use modules_intro::colors::red as crimson;
    assert_eq!(crimson(), (255, 0, 0));
}

#[test]
fn test_color_mix() {
    let r = colors::red();
    let b = colors::blue();
    let mixed = colors::mix(r, b);
    assert_eq!(mixed, (127, 0, 127));
}

#[test]
fn test_color_mix_same() {
    let g = colors::green();
    assert_eq!(colors::mix(g, g), (0, 255, 0));
}

#[test]
fn test_color_to_hex() {
    assert_eq!(colors::to_hex((255, 0, 0)), "#FF0000");
    assert_eq!(colors::to_hex((0, 255, 0)), "#00FF00");
    assert_eq!(colors::to_hex((0, 0, 0)), "#000000");
    assert_eq!(colors::to_hex((255, 255, 255)), "#FFFFFF");
}

#[test]
fn test_color_brightness_white() {
    // White (255,255,255): (255*299 + 255*587 + 255*114)/1000 = 255
    assert_eq!(colors::brightness((255, 255, 255)), 255);
}

#[test]
fn test_color_brightness_black() {
    assert_eq!(colors::brightness((0, 0, 0)), 0);
}

#[test]
fn test_color_brightness_red() {
    // Red (255,0,0): (255*299)/1000 = 76
    assert_eq!(colors::brightness((255, 0, 0)), 76);
}

#[test]
fn test_color_is_dark() {
    assert!(colors::is_dark((0, 0, 0)));       // black
    assert!(colors::is_dark((255, 0, 0)));     // red (brightness 76)
    assert!(!colors::is_dark((255, 255, 255))); // white
    assert!(!colors::is_dark((0, 255, 0)));     // green (brightness ~150)
}

#[test]
fn test_color_from_name_known() {
    assert_eq!(colors::from_name("red"), Some((255, 0, 0)));
    assert_eq!(colors::from_name("green"), Some((0, 255, 0)));
    assert_eq!(colors::from_name("blue"), Some((0, 0, 255)));
    assert_eq!(colors::from_name("white"), Some((255, 255, 255)));
    assert_eq!(colors::from_name("black"), Some((0, 0, 0)));
}

#[test]
fn test_color_from_name_case_insensitive() {
    assert_eq!(colors::from_name("RED"), Some((255, 0, 0)));
    assert_eq!(colors::from_name("Blue"), Some((0, 0, 255)));
}

#[test]
fn test_color_from_name_unknown() {
    assert_eq!(colors::from_name("purple"), None);
    assert_eq!(colors::from_name(""), None);
}

#[test]
fn test_color_palettes() {
    let sunset = colors::palettes::sunset();
    assert_eq!(sunset.len(), 3);
    assert_eq!(sunset[0], (255, 94, 77));

    let ocean = colors::palettes::ocean();
    assert_eq!(ocean.len(), 3);
}

#[test]
fn test_nested_use_statement() {
    use modules_intro::colors::palettes::{ocean, sunset};
    assert!(!sunset().is_empty());
    assert!(!ocean().is_empty());
}

#[test]
fn test_palette_average_sunset() {
    let sunset = colors::palettes::sunset();
    let avg = colors::palettes::average(&sunset);
    // (255+255+255)/3=255, (94+154+206)/3=151, (77+0+0)/3=25
    assert_eq!(avg, (255, 151, 25));
}

#[test]
fn test_palette_average_empty() {
    let avg = colors::palettes::average(&[]);
    assert_eq!(avg, (0, 0, 0));
}

#[test]
fn test_palette_average_single() {
    let avg = colors::palettes::average(&[(100, 150, 200)]);
    assert_eq!(avg, (100, 150, 200));
}

// =============================================================
// Topic 6: Re-exports with `pub use`
// Reinforces: structs, Vec ownership, &self/&mut self
// =============================================================

#[test]
fn test_reexported_instruments() {
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

#[test]
fn test_playlist_new_is_empty() {
    let pl = music::Playlist::new("My Playlist");
    assert_eq!(pl.name(), "My Playlist");
    assert!(pl.is_empty());
    assert_eq!(pl.len(), 0);
}

#[test]
fn test_playlist_add_songs() {
    let mut pl = music::Playlist::new("Road Trip");
    pl.add_song("Bohemian Rhapsody");
    pl.add_song("Hotel California");
    assert_eq!(pl.len(), 2);
    assert!(!pl.is_empty());
}

#[test]
fn test_playlist_contains() {
    let mut pl = music::Playlist::new("Favs");
    pl.add_song("Yesterday");
    pl.add_song("Imagine");
    assert!(pl.contains("Yesterday"));
    assert!(!pl.contains("Stairway to Heaven"));
}

#[test]
fn test_playlist_songs_returns_slice() {
    let mut pl = music::Playlist::new("Rock");
    pl.add_song("Thunderstruck");
    pl.add_song("Back in Black");
    let songs = pl.songs();
    assert_eq!(songs.len(), 2);
    assert_eq!(songs[0], "Thunderstruck");
    assert_eq!(songs[1], "Back in Black");
}

#[test]
fn test_playlist_describe_empty() {
    let pl = music::Playlist::new("Empty");
    assert_eq!(pl.describe(), "Playlist 'Empty': (empty)");
}

#[test]
fn test_playlist_describe_with_songs() {
    let mut pl = music::Playlist::new("Jazz Night");
    pl.add_song("So What");
    pl.add_song("Take Five");
    assert_eq!(pl.describe(), "Playlist 'Jazz Night': So What, Take Five");
}

// =============================================================
// Topic 7: Fine-grained Visibility — pub(crate), pub(super)
// Reinforces: builder pattern (ownership chain)
// =============================================================

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

#[test]
fn test_config_custom_summary() {
    let summary = config::custom_summary(10, 60);
    assert!(summary.contains("max_retries=10"));
    assert!(summary.contains("timeout=60s"));
    // pool stays at default
    assert!(summary.contains("db_pool=5"));
}