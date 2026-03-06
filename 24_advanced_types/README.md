# Advanced Types — Type System Deep-Dive

Type aliases, newtype validation, never type, DSTs, function pointers, PhantomData

**6 topics | Progressive difficulty**

```bash
cargo test -p advanced_types
```

## Topics

1. **Type Aliases** — `type Result<T> = ...`, reducing repetition
2. **Newtype for Validation** — Wrapper types enforcing invariants
3. **The Never Type (`!`)** — Diverging functions, infinite loops
4. **DSTs & `Sized`** — `?Sized`, `str` vs `String`, trait objects
5. **Function Pointers** — `fn` vs `Fn`, passing functions as arguments
6. **Advanced — PhantomData & Type-State** — Marker types, compile-time state machines
