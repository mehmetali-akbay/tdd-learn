# Patterns

Pattern matching practice with `match`, `if let`, `while let`, destructuring, and recursive enums.

**44 functions | 44 tests**

```bash
cargo test -p patterns
```

## Learning goals

1. Cover core Rust Book Chapter 6 matching patterns:
   - data-carrying enums
   - `Option<T>` + `plus_one` style handling
   - `match` exhaustiveness
   - `if let` / `let...else`
2. Use `match` deliberately:
   - exact values
   - ranges
   - wildcard (`_`)
   - match guards
3. Destructure enums and tuples to model behavior cleanly.
4. Move from `Option<T>`/`Result<T, E>` basics to nested matching.
5. Practice recursive pattern matching on expression trees and binary trees.
6. Keep tests concept-focused: each test validates one pattern idea.

## Recommended TDD order

1. **Topic 1** — pure `match` basics (`describe_number`, `day_name`, `fizzbuzz`)
2. **Topic 2** — enum variants, payload destructuring, and enum methods (`Message::call`)
3. **Topic 3** — `Option` matching and short chains (`plus_one`, `parse_and_double`)
4. **Topic 4** — `if let`, `while let`, `let...else`, `matches!`
5. **Topic 5** — tuple/struct destructuring
6. **Topic 6** — recursive enums + command parsing

## Scope note

This project stays focused on Chapter-6-style matching fundamentals first, then extends into recursive matching drills.
