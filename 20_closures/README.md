# Closures & Fn Traits

Fn, FnMut, FnOnce, higher-order functions, closure capture

**37 functions | 37 tests**

```bash
cargo test -p closures
```

## Topics

1. **Closure Basics — Capture & Call** — Closures capture variables, || syntax, move
2. **Fn vs FnMut vs FnOnce** — When each trait is required
3. **Returning Closures** — impl Fn, Box<dyn Fn>, closure factories
4. **Higher-Order Functions — Combinators** — Building complex behavior from simple closures
5. **Closure Patterns — Callbacks & Strategies** — Strategy pattern, event handlers, middleware
6. **Advanced — Memoization & Lazy Evaluation** — Caching with closures, deferred computation
