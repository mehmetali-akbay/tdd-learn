# Structs — Chapter 5

Struct-first TDD practice aligned to Rust Book Chapter 5 (`struct`s and `impl`).

**54 functions | 50 tests**

```bash
cargo test -p structs
```

## Learning goals

1. Define and instantiate named-field structs.
2. Practice ownership and borrowing with `String` fields inside structs.
3. Use field init shorthand and struct update syntax (`..`).
4. Work with tuple structs and unit-like structs.
5. Recreate and extend the Rust Book rectangle flow (`area`, `can_hold`, `square`, methods).
6. Model a realistic account domain with validated builders, typed errors, and state transitions.

## Topic map

1. **User** — struct fields, mutation methods, and role management.
2. **Ownership** — move vs borrow behavior for struct fields.
3. **Update Syntax** — clone and move-based updates.
4. **Tuple/Unit Structs** — newtypes and marker type.
5. **Rectangle** — book-aligned APIs plus extended utility methods.
6. **Account Builder** — validation, typed errors, and lifecycle transitions.

## Recommended TDD order

1. User constructors and mutation methods.
2. Ownership helper functions.
3. Struct update functions.
4. Tuple/unit struct helpers.
5. Rectangle methods and associated functions.
6. Builder validation and account state transitions.
