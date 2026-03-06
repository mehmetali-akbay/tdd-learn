# Advanced Traits — Associated Types, Operators & Supertraits

Associated types, operator overloading, supertraits, newtype, fully qualified syntax

**6 topics | Progressive difficulty**

```bash
cargo test -p advanced_traits
```

## Topics

1. **Associated Types** — `type Item`, Iterator pattern, why not generics
2. **Operator Overloading** — `Add`, `Sub`, `Mul`, `Neg`, `Index`
3. **Supertraits** — `trait A: B + C`, trait inheritance
4. **Newtype Pattern** — External traits on external types, wrapper types
5. **Fully Qualified Syntax** — `<Type as Trait>::method()`, disambiguation
6. **Advanced — Blanket Implementations** — Conditional impls, `impl<T: Display>`
