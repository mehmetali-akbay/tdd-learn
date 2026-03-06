# Lifetimes — References & Borrowing

Lifetime annotations, structs with refs, lifetime bounds

**27 functions | 26 tests**

```bash
cargo test -p lifetimes
```

## Topics

1. **Lifetime Basics — Function Annotations** — 'a syntax, input/output lifetime relationships
2. **Structs with Lifetimes** — Structs holding references, lifetime in impl blocks
3. **Iterators with Lifetimes** — Returning iterators over borrowed data
4. **Multiple Lifetimes** — Different lifetimes for different parameters
5. **Lifetime Elision & Static** — When lifetimes can be omitted, 'static
6. **Advanced — Lifetime Bounds on Generics** — T: 'a, where clauses with lifetimes
