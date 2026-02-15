# Unsafe Rust — Raw Pointers, FFI & Send/Sync

Raw pointers, unsafe fn, unsafe blocks, static mut, Send & Sync, safe abstractions

**6 topics | Progressive difficulty**

```bash
cargo test -p unsafe_rust
```

## Topics

1. **Raw Pointers** — `*const T`, `*mut T`, creating from references
2. **Unsafe Functions & Blocks** — `unsafe fn`, calling unsafe code
3. **Dereferencing Raw Pointers** — Safe abstractions over unsafe code
4. **Mutable Static Variables** — `static mut`, why it's unsafe, alternatives
5. **Send & Sync Marker Traits** — Thread safety guarantees
6. **Advanced — Safe Abstraction: Custom `split_at_mut`** — Building safe APIs on unsafe
