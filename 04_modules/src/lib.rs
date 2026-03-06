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

    /// A submodule with named color palettes.
    pub mod palettes {
        pub fn sunset() -> Vec<(u8, u8, u8)> {
            vec![(255, 94, 77), (255, 154, 0), (255, 206, 0)]
        }

        pub fn ocean() -> Vec<(u8, u8, u8)> {
            vec![(0, 105, 148), (0, 154, 178), (72, 202, 228)]
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

    // Re-export: users can call `music::guitar()` directly.
    // Without this, `instruments` is private so users can't
    // write `music::instruments::guitar()`.
    pub use instruments::guitar;
    pub use instruments::piano;
    pub use instruments::drums;

    pub use genres::rock;
    pub use genres::jazz;
    pub use genres::classical;

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
    // pub(crate): accessible from anywhere within this crate
    pub(crate) fn default_max_retries() -> u32 {
        3
    }

    // pub(crate): accessible from anywhere within this crate
    pub(crate) fn default_timeout_secs() -> u64 {
        30
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
    /// This can be called from integration tests.
    pub fn summary() -> String {
        format!(
            "Config: max_retries={}, timeout={}s, db_pool={}",
            default_max_retries(),
            default_timeout_secs(),
            database::pool_size()
        )
    }

    /// Uses `database::connection_string()` which is `pub(super)`.
    /// This works because `config` is the parent of `database`.
    pub fn full_summary() -> String {
        format!(
            "Config: retries={}, timeout={}s, pool={}, conn={}",
            default_max_retries(),
            default_timeout_secs(),
            database::pool_size(),
            database::connection_string()
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
    fn test_pub_crate_max_retries() {
        assert_eq!(config::default_max_retries(), 3);
    }

    #[test]
    fn test_pub_crate_timeout() {
        assert_eq!(config::default_timeout_secs(), 30);
    }

    #[test]
    fn test_pub_crate_pool_size() {
        assert_eq!(config::database::pool_size(), 5);
    }
}