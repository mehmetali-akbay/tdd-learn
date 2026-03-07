# Smart Pointers — Box, Rc, RefCell, Arc

Heap allocation, shared ownership, interior mutability, thread-safe shared state with `Arc<Mutex<>>`, observer pattern, event bus, and custom smart pointer wrappers.

**104 functions | 79 tests**

```bash
cargo test -p smartptrs
```

## Topics

1. **Box<T> — Heap Allocation** — Box for recursive types, trait objects, tree operations
2. **Rc<T> — Shared Ownership** — Reference counting, Rc::clone, strong_count, graph structures
3. **RefCell<T> — Interior Mutability** — Borrow checking at runtime, shared logging, filtering
4. **Rc<RefCell<T>> — Shared Mutable State** — Bank accounts, transfers, shared cache
5. **Drop Trait & Custom Smart Pointers** — Drop for cleanup, tracked instances, undo history, verbose wrapper
6. **Observer Pattern & Callbacks** — Rc<RefCell<>> for callbacks, event emitter, typed event bus
7. **Arc<T> & Mutex<T> — Thread Safety** — AtomicCounter, SharedLog, ConcurrentMap across threads
