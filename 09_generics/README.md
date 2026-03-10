# Generics — Generic Types & Functions

Type parameters, constraints, generic structs/enums, where clauses,
multiple type params, Fn bounds, closures, pipelines

**~55 functions/methods | 90 tests**

```bash
cargo test -p generics
```

## Topics

1. **Generic Functions** — `first`, `last`, `contains`, `max_of_two`, `min_of_two`, `swap`, `deduplicate`, `find_first`, `zip_slices` *(Reinforces: slices, borrowing, Option, Vec)*
2. **Wrapper\<T\> — Generic Struct** — `new`, `get`, `set`, `into_inner`, `map`, `unwrap_or`, `zip`, conditional `Display` impl *(Reinforces: ownership/move, FnOnce, Display)*
3. **Pair\<T, U\> — Multiple Type Parameters** — `swap`, `map_first`, `map_second`, `mixup` (Rust Book §10.1), `into_tuple`, `Display` *(Reinforces: ownership, type transformations)*
4. **Generic Enums — Maybe\<T\> & Either\<L, R\>** *(New!)* — Custom `Option`/`Result`-like enums with `unwrap`, `unwrap_or`, `map`, `to_option`, `from_option`, `left`, `right`, `map_left`, `map_right` *(Reinforces: enums, pattern matching, Option/Result semantics)*
5. **Stack\<T\> — Generic Collection** — `from_vec`, `push`, `pop`, `peek`, `reverse`, `drain_to_vec`, `Default`, `Display` *(Reinforces: Vec, ownership, iterating)*
6. **Trait Bounds & Constraints** — `min_of_three`, `clamp`, `sort_three`, `is_sorted` (where clause), `partition`, `format_joined`, `group_by`, `Bounded<T>` struct *(Reinforces: PartialOrd, Display, HashMap, Vec)*
7. **Fn Bounds, Closures & Pipeline** — `apply`, `apply_twice`, `compose`, `Pipeline`, `filter_map_ok`, `try_apply`, `try_map`, `fold` *(Reinforces: closures, Result, error handling)*
