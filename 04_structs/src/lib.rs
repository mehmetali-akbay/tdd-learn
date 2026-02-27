// ============================================
// Level 2, Project 1: Structs — Rust Book Chapter 5
// Learn: Struct definitions, ownership, update syntax, methods
// ============================================

fn normalize_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_ascii_lowercase())
    }
}

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
    pub fn rename(&mut self, new_username: String) {
        self.username = new_username;
    }

    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn sign_in(&mut self) {
        self.sign_in_count = self.sign_in_count.saturating_add(1);
    }

    pub fn contact_label(&self) -> String {
        format!("{} <{}>", self.display_name, self.email)
    }

    pub fn grant_role(&mut self, role: String) {
        if let Some(normalized) = normalize_token(&role) {
            if !self.roles.iter().any(|r| r == &normalized) {
                self.roles.push(normalized);
                self.roles.sort();
            }
        }
    }

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

pub fn consume_username(user: User) -> String {
    user.username
}

pub fn duplicate_email(user: &User) -> String {
    user.email.clone()
}

pub fn user_with_new_email(user: &User, email: String) -> User {
    User {
        email,
        ..user.clone()
    }
}

// ============================================
// Topic 3: Struct update syntax
// ============================================

pub fn clone_with_username(user: &User, username: String) -> User {
    User {
        username,
        ..user.clone()
    }
}

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

pub fn meters_to_mm(m: Meters) -> Millimeters {
    Millimeters(m.0.saturating_mul(1000))
}

pub fn add_mm(a: Millimeters, b: Millimeters) -> Millimeters {
    Millimeters(a.0.saturating_add(b.0))
}

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

pub fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0.saturating_mul(dim.1)
}

pub fn area_rect(rect: &Rectangle) -> u32 {
    rect.width.saturating_mul(rect.height)
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        area_rect(self)
    }

    pub fn width(&self) -> bool {
        self.width > 0
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    pub fn scale(&mut self, factor: u32) {
        self.width = self.width.saturating_mul(factor);
        self.height = self.height.saturating_mul(factor);
    }

    pub fn perimeter(&self) -> u32 {
        self.width
            .saturating_mul(2)
            .saturating_add(self.height.saturating_mul(2))
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    pub fn fits_inside(&self, other: &Rectangle, allow_rotation: bool) -> bool {
        (self.width <= other.width && self.height <= other.height)
            || (allow_rotation && self.width <= other.height && self.height <= other.width)
    }

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: UserId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn tier(mut self, tier: AccountTier) -> Self {
        self.tier = Some(tier);
        self
    }

    pub fn created_at_epoch(mut self, created_at_epoch: u64) -> Self {
        self.created_at_epoch = Some(created_at_epoch);
        self
    }

    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

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
    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn login_count(&self) -> u64 {
        self.login_count
    }

    pub fn tier(&self) -> AccountTier {
        self.tier
    }

    pub fn created_at_epoch(&self) -> u64 {
        self.created_at_epoch
    }

    pub fn last_login_ip(&self) -> Option<&str> {
        self.last_login_ip.as_deref()
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        match normalize_token(tag) {
            Some(normalized) => self.tags.iter().any(|t| t == &normalized),
            None => false,
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn reactivate(&mut self) {
        self.active = true;
    }

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

    pub fn change_email(&mut self, email: String) -> Result<(), AccountActionError> {
        if !is_valid_email(&email) {
            return Err(AccountActionError::InvalidEmail);
        }
        self.email = email;
        Ok(())
    }

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

    pub fn summary(&self) -> String {
        let state = if self.active { "active" } else { "inactive" };
        format!("#{} {} ({state})", self.id.0, self.owner)
    }
}
