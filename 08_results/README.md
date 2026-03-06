# Results — Error Handling

Result<T,E>, ? operator, custom errors, error conversion

**22 functions | 26 tests**

```bash
cargo test -p results
```

## Topics

1. **Result Basics** — Ok/Err, unwrap_or, map, and_then
2. **The ? Operator** — Early return with ?, propagating errors
3. **Custom Error Types** — Define error enums, implement Display, std::error::Error
4. **Error Conversion — From trait** — impl From<E1> for E2, automatic conversion with ?
5. **Collecting Results** — collect() into Result, partition results
6. **Advanced — Result Combinators** — and_then, or_else, map_err, flatten
