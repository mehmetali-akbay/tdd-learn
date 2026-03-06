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
        todo!()
    }

    /// Subtracts b from a.
    pub fn subtract(a: i32, b: i32) -> i32 {
        todo!()
    }

    /// Multiplies two numbers.
    pub fn multiply(a: i32, b: i32) -> i32 {
        todo!()
    }

    /// Divides a by b (integer division). Returns 0 if b is 0.
    pub fn safe_divide(a: i32, b: i32) -> i32 {
        todo!()
    }

    /// Clamps `value` so it stays within [min, max].
    /// If value < min, returns min. If value > max, returns max.
    /// Otherwise returns value.
    /// (Reinforces: if/else chains)
    pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
        todo!()
    }

    /// Evaluates a simple math expression given an operator string.
    /// Supported operators: "+", "-", "*", "/"
    /// Returns None for unknown operators or division by zero.
    /// (Reinforces: match + Option<T>)
    pub fn eval(op: &str, a: i32, b: i32) -> Option<i32> {
        todo!()
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
        todo!()
    }

    /// Returns a farewell message.
    /// Example: goodbye("Bob") => "Goodbye, Bob!"
    pub fn goodbye(name: &str) -> String {
        todo!()
    }

    // This helper is private — it CANNOT be called from outside `greetings`.
    // Capitalize the first character of a string.
    // Empty string returns empty string.
    fn capitalize_first(s: &str) -> String {
        todo!()
    }

    /// Returns a formal greeting. Uses the private `capitalize_first` helper.
    /// Example: formal_hello("alice") => "Good day, Alice."
    pub fn formal_hello(name: &str) -> String {
        todo!()
    }

    /// Greets everyone in the slice, returning a Vec of greetings.
    /// Example: greet_all(&["Alice", "Bob"]) => vec!["Hello, Alice!", "Hello, Bob!"]
    /// (Reinforces: borrowing a slice &[&str], returning owned Vec<String>)
    pub fn greet_all(names: &[&str]) -> Vec<String> {
        todo!()
    }

    /// Greets an optional name. Returns "Hello, stranger!" for None.
    /// (Reinforces: match on Option<T>)
    pub fn greet_option(name: Option<&str>) -> String {
        todo!()
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
        todo!()
    }

    pub mod dog {
        /// Uses `super::describe` to access the parent's private function.
        /// Returns: "The dog says woof!"
        pub fn speak() -> String {
            todo!()
        }

        /// Returns the dog's name: "Buddy"
        pub fn name() -> &'static str {
            todo!()
        }
    }

    pub mod cat {
        /// Returns: "The cat says meow!"
        pub fn speak() -> String {
            todo!()
        }

        /// Returns the cat's name: "Whiskers"
        pub fn name() -> &'static str {
            todo!()
        }
    }

    pub mod bird {
        /// Returns: "The bird says tweet!"
        pub fn speak() -> String {
            todo!()
        }

        /// Returns the bird's name: "Tweety"
        pub fn name() -> &'static str {
            todo!()
        }
    }

    /// Collects all animal sounds into a Vec.
    /// Returns: vec![dog::speak(), cat::speak(), bird::speak()]
    /// (Reinforces: Vec ownership, calling across nested modules)
    pub fn all_sounds() -> Vec<String> {
        todo!()
    }

    /// Finds an animal's sound by name (case-insensitive).
    /// Known names: "buddy" → dog, "whiskers" → cat, "tweety" → bird.
    /// Returns None if no animal matches.
    /// (Reinforces: match + Option<T>, String ownership)
    pub fn find_by_name(name: &str) -> Option<String> {
        todo!()
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
            todo!()
        }

        /// Check out the book (marks as unavailable).
        pub fn check_out(&mut self) {
            todo!()
        }

        /// Return the book (marks as available).
        pub fn return_book(&mut self) {
            todo!()
        }

        /// Accessor for the private `available` field.
        pub fn is_available(&self) -> bool {
            todo!()
        }

        /// Accessor for the private `isbn` field.
        pub fn isbn(&self) -> &str {
            todo!()
        }

        /// Returns a formatted summary string.
        /// Format: "{title} by {author} [ISBN: {isbn}] - {Available/Checked out}"
        /// (Reinforces: if/else, String formatting, &self borrowing)
        pub fn summary(&self) -> String {
            todo!()
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
            todo!()
        }
    }

    /// A shelf that holds books. The books field is private —
    /// all access goes through methods (encapsulation).
    /// (Reinforces: Vec ownership, &self/&mut self, Option<&T>, iterators)
    pub struct Shelf {
        books: Vec<Book>,
    }

    impl Shelf {
        /// Creates an empty shelf.
        pub fn new() -> Shelf {
            todo!()
        }

        /// Adds a book to the shelf (takes ownership of the Book).
        pub fn add(&mut self, book: Book) {
            todo!()
        }

        /// Returns how many books are on the shelf.
        pub fn len(&self) -> usize {
            todo!()
        }

        /// Returns true if the shelf has no books.
        pub fn is_empty(&self) -> bool {
            todo!()
        }

        /// Finds a book by exact title. Returns a borrowed reference.
        /// Returns None if not found.
        /// (Reinforces: Option<&T>, borrowing from a collection)
        pub fn find_by_title(&self, title: &str) -> Option<&Book> {
            todo!()
        }

        /// Returns the count of currently available books.
        /// (Reinforces: iterator + filter + count)
        pub fn available_count(&self) -> usize {
            todo!()
        }

        /// Returns a list of all book titles (borrowed from the shelf).
        /// (Reinforces: returning Vec<&str> borrowed from owned data)
        pub fn titles(&self) -> Vec<&str> {
            todo!()
        }

        /// Checks out a book by title. Returns true if found and was available,
        /// false otherwise (not found or already checked out).
        /// (Reinforces: &mut self, find + mutate, boolean logic)
        pub fn check_out_by_title(&mut self, title: &str) -> bool {
            todo!()
        }
    }
}

// ── Topic 5: The `use` Keyword and Aliases ──────────────────
// `use` brings paths into scope so you don't have to write
// the full path every time. `as` lets you rename imports.

pub mod colors {
    /// Returns the RGB tuple for red: (255, 0, 0)
    pub fn red() -> (u8, u8, u8) {
        todo!()
    }

    /// Returns the RGB tuple for green: (0, 255, 0)
    pub fn green() -> (u8, u8, u8) {
        todo!()
    }

    /// Returns the RGB tuple for blue: (0, 0, 255)
    pub fn blue() -> (u8, u8, u8) {
        todo!()
    }

    /// Mix two colors by averaging their channels.
    /// Use u16 for intermediate calculations to avoid overflow.
    pub fn mix(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> (u8, u8, u8) {
        todo!()
    }

    /// Format a color as a hex string: "#RRGGBB".
    /// Example: to_hex((255, 0, 0)) => "#FF0000"
    pub fn to_hex(color: (u8, u8, u8)) -> String {
        todo!()
    }

    /// Calculates perceived brightness (0–255) using the formula:
    /// (r * 299 + g * 587 + b * 114) / 1000
    /// Use u32 for intermediate calculations.
    /// (Reinforces: arithmetic, type casting)
    pub fn brightness(color: (u8, u8, u8)) -> u8 {
        todo!()
    }

    /// Returns true if the color is considered dark (brightness < 128).
    pub fn is_dark(color: (u8, u8, u8)) -> bool {
        todo!()
    }

    /// Converts a color name to its RGB tuple.
    /// Supports (case-insensitive): "red", "green", "blue", "white", "black".
    /// Returns None for unknown names.
    /// (Reinforces: match + Option<T>)
    pub fn from_name(name: &str) -> Option<(u8, u8, u8)> {
        todo!()
    }

    /// A submodule with named color palettes.
    pub mod palettes {
        /// Returns a sunset palette: [(255,94,77), (255,154,0), (255,206,0)]
        pub fn sunset() -> Vec<(u8, u8, u8)> {
            todo!()
        }

        /// Returns an ocean palette: [(0,105,148), (0,154,178), (72,202,228)]
        pub fn ocean() -> Vec<(u8, u8, u8)> {
            todo!()
        }

        /// Returns the average color of a palette.
        /// Empty palette returns (0, 0, 0).
        /// (Reinforces: slices, iteration, type casting, ownership)
        pub fn average(palette: &[(u8, u8, u8)]) -> (u8, u8, u8) {
            todo!()
        }
    }
}

// ── Topic 6: Re-exports with `pub use` ──────────────────────
// `pub use` re-exports an item, making it available at a
// different (usually shorter) path. This is great for creating
// a clean public API while keeping the internal structure organized.

pub mod music {
    // These submodules are private — users can't reach them directly.
    mod instruments {
        /// Returns "🎸 Guitar"
        pub fn guitar() -> &'static str {
            todo!()
        }

        /// Returns "🎹 Piano"
        pub fn piano() -> &'static str {
            todo!()
        }

        /// Returns "🥁 Drums"
        pub fn drums() -> &'static str {
            todo!()
        }
    }

    mod genres {
        /// Returns "Rock"
        pub fn rock() -> &'static str {
            todo!()
        }

        /// Returns "Jazz"
        pub fn jazz() -> &'static str {
            todo!()
        }

        /// Returns "Classical"
        pub fn classical() -> &'static str {
            todo!()
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
                todo!()
            }

            /// Adds a song to the playlist.
            pub fn add_song(&mut self, song: &str) {
                todo!()
            }

            /// Returns the playlist name.
            pub fn name(&self) -> &str {
                todo!()
            }

            /// Returns the number of songs.
            pub fn len(&self) -> usize {
                todo!()
            }

            /// Returns true if the playlist has no songs.
            pub fn is_empty(&self) -> bool {
                todo!()
            }

            /// Returns a slice of all songs.
            pub fn songs(&self) -> &[String] {
                todo!()
            }

            /// Checks if a song is in the playlist.
            pub fn contains(&self, song: &str) -> bool {
                todo!()
            }

            /// Returns a formatted description:
            /// "Playlist '{name}': {song1}, {song2}, ..."
            /// or "Playlist '{name}': (empty)" if no songs.
            pub fn describe(&self) -> String {
                todo!()
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
        todo!()
    }
}

// ── Topic 7: Fine-grained Visibility ────────────────────────
// `pub(crate)` — visible anywhere within this crate, but NOT
//                to external crates (including integration tests!).
// `pub(super)` — visible only to the parent module.
//
// Note: Integration tests (in the `tests/` folder) are compiled
// as separate crates, so they CANNOT access `pub(crate)` items.
// We provide public wrapper functions to test the behavior.

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
            todo!()
        }

        /// Builder-style: returns a new config with updated retries.
        /// (Reinforces: ownership — consumes self, returns new owned value)
        pub(crate) fn with_retries(mut self, retries: u32) -> AppConfig {
            todo!()
        }

        /// Builder-style: returns a new config with updated timeout.
        pub(crate) fn with_timeout(mut self, secs: u64) -> AppConfig {
            todo!()
        }
    }

    pub mod database {
        // pub(super): only visible to the parent module (`config`)
        // Returns "postgres://localhost:5432/mydb"
        pub(super) fn connection_string() -> String {
            todo!()
        }

        // pub(crate): visible throughout the crate
        // Returns 5
        pub(crate) fn pool_size() -> u32 {
            todo!()
        }
    }

    /// Public function that internally uses `pub(crate)` items.
    /// Returns: "Config: max_retries=3, timeout=30s, db_pool=5"
    pub fn summary() -> String {
        todo!()
    }

    /// Uses `database::connection_string()` which is `pub(super)`.
    /// Also reads the private `db_connection` field from AppConfig.
    /// Returns: "Config: retries=3, timeout=30s, pool=5, conn=postgres://..."
    pub fn full_summary() -> String {
        todo!()
    }

    /// Creates a customized config summary with the builder pattern.
    /// Uses AppConfig::default_config().with_retries(retries).with_timeout(timeout)
    /// Returns: "Config: max_retries={retries}, timeout={timeout}s, db_pool=5"
    /// (Reinforces: ownership chain — each .with_*() consumes and returns)
    pub fn custom_summary(retries: u32, timeout: u64) -> String {
        todo!()
    }
}

// ── Unit tests for pub(crate) items ─────────────────────────
// These MUST live inside the crate because integration tests
// (in tests/) are separate crates and can't see pub(crate) items.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pub_crate_default_config() {
        let cfg = config::AppConfig::default_config();
        assert_eq!(cfg.max_retries, 3);
        assert_eq!(cfg.timeout_secs, 30);
        assert_eq!(cfg.db_pool_size, 5);
    }

    #[test]
    fn test_pub_crate_pool_size() {
        assert_eq!(config::database::pool_size(), 5);
    }

    #[test]
    fn test_pub_crate_builder_retries() {
        let cfg = config::AppConfig::default_config().with_retries(10);
        assert_eq!(cfg.max_retries, 10);
        assert_eq!(cfg.timeout_secs, 30); // unchanged
    }

    #[test]
    fn test_pub_crate_builder_chain() {
        let cfg = config::AppConfig::default_config()
            .with_retries(5)
            .with_timeout(60);
        assert_eq!(cfg.max_retries, 5);
        assert_eq!(cfg.timeout_secs, 60);
        assert_eq!(cfg.db_pool_size, 5); // unchanged
    }
}