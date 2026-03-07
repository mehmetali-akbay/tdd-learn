# Calculator — Full Mini Project

Tokenizer, parser, evaluator, end-to-end design with `+`, `-`, `*`, `/`, `%`, `^`, parentheses, variables, and multiple notations (infix, postfix, prefix).

**53 functions | 90 tests**

```bash
cargo test -p calculator
```

## Topics

1. **Tokens — Lexical Analysis** — Enums for tokens, character scanning, token classification
2. **AST — Abstract Syntax Tree** — Recursive data structures, tree traversal, node operations
3. **Parser — Recursive Descent** — Operator precedence, right-associativity, error recovery
4. **Evaluator** — Recursive evaluation, error handling, numeric utilities
5. **End-to-End — calc()** — Composing stages, batch processing, result formatting
6. **Variables & Environment** — HashMap state management, substitution, merging
7. **Postfix, Prefix & Utilities** — Stack-based evaluation, notation conversion, validation
