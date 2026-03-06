# Traits — Defining & Implementing Traits

Trait definition, impl, Display, Default, operator overloading, From/Into,
trait bounds, where clauses, default methods, impl Trait, trait objects

**~50 functions/methods | 86 tests**

```bash
cargo test -p traits
```

## Topics

1. **Basic Traits — Defining & Implementing** — Define a trait, implement for multiple structs, `&dyn Trait`, collect descriptions *(Reinforces: Vec, slices, Option)*
2. **Display & Debug — Standard Library Traits** — Implement `fmt::Display`, derive macros, `Temperature` with enum units, `Color` hex formatting *(Reinforces: enums, match, borrowing, arithmetic)*
3. **Default Trait & Builder Pattern** — Implement `Default`, consuming-self builder methods, `is_default()` *(Reinforces: struct construction, String ownership, PartialEq)*
4. **Operator Overloading** *(New!)* — Implement `Add`, `Sub`, `Neg` for `Vec2d`; derive `PartialEq`/`PartialOrd`/`Ord` for `Money`; manual `PartialEq` with epsilon *(Reinforces: structs, arithmetic, Display formatting)*
5. **From / Into Conversions** — `Celsius`↔`Fahrenheit`, `Slug`, `Rgb`, `Coord` from array, `Book` from tuple, `Stats` from slice, `Priority` enum from `&str` *(Reinforces: tuples, enums, pattern matching, slices, Vec operations)*
6. **Trait Bounds & Generic Functions** — `largest`/`smallest`, `sort_and_dedup` with `where` clause, `unique_ordered` with multiple bounds (`Eq + Hash + Clone`), `frequency_map` *(Reinforces: Vec, HashMap, generics, iteration)*
7. **Default Methods, Trait Objects & impl Trait** — `Summarizable` with default calling required method (Rust Book pattern), `Taggable` with default methods, `collect_summaries`, `make_social_post` returning `impl Summarizable`, `filter_by_tag` with lifetimes *(Reinforces: Vec, String, trait objects)*
