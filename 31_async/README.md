# Async — Async/Await, Futures & Streams

async fn, .await, tokio, join!, select!, channels, timeouts

**6 topics | Progressive difficulty**

```bash
cargo test -p async_rust
```

## Topics

1. **Future Basics** — `async fn`, `.await`, `Future` trait concept
2. **Tokio Runtime** — `#[tokio::test]`, spawning tasks
3. **Concurrent Futures** — `join!`, running multiple futures
4. **Async Channels** — `tokio::sync::mpsc`, `oneshot`
5. **Async Utilities** — `tokio::time::sleep`, `timeout`
6. **Advanced — Async Patterns** — select!, graceful shutdown, error handling in async
