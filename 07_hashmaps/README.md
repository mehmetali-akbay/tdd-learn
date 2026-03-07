# HashMaps — Associative Collections

HashMap, BTreeMap, HashSet, Entry API, counting, grouping, set operations

**68 public items | 186 tests**

Covers: The Rust Book Chapter 8, Section 3

```bash
cargo test -p hashmaps
```

## Topics

1. **Creating Hash Maps** — `HashMap::new()` + insert, collect from iterators, `HashMap::from()`
2. **Accessing Values** — `get()` → `Option<&V>`, `copied()`, `contains_key`, `keys()`, `values()`
3. **Ownership and Hash Maps** — String keys are moved, `insert()` returns old value, `clone`, `extend`
4. **Updating — Entry API** — `or_insert`, `or_insert_with`, `and_modify`, `values_mut`, `retain`, `remove`
5. **Counting and Grouping** — word count (Rust Book classic), char frequency, group by criteria
6. **BTreeMap — Ordered Maps** — sorted keys, range queries, `min`/`max`, nth key
7. **Map Transformations** — filter, map values, sum, max/min key, sort by value, partition
8. **HashSet Fundamentals** — unique, intersection, union, difference, symmetric difference, subset, disjoint
9. **Nested Maps & Complex Structures** — grade book, department directory (Rust Book exercise)
10. **Advanced Patterns** — two-sum, most frequent, anagram groups, first unique char, longest consecutive
