# Patterns

Pattern matching practice with `match`, `if let`, `while let`, destructuring, and recursive enums.

**39 functions | 40 tests**

```bash
cargo test -p patterns
```

## Learning goals

1. Use `match` deliberately:
   - exact values
   - ranges
   - wildcard (`_`)
   - match guards
2. Destructure enums and tuples to model behavior cleanly.
3. Move from `Option<T>`/`Result<T, E>` basics to nested matching.
4. Practice recursive pattern matching on expression trees and binary trees.
5. Keep tests concept-focused: each test validates one pattern idea.

## Recommended TDD order

1. **Topic 1** — pure `match` basics (`describe_number`, `day_name`, `fizzbuzz`)
2. **Topic 2** — enum variants and payload destructuring
3. **Topic 3** — `Option` matching and short chains (`parse_and_double`)
4. **Topic 4** — `if let`, `while let`, `matches!`
5. **Topic 5** — tuple/struct destructuring
6. **Topic 6** — recursive enums + command parsing

## Scope note

This project intentionally avoids "extra practice" APIs in the core test suite so the main path stays focused on pattern matching fundamentals.
