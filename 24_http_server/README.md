# Mini HTTP Server (Parser)

HTTP protocol, request/response parsing, routing

**41 functions | 36 tests**

```bash
cargo test -p http_server
```

## Topics

1. **HTTP Method & Status Code** — Enums for protocol elements
2. **HTTP Headers** — Case-insensitive header map
3. **HTTP Request — Parsing** — Parsing raw HTTP text into structured data
4. **HTTP Response — Building** — Constructing HTTP responses
5. **Router — Path Matching** — Pattern matching for routes, path parameters
6. **URL Encoding/Decoding** — Percent-encoding, URL parsing
