use kv_store::*;
use std::time::Duration;

// ===== Topic 1: Basic CRUD =====

#[test]
fn test_set_get() {
    let mut store = KvStore::new();
    store.set("name", "Alice");
    assert_eq!(store.get("name"), Some("Alice"));
    assert_eq!(store.get("missing"), None);
}

#[test]
fn test_set_overwrite() {
    let mut store = KvStore::new();
    store.set("key", "v1");
    store.set("key", "v2");
    assert_eq!(store.get("key"), Some("v2"));
}

#[test]
fn test_delete() {
    let mut store = KvStore::new();
    store.set("key", "value");
    assert!(store.delete("key"));
    assert!(!store.delete("key")); // already gone
    assert!(!store.exists("key"));
}

#[test]
fn test_exists() {
    let mut store = KvStore::new();
    store.set("a", "1");
    assert!(store.exists("a"));
    assert!(!store.exists("b"));
}

#[test]
fn test_clear() {
    let mut store = KvStore::new();
    store.set("a", "1");
    store.set("b", "2");
    store.clear();
    assert!(store.is_empty());
}

#[test]
fn test_ttl_not_expired() {
    let mut store = KvStore::new();
    store.set_with_ttl("key", "value", Duration::from_secs(60));
    assert_eq!(store.get("key"), Some("value"));
}

// ===== Topic 2: Keys & Iteration =====

#[test]
fn test_keys() {
    let mut store = KvStore::new();
    store.set("a", "1");
    store.set("b", "2");
    let mut keys = store.keys();
    keys.sort();
    assert_eq!(keys, vec!["a", "b"]);
}

#[test]
fn test_keys_with_prefix() {
    let mut store = KvStore::new();
    store.set("user:1", "Alice");
    store.set("user:2", "Bob");
    store.set("post:1", "Hello");
    let mut user_keys = store.keys_with_prefix("user:");
    user_keys.sort();
    assert_eq!(user_keys, vec!["user:1", "user:2"]);
}

#[test]
fn test_entries() {
    let mut store = KvStore::new();
    store.set("x", "10");
    let entries = store.entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0], ("x", "10"));
}

#[test]
fn test_active_count() {
    let mut store = KvStore::new();
    store.set("a", "1");
    store.set("b", "2");
    assert_eq!(store.active_count(), 2);
}

// ===== Topic 3: Namespaces =====

#[test]
fn test_namespace_set_get() {
    let mut store = KvStore::new();
    {
        let mut ns = Namespace::new(&mut store, "users");
        ns.set("1", "Alice");
        ns.set("2", "Bob");
        assert_eq!(ns.get("1"), Some("Alice"));
        assert_eq!(ns.get("missing"), None);
    }
    // Verify it's stored with prefix in the underlying store
    assert_eq!(store.get("users:1"), Some("Alice"));
}

#[test]
fn test_namespace_keys() {
    let mut store = KvStore::new();
    store.set("other", "value");
    {
        let mut ns = Namespace::new(&mut store, "app");
        ns.set("name", "MyApp");
        ns.set("version", "1.0");
        let mut keys = ns.keys();
        keys.sort();
        assert_eq!(keys, vec!["name", "version"]);
    }
}

#[test]
fn test_namespace_clear() {
    let mut store = KvStore::new();
    store.set("keep", "me");
    {
        let mut ns = Namespace::new(&mut store, "temp");
        ns.set("a", "1");
        ns.set("b", "2");
        let removed = ns.clear();
        assert_eq!(removed, 2);
    }
    assert!(store.exists("keep"));
    assert_eq!(store.active_count(), 1);
}

// ===== Topic 4: Bulk Operations =====

#[test]
fn test_mset_mget() {
    let mut store = KvStore::new();
    store.mset(&[("a", "1"), ("b", "2"), ("c", "3")]);
    let values = store.mget(&["a", "c", "missing"]);
    assert_eq!(values, vec![Some("1"), Some("3"), None]);
}

#[test]
fn test_incr() {
    let mut store = KvStore::new();
    assert_eq!(store.incr("counter"), Ok(1)); // starts at 0
    assert_eq!(store.incr("counter"), Ok(2));
    assert_eq!(store.incr("counter"), Ok(3));
}

#[test]
fn test_incr_invalid() {
    let mut store = KvStore::new();
    store.set("key", "not_a_number");
    assert!(store.incr("key").is_err());
}

#[test]
fn test_append() {
    let mut store = KvStore::new();
    assert_eq!(store.append("msg", "hello"), 5);
    assert_eq!(store.append("msg", " world"), 11);
    assert_eq!(store.get("msg"), Some("hello world"));
}

// ===== Topic 5: Export/Import =====

#[test]
fn test_export() {
    let mut store = KvStore::new();
    store.set("b", "2");
    store.set("a", "1");
    let exported = store.export();
    assert_eq!(exported, "a=1\nb=2"); // sorted
}

#[test]
fn test_import() {
    let mut store = KvStore::new();
    let count = store.import("x=10\ny=20\n# comment\n\nz=30");
    assert_eq!(count, 3);
    assert_eq!(store.get("x"), Some("10"));
    assert_eq!(store.get("y"), Some("20"));
    assert_eq!(store.get("z"), Some("30"));
}

#[test]
fn test_roundtrip() {
    let mut store1 = KvStore::new();
    store1.set("name", "test");
    store1.set("count", "42");
    let exported = store1.export();

    let mut store2 = KvStore::new();
    store2.import(&exported);
    assert_eq!(store2.get("name"), Some("test"));
    assert_eq!(store2.get("count"), Some("42"));
}

// ===== Topic 6: Command Parser =====

#[test]
fn test_parse_set() {
    assert_eq!(
        parse_command("SET name Alice"),
        Command::Set("name".to_string(), "Alice".to_string())
    );
}

#[test]
fn test_parse_get() {
    assert_eq!(parse_command("GET name"), Command::Get("name".to_string()));
}

#[test]
fn test_parse_del() {
    assert_eq!(parse_command("DEL key"), Command::Delete("key".to_string()));
}

#[test]
fn test_parse_unknown() {
    assert!(matches!(parse_command("FOOBAR"), Command::Unknown(_)));
}

#[test]
fn test_execute_commands() {
    let mut store = KvStore::new();
    assert_eq!(
        execute_command(&mut store, &Command::Set("x".into(), "42".into())),
        "OK"
    );
    assert_eq!(execute_command(&mut store, &Command::Get("x".into())), "42");
    assert_eq!(
        execute_command(&mut store, &Command::Get("missing".into())),
        "(nil)"
    );
    assert_eq!(
        execute_command(&mut store, &Command::Exists("x".into())),
        "1"
    );
    assert_eq!(
        execute_command(&mut store, &Command::Delete("x".into())),
        "1"
    );
    assert_eq!(
        execute_command(&mut store, &Command::Delete("x".into())),
        "0"
    );
}

#[test]
fn test_execute_incr() {
    let mut store = KvStore::new();
    assert_eq!(execute_command(&mut store, &Command::Incr("n".into())), "1");
    assert_eq!(execute_command(&mut store, &Command::Incr("n".into())), "2");
}
