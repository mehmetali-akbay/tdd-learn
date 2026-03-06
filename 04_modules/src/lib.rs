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
    fn capitalize_first(s: &str) -> String {
        todo!()
    }

    /// Returns a formal greeting. Uses the private `capitalize_first` helper.
    /// Example: formal_hello("alice") => "Good day, Alice."
    pub fn formal_hello(name: &str) -> String {
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
    // pub(crate): accessible from anywhere within this crate
    pub(crate) fn default_max_retries() -> u32 {
        todo!()
    }

    // pub(crate): accessible from anywhere within this crate
    pub(crate) fn default_timeout_secs() -> u64 {
        todo!()
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
    /// Returns: "Config: max_retries={}, timeout={}s, db_pool={}"
    pub fn summary() -> String {
        todo!()
    }

    /// Uses `database::connection_string()` which is `pub(super)`.
    /// Returns: "Config: retries={}, timeout={}s, pool={}, conn={}"
    pub fn full_summary() -> String {
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