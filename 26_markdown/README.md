# Markdown Parser

Parsing, AST, recursion, HTML rendering

**16 functions | 29 tests**

```bash
cargo test -p markdown
```

## Topics

1. **Inline Elements — Bold, Italic, Code, Links** — Parsing inline markup, nested elements
2. **Block Elements — AST** — Headings, paragraphs, lists, code blocks, blockquotes
3. **Block Parser** — Line-based parsing, state machine
4. **HTML Renderer** — AST to HTML conversion, escaping
5. **Document Structure — Extraction** — Extracting metadata from parsed documents
6. **Table of Contents Generator** — Building structure from flat data
