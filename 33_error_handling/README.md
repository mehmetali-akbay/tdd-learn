# Error Handling — Production Patterns

thiserror, anyhow, error composition, hierarchies, panic vs Result

**6 topics | Progressive difficulty**

```bash
cargo test -p error_handling
```

## Topics

1. **`thiserror` Basics** — `#[derive(Error)]`, struct and enum errors
2. **`anyhow` for Applications** — `anyhow::Result`, `context()`, `bail!`
3. **Error Composition** — `#[from]`, `#[source]`, multiple error sources
4. **Custom Error Hierarchies** — Domain errors, mapping between layers
5. **`panic` vs `Result` Strategies** — When to panic, `catch_unwind`
6. **Advanced — Error Reporting** — Backtraces, error chains, formatting
