# Async — Async/Await, Futures & Streams

async fn, .await, tokio, join!, select!, channels, timeouts, streams

**67 functions | 89 tests**

```bash
cargo test -p async_rust
```

## Topics

1. **Future Basics** — async fn, .await, identity, fibonacci, chained transforms
2. **Tokio Runtime & Spawning** — tokio::spawn, JoinHandle, fallible tasks, spawn_map
3. **Concurrent Futures** — join!, fetch_all, concurrent_sum, race with select!
4. **Async Channels** — mpsc, oneshot, multi-producer, pipeline, request-reply
5. **Async Utilities** — sleep, timeout, retry, interval, map_with_timeout
6. **Streams** — Countdown, RangeStream, stream combinators (sum, count, take, skip, chain)
7. **Shared Async State** — AsyncCounter, AsyncCache, AsyncTaskQueue, limited concurrency
