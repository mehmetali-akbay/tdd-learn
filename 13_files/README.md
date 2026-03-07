# Files — File I/O & String Processing

Reading/writing strings as "files", parsing, CSV, config, text stats, log parsing, path ops.
Note: We simulate file I/O with String processing to keep tests simple.

**7 topics | 58 functions | 100 tests**

```bash
cargo test -p files
```

## Topics

1. **Line Processing** — split lines, trim, filter empty, enumerate, take/skip, dedup
   *Reinforces: 05_vecs (iterators, collect), 06_strings (string slices), 11_iterators*
2. **CSV Parsing** — Splitting, parsing structured data, filtering, aggregating
   *Reinforces: 02_structs (CsvRecord), 05_vecs (Vec), 08_results (Option)*
3. **Config File Parsing** — Key=value parsing, comments, typed getters, serialization
   *Reinforces: 07_hashmaps (HashMap), 08_results (Result), 10_generics (parse)*
4. **Text Statistics** — Counting, frequency analysis, aggregation
   *Reinforces: 02_structs (TextStats), 07_hashmaps (frequency), 11_lifetimes (refs)*
5. **Text Transformations** — Line-by-line transforms, sorting, reversing, wrapping
   *Reinforces: 06_strings (String ops), 05_vecs (collect), 11_iterators*
6. **Log Parsing** — Complex multi-field parsing, filtering, aggregation
   *Reinforces: 02_structs (LogEntry), 03_patterns (match/if-let), 07_hashmaps*
7. **Path String Operations** — String-based path manipulation (no actual filesystem)
   *Reinforces: 08_results (Option), 06_strings (split/join), 03_patterns (match)*
