# Todo List — Structs & Data Management

CRUD, filtering, sorting, serialization, IDs,
builder patterns, bulk ops, iteration, transformation

**7 topics | 65 functions | 91 tests**

```bash
cargo test -p todolist
```

## Topics

1. **TodoItem — Struct Design** — Struct with multiple fields, Display, equality, builder pattern
   *Reinforces: structs (field design), enums (Priority), patterns (matching)*
2. **TodoList — Collection Management** — Vec-backed collection, CRUD, ID generation
   *Reinforces: vecs (Vec operations), results (Option)*
3. **Filtering & Searching** — Iterator-based filtering, closures, combined predicates
   *Reinforces: iterators (filter, map), closures (Fn trait)*
4. **Sorting & Ordering** — sort_by, Ord, multi-key sorting, custom comparators
   *Reinforces: iterators (sort), patterns (matching), closures*
5. **Serialization — to/from string** — Custom serialization (simple text format), formatting
   *Reinforces: strings (parsing, formatting), results (Option/Result)*
6. **Statistics & Bulk Operations** — Aggregation, batch operations, HashMap counting
   *Reinforces: hashmaps (counting), iterators (fold, collect)*
7. **Iteration & Transformation** — Custom iteration, merging, transforming collections
   *Reinforces: iterators (adapters, collect), closures (FnMut)*
