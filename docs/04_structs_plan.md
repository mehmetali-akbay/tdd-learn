# 04_structs Plan (Rust Book Chapter 5)

This is a concrete, struct-first project plan aligned to:
- Chapter 5.1: Defining and instantiating structs
- Chapter 5.2: Example program using structs
- Chapter 5.3: Method syntax

The goal is to teach struct concepts before geometry-heavy modeling in `04_structs`.

## Project shape

- Folder: `04_structs/`
- Files:
  - `04_structs/Cargo.toml`
  - `04_structs/src/lib.rs`
  - `04_structs/tests/tests.rs`
  - `04_structs/README.md`

## Topic layout (6 topics)

1. Struct definition and instantiation (named-field structs)
2. Ownership with struct fields (`String` move/clone behavior)
3. Field init shorthand and struct update syntax (`..`)
4. Tuple structs and unit-like structs (newtypes + marker type)
5. Rectangle example from the book (`area`, `can_hold`, `square`)
6. Complex scenario: validated builder + multi-step state updates

## API plan (functions and methods)

### Topic 1: Basic named-field struct

```rust
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User;
pub fn default_guest() -> User;
```

Methods:

```rust
impl User {
    pub fn rename(&mut self, new_username: String);
    pub fn deactivate(&mut self);
    pub fn sign_in(&mut self);
    pub fn contact_label(&self) -> String; // "username <email>"
}
```

### Topic 2: Ownership in structs

```rust
pub fn consume_username(user: User) -> String;
pub fn duplicate_email(user: &User) -> String;
pub fn user_with_new_email(user: &User, email: String) -> User;
```

### Topic 3: Struct update syntax

```rust
pub fn clone_with_username(user: &User, username: String) -> User;
pub fn move_with_username(user: User, username: String) -> User; // uses ..user
```

### Topic 4: Tuple structs and unit-like structs

```rust
pub struct UserId(pub u64);
pub struct Millimeters(pub u32);
pub struct Meters(pub u32);
pub struct AlwaysEqual;

pub fn meters_to_mm(m: Meters) -> Millimeters;
pub fn add_mm(a: Millimeters, b: Millimeters) -> Millimeters;
pub fn user_id_to_string(id: UserId) -> String;
```

### Topic 5: Rectangle (book-aligned)

```rust
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn area_tuple(dim: (u32, u32)) -> u32;
pub fn area_rect(rect: &Rectangle) -> u32;
```

Methods:

```rust
impl Rectangle {
    pub fn area(&self) -> u32;
    pub fn width(&self) -> bool; // same style as Rust Book
    pub fn can_hold(&self, other: &Rectangle) -> bool;
    pub fn square(size: u32) -> Rectangle;
    pub fn scale(&mut self, factor: u32);
}
```

### Topic 6: Complex struct problem (validated builder + state)

```rust
pub struct Account {
    id: UserId,
    owner: String,
    email: String,
    active: bool,
    login_count: u64,
}

pub struct AccountBuilder {
    id: Option<UserId>,
    owner: Option<String>,
    email: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum AccountBuildError {
    MissingId,
    MissingOwner,
    MissingEmail,
    InvalidEmail,
}
```

Methods:

```rust
impl AccountBuilder {
    pub fn new() -> Self;
    pub fn id(mut self, id: UserId) -> Self;
    pub fn owner(mut self, owner: String) -> Self;
    pub fn email(mut self, email: String) -> Self;
    pub fn build(self) -> Result<Account, AccountBuildError>;
}

impl Account {
    pub fn id(&self) -> UserId;
    pub fn owner(&self) -> &str;
    pub fn email(&self) -> &str;
    pub fn is_active(&self) -> bool;
    pub fn login_count(&self) -> u64;
    pub fn deactivate(&mut self);
    pub fn reactivate(&mut self);
    pub fn record_login(&mut self);
    pub fn change_email(&mut self, email: String) -> Result<(), AccountBuildError>;
}
```

## Test plan

Target: 40-50 tests, split by topic.

1. Topic 1 tests:
- constructor helpers set expected fields
- `rename`, `deactivate`, `sign_in` mutate expected fields
- `contact_label` formatting

2. Topic 2 tests:
- consuming function moves `User`
- borrowing function clones data without moving
- copy-from-reference preserves original user

3. Topic 3 tests:
- clone path keeps original unchanged
- move-update path uses `..user` semantics
- changed vs inherited fields are correct

4. Topic 4 tests:
- newtype conversion math (`Meters -> Millimeters`)
- no accidental mixing of raw `u32` and newtypes in API
- unit-like struct can be instantiated

5. Topic 5 tests:
- rectangle area via tuple and struct paths
- `width()` true/false behavior
- `can_hold` true/false cases
- `square` associated function
- mutable method `scale` behavior

6. Topic 6 tests (complex):
- builder fails with precise typed errors
- valid builder output has correct defaults (`active`, `login_count`)
- state transition chain:
  - build account
  - record multiple logins
  - deactivate/reactivate
  - change email with validation
- ownership check through APIs returning `&str` vs owned `String`

## TDD implementation order

1. Topic 1 basic struct + first tests
2. Topic 2 ownership-focused functions
3. Topic 3 update syntax
4. Topic 4 tuple/unit structs
5. Topic 5 rectangle methods
6. Topic 6 builder/state challenge

## Integration with existing curriculum

- Keep `04_structs` as applied struct modeling.
- Place this as a pre-step:
  - either rename existing order (`04_structs`, `05_patterns`, `06_types`, ...)
  - or keep numbering stable and add as `04b_structs_book`/`41_structs_book`.

For minimal disruption, add now as `41_structs_book` first, then renumber later if needed.
