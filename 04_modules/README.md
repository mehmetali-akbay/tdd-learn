# Module 04: Modules — Organizing Code in Rust

Covers **The Rust Book Chapter 7**: Managing Growing Projects with Packages, Crates, and Modules.

**7 topics | Progressive difficulty**

```bash
cargo test -p modules_intro
```

## Topics

### Topic 1: Module Basics
- Defining modules with `mod`
- Accessing items via absolute paths (`crate::module::item`)
- Accessing items via relative paths (`module::item`)

### Topic 2: Visibility with `pub`
- Items are **private by default**
- Use `pub` to make items accessible from outside
- Making a module `pub` does **NOT** make its contents `pub`
- Private helper functions (internal implementation details)

### Topic 3: Nested Modules and `super`
- Modules inside modules
- Using `super::` to refer to the parent module
- Child modules can see parent's private items

### Topic 4: Struct and Enum Visibility
- Public structs with private fields
- Constructor pattern (`pub fn new(...)`)
- Accessor methods for private fields
- Public enums: **all variants** are automatically public

### Topic 5: The `use` Keyword and Aliases
- Bringing paths into scope with `use`
- Renaming imports with `as`
- Nested `use` statements (`use module::{a, b}`)
- Idiomatic `use` practices

### Topic 6: Re-exports with `pub use`
- Re-exporting items to create a cleaner public API
- Hiding internal module structure from users
- Shortening paths for external consumers

### Topic 7: Fine-grained Visibility
- `pub(crate)`: visible within the crate only
- `pub(super)`: visible to the parent module only
- Why integration tests can't access `pub(crate)` items (they're separate crates!)

## Key Takeaways

- **Modules** organize code into namespaces; they don't need to be in separate files.
- **Privacy is the default** — use `pub` deliberately to expose your API.
- **Structs vs Enums**: Struct fields default to private; enum variants are always public.
- **`use`** creates shortcuts; **`pub use`** creates shortcuts for your users too.
- **`pub(crate)` / `pub(super)`** give you fine-grained control between "fully private" and "fully public."

## References

- [The Rust Book — Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [rust-exercises.com — Modules](https://rust-exercises.com/100-exercises/03_ticket_v1/03_modules)
- [rust-exercises.com — Visibility](https://rust-exercises.com/100-exercises/03_ticket_v1/04_visibility)
- [rust-exercises.com — Encapsulation](https://rust-exercises.com/100-exercises/03_ticket_v1/05_encapsulation)