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
            0
        } else {
            a / b
        }
    }

    /// Clamps `value` so it stays within [min, max].
    /// If value < min, returns min. If value > max, returns max.
    /// Otherwise returns value.
    /// (Reinforces: if/else chains)
    pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
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
    pub fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    /// Returns a farewell message.
    pub fn goodbye(name: &str) -> String {
        format!("Goodbye, {}!", name)
    }

    // This helper is private — it CANNOT be called from outside `greetings`.
    fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
        }
    }

    /// Returns a formal greeting. Uses the private `capitalize_first` helper.
    pub fn formal_hello(name: &str) -> String {
        let capitalized = capitalize_first(name);
        format!("Good day, {}.", capitalized)
    }

    /// Greets everyone in the slice, returning a Vec of greetings.
    /// (Reinforces: borrowing a slice &[&str], returning owned Vec<String>)
    pub fn greet_all(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| hello(name)).collect()
    }

    /// Greets an optional name. Returns "Hello, stranger!" for None.
    /// (Reinforces: match on Option<T>)
    pub fn greet_option(name: Option<&str>) -> String {
        match name {
            Some(n) => hello(n),
            None => "Hello, stranger!".to_string(),
        }
    }
}

// ── Topic 3: Nested Modules and `super` ─────────────────────
// Modules can be nested inside other modules.
// Use `super` to refer to the parent module.
// Child modules can see private items in their parent.

pub mod animals {
    // Private helper — only children of `animals` can access this via `super`.
    fn describe(animal: &str, sound: &str) -> String {
        format!("The {} says {}!", animal, sound)
    }

    pub mod dog {
        /// Uses `super::describe` to access the parent's private function.
        pub fn speak() -> String {
            super::describe("dog", "woof")
        }

        pub fn name() -> &'static str {
            "Buddy"
        }
    }

    pub mod cat {
        pub fn speak() -> String {
            super::describe("cat", "meow")
        }

        pub fn name() -> &'static str {
            "Whiskers"
        }
    }

    pub mod bird {
        pub fn speak() -> String {
            super::describe("bird", "tweet")
        }

        pub fn name() -> &'static str {
            "Tweety"
        }
    }

    /// Collects all animal sounds into a Vec.
    /// (Reinforces: Vec ownership, calling across nested modules)
    pub fn all_sounds() -> Vec<String> {
        vec![dog::speak(), cat::speak(), bird::speak()]
    }

    /// Finds an animal's sound by name (case-insensitive).
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
            self.available = false;
        }

        /// Return the book (marks as available).
        pub fn return_book(&mut self) {
            self.available = true;
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
            let status = if self.available {
                "Available"
            } else {
                "Checked out"
            };
            format!(
                "{} by {} [ISBN: {}] - {}",
                self.title, self.author, self.isbn, status
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
            self.books.len()
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
            self.books.iter().filter(|b| b.available).count()
        }

        /// Returns a list of all book titles (borrowed from the shelf).
        /// (Reinforces: returning Vec<&str> borrowed from owned data)
        pub fn titles(&self) -> Vec<&str> {
            self.books.iter().map(|b| b.title.as_str()).collect()
        }

        /// Checks out a book by title. Returns true if found and was available,
        /// false otherwise.
        /// (Reinforces: &mut self, find + mutate, boolean logic)
        pub fn check_out_by_title(&mut self, title: &str) -> bool {
            if let Some(book) = self.books.iter_mut().find(|b| b.title == title)
                && book.available
            {
                book.available = false;
                return true;
            }
            false
        }
    }
}

// ── Topic 5: The `use` Keyword and Aliases ──────────────────
// `use` brings paths into scope so you don't have to write
// the full path every time. `as` lets you rename imports.

pub mod colors {
    pub fn red() -> (u8, u8, u8) {
        (255, 0, 0)
    }

    pub fn green() -> (u8, u8, u8) {
        (0, 255, 0)
    }

    pub fn blue() -> (u8, u8, u8) {
        (0, 0, 255)
    }

    /// Mix two colors by averaging their channels.
    pub fn mix(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> (u8, u8, u8) {
        (
            ((c1.0 as u16 + c2.0 as u16) / 2) as u8,
            ((c1.1 as u16 + c2.1 as u16) / 2) as u8,
            ((c1.2 as u16 + c2.2 as u16) / 2) as u8,
        )
    }

    /// Format a color as a hex string: "#RRGGBB".
    pub fn to_hex(color: (u8, u8, u8)) -> String {
        format!("#{:02X}{:02X}{:02X}", color.0, color.1, color.2)
    }

    /// Calculates perceived brightness (0–255) using the formula:
    /// (r * 299 + g * 587 + b * 114) / 1000
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
    /// Supports: "red", "green", "blue", "white", "black".
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
        pub fn sunset() -> Vec<(u8, u8, u8)> {
            vec![(255, 94, 77), (255, 154, 0), (255, 206, 0)]
        }

        pub fn ocean() -> Vec<(u8, u8, u8)> {
            vec![(0, 105, 148), (0, 154, 178), (72, 202, 228)]
        }

        /// Returns the average color of a palette.
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

pub mod music {
    // These submodules are private — users can't reach them directly.
    mod instruments {
        pub fn guitar() -> &'static str {
            "🎸 Guitar"
        }

        pub fn piano() -> &'static str {
            "🎹 Piano"
        }

        pub fn drums() -> &'static str {
            "🥁 Drums"
        }
    }

    mod genres {
        pub fn rock() -> &'static str {
            "Rock"
        }

        pub fn jazz() -> &'static str {
            "Jazz"
        }

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
            pub fn new(name: &str) -> Playlist {
                Playlist {
                    name: name.to_string(),
                    songs: Vec::new(),
                }
            }

            pub fn add_song(&mut self, song: &str) {
                self.songs.push(song.to_string());
            }

            pub fn name(&self) -> &str {
                &self.name
            }

            pub fn len(&self) -> usize {
                self.songs.len()
            }

            pub fn is_empty(&self) -> bool {
                self.songs.is_empty()
            }

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
        /// Creates a default config.
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
        pub(super) fn connection_string() -> String {
            "postgres://localhost:5432/mydb".to_string()
        }

        // pub(crate): visible throughout the crate
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
    /// Returns full config summary including connection string.
    pub fn full_summary() -> String {
        let cfg = AppConfig::default_config();
        // Verify the stored connection matches the database module's value.
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