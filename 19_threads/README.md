# Threads — Concurrency

std::thread, Mutex, Arc, channels, thread safety

**26 functions | 28 tests**

```bash
cargo test -p threads
```

## Topics

1. **Thread Basics — Spawning & Joining** — thread::spawn, JoinHandle, move closures
2. **Shared State — Mutex** — Mutex<T>, lock, poisoned mutex
3. **Channels — Message Passing** — mpsc::channel, Sender, Receiver
4. **Arc<Mutex<T>> — Shared Mutable State** — Combining Arc with Mutex for thread-safe shared state
5. **Thread Pool Pattern** — Fixed thread pool, work distribution
6. **Advanced — Parallel Reduce** — Divide-and-conquer with threads
