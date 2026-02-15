# Shapes — Structs & Methods

Struct-first TDD practice: define data types, keep invariants, and compose behavior with methods.

**45 functions | 36 tests**

```bash
cargo test -p shapes
```

## Learning goals

1. **Model data with structs**: named fields, associated constructors (`new`, `origin`, `unit`).
2. **Write method APIs with references**: `&self` readers and pure transformation methods that return new values.
3. **Enforce invariants in constructors**:
   - `Rectangle::new` and `Circle::new` reject non-positive sizes.
   - `BoundingBox::new` normalizes corners so `min <= max`.
4. **Compose structs**: `Circle` and `BoundingBox` reuse `Point` methods.
5. **Balance behavior + formulas**: geometry math exists, but tests focus on struct behavior and ownership-friendly design.

## Topic map

1. **Point** — immutable transforms, symmetry, display.
2. **Rectangle** — validation, derived properties, containment.
3. **Circle** — validation, scaling, center-aware containment.
4. **Triangle** — validity checks and side-based classification.
5. **Color RGB** — value object behavior, parsing/formatting, transforms.
6. **BoundingBox** — normalization + composition over collections of points.
