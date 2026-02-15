# TDD Rust Learning Path

A progressive Rust learning path with **987 tests** across **34 projects**, organized into 9 levels of increasing difficulty. Learn by implementing functions to make failing tests pass.

## How to Use

```bash
# Run tests for a project:
cargo test -p strings

# Run a single test:
cargo test -p strings -- test_hello_string

# See the solution:
git diff main -- 02_strings/src/lib.rs

# Switch to full solution branch:
git checkout main
```

## Levels

| Level | Projects | Focus |
|-------|----------|-------|
| **1** | `01_vecs` `02_strings` `03_ownership` `04_patterns` | Fundamentals |
| **2** | `05_shapes` `06_types` `07_traits` `08_generics` | Type system |
| **3** | `09_results` `10_hashmaps` `11_iterators` `12_files` | Collections & I/O |
| **4** | `13_todolist` `14_calculator` `15_enums_deep` `16_modules` `17_testing` `18_lifetimes` `19_smartptrs` | Intermediate |
| **5** | `20_closures` `21_macros` `22_threads` | Closures, macros, concurrency |
| **6** | `23_lru_cache` `24_http_server` `25_kv_store` `26_markdown` | Full projects |
| **7** | `27_advanced_traits` `28_advanced_types` `29_trait_objects` `30_unsafe` | Expert language features |
| **8** | `31_async` `32_serde` `33_error_handling` | Ecosystem & async |
| **9** | `34_async_project` | Capstone project |

Each project has 6 topics with progressive difficulty. See each project's `README.md` for details.
