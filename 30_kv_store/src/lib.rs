// ============================================
// Level 6, Project 3: Key-Value Store
// Learn: Full project — storage engine, namespaces, TTL, CLI parsing
// ============================================

use std::collections::HashMap;
use std::time::{Duration, Instant};

// ============================================
// Topic 1: Basic KV Store — CRUD
// Learn: HashMap-backed store, Option returns
// ============================================

/// A value entry with optional expiration
#[derive(Debug, Clone)]
struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

impl Entry {
    fn new(value: String) -> Self {
        Entry {
            value,
            expires_at: None,
        }
    }

    fn with_ttl(value: String, ttl: Duration) -> Self {
        Entry {
            value,
            expires_at: Some(Instant::now() + ttl),
        }
    }

    fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| Instant::now() > exp)
            .unwrap_or(false)
    }
}

/// A key-value store
#[derive(Debug)]
pub struct KvStore {
    data: HashMap<String, Entry>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Set a key-value pair
    pub fn set(&mut self, key: &str, value: &str) {
        self.data
            .insert(key.to_string(), Entry::new(value.to_string()));
    }

    /// Set a key-value pair with a TTL
    pub fn set_with_ttl(&mut self, key: &str, value: &str, ttl: Duration) {
        self.data
            .insert(key.to_string(), Entry::with_ttl(value.to_string(), ttl));
    }

    /// Get a value by key (returns None if expired)
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(entry.value.as_str())
            }
        })
    }

    /// Delete a key, returning whether it existed
    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    /// Check if a key exists (and is not expired)
    pub fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Number of entries (including possibly expired)
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Remove expired entries, return count removed
    pub fn cleanup_expired(&mut self) -> usize {
        let before = self.data.len();
        self.data.retain(|_, entry| !entry.is_expired());
        before - self.data.len()
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 2: Keys & Iteration
// Learn: Listing, searching, pattern matching on keys
// ============================================

impl KvStore {
    /// Return all non-expired keys
    pub fn keys(&self) -> Vec<&str> {
        self.data
            .iter()
            .filter(|(_, entry)| !entry.is_expired())
            .map(|(k, _)| k.as_str())
            .collect()
    }

    /// Return keys matching a glob-like prefix
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<&str> {
        self.data
            .iter()
            .filter(|(k, entry)| k.starts_with(prefix) && !entry.is_expired())
            .map(|(k, _)| k.as_str())
            .collect()
    }

    /// Return all key-value pairs as tuples
    pub fn entries(&self) -> Vec<(&str, &str)> {
        self.data
            .iter()
            .filter(|(_, entry)| !entry.is_expired())
            .map(|(k, entry)| (k.as_str(), entry.value.as_str()))
            .collect()
    }

    /// Count of non-expired entries
    pub fn active_count(&self) -> usize {
        self.data.iter().filter(|(_, e)| !e.is_expired()).count()
    }
}

// ============================================
// Topic 3: Namespaces — Prefixed Keys
// Learn: Key organization, scoped operations
// ============================================

/// A namespaced view onto a KvStore
pub struct Namespace<'a> {
    store: &'a mut KvStore,
    prefix: String,
}

impl<'a> Namespace<'a> {
    pub fn new(store: &'a mut KvStore, namespace: &str) -> Self {
        Namespace {
            store,
            prefix: format!("{}:", namespace),
        }
    }

    fn full_key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store.set(&self.full_key(key), value);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.store.get(&self.full_key(key))
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.store.delete(&self.full_key(key))
    }

    pub fn keys(&self) -> Vec<String> {
        self.store
            .keys_with_prefix(&self.prefix)
            .into_iter()
            .map(|k| k[self.prefix.len()..].to_string())
            .collect()
    }

    /// Delete all keys in this namespace
    pub fn clear(&mut self) -> usize {
        let to_remove: Vec<String> = self
            .store
            .keys_with_prefix(&self.prefix)
            .into_iter()
            .map(|k| k.to_string())
            .collect();
        let count = to_remove.len();
        for key in to_remove {
            self.store.delete(&key);
        }
        count
    }
}

// ============================================
// Topic 4: Bulk Operations
// Learn: Multi-set, multi-get, import/export
// ============================================

impl KvStore {
    /// Set multiple key-value pairs at once
    pub fn mset(&mut self, pairs: &[(&str, &str)]) {
        for (k, v) in pairs {
            self.set(k, v);
        }
    }

    /// Get multiple values at once
    pub fn mget(&self, keys: &[&str]) -> Vec<Option<&str>> {
        keys.iter().map(|k| self.get(k)).collect()
    }

    /// Increment a numeric value, returning the new value
    pub fn incr(&mut self, key: &str) -> Result<i64, String> {
        let current = self
            .get(key)
            .unwrap_or("0")
            .parse::<i64>()
            .map_err(|_| "value is not an integer".to_string())?;
        let new_val = current + 1;
        self.set(key, &new_val.to_string());
        Ok(new_val)
    }

    /// Append to a value
    pub fn append(&mut self, key: &str, suffix: &str) -> usize {
        let new_value = match self.get(key) {
            Some(existing) => format!("{}{}", existing, suffix),
            None => suffix.to_string(),
        };
        let len = new_value.len();
        self.set(key, &new_value);
        len
    }
}

// ============================================
// Topic 5: Serialization — Export/Import
// Learn: Custom text format, round-trip persistence
// ============================================

impl KvStore {
    /// Serialize all non-expired entries to a string
    pub fn export(&self) -> String {
        let mut lines: Vec<String> = self
            .data
            .iter()
            .filter(|(_, entry)| !entry.is_expired())
            .map(|(k, entry)| format!("{}={}", k, entry.value))
            .collect();
        lines.sort(); // deterministic output
        lines.join("\n")
    }

    /// Import entries from a serialized string
    pub fn import(&mut self, data: &str) -> usize {
        let mut count = 0;
        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(eq_pos) = line.find('=') {
                let key = &line[..eq_pos];
                let value = &line[eq_pos + 1..];
                self.set(key, value);
                count += 1;
            }
        }
        count
    }
}

// ============================================
// Topic 6: Command Parser
// Learn: Parsing text commands into structured operations
// ============================================

#[derive(Debug, PartialEq)]
pub enum Command {
    Set(String, String),
    Get(String),
    Delete(String),
    Exists(String),
    Keys,
    Clear,
    Incr(String),
    Append(String, String),
    MSet(Vec<(String, String)>),
    MGet(Vec<String>),
    Export,
    Unknown(String),
}

/// Parse a command string into a Command
pub fn parse_command(input: &str) -> Command {
    let input = input.trim();
    let parts: Vec<&str> = input.splitn(3, ' ').collect();

    match parts[0].to_uppercase().as_str() {
        "SET" if parts.len() >= 3 => Command::Set(parts[1].to_string(), parts[2].to_string()),
        "GET" if parts.len() >= 2 => Command::Get(parts[1].to_string()),
        "DEL" | "DELETE" if parts.len() >= 2 => Command::Delete(parts[1].to_string()),
        "EXISTS" if parts.len() >= 2 => Command::Exists(parts[1].to_string()),
        "KEYS" => Command::Keys,
        "CLEAR" => Command::Clear,
        "INCR" if parts.len() >= 2 => Command::Incr(parts[1].to_string()),
        "APPEND" if parts.len() >= 3 => Command::Append(parts[1].to_string(), parts[2].to_string()),
        "EXPORT" => Command::Export,
        "MSET" if parts.len() >= 2 => {
            let rest = &input[parts[0].len()..].trim();
            let pairs: Vec<(String, String)> = rest
                .split(' ')
                .collect::<Vec<_>>()
                .chunks(2)
                .filter_map(|chunk| {
                    if chunk.len() == 2 {
                        Some((chunk[0].to_string(), chunk[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            Command::MSet(pairs)
        }
        "MGET" if parts.len() >= 2 => {
            let rest = &input[parts[0].len()..].trim();
            let keys: Vec<String> = rest.split(' ').map(|s| s.to_string()).collect();
            Command::MGet(keys)
        }
        _ => Command::Unknown(input.to_string()),
    }
}

/// Execute a command against a store, return a response string
pub fn execute_command(store: &mut KvStore, cmd: &Command) -> String {
    match cmd {
        Command::Set(k, v) => {
            store.set(k, v);
            "OK".to_string()
        }
        Command::Get(k) => match store.get(k) {
            Some(v) => v.to_string(),
            None => "(nil)".to_string(),
        },
        Command::Delete(k) => {
            if store.delete(k) {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        Command::Exists(k) => {
            if store.exists(k) {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        Command::Keys => {
            let mut keys = store.keys();
            keys.sort();
            keys.join("\n")
        }
        Command::Clear => {
            store.clear();
            "OK".to_string()
        }
        Command::Incr(k) => match store.incr(k) {
            Ok(v) => v.to_string(),
            Err(e) => format!("ERR: {}", e),
        },
        Command::Append(k, v) => {
            let len = store.append(k, v);
            len.to_string()
        }
        Command::MSet(pairs) => {
            let refs: Vec<(&str, &str)> = pairs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            store.mset(&refs);
            "OK".to_string()
        }
        Command::MGet(keys) => {
            let refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
            let vals = store.mget(&refs);
            vals.iter()
                .map(|v| v.unwrap_or("(nil)"))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Command::Export => store.export(),
        Command::Unknown(s) => format!("ERR: unknown command '{}'", s),
    }
}
