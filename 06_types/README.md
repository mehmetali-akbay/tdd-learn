# Types — Complex Enums

Enum-focused practice: data-carrying variants, recursive enums, and state-machine style modeling.

**35 functions | 38 tests**

```bash
cargo test -p types
```

## Learning goals

1. Model mixed data with enums and attach behavior via `impl`.
2. Practice exhaustive matching on variants with payloads.
3. Build recursive types with `Box` and evaluate them recursively.
4. Use enum-based error modeling with `Result<T, AppError>`.
5. Compose nested enums (`Payload` containing `JsonValue`) with clear semantics.

## Recommended TDD order

1. **JsonValue**: variant semantics (`truthy`, type names, display).
2. **TrafficLight**: transition/state behavior.
3. **List**: recursive enum basics (`len`, `sum`, `contains`, conversions).
4. **Expr**: recursive evaluation + rendering.
5. **AppError** + helpers: explicit error paths.
6. **Payload**: nested enum composition and merge rules.

## Scope note

This module keeps the core path focused on enum design and pattern matching. Extra conversion drills are intentionally excluded from the main test progression.
