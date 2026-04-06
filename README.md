# TDD Rust Learning Path

A progressive Rust learning path with hundreds of tests across **39 projects**, organized to follow [The Rust Programming Language](https://doc.rust-lang.org/book/) book chapter order. Learn by implementing functions to make failing tests pass.

## How to Use

```bash
# Run tests for a project:
cargo test -p ownership

# Run a single test:
cargo test -p ownership -- test_move_semantics

# Start practicing (function bodies are replaced with todo!()):
git checkout practice

# See the solution for a project:
git diff practice main -- 01_ownership/src/lib.rs

# Switch to full solution branch:
git checkout main
```

## Recommended Learning Path

Follow this order — it aligns with [The Rust Book](https://doc.rust-lang.org/book/) and [100 Exercises to Learn Rust](https://rust-exercises.com/100-exercises/).

### Phase 1 — Core Language

| # | Project | Rust Book | Concepts |
|---|---------|-----------|----------|
| 1 | `01_ownership` | [Ch. 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) | Move, clone, Copy, borrowing, slices |
| 2 | `02_structs` | [Ch. 5](https://doc.rust-lang.org/book/ch05-00-structs.html) | Named/tuple/unit structs, methods, builders |
| 3 | `03_patterns` | [Ch. 6](https://doc.rust-lang.org/book/ch06-00-enums.html) | Enums, `match`, `if let`, `Option<T>`, destructuring |
| 4 | `04_modules` | [Ch. 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) | `mod`, `pub`, `use`, visibility, re-exports |

### Phase 2 — Collections & Error Handling

| # | Project | Rust Book | Concepts |
|---|---------|-----------|----------|
| 5 | `05_vecs` | [Ch. 8](https://doc.rust-lang.org/book/ch08-01-vectors.html) | Vec CRUD, iteration, slices, sorting |
| 6 | `06_strings` | [Ch. 8](https://doc.rust-lang.org/book/ch08-02-strings.html) | String vs &str, manipulation, parsing |
| 7 | `07_hashmaps` | [Ch. 8](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) | HashMap, Entry API, BTreeMap |
| 8 | `08_results` | [Ch. 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html) | `Result<T,E>`, `?` operator, custom errors |

### Phase 3 — Generics, Traits & Lifetimes

| # | Project | Rust Book | Concepts |
|---|---------|-----------|----------|
| 9 | `09_generics` | [Ch. 10](https://doc.rust-lang.org/book/ch10-01-generic-data-types.html) | Generic functions, structs, bounds, `where` |
| 10 | `10_traits` | [Ch. 10](https://doc.rust-lang.org/book/ch10-02-traits.html) | Trait definition, `Display`, `Default`, `From`/`Into` |
| 11 | `11_lifetimes` | [Ch. 10](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) | Lifetime annotations, elision, `'static` |

### Phase 4 — I/O & Functional Features

| # | Project | Rust Book | Concepts |
|---|---------|-----------|----------|
| 13 | `13_files` | [Ch. 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) | Line processing, CSV, config parsing, text stats |
| 14 | `14_closures` | [Ch. 13](https://doc.rust-lang.org/book/ch13-01-closures.html) | Capture, `Fn`/`FnMut`/`FnOnce`, higher-order functions |
| 15 | `15_iterators` | [Ch. 13](https://doc.rust-lang.org/book/ch13-02-iterators.html) | Iterator trait, adapters, `collect`, custom iterators |

### 🔨 Checkpoint — Mini Projects 1

| # | Project | Prerequisites | What you build |
|---|---------|---------------|----------------|
| 16 | `16_todolist` | Phases 1–3 | Struct-based CRUD with filtering & sorting |
| 17 | `17_calculator` | Phases 1–3 | Tokenizer → parser → evaluator pipeline |

### Phase 5 — Smart Pointers, Concurrency & Async

| # | Project | Rust Book | Concepts |
|---|---------|-----------|----------|
| 18 | `18_smartptrs` | [Ch. 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) | `Box`, `Rc`, `RefCell`, `Arc`, `Drop` |
| 19 | `19_threads` | [Ch. 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | `spawn`, `Mutex`, `mpsc`, `Arc`, thread pools |
| 20 | `20_async` | [Ch. 17](https://doc.rust-lang.org/book/ch17-00-async-await.html) | `async`/`.await`, Tokio, channels, `select!` |

### Phase 6 — OOP & Advanced Patterns

| # | Project | Rust Book | Concepts |
|---|---------|-----------|----------|
| 21 | `21_trait_objects` | [Ch. 18](https://doc.rust-lang.org/book/ch18-00-oop.html) | `dyn Trait`, object safety, State & Strategy patterns |
| 22 | `22_enums_deep` | [Ch. 19](https://doc.rust-lang.org/book/ch19-00-patterns.html) | Slices, `Option` combinators, `Cow`, enum dispatch |

### Phase 7 — Advanced Features

| # | Project | Rust Book | Concepts |
|---|---------|-----------|----------|
| 23 | `23_advanced_traits` | [Ch. 20](https://doc.rust-lang.org/book/ch20-00-advanced-features.html) | Associated types, operator overloading, supertraits |
| 24 | `24_advanced_types` | [Ch. 20](https://doc.rust-lang.org/book/ch20-00-advanced-features.html) | Type aliases, newtype, never type, `PhantomData` |
| 25 | `25_unsafe` | [Ch. 20](https://doc.rust-lang.org/book/ch20-00-advanced-features.html) | Raw pointers, `unsafe fn`, `Send`/`Sync` |
| 26 | `26_macros` | [Ch. 20](https://doc.rust-lang.org/book/ch20-00-advanced-features.html) | `macro_rules!`, repetition, struct/enum generation |

### Phase 8 — Ecosystem & Production Patterns

| # | Project | Focus | Concepts |
|---|---------|-------|----------|
| 27 | `27_error_handling` | Error crates | `thiserror`, `anyhow`, error hierarchies |
| 28 | `28_serde` | Serialization | `serde_json`, derive macros, tagged enums, TOML |
| 29 | `29_types` | Consolidation | Complex enums, `Box` recursion, `Result` chains, nested enums |

### 🔨 Checkpoint — Mini Projects 2

| # | Project | What you build |
|---|---------|----------------|
| 30 | `30_kv_store` | HashMap-backed store with namespaces & command parser |
| 31 | `31_http_server` | HTTP request parser, router & response builder |
| 32 | `32_lru_cache` | Generic LRU cache with linked-list internals |
| 33 | `33_markdown` | Markdown → HTML parser with AST |

### 🔨 Capstone

| # | Project | What you build |
|---|---------|----------------|
| 34 | `34_async_project` | Async chat server with Tokio |

### Phase 9 — Systems Programming

| # | Project | Focus |
|---|---------|-------|
| 35 | `35_ring_buffer` | Circular buffer, index wrapping |
| 36 | `36_bitwise` | Bit manipulation, flags, packed data |
| 37 | `37_binary_protocol` | Binary parsing, endianness, checksums |
| 38 | `38_arena_allocator` | Custom allocation, bump allocator |
| 39 | `39_mini_shell` | Command parsing, pipes, env vars |
| 40 | `40_concurrency_primitives` | Atomics, `Ordering`, custom locks |

Each project has 6 topics with progressive difficulty. See each project's `README.md` for details.
