# Macros — macro_rules!

Declarative macro practice from easy to hard.

**6 topic blocks | 26 tests | progressive difficulty**

```bash
cargo test -p macros

# Run only a difficulty slice:
cargo test -p macros -- test_easy_
cargo test -p macros -- test_medium_
cargo test -p macros -- test_hard_
```

In the `practice` branch, macro bodies are intentionally scaffolded with `todo!()` so you can implement them yourself.

## Topics

1. **Easy: Foundation Macros** — `hashmap!`, `vec_of!`, `min!`/`max!`, `repeat_vec!`, `count!`, `sum!`, `avg!`, `newtype!`
2. **Medium: Multi-arm + Item Generation** — `when!`, `str_join!`, `assert_between!`, `coalesce!`, `string_enum!`, `builder!`
3. **Hard: Utility + Recursive Patterns** — `timed!`, `retry!`, `match_or!`, `dbg_expr!`, `reverse_vec!`, `nest!`, `chain_calls!`, `const_add!`
