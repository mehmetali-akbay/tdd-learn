# Smart Pointers — Box, Rc, RefCell, Arc

Heap allocation, shared ownership, interior mutability

**40 functions | 35 tests**

```bash
cargo test -p smartptrs
```

## Topics

1. **Box<T> — Heap Allocation** — Box for recursive types, trait objects
2. **Rc<T> — Shared Ownership** — Reference counting, Rc::clone, strong_count
3. **RefCell<T> — Interior Mutability** — Borrow checking at runtime, borrow/borrow_mut
4. **Rc<RefCell<T>> — Shared Mutable State** — Combining Rc and RefCell for shared mutable data
5. **Drop Trait & Custom Smart Pointers** — Drop for cleanup, custom wrapper types
6. **Advanced — Observer Pattern** — Rc<RefCell<>> for callbacks, dynamic dispatch
