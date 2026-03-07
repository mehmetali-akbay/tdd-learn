# Trait Objects — OOP & Dynamic Dispatch

dyn Trait, Box<dyn Trait>, object safety, State pattern, Strategy pattern, Observer pattern, Command pattern, Any, closures as trait objects

**10 topics | 116 public items | 119 tests | Progressive difficulty**

```bash
cargo test -p trait_objects
```

## Topics

1. **Trait Objects Basics** — `dyn Trait`, `Box<dyn Trait>`, object safety, `area()`, `total_area`, `largest_by_area`
2. **Heterogeneous Collections** — `Vec<Box<dyn Trait>>`, Canvas, `filter_by_min_area`, `remove_by_name`
3. **State Pattern** — State machine with trait objects, `Draft → PendingReview → Published`
4. **Strategy Pattern** — Swappable algorithms (Bubble, Insertion, Quick, Selection), `sort_with_all`
5. **`Any` & Downcasting** — `AnyBox`, `TypeMap`, `get_all`, `count_of`, `remove_all_of`
6. **Static vs Dynamic Dispatch** — `&impl Trait` vs `&dyn Trait`, `format_chain`, `TrimFormatter`, `ReverseFormatter`
7. **Observer Pattern** — `EventBus`, `Logger`, `Counter`, `subscribe`/`unsubscribe`/`emit`
8. **Command Pattern** — `AddCommand`, `RemoveCommand`, `ClearCommand`, `CommandHistory` with undo
9. **Trait Object Composition** — Supertraits (`Product: Describable + Priceable`), `Book`, `Gadget`, filtering
10. **Closure Trait Objects** — `Box<dyn Fn>`, `TransformRegistry`, `PredicateFn`, `and`/`or`/`not` combinators
