# Key-Value Store

Full project — storage engine, namespaces, TTL, CLI parsing

**28 functions | 26 tests**

```bash
cargo test -p kv_store
```

## Topics

1. **Basic KV Store — CRUD** — HashMap-backed store, Option returns
2. **Keys & Iteration** — Listing, searching, pattern matching on keys
3. **Namespaces — Prefixed Keys** — Key organization, scoped operations
4. **Bulk Operations** — Multi-set, multi-get, import/export
5. **Serialization — Export/Import** — Custom text format, round-trip persistence
6. **Command Parser** — Parsing text commands into structured operations
