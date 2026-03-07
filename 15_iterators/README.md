# Iterators — Custom Iterators & Chains

Iterator trait, custom iterators, method chaining,
adaptor patterns, infinite sequences, combinator pipelines

**7 topics | 50 functions | 77 tests**

```bash
cargo test -p iterators
```

## Topics

1. **Iterator Basics — Method Chains** — map, filter, fold, collect, enumerate, zip, sum, product
   *Reinforces: 05_vecs (collect, Vec), 14_closures (Fn trait, HOFs)*
2. **Chained Transformations** — Complex pipelines, take_while, skip_while, dedup, windows
   *Reinforces: 06_strings (string processing), 13_files (line operations)*
3. **Custom Iterator — Counter** — Implementing Iterator trait, next(), stateful iteration
   *Reinforces: 04_structs (struct design), 09_traits (trait impl)*
4. **Custom Iterator — Fibonacci & Sequences** — Stateful iterators, infinite sequences, take/take_while
   *Reinforces: 10_generics (generic thinking), 08_results (Option)*
5. **Custom Iterator — Chunks & Adapters** — Iterator adaptors, chunks, cycle, stepping
   *Reinforces: 10_generics (generic structs), 05_vecs (slicing)*
6. **Advanced — Iterator Combinators** — scan, chain, interleave, windows, complex pipelines
   *Reinforces: 14_closures (closure composition), 07_hashmaps (grouping)*
7. **Iterator Utilities & Conversions** — Practical iterator utilities, batching, folding patterns
   *Reinforces: 08_results (Option/Result with iterators), 06_strings (collect)*
