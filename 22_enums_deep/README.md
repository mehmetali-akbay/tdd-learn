# Enums Deep — Slices, Option & Cow

Slices, Option combinators, if let/let else, Cow, enum dispatch

**6 topics | Progressive difficulty**

```bash
cargo test -p enums_deep
```

## Topics

1. **Slice Basics** — `&[T]`, `&str`, indexing, windows, chunks
2. **Option Combinators** — `map`, `and_then`, `unwrap_or_else`, `filter`, `zip`
3. **`if let` / `let...else` / `matches!`** — Concise control flow beyond basic match
4. **Nested Enums & Flattening** — `Option<Option<T>>`, `Result<Option<T>>`, flatten
5. **`Cow<T>` & Borrow/ToOwned** — Zero-copy patterns, `Cow::Borrowed` vs `Cow::Owned`
6. **Advanced — Enum Dispatch** — Enum vs trait object dispatch, performance
