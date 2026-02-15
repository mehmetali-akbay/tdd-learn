# Async Chat Server — Capstone Project

TCP-based async chat server with rooms, message protocol, broadcast, graceful shutdown

**6 topics | Progressive difficulty**

```bash
cargo test -p async_project
```

## Topics

1. **Message Protocol** — Define and parse chat message types
2. **Broadcast Channel** — `tokio::sync::broadcast` for message fanout
3. **Session Management** — Track connected users, nicknames
4. **Chat Room Logic** — Multiple rooms, join/leave
5. **Graceful Shutdown** — Signal handling, clean disconnect
6. **Advanced — Full Chat Server Integration** — End-to-end message flow
