// =============================================================
// Module 04: Modules — Organizing Code in Rust
// =============================================================
// Covers: The Rust Book Chapter 7
//   1. Defining modules with `mod`
//   2. Visibility with `pub`
//   3. Nested modules and `super`
//   4. Struct and enum visibility
//   5. `use` keyword and aliases (`as`)
//   6. Re-exports with `pub use`
//   7. Fine-grained visibility: `pub(crate)` and `pub(super)`
//
// Practice Mode:
//   Topics 1–4: Implement the function bodies (todo!())
//   Topics 5–7: Code is provided — write the TESTS instead!
//     The real module learning happens when you write use statements,
//     exercise module paths, and understand visibility from the
//     caller's perspective.
//
// Reinforces prior topics:
//   - Ownership & borrowing (slices, &self, &mut self, Vec ownership)
//   - Pattern matching (match, Option<T>)
//   - Structs with methods
//   - Control flow (if/else, loops)
// =============================================================

// ── Topic 1: Module Basics ──────────────────────────────────
// Modules group related code under a namespace.
// Items inside a module are private by default.
// You access items using paths:
//   - Absolute: crate::module::item
//   - Relative: module::item

pub mod math {
    /// Adds two numbers together.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtracts b from a.
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    /// Multiplies two numbers.
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// Divides a by b (integer division). Returns 0 if b is 0.
    pub fn safe_divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            return 0;
        }
        a / b
    }

    /// Clamps `value` so it stays within [min, max].
    /// If value < min, returns min. If value > max, returns max.
    /// Otherwise returns value.
    /// (Reinforces: if/else chains)
    pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
        if value < min {
            return min;
        } else if value > max {
            return max;
        } else {
            return value;
        }
    }

    /// Evaluates a simple math expression given an operator string.
    /// Supported operators: "+", "-", "*", "/"
    /// Returns None for unknown operators or division by zero.
    /// (Reinforces: match + Option<T>)
    pub fn eval(op: &str, a: i32, b: i32) -> Option<i32> {
        match op {
            "+" => Some(a + b),
            "-" => Some(a - b),
            "*" => Some(a * b),
            "/" => {
                if b == 0 {
                    None
                } else {
                    Some(a / b)
                }
            }
            _ => None,
        }
    }
}

// ── Topic 2: Visibility with `pub` ─────────────────────────
// By default, everything in Rust is private.
// Use `pub` to make items accessible from outside the module.
// Making a module `pub` does NOT make its contents `pub` —
// you must mark each item individually.

pub mod greetings {
    /// Returns a greeting for the given name.
    /// Example: hello("Alice") => "Hello, Alice!"
    pub fn hello(name: &str) -> String {
        format!("Hello, {name}!")
    }

    /// Returns a farewell message.
    /// Example: goodbye("Bob") => "Goodbye, Bob!"
    pub fn goodbye(name: &str) -> String {
        format!("Goodbye, {name}!")
    }

    // This helper is private — it CANNOT be called from outside `greetings`.
    // Capitalize the first character of a string.
    // Empty string returns empty string.
    fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
        }
    }

    /// Returns a formal greeting. Uses the private `capitalize_first` helper.
    /// Example: formal_hello("alice") => "Good day, Alice."
    pub fn formal_hello(name: &str) -> String {
        let cpt = capitalize_first(name);
        format!("Good day, {cpt}.")
    }

    /// Greets everyone in the slice, returning a Vec of greetings.
    /// Example: greet_all(&["Alice", "Bob"]) => vec!["Hello, Alice!", "Hello, Bob!"]
    /// (Reinforces: borrowing a slice &[&str], returning owned Vec<String>)
    pub fn greet_all(names: &[&str]) -> Vec<String> {
        names.iter().map(|e| hello(e)).collect()
    }

    /// Greets an optional name. Returns "Hello, stranger!" for None.
    /// (Reinforces: match on Option<T>)
    pub fn greet_option(name: Option<&str>) -> String {
        hello(name.unwrap_or("stranger"))
    }
}

// ── Topic 3: Nested Modules and `super` ─────────────────────
// Modules can be nested inside other modules.
// Use `super` to refer to the parent module.
// Child modules can see private items in their parent.

pub mod animals {
    // Private helper — only children of `animals` can access this via `super`.
    // Returns: "The {animal} says {sound}!"
    fn describe(animal: &str, sound: &str) -> String {
        format!("The {animal} says {sound}!")
    }

    pub mod dog {
        /// Uses `super::describe` to access the parent's private function.
        /// Returns: "The dog says woof!"
        pub fn speak() -> String {
            super::describe("dog", "woof")
        }

        /// Returns the dog's name: "Buddy"
        pub fn name() -> &'static str {
            "Buddy"
        }
    }

    pub mod cat {
        /// Returns: "The cat says meow!"
        pub fn speak() -> String {
            super::describe("cat", "meow")
        }

        /// Returns the cat's name: "Whiskers"
        pub fn name() -> &'static str {
            "Whiskers"
        }
    }

    pub mod bird {
        /// Returns: "The bird says tweet!"
        pub fn speak() -> String {
            super::describe("bird", "tweet")
        }

        /// Returns the bird's name: "Tweety"
        pub fn name() -> &'static str {
            "Tweety"
        }
    }

    /// Collects all animal sounds into a Vec.
    /// Returns: vec![dog::speak(), cat::speak(), bird::speak()]
    /// (Reinforces: Vec ownership, calling across nested modules)
    pub fn all_sounds() -> Vec<String> {
        vec![dog::speak(), cat::speak(), bird::speak()]
    }

    /// Finds an animal's sound by name (case-insensitive).
    /// Known names: "buddy" → dog, "whiskers" → cat, "tweety" → bird.
    /// Returns None if no animal matches.
    /// (Reinforces: match + Option<T>, String ownership)
    pub fn find_by_name(name: &str) -> Option<String> {
        match name.to_lowercase().as_str() {
            "buddy" => Some(dog::speak()),
            "whiskers" => Some(cat::speak()),
            "tweety" => Some(bird::speak()),
            _ => None,
        }
    }
}

// ── Topic 4: Struct and Enum Visibility ─────────────────────
// A `pub` struct can have a mix of public and private fields.
// If any field is private, the struct cannot be constructed
// directly from outside — a constructor function is required.
//
// A `pub` enum makes ALL its variants public automatically.

pub mod library {
    /// A book with some public and some private fields.
    pub struct Book {
        pub title: String,
        pub author: String,
        // Private fields: can only be set through the constructor
        isbn: String,
        available: bool,
    }

    impl Book {
        /// Constructor — the only way to create a Book from outside,
        /// since `isbn` and `available` are private.
        /// New books start as available (available = true).
        pub fn new(title: &str, author: &str, isbn: &str) -> Book {
            Book {
                title: title.to_string(),
                author: author.to_string(),
                isbn: isbn.to_string(),
                available: true,
            }
        }

        /// Check out the book (marks as unavailable).
        pub fn check_out(&mut self) {
            self.available = false
        }

        /// Return the book (marks as available).
        pub fn return_book(&mut self) {
            self.available = true
        }

        /// Accessor for the private `available` field.
        pub fn is_available(&self) -> bool {
            self.available
        }

        /// Accessor for the private `isbn` field.
        pub fn isbn(&self) -> &str {
            &self.isbn
        }

        /// Returns a formatted summary string.
        /// Format: "{title} by {author} [ISBN: {isbn}] - {Available/Checked out}"
        /// (Reinforces: if/else, String formatting, &self borrowing)
        pub fn summary(&self) -> String {
            format!(
                "{} by {} [ISBN: {}] - {}",
                &self.title,
                &self.author,
                &self.isbn,
                if self.available {
                    "Available"
                } else {
                    "Checked out"
                }
            )
        }
    }

    /// A public enum — all variants are automatically public.
    pub enum Genre {
        Fiction,
        NonFiction,
        Science,
        History,
    }

    impl Genre {
        /// Returns a human-readable description.
        /// Fiction => "Fiction: novels, stories, and imagination"
        /// NonFiction => "Non-Fiction: facts, essays, and real events"
        /// Science => "Science: exploration and discovery"
        /// History => "History: lessons from the past"
        pub fn description(&self) -> &str {
            match self {
                Genre::Fiction => "Fiction: novels, stories, and imagination",
                Genre::NonFiction => "Non-Fiction: facts, essays, and real events",
                Genre::Science => "Science: exploration and discovery",
                Genre::History => "History: lessons from the past",
            }
        }
    }

    /// A shelf that holds books. The books field is private —
    /// all access goes through methods (encapsulation).
    /// (Reinforces: Vec ownership, &self/&mut self, Option<&T>, iterators)
    pub struct Shelf {
        books: Vec<Book>,
    }

    impl Default for Shelf {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Shelf {
        /// Creates an empty shelf.
        pub fn new() -> Shelf {
            Shelf { books: Vec::new() }
        }

        /// Adds a book to the shelf (takes ownership of the Book).
        pub fn add(&mut self, book: Book) {
            self.books.push(book);
        }

        /// Returns how many books are on the shelf.
        pub fn len(&self) -> usize {
            self.books.iter().count()
        }

        /// Returns true if the shelf has no books.
        pub fn is_empty(&self) -> bool {
            self.books.is_empty()
        }

        /// Finds a book by exact title. Returns a borrowed reference.
        /// Returns None if not found.
        /// (Reinforces: Option<&T>, borrowing from a collection)
        pub fn find_by_title(&self, title: &str) -> Option<&Book> {
            self.books.iter().find(|b| b.title == title)
        }

        /// Returns the count of currently available books.
        /// (Reinforces: iterator + filter + count)
        pub fn available_count(&self) -> usize {
            self.books.iter().filter(|&b| b.available).count()
        }

        /// Returns a list of all book titles (borrowed from the shelf).
        /// (Reinforces: returning Vec<&str> borrowed from owned data)
        pub fn titles(&self) -> Vec<&str> {
            self.books.iter().map(|b| b.title.as_str()).collect()
        }

        /// Checks out a book by title. Returns true if found and was available,
        /// false otherwise (not found or already checked out).
        /// (Reinforces: &mut self, find + mutate, boolean logic)
        pub fn check_out_by_title(&mut self, title: &str) -> bool {
            if let Some(book) = self.books.iter_mut().find(|e| e.title == title) {
                if book.available {
                    book.available = false;
                    return true;
                }
            }
            false
        }
    }
}

// ══════════════════════════════════════════════════════════════
// ⬇ Topics 5–7: CODE IS PROVIDED — Write the TESTS instead! ⬇
// ══════════════════════════════════════════════════════════════
// The module structures below are complete. Your task is to
// write integration tests (in tests/tests.rs) that exercise:
//   - Module paths (e.g. colors::red())
//   - `use` statements to bring items into scope
//   - `use ... as` to create aliases
//   - Nested `use` (e.g. use colors::palettes::{sunset, ocean})
//   - `pub use` re-exports (e.g. music::guitar())
//   - Understanding what is/isn't accessible
// Also write unit tests below (in #[cfg(test)] mod tests)
// for `pub(crate)` items that can't be tested from outside.
// ══════════════════════════════════════════════════════════════

// ── Topic 5: The `use` Keyword and Aliases ──────────────────
// `use` brings paths into scope so you don't have to write
// the full path every time. `as` lets you rename imports.

pub mod colors {
    /// Returns the RGB tuple for red: (255, 0, 0)
    pub fn red() -> (u8, u8, u8) {
        (255, 0, 0)
    }

    /// Returns the RGB tuple for green: (0, 255, 0)
    pub fn green() -> (u8, u8, u8) {
        (0, 255, 0)
    }

    /// Returns the RGB tuple for blue: (0, 0, 255)
    pub fn blue() -> (u8, u8, u8) {
        (0, 0, 255)
    }

    /// Mix two colors by averaging their channels.
    /// Use u16 for intermediate calculations to avoid overflow.
    pub fn mix(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> (u8, u8, u8) {
        (
            ((c1.0 as u16 + c2.0 as u16) / 2) as u8,
            ((c1.1 as u16 + c2.1 as u16) / 2) as u8,
            ((c1.2 as u16 + c2.2 as u16) / 2) as u8,
        )
    }

    /// Format a color as a hex string: "#RRGGBB".
    /// Example: to_hex((255, 0, 0)) => "#FF0000"
    pub fn to_hex(color: (u8, u8, u8)) -> String {
        format!("#{:02X}{:02X}{:02X}", color.0, color.1, color.2)
    }

    /// Calculates perceived brightness (0–255) using the formula:
    /// (r * 299 + g * 587 + b * 114) / 1000
    /// Use u32 for intermediate calculations.
    /// (Reinforces: arithmetic, type casting)
    pub fn brightness(color: (u8, u8, u8)) -> u8 {
        let (r, g, b) = color;
        ((r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000) as u8
    }

    /// Returns true if the color is considered dark (brightness < 128).
    pub fn is_dark(color: (u8, u8, u8)) -> bool {
        brightness(color) < 128
    }

    /// Converts a color name to its RGB tuple.
    /// Supports (case-insensitive): "red", "green", "blue", "white", "black".
    /// Returns None for unknown names.
    /// (Reinforces: match + Option<T>)
    pub fn from_name(name: &str) -> Option<(u8, u8, u8)> {
        match name.to_lowercase().as_str() {
            "red" => Some((255, 0, 0)),
            "green" => Some((0, 255, 0)),
            "blue" => Some((0, 0, 255)),
            "white" => Some((255, 255, 255)),
            "black" => Some((0, 0, 0)),
            _ => None,
        }
    }

    /// A submodule with named color palettes.
    pub mod palettes {
        /// Returns a sunset palette: [(255,94,77), (255,154,0), (255,206,0)]
        pub fn sunset() -> Vec<(u8, u8, u8)> {
            vec![(255, 94, 77), (255, 154, 0), (255, 206, 0)]
        }

        /// Returns an ocean palette: [(0,105,148), (0,154,178), (72,202,228)]
        pub fn ocean() -> Vec<(u8, u8, u8)> {
            vec![(0, 105, 148), (0, 154, 178), (72, 202, 228)]
        }

        /// Returns the average color of a palette.
        /// Empty palette returns (0, 0, 0).
        /// (Reinforces: slices, iteration, type casting, ownership)
        pub fn average(palette: &[(u8, u8, u8)]) -> (u8, u8, u8) {
            if palette.is_empty() {
                return (0, 0, 0);
            }
            let len = palette.len() as u32;
            let (r, g, b) = palette.iter().fold((0u32, 0u32, 0u32), |acc, c| {
                (acc.0 + c.0 as u32, acc.1 + c.1 as u32, acc.2 + c.2 as u32)
            });
            ((r / len) as u8, (g / len) as u8, (b / len) as u8)
        }
    }
}

// ── Topic 6: Re-exports with `pub use` ──────────────────────
// `pub use` re-exports an item, making it available at a
// different (usually shorter) path. This is great for creating
// a clean public API while keeping the internal structure organized.
//
// Notice: `instruments`, `genres`, and `collections` are PRIVATE modules.
// You CANNOT access music::instruments::guitar() from outside.
// But `pub use` re-exports them at the `music::` level.

pub mod music {
    // These submodules are private — users can't reach them directly.
    mod instruments {
        /// Returns "🎸 Guitar"
        pub fn guitar() -> &'static str {
            "🎸 Guitar"
        }

        /// Returns "🎹 Piano"
        pub fn piano() -> &'static str {
            "🎹 Piano"
        }

        /// Returns "🥁 Drums"
        pub fn drums() -> &'static str {
            "🥁 Drums"
        }
    }

    mod genres {
        /// Returns "Rock"
        pub fn rock() -> &'static str {
            "Rock"
        }

        /// Returns "Jazz"
        pub fn jazz() -> &'static str {
            "Jazz"
        }

        /// Returns "Classical"
        pub fn classical() -> &'static str {
            "Classical"
        }
    }

    mod collections {
        /// A playlist with a name and a list of songs.
        /// (Reinforces: structs, Vec ownership, &self/&mut self)
        pub struct Playlist {
            name: String,
            songs: Vec<String>,
        }

        impl Playlist {
            /// Creates an empty playlist with the given name.
            pub fn new(name: &str) -> Playlist {
                Playlist {
                    name: name.to_string(),
                    songs: Vec::new(),
                }
            }

            /// Adds a song to the playlist.
            pub fn add_song(&mut self, song: &str) {
                self.songs.push(song.to_string());
            }

            /// Returns the playlist name.
            pub fn name(&self) -> &str {
                &self.name
            }

            /// Returns the number of songs.
            pub fn len(&self) -> usize {
                self.songs.len()
            }

            /// Returns true if the playlist has no songs.
            pub fn is_empty(&self) -> bool {
                self.songs.is_empty()
            }

            /// Returns a slice of all songs.
            pub fn songs(&self) -> &[String] {
                &self.songs
            }

            /// Checks if a song is in the playlist.
            pub fn contains(&self, song: &str) -> bool {
                self.songs.iter().any(|s| s == song)
            }

            /// Returns a formatted description:
            /// "Playlist '{name}': {song1}, {song2}, ..."
            /// or "Playlist '{name}': (empty)" if no songs.
            pub fn describe(&self) -> String {
                if self.songs.is_empty() {
                    format!("Playlist '{}': (empty)", self.name)
                } else {
                    format!("Playlist '{}': {}", self.name, self.songs.join(", "))
                }
            }
        }
    }

    // Re-export instruments and genres at the music level.
    pub use instruments::guitar;
    pub use instruments::piano;
    pub use instruments::drums;

    pub use genres::rock;
    pub use genres::jazz;
    pub use genres::classical;

    // Re-export Playlist — users can create music::Playlist
    // without knowing about the private `collections` module.
    pub use collections::Playlist;

    /// Combine an instrument and genre into a description.
    /// Example: describe_style("🎸 Guitar", "Rock") => "🎸 Guitar playing Rock"
    pub fn describe_style(instrument: &str, genre: &str) -> String {
        format!("{} playing {}", instrument, genre)
    }
}

// ── Topic 7: Fine-grained Visibility ────────────────────────
// `pub(crate)` — visible anywhere within this crate, but NOT
//                to external crates (including integration tests!).
// `pub(super)` — visible only to the parent module.
//
// Note: Integration tests (in the `tests/` folder) are compiled
// as separate crates, so they CANNOT access `pub(crate)` items.
// You'll write unit tests in the #[cfg(test)] module below
// to test pub(crate) items, and integration tests for the
// public wrapper functions.

pub mod config {
    /// Application config with mixed visibility.
    /// (Reinforces: struct visibility, constructors, method chaining)
    pub struct AppConfig {
        pub(crate) max_retries: u32,
        pub(crate) timeout_secs: u64,
        pub(crate) db_pool_size: u32,
        db_connection: String,
    }

    impl AppConfig {
        /// Creates a default config:
        /// max_retries=3, timeout_secs=30, db_pool_size=5,
        /// db_connection="postgres://localhost:5432/mydb"
        pub(crate) fn default_config() -> AppConfig {
            AppConfig {
                max_retries: 3,
                timeout_secs: 30,
                db_pool_size: 5,
                db_connection: "postgres://localhost:5432/mydb".to_string(),
            }
        }

        /// Builder-style: returns a new config with updated retries.
        /// (Reinforces: ownership — consumes self, returns new owned value)
        pub(crate) fn with_retries(mut self, retries: u32) -> AppConfig {
            self.max_retries = retries;
            self
        }

        /// Builder-style: returns a new config with updated timeout.
        pub(crate) fn with_timeout(mut self, secs: u64) -> AppConfig {
            self.timeout_secs = secs;
            self
        }
    }

    pub mod database {
        // pub(super): only visible to the parent module (`config`)
        // Returns "postgres://localhost:5432/mydb"
        pub(super) fn connection_string() -> String {
            "postgres://localhost:5432/mydb".to_string()
        }

        // pub(crate): visible throughout the crate
        // Returns 5
        pub(crate) fn pool_size() -> u32 {
            5
        }
    }

    /// Public function that internally uses `pub(crate)` items.
    /// Returns: "Config: max_retries=3, timeout=30s, db_pool=5"
    pub fn summary() -> String {
        let cfg = AppConfig::default_config();
        format!(
            "Config: max_retries={}, timeout={}s, db_pool={}",
            cfg.max_retries, cfg.timeout_secs, database::pool_size()
        )
    }

    /// Uses `database::connection_string()` which is `pub(super)`.
    /// Also reads the private `db_connection` field from AppConfig.
    /// Returns: "Config: retries=3, timeout=30s, pool=5, conn=postgres://..."
    pub fn full_summary() -> String {
        let cfg = AppConfig::default_config();
        assert_eq!(cfg.db_connection, database::connection_string());
        format!(
            "Config: retries={}, timeout={}s, pool={}, conn={}",
            cfg.max_retries,
            cfg.timeout_secs,
            database::pool_size(),
            cfg.db_connection
        )
    }

    /// Creates a customized config summary with the builder pattern.
    /// Uses AppConfig::default_config().with_retries(retries).with_timeout(timeout)
    /// Returns: "Config: max_retries={retries}, timeout={timeout}s, db_pool=5"
    /// (Reinforces: ownership chain — each .with_*() consumes and returns)
    pub fn custom_summary(retries: u32, timeout: u64) -> String {
        let cfg = AppConfig::default_config()
            .with_retries(retries)
            .with_timeout(timeout);
        format!(
            "Config: max_retries={}, timeout={}s, db_pool={}",
            cfg.max_retries, cfg.timeout_secs, cfg.db_pool_size
        )
    }
}

// ── Unit tests for pub(crate) items ─────────────────────────
// These MUST live inside the crate because integration tests
// (in tests/) are separate crates and can't see pub(crate) items.
//
// YOUR TASK: Write the unit tests below to verify:
//   - AppConfig::default_config() returns correct defaults
//   - database::pool_size() returns 5
//   - The builder methods (.with_retries, .with_timeout) work
//   - Chaining builders preserves unchanged fields
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pub_crate_default_config() {
        // TODO: Create a default config and assert its fields:
        //   max_retries == 3, timeout_secs == 30, db_pool_size == 5
        // Hint: let cfg = config::AppConfig::default_config();
        todo!()
    }

    #[test]
    fn test_pub_crate_pool_size() {
        // TODO: Assert that config::database::pool_size() returns 5
        // Note: This works because we're inside the crate (pub(crate))
        todo!()
    }

    #[test]
    fn test_pub_crate_builder_retries() {
        // TODO: Create default config, call .with_retries(10),
        //   assert max_retries changed but timeout_secs didn't
        todo!()
    }

    #[test]
    fn test_pub_crate_builder_chain() {
        // TODO: Chain .with_retries(5).with_timeout(60) on default_config,
        //   assert all three fields have expected values
        todo!()
    }
}
