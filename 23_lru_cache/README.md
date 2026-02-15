# LRU Cache

Generics, HashMap, doubly-linked list, capacity management

**27 functions | 28 tests**

```bash
cargo test -p lru_cache
```

## Topics

1. **Node & Doubly-Linked List Primitives** — Raw index-based linked list, Option-based links
2. **LruCache Core — get, put, capacity** — HashMap + linked list combination
3. **Iteration & Inspection** — Iterating over cache entries in LRU order
4. **Resize & Advanced Operations** — Dynamic capacity changes, bulk operations
5. **Stats & Metrics** — Tracking cache performance
6. **Peek & Bulk Operations** — Non-updating reads, bulk filtering
