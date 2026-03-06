# Results — Error Handling

Result<T,E>, ? operator, custom errors, error conversion, custom validation types

**~46 functions/methods | ~68 tests**

```bash
cargo test -p results
```

## Topics

1. **Result Basics** — Ok/Err, unwrap_or, map, and_then
   - *New*: `find_first_even`, `lookup_grade`
   - *Reinforces*: slice borrowing, match on ranges, iteration

2. **The ? Operator** — Early return with ?, propagating errors
   - *New*: `parse_point`, `sum_positive_from_csv`, `parse_int_range`
   - *Reinforces*: String splitting, Vec collection, tuples

3. **Custom Error Types** — Error enums, Display, std::error::Error
   - *New*: Enhanced `ValidationError` (8 variants), `validate_email`, `ValidatedName` (Guess pattern §9.3), `validate_registration`
   - *Reinforces*: enum pattern matching, struct encapsulation, character iteration

4. **Error Conversion — From trait** — impl From, automatic conversion with ?
   - *New*: `UserProfile` struct, `create_user_profile`, `parse_config_map`, `find_user_summary`
   - *Reinforces*: struct construction, HashMap building, Option → Result

5. **Collecting Results** — collect() into Result, partition, filter
   - *New*: `filter_valid_in_range`, `parse_matrix_row`, `validate_usernames`
   - *Reinforces*: Vec operations, match guards, tuples of (name, error)

6. **Advanced — Result Combinators & Patterns** — and_then, or_else, map_err, custom types
   - *New*: `BoundedI32` (Guess pattern §9.3), `option_to_result`, `result_to_option`, `first_valid_parse`, `apply_operation`
   - *Reinforces*: struct encapsulation, Option ↔ Result bridging, match on string slices

## Prior Topics Reinforced

- **Ownership & Borrowing** (Module 01): slice borrowing (`&[i32]`, `&[&str]`), reference params, owned vs borrowed returns
- **Structs** (Module 02): `UserProfile`, `ValidatedName`, `BoundedI32` — constructors, methods, encapsulation
- **Pattern Matching** (Module 03): match on enum variants, ranges, guards, if let, destructuring
- **Vecs** (Module 05): collect into `Vec`, `filter_map`, iteration, nested `Vec<(String, Error)>`
- **Strings** (Module 06): parsing, splitting, formatting, `String` vs `&str`
- **HashMaps** (Module 07): building `HashMap` from parsed config lines

## Key Concepts from Rust Book Chapter 9

- `Result<T, E>` and matching on variants
- The `?` operator for error propagation and From-based conversion
- Custom error types with `Display` and `std::error::Error`
- `From` trait for automatic error conversion
- Custom types for validation (the "Guess" pattern, §9.3)
- When to panic vs when to return `Result`

## References

- [The Rust Book — Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [rust-exercises.com](https://rust-exercises.com)
