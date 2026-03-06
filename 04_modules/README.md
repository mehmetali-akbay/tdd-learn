# Module 04: Modules — Organizing Code in Rust

Covers **The Rust Book Chapter 7**: Managing Growing Projects with Packages, Crates, and Modules.

**7 topics | Progressive difficulty | Reinforces prior topics**

```bash
cargo test -p modules_intro
```

## Topics

### Topic 1: Module Basics
- Defining modules with `mod`, accessing items via paths
- **New:** `clamp` (if/else chains), `eval` (match + Option\<T\>)
- *Reinforces: control flow, pattern matching*

### Topic 2: Visibility with `pub`
- Items are **private by default**, `pub` exposes them
- Private helper functions stay hidden from outside
- **New:** `greet_all` (borrowed slices → owned Vec), `greet_option` (Option\<T\>)
- *Reinforces: borrowing (&[&str]), ownership (Vec\<String\>), match on Option*

### Topic 3: Nested Modules and `super`
- Modules inside modules, `super::` for parent access
- **New:** `all_sounds` (collect from submodules), `find_by_name` (match → Option)
- *Reinforces: Vec ownership, pattern matching*

### Topic 4: Struct and Enum Visibility ⭐
- Public structs with private fields, constructors, accessors
- Public enums: all variants are automatically public
- **New:** `Book::summary()`, `Shelf` struct (owns Vec\<Book\>)
  - `find_by_title` → Option\<&Book\>, `check_out_by_title`, `available_count`, `titles`
- *Reinforces: ownership (Vec\<T\>), borrowing (&self, &mut self), Option\<&T\>, iterators*

### Topic 5: The `use` Keyword and Aliases
- Bringing paths into scope with `use`, renaming with `as`
- Nested `use` statements
- **New:** `brightness`, `is_dark`, `from_name` (match → Option), `palettes::average`
- *Reinforces: arithmetic, type casting, match, slices*

### Topic 6: Re-exports with `pub use`
- Re-exporting items to create a cleaner public API
- **New:** `Playlist` struct (in private submodule, re-exported with `pub use`)
  - `add_song`, `contains`, `songs`, `describe`
- *Reinforces: struct methods, Vec ownership, &self/&mut self*

### Topic 7: Fine-grained Visibility
- `pub(crate)`, `pub(super)` — when and why
- **New:** `AppConfig` struct with builder-style methods (`with_retries`, `with_timeout`)
- `custom_summary` uses ownership chain (each builder call consumes and returns)
- *Reinforces: builder pattern, ownership transfer*

## Key Takeaways

- **Modules** organize code into namespaces — privacy is the default.
- **Structs vs Enums**: Struct fields default to private; enum variants are always public.
- **`use`** creates shortcuts; **`pub use`** creates shortcuts for your users too.
- **`pub(crate)` / `pub(super)`** give fine-grained control between private and public.
- **Encapsulation**: private fields + constructors + accessors = controlled invariants.

## References

- [The Rust Book — Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [rust-exercises.com — Modules](https://rust-exercises.com/100-exercises/03_ticket_v1/03_modules)
- [rust-exercises.com — Visibility](https://rust-exercises.com/100-exercises/03_ticket_v1/04_visibility)
- [rust-exercises.com — Encapsulation](https://rust-exercises.com/100-exercises/03_ticket_v1/05_encapsulation)