# Testing — Patterns & Organization

`#[test]`, `#[should_panic]`, Result tests, `#[ignore]`, property tests, test helpers

**7 topics | 56 functions | 98 tests (2 ignored) | Progressive difficulty**

```bash
cargo test -p testing_patterns
cargo test -p testing_patterns -- --include-ignored   # run slow tests too
```

## Topics

1. **Pure Functions — Basic Assertions** — `assert!`, `assert_eq!`, `assert_ne!`, custom messages
   *Reinforces: 01_ownership (values), 06_strings (format!)*

2. **Panic & Result — Error Testing** — `#[should_panic(expected="...")]`, `Result<(), String>` return tests
   *Reinforces: 08_results (Result/Error), 03_patterns (match)*

3. **Struct Testing & Helpers** — Test fixtures/factories, struct methods, `Display`/`Clone`/`PartialEq`
   *Reinforces: 02_structs (methods), 09_traits (Display), 11_lifetimes (refs)*

4. **Expensive Computations — `#[ignore]`** — Marking slow tests, `--include-ignored`, Fibonacci/primes
   *Reinforces: 05_vecs (building vectors), 03_patterns (loops)*

5. **Data Structures — Boundary & State** — Stack state transitions, empty/full/overflow, sort properties
   *Reinforces: 10_generics (Stack\<T\>), 05_vecs (Vec ops), 07_hashmaps (HashSet)*

6. **Roundtrip & Property Testing** — Encode/decode roundtrips, idempotency, invariant checking
   *Reinforces: 06_strings (char manipulation), 08_results (error returns)*

7. **Validation & Trait Testing** — Table-driven tests, trait impls, parameterized-style patterns
   *Reinforces: 09_traits (trait defs/impls), 10_generics (dyn Trait), 03_patterns (match)*
