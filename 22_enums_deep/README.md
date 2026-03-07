# 22 Enums Deep — Slices, Option, Cow & Enum Patterns

10 topics | 91 public items | 167 tests | Progressive difficulty

```bash
cargo test -p enums_deep
```

## Topics

1. **Slice Basics** — `&[T]`, indexing, windows, chunks, rotate, sort-check, dedup
2. **Option Combinators** — `map`, `and_then`, `unwrap_or`, `filter`, `zip`, max, multiply
3. **if let / let...else / matches!** — Concise control flow, range checks, char classification
4. **Nested Enums & Flattening** — `Option<Option<T>>`, `Result<Option<T>>`, transpose, partition
5. **Cow\<T\> & Borrow/ToOwned** — Zero-copy patterns, `Cow::Borrowed` vs `Cow::Owned`, replace, truncate
6. **Enum Dispatch** — `Shape` enum with area, perimeter, scale, sort, count, filter
7. **Rich Enums with Data** — `Expr` tree: eval, node_count, depth, has_negation
8. **State Machines with Enums** — `TrafficLight` transitions, `OrderStatus` workflow with guards
9. **Result Combinators** — `map_err`, `and_then`, `or_else`, parse chains, fallback
10. **Enum-based Error Types** — Custom `AppError`, `Display`, `From`, error classification
