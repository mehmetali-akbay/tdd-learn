// ============================================
// Level 2, Project 1: Structs — Rust Book Chapter 5
// Learn: Struct definitions, ownership, update syntax, methods
// ============================================

/// Normalize user-provided text for tags/roles.
///
/// Expected behavior:
/// - trim surrounding whitespace
/// - lowercase for case-insensitive matching
/// - return None when the result is empty
fn normalize_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_ascii_lowercase())
    }
}

/// Validate a basic email format used by this learning project.
///
/// Keep this intentionally simple (not RFC-complete).
fn is_valid_email(email: &str) -> bool {
    let mut parts = email.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();

    if parts.next().is_some() {
        return false;
    }

    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

// ============================================
// Topic 1: Named-field structs
// ============================================

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
pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        display_name: username.clone(),
        username,
        email,
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
    pub fn rename(&mut self, new_username: String) {
        self.username = new_username;
    }

    /// Update the display name field.
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    /// Mark the user as inactive.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Mark the user as active again.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Increment the sign-in counter.
    pub fn sign_in(&mut self) {
        self.sign_in_count = self.sign_in_count.saturating_add(1);
    }

    /// Produce a human-friendly label like "Name <email>".
    pub fn contact_label(&self) -> String {
        format!("{} <{}>", self.display_name, self.email)
    }

    /// Add a role after normalization, ignoring duplicates/invalid input.
    pub fn grant_role(&mut self, role: String) {
        if let Some(normalized) = normalize_token(&role) {
            if !self.roles.iter().any(|r| r == &normalized) {
                self.roles.push(normalized);
                self.roles.sort();
            }
        }
    }

    /// Check if the user has a given role (case-insensitive).
    pub fn has_role(&self, role: &str) -> bool {
        match normalize_token(role) {
            Some(normalized) => self.roles.iter().any(|r| r == &normalized),
            None => false,
        }
    }
}

// ============================================
// Topic 2: Ownership with struct fields
// ============================================

/// Consume a User and return the owned username String.
pub fn consume_username(user: User) -> String {
    user.username
}

/// Borrow a User and clone the email out.
pub fn duplicate_email(user: &User) -> String {
    user.email.clone()
}

/// Return a cloned user with only email replaced.
pub fn user_with_new_email(user: &User, email: String) -> User {
    User {
        email,
        ..user.clone()
    }
}

// ============================================
// Topic 3: Struct update syntax
// ============================================

/// Build a new user by cloning and changing only username.
pub fn clone_with_username(user: &User, username: String) -> User {
    User {
        username,
        ..user.clone()
    }
}

/// Build a new user by moving source user with struct update syntax.
pub fn move_with_username(user: User, username: String) -> User {
    User { username, ..user }
}

// ============================================
// Topic 4: Tuple structs and unit-like structs
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Millimeters(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meters(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlwaysEqual;

/// Convert meters to millimeters.
pub fn meters_to_mm(m: Meters) -> Millimeters {
    Millimeters(m.0.saturating_mul(1000))
}

/// Add two millimeter values.
pub fn add_mm(a: Millimeters, b: Millimeters) -> Millimeters {
    Millimeters(a.0.saturating_add(b.0))
}

/// Render a UserId as a readable string.
pub fn user_id_to_string(id: UserId) -> String {
    format!("UserId({})", id.0)
}

// ============================================
// Topic 5: Rectangle example from the Rust Book
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

/// Compute rectangle area from a tuple.
pub fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0.saturating_mul(dim.1)
}

/// Compute rectangle area from a struct reference.
pub fn area_rect(rect: &Rectangle) -> u32 {
    rect.width.saturating_mul(rect.height)
}

impl Rectangle {
    /// Compute the area of this rectangle.
    pub fn area(&self) -> u32 {
        area_rect(self)
    }

    /// Return true when width is non-zero.
    pub fn width(&self) -> bool {
        self.width > 0
    }

    /// Return true when self is strictly larger than other in both dimensions.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// Associated constructor for a square.
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    /// Scale dimensions by a multiplier.
    pub fn scale(&mut self, factor: u32) {
        self.width = self.width.saturating_mul(factor);
        self.height = self.height.saturating_mul(factor);
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AccountBuilder {
    id: Option<UserId>,
    owner: Option<String>,
    email: Option<String>,
    tier: Option<AccountTier>,
    created_at_epoch: Option<u64>,
    tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountBuildError {
    MissingId,
    MissingOwner,
    MissingEmail,
    InvalidOwner,
    InvalidEmail,
    InvalidTag,
}

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
    pub fn id(mut self, id: UserId) -> Self {
        self.id = Some(id);
        self
    }

    /// Set account owner.
    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Set account email.
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// Set account tier.
    pub fn tier(mut self, tier: AccountTier) -> Self {
        self.tier = Some(tier);
        self
    }

    /// Set creation timestamp.
    pub fn created_at_epoch(mut self, created_at_epoch: u64) -> Self {
        self.created_at_epoch = Some(created_at_epoch);
        self
    }

    /// Add a tag candidate to the builder.
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Validate all fields and create the final Account.
    pub fn build(self) -> Result<Account, AccountBuildError> {
        let id = self.id.ok_or(AccountBuildError::MissingId)?;
        let owner = self.owner.ok_or(AccountBuildError::MissingOwner)?;
        let owner = owner.trim().to_string();
        if owner.is_empty() {
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
            login_count: 0,
            tier: self.tier.unwrap_or_default(),
            created_at_epoch: self.created_at_epoch.unwrap_or(0),
            last_login_ip: None,
            tags: normalized_tags,
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
    pub fn has_tag(&self, tag: &str) -> bool {
        match normalize_token(tag) {
            Some(normalized) => self.tags.iter().any(|t| t == &normalized),
            None => false,
        }
    }

    /// Deactivate the account.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Reactivate the account.
    pub fn reactivate(&mut self) {
        self.active = true;
    }

    /// Record a login attempt with IP.
    pub fn record_login(&mut self, ip: String) -> Result<(), AccountActionError> {
        if !self.active {
            return Err(AccountActionError::Inactive);
        }

        let Some(normalized_ip) = normalize_token(&ip) else {
            return Err(AccountActionError::InvalidIp);
        };

        self.login_count = self.login_count.saturating_add(1);
        self.last_login_ip = Some(normalized_ip);
        Ok(())
    }

    /// Update email if the new value is valid.
    pub fn change_email(&mut self, email: String) -> Result<(), AccountActionError> {
        if !is_valid_email(&email) {
            return Err(AccountActionError::InvalidEmail);
        }
        self.email = email;
        Ok(())
    }

    /// Add a normalized tag to the account.
    pub fn add_tag(&mut self, tag: String) -> Result<(), AccountActionError> {
        let Some(normalized) = normalize_token(&tag) else {
            return Err(AccountActionError::InvalidTag);
        };
        if !self.tags.iter().any(|t| t == &normalized) {
            self.tags.push(normalized);
            self.tags.sort();
        }
        Ok(())
    }

    /// Remove a tag; return whether removal happened.
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        let Some(normalized) = normalize_token(tag) else {
            return false;
        };
        if let Some(idx) = self.tags.iter().position(|t| t == &normalized) {
            self.tags.remove(idx);
            true
        } else {
            false
        }
    }

    /// Return a short human-friendly summary line.
    pub fn summary(&self) -> String {
        let state = if self.active { "active" } else { "inactive" };
        format!("#{} {} ({state})", self.id.0, self.owner)
    }
}

#[cfg(test)]
mod helper_tests {
    use super::{is_valid_email, normalize_token};

    #[test]
    fn normalize_token_trims_and_lowercases_ascii_input() {
        assert_eq!(normalize_token("  Admin  "), Some("admin".to_string()));
        assert_eq!(normalize_token("Read Only"), Some("read only".to_string()));
    }

    #[test]
    fn normalize_token_rejects_empty_or_whitespace_only_values() {
        assert_eq!(normalize_token(""), None);
        assert_eq!(normalize_token("   \t\n  "), None);
    }

    #[test]
    fn is_valid_email_accepts_simple_project_valid_cases() {
        assert!(is_valid_email("alice@example.com"));
        assert!(is_valid_email("bob.smith+ops@sub.example.com"));
    }

    #[test]
    fn is_valid_email_rejects_missing_parts_or_multiple_at_symbols() {
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("alice@"));
        assert!(!is_valid_email("alice@example"));
        assert!(!is_valid_email("alice@@example.com"));
    }

    #[test]
    fn is_valid_email_rejects_domain_starting_or_ending_with_dot() {
        assert!(!is_valid_email("alice@.example.com"));
        assert!(!is_valid_email("alice@example.com."));
    }
}
