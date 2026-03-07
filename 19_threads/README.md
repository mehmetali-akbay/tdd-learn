# Threads — Concurrency

std::thread, Mutex, Arc, channels, thread safety

**77 functions | 98 tests**

```bash
cargo test -p threads
```

## Topics

1. **Thread Basics — Spawning & Joining** — thread::spawn, JoinHandle, move closures, named threads
2. **Shared State — Mutex** — Mutex<T>, lock, parallel filter/max/min
3. **Channels — Message Passing** — mpsc::channel, Sender, Receiver, fanout, accumulate, ping-pong
4. **Arc<Mutex<T>> — Shared Mutable State** — AtomicCounter, BoundedBuffer (producer-consumer)
5. **Thread Pool Pattern** — Fixed thread pool, work distribution, pool_map
6. **Advanced — Parallel Algorithms** — parallel reduce, count, any, all
7. **Thread-Safe Data Structures** — ConcurrentMap, SharedLog, SharedStats
