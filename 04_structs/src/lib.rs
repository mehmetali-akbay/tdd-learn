// ============================================
// Level 2, Project 1: Structs - Rust Book Chapter 5
// Learn: Struct definitions, ownership, update syntax, methods
// ============================================

/// Normalize user-provided text for tags/roles.
///
/// Expected behavior:
/// - trim surrounding whitespace
/// - lowercase for case-insensitive matching
/// - return None when the result is empty
fn normalize_token(_raw: &str) -> Option<String> {
    if _raw.trim().is_empty() {
        return None;
    }
    Some(_raw.trim().to_ascii_lowercase())
}

/// Validate a basic email format used by this learning project.
///
/// Keep this intentionally simple (not RFC-complete).
fn is_valid_email(_email: &str) -> bool {
    let mut parts = _email.split("@");
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();

    if parts.next().is_some() {
        return false;
    }

    !local.is_empty()
        && !domain.is_empty()
        && domain.contains(".")
        && !domain.starts_with(".")
        && !domain.ends_with(".")
}

// ============================================
// Topic 1: Named-field structs
// ============================================

/// A user profile modeled as a named-field struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub display_name: String,
    pub roles: Vec<String>,
}

/// Build an active user with default counters and role list.
pub fn build_user(_email: String, _username: String) -> User {
    User {
        active: true,
        display_name: _username.clone(),
        username: _username,
        email: _email,
        sign_in_count: 1,
        roles: vec!["user".to_string()],
    }
}

/// Create a predefined guest user profile.
pub fn default_guest() -> User {
    User {
        active: true,
        username: "guest".to_string(),
        email: "guest@example.com".to_string(),
        sign_in_count: 0,
        display_name: "Guest User".to_string(),
        roles: vec!["guest".to_string()],
    }
}

impl User {
    /// Update the username field.
    pub fn rename(&mut self, _new_username: String) {
        self.username = _new_username
    }

    /// Update the display name field.
    pub fn set_display_name(&mut self, _display_name: String) {
        self.display_name = _display_name
    }

    /// Mark the user as inactive.
    pub fn deactivate(&mut self) {
        self.active = false
    }

    /// Mark the user as active again.
    pub fn activate(&mut self) {
        self.active = true
    }

    /// Increment the sign-in counter.
    pub fn sign_in(&mut self) {
        self.sign_in_count += 1
    }

    /// Produce a human-friendly label like "Name <email>".
    pub fn contact_label(&self) -> String {
        format!("{} <{}>", self.display_name, self.email)
    }

    /// Add a role after normalization, ignoring duplicates/invalid input.
    pub fn grant_role(&mut self, _role: String) {
        if let Some(role) = normalize_token(&_role)
            && !self.roles.contains(&role)
        {
            self.roles.insert(0, role)
        }
    }

    /// Check if the user has a given role (case-insensitive).
    pub fn has_role(&self, _role: &str) -> bool {
        if let Some(role) = normalize_token(_role) {
            self.roles.contains(&role)
        } else {
            false
        }
    }
}

// ============================================
// Topic 2: Ownership with struct fields
// ============================================

/// Consume a User and return the owned username String.
pub fn consume_username(_user: User) -> String {
    _user.username
}

/// Borrow a User and clone the email out.
pub fn duplicate_email(_user: &User) -> String {
    _user.email.clone()
}

/// Return a cloned user with only email replaced.
pub fn user_with_new_email(_user: &User, _email: String) -> User {
    User {
        email: _email,
        .._user.clone()
    }
}

// ============================================
// Topic 3: Struct update syntax
// ============================================

/// Build a new user by cloning and changing only username.
pub fn clone_with_username(_user: &User, _username: String) -> User {
    User {
        username: _username,
        .._user.clone()
    }
}

/// Build a new user by moving source user with struct update syntax.
pub fn move_with_username(user: User, username: String) -> User {
    User { username, ..user }
}

// ============================================
// Topic 4: Tuple structs and unit-like structs
// ============================================

/// Strongly-typed user identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

/// Length in millimeters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Millimeters(pub u32);

/// Length in meters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meters(pub u32);

/// Unit-like marker struct for demonstration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlwaysEqual;

/// Convert meters to millimeters.
pub fn meters_to_mm(_m: Meters) -> Millimeters {
    Millimeters(_m.0.saturating_mul(1000))
}

/// Add two millimeter values.
pub fn add_mm(_a: Millimeters, _b: Millimeters) -> Millimeters {
    Millimeters(_a.0.saturating_add(_b.0))
}

/// Render a UserId as a readable string.
pub fn user_id_to_string(_id: UserId) -> String {
    format!("{:?}", _id)
}

// ============================================
// Topic 5: Rectangle example from the Rust Book
// ============================================

/// Rectangle represented with named fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

/// Compute rectangle area from a tuple.
pub fn area_tuple(_dim: (u32, u32)) -> u32 {
    _dim.0.saturating_mul(_dim.1)
}

/// Compute rectangle area from a struct reference.
pub fn area_rect(_rect: &Rectangle) -> u32 {
    _rect.height.saturating_mul(_rect.width)
}

impl Rectangle {
    /// Compute the area of this rectangle.
    pub fn area(&self) -> u32 {
        self.height.saturating_mul(self.width)
    }

    /// Return true when width is non-zero.
    pub fn width(&self) -> bool {
        self.width > 0
    }

    /// Return true when self is strictly larger than other in both dimensions.
    pub fn can_hold(&self, _other: &Rectangle) -> bool {
        self.width > _other.width && self.height > _other.width
    }

    /// Associated constructor for a square.
    pub fn square(_size: u32) -> Rectangle {
        Rectangle {
            width: _size,
            height: _size,
        }
    }

    /// Scale dimensions by a multiplier.
    pub fn scale(&mut self, _factor: u32) {
        self.width = self.width.saturating_mul(_factor);
        self.height = self.height.saturating_mul(_factor);
    }

    /// Compute perimeter.
    pub fn perimeter(&self) -> u32 {
        self.width
            .saturating_mul(2)
            .saturating_add(self.height.saturating_mul(2))
    }

    /// Return true when width and height are equal.
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    // boolean fits = (w1 <= w2 && h1 <= h2) ||
    //                (w1 <= h2 && h1 <= w2);
    //

    // 3x4 6×2 Döndür: 4≤6 ✓, 3≤2 ✗ → Sığmaz

    /// Check containment with optional rotation support.
    pub fn fits_inside(&self, other: &Rectangle, allow_rotation: bool) -> bool {
        (self.width <= other.width && self.height <= other.height)
            || (allow_rotation && self.width <= other.height && self.height <= other.width)
    }

    /// Compute diagonal length.
    pub fn diagonal(&self) -> f64 {
        let w = self.width as f64;
        let h = self.height as f64;
        (w * w + h * h).sqrt()
    }
}

// ============================================
// Topic 6: Complex struct challenge (builder + state)
// ============================================

/// Account subscription tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountTier {
    Free,
    Pro,
    Enterprise,
}

impl Default for AccountTier {
    /// Default account tier for newly built accounts.
    fn default() -> Self {
        Self::Free
    }
}

/// Domain model for an account lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    id: UserId,
    owner: String,
    email: String,
    active: bool,
    login_count: u64,
    tier: AccountTier,
    created_at_epoch: u64,
    last_login_ip: Option<String>,
    tags: Vec<String>,
}

/// Builder used to validate and construct Account values.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AccountBuilder {
    id: Option<UserId>,
    owner: Option<String>,
    email: Option<String>,
    tier: Option<AccountTier>,
    created_at_epoch: Option<u64>,
    tags: Vec<String>,
}

/// Errors returned while building an Account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountBuildError {
    MissingId,
    MissingOwner,
    MissingEmail,
    InvalidOwner,
    InvalidEmail,
    InvalidTag,
}

/// Errors returned by Account state-changing actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountActionError {
    Inactive,
    InvalidEmail,
    InvalidTag,
    InvalidIp,
}

impl AccountBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set account id.
    pub fn id(mut self, _id: UserId) -> Self {
        self.id = Some(_id);
        self
    }

    /// Set account owner.
    pub fn owner(mut self, _owner: String) -> Self {
        self.owner = Some(_owner);
        self
    }

    /// Set account email.
    pub fn email(mut self, _email: String) -> Self {
        self.email = Some(_email);
        self
    }

    /// Set account tier.
    pub fn tier(mut self, _tier: AccountTier) -> Self {
        self.tier = Some(_tier);
        self
    }

    /// Set creation timestamp.
    pub fn created_at_epoch(mut self, _created_at_epoch: u64) -> Self {
        self.created_at_epoch = Some(_created_at_epoch);
        self
    }

    /// Add a tag candidate to the builder.
    pub fn add_tag(mut self, _tag: String) -> Self {
        self.tags.push(_tag);
        self
    }

    /// Validate all fields and create the final Account.
    pub fn build(self) -> Result<Account, AccountBuildError> {
        let id = self.id.ok_or(AccountBuildError::MissingId)?;
        let owner = self.owner.ok_or(AccountBuildError::MissingOwner)?;
        if owner.trim().is_empty() {
            return Err(AccountBuildError::InvalidOwner);
        }

        let email = self.email.ok_or(AccountBuildError::MissingEmail)?;
        if !is_valid_email(&email) {
            return Err(AccountBuildError::InvalidEmail);
        }

        let mut normalized_tags = Vec::new();
        for raw in self.tags {
            let Some(tag) = normalize_token(&raw) else {
                return Err(AccountBuildError::InvalidTag);
            };
            if !normalized_tags.contains(&tag) {
                normalized_tags.push(tag);
            }
        }
        normalized_tags.sort();

        Ok(Account {
            id,
            owner,
            email,
            active: true,
            created_at_epoch: self.created_at_epoch.unwrap_or(0),
            last_login_ip: None,
            login_count: 0,
            tags: normalized_tags,
            tier: self.tier.unwrap_or_default(),
        })
    }
}

impl Account {
    /// Get account id.
    pub fn id(&self) -> UserId {
        self.id
    }

    /// Get account owner.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Get account email.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Check whether account is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get successful login count.
    pub fn login_count(&self) -> u64 {
        self.login_count
    }

    /// Get current tier.
    pub fn tier(&self) -> AccountTier {
        self.tier
    }

    /// Get creation timestamp.
    pub fn created_at_epoch(&self) -> u64 {
        self.created_at_epoch
    }

    /// Get most recent login IP, if any.
    pub fn last_login_ip(&self) -> Option<&str> {
        self.last_login_ip.as_deref()
    }

    /// Borrow normalized account tags.
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Check whether a tag is present.
    pub fn has_tag(&self, _tag: &str) -> bool {
        self.tags.contains(&_tag.to_ascii_lowercase())
    }

    /// Deactivate the account.
    pub fn deactivate(&mut self) {
        self.active = false
    }

    /// Reactivate the account.
    pub fn reactivate(&mut self) {
        self.active = true
    }

    /// Record a login attempt with IP.
    pub fn record_login(&mut self, _ip: String) -> Result<(), AccountActionError> {
        if !_ip.trim().is_empty() && self.active {
            self.last_login_ip = normalize_token(&_ip);
            self.login_count += 1;
            Ok(())
        } else if !self.active {
            Err(AccountActionError::Inactive)
        } else {
            Err(AccountActionError::InvalidIp)
        }
    }

    /// Update email if the new value is valid.
    pub fn change_email(&mut self, _email: String) -> Result<(), AccountActionError> {
        if is_valid_email(&_email) {
            self.email = _email;
            Ok(())
        } else {
            Err(AccountActionError::InvalidEmail)
        }
    }

    /// Add a normalized tag to the account.
    pub fn add_tag(&mut self, _tag: String) -> Result<(), AccountActionError> {
        if let Some(role) = normalize_token(&_tag) {
            if !self.tags.contains(&role) {
                self.tags.push(role);
            }
            Ok(())
        } else {
            Err(AccountActionError::InvalidTag)
        }
    }

    /// Remove a tag; return whether removal happened.
    pub fn remove_tag(&mut self, _tag: &str) -> bool {
        if let Some(index) = self
            .tags
            .iter()
            .position(|e| *e == _tag.trim().to_ascii_lowercase())
        {
            self.tags.remove(index);
            return true;
        }
        false
    }

    /// Return a short human-friendly summary line.
    pub fn summary(&self) -> String {
        let state = if self.active { "active" } else { "inactive" };
        format!("#{} {} ({state})", self.id.0, self.owner)
    }
}
