# Lifetimes — References That Last

Lifetime annotations, structs with refs, elision rules, lifetime bounds, combining lifetimes with traits & generics

**76 functions | 89 tests**

```bash
cargo test -p lifetimes
```

## Topics

1. **Lifetime Annotations on Functions** — `'a` syntax, when annotations are needed, input/output lifetime relationships ← Reinforces: 01_ownership, 06_strings
2. **Structs Holding References** — Lifetime parameters on structs, methods with `&self`, Display ← Reinforces: 04_structs, 09_traits, 06_strings
3. **Custom Iterators with Lifetimes** — Building iterators that yield borrowed data, lifetime in Item type ← Reinforces: 11_iterators, 05_vecs
4. **Multiple Lifetimes** — Different lifetime parameters for different inputs, `with_text`/`with_tag` patterns ← Reinforces: 10_generics, 05_vecs
5. **'static & Lifetime Elision** — `'static` lifetime, three elision rules, methods on owned data returning borrows ← Reinforces: 02_strings, 08_results
6. **Lifetime Bounds on Generics** — `T: 'a`, where clauses, combining generics + lifetimes, `AsRef` bounds ← Reinforces: 10_generics, 09_traits
7. **Combined — Lifetimes + Traits + Generics** — Trait impls on lifetime-parameterized types, `Ref<'a, T>`, `LookupTable<'a>`, parsing with lifetimes ← Reinforces: 09_traits, 10_generics, 08_results
