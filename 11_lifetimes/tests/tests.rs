use lifetimes::*;

// ===== Topic 1: Lifetime Basics =====

#[test]
fn test_longest() {
    assert_eq!(longest("hello", "hi"), "hello");
    assert_eq!(longest("ab", "abc"), "abc");
    assert_eq!(longest("same", "size"), "same"); // same len, returns first
}

#[test]
fn test_first_word() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("single"), "single");
    assert_eq!(first_word(""), "");
}

#[test]
fn test_after_substring() {
    assert_eq!(after_substring("hello world", "hello "), Some("world"));
    assert_eq!(after_substring("foobar", "baz"), None);
    assert_eq!(after_substring("key=value", "="), Some("value"));
}

#[test]
fn test_common_prefix() {
    assert_eq!(common_prefix("hello", "help"), "hel");
    assert_eq!(common_prefix("abc", "xyz"), "");
    assert_eq!(common_prefix("same", "same"), "same");
}

#[test]
fn test_longest_in_slice() {
    assert_eq!(longest_in_slice(&["a", "bbb", "cc"]), Some("bbb"));
    assert_eq!(longest_in_slice(&[]), None);
}

// ===== Topic 2: Structs with Lifetimes =====

#[test]
fn test_excerpt_new() {
    let text = "Hello, world!";
    let exc = Excerpt::new(text, 0, 5).unwrap();
    assert_eq!(exc.text, "Hello");
    assert_eq!(exc.len(), 5);
    assert!(!exc.is_empty());
}

#[test]
fn test_excerpt_out_of_bounds() {
    let text = "short";
    assert!(Excerpt::new(text, 0, 100).is_none());
    assert!(Excerpt::new(text, 3, 2).is_none()); // start > end
}

#[test]
fn test_key_value_parse() {
    let input = "name = Alice";
    let kv = KeyValue::parse(input).unwrap();
    assert_eq!(kv.key, "name");
    assert_eq!(kv.value, "Alice");
}

#[test]
fn test_key_value_no_equals() {
    assert!(KeyValue::parse("no equals here").is_none());
}

// ===== Topic 3: Iterators with Lifetimes =====

#[test]
fn test_lines_iter() {
    assert_eq!(lines_iter("a\nb\nc"), vec!["a", "b", "c"]);
}

#[test]
fn test_long_words() {
    assert_eq!(
        long_words("I am a test sentence", 3),
        vec!["test", "sentence"]
    );
    assert_eq!(long_words("hi", 5), Vec::<&str>::new());
}

#[test]
fn test_find_all_matches() {
    assert_eq!(find_all_matches("abcabc", "abc").len(), 2);
    assert_eq!(find_all_matches("hello", "xyz"), Vec::<&str>::new());
    assert_eq!(find_all_matches("aaa", "aa").len(), 2); // overlapping
}

#[test]
fn test_sentences() {
    let text = "Hello world. How are you. Fine.";
    let sents: Vec<&str> = Sentences::new(text).collect();
    assert_eq!(sents, vec!["Hello world", "How are you", "Fine"]);
}

#[test]
fn test_sentences_empty() {
    let sents: Vec<&str> = Sentences::new("").collect();
    assert!(sents.is_empty());
}

// ===== Topic 4: Multiple Lifetimes =====

#[test]
fn test_filter_containing() {
    let items = vec!["apple", "banana", "apricot", "cherry"];
    assert_eq!(filter_containing(&items, "ap"), vec!["apple", "apricot"]);
}

#[test]
fn test_highlight() {
    let h = Highlight::new("important", "b");
    assert_eq!(h.render(), "<b>important</b>");
}

#[test]
fn test_merge_sorted() {
    let a = vec!["apple", "cherry"];
    let b = vec!["banana", "date"];
    assert_eq!(
        merge_sorted(&a, &b),
        vec!["apple", "banana", "cherry", "date"]
    );
}

#[test]
fn test_merge_sorted_empty() {
    let a: Vec<&str> = vec![];
    let b = vec!["x"];
    assert_eq!(merge_sorted(&a, &b), vec!["x"]);
}

// ===== Topic 5: Static & Elision =====

#[test]
fn test_greeting() {
    assert_eq!(greeting(true), "Good day");
    assert_eq!(greeting(false), "Hey");
}

#[test]
fn test_status_message() {
    assert_eq!(status_message(200), "OK");
    assert_eq!(status_message(404), "Not Found");
    assert_eq!(status_message(999), "Unknown");
}

#[test]
fn test_trim_prefix() {
    assert_eq!(trim_prefix("---hello"), "hello");
    assert_eq!(trim_prefix("  ##title"), "title");
    assert_eq!(trim_prefix("already"), "already");
}

#[test]
fn test_config_get_value() {
    let config = Config::new("host=localhost\nport=8080");
    assert_eq!(config.get_value("host"), Some("localhost"));
    assert_eq!(config.get_value("port"), Some("8080"));
    assert_eq!(config.get_value("missing"), None);
}

// ===== Topic 6: Lifetime Bounds =====

#[test]
fn test_find_first() {
    let nums = vec![1, 5, 3, 8, 2];
    assert_eq!(find_first(&nums, |&n| n > 4), Some(&5));
    assert_eq!(find_first(&nums, |&n| n > 10), None);
}

#[test]
fn test_cached_lookup() {
    let data = [("x", 10), ("y", 20), ("z", 30)];
    let cache = CachedLookup::new(&data);
    assert_eq!(cache.lookup("x"), Some(10));
    assert_eq!(cache.lookup("w"), None);
}

#[test]
fn test_cached_keys() {
    let data = [("a", 1), ("b", 2)];
    let cache = CachedLookup::new(&data);
    assert_eq!(cache.keys(), vec!["a", "b"]);
}

#[test]
fn test_cached_values_above() {
    let data = [("a", 10), ("b", 5), ("c", 20)];
    let cache = CachedLookup::new(&data);
    let above = cache.values_above(8);
    assert_eq!(above.len(), 2);
    assert!(above.contains(&("a", 10)));
    assert!(above.contains(&("c", 20)));
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_longer() {
    assert_eq!(longer("hello", "hi"), "hello");
    assert_eq!(longer("a", "ab"), "ab");
}

#[test]
fn test_find_with_prefix() {
    let words = vec!["apple", "banana", "avocado"];
    assert_eq!(find_with_prefix(&words, "av"), Some("avocado"));
    assert_eq!(find_with_prefix(&words, "ch"), None);
}

#[test]
fn test_find_containing() {
    let items = vec!["hello", "world", "help"];
    assert_eq!(find_containing(&items, "hel"), vec!["hello", "help"]);
}

#[test]
fn test_first_and_last_word() {
    assert_eq!(first_and_last_word("hello world foo"), Some(("hello", "foo")));
    assert_eq!(first_and_last_word("single"), Some(("single", "single")));
    assert_eq!(first_and_last_word(""), None);
}
