# Trait Objects — OOP & Dynamic Dispatch

dyn Trait, Box<dyn Trait>, object safety, State pattern, strategy pattern, Any

**6 topics | Progressive difficulty**

```bash
cargo test -p trait_objects
```

## Topics

1. **Trait Objects Basics** — `dyn Trait`, `Box<dyn Trait>`, object safety
2. **Heterogeneous Collections** — `Vec<Box<dyn Trait>>`, processing mixed types
3. **State Pattern** — State machine with trait objects
4. **Strategy Pattern** — Swappable algorithms via trait objects
5. **`Any` & Downcasting** — Runtime type info, `downcast_ref`
6. **Advanced — Static vs Dynamic Dispatch** — Monomorphization vs vtable trade-offs
