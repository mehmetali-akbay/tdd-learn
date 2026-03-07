# Closures & Fn Traits

Fn, FnMut, FnOnce, higher-order functions, closure capture, returning closures,
combinators, callbacks, memoization, generators.

**7 topics | 61 functions | 89 tests**

```bash
cargo test -p closures
```

## Topics

1. **Closure Basics — Capture & Call** — Closures capture variables, || syntax, move, basic HOFs
   *Reinforces: 05_vecs (iterators, collect), 10_generics (generic bounds)*
2. **Fn vs FnMut vs FnOnce** — When each trait is required, mutation vs ownership
   *Reinforces: 03_ownership (move semantics), 09_traits (trait bounds)*
3. **Returning Closures** — impl Fn, Box<dyn Fn>, closure factories, move captures
   *Reinforces: 19_smartptrs (Box), 09_traits (dyn trait)*
4. **Higher-Order Functions — Combinators** — Building complex behavior from simple closures
   *Reinforces: 07_hashmaps (grouping), 11_iterators (chaining)*
5. **Closure Patterns — Callbacks & Strategies** — Strategy pattern, validation, middleware pipelines
   *Reinforces: 04_structs (struct methods), 09_traits (trait objects)*
6. **Advanced — Memoization & Lazy Evaluation** — Caching with closures, deferred computation
   *Reinforces: 07_hashmaps (cache), 10_generics (generic structs)*
7. **Closure-based Generators** — Creating sequences with closures, iterator-like patterns
   *Reinforces: 11_iterators (lazy iteration), 08_results (Option)*
