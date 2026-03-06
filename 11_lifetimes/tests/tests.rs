use lifetimes::*;

// ===== Topic 1: Lifetime Annotations on Functions =====

#[test]
fn test_longest_different() {
    assert_eq!(longest("hello", "hi"), "hello");
    assert_eq!(longest("ab", "abc"), "abc");
}

#[test]
fn test_longest_equal() {
    assert_eq!(longest("same", "size"), "same"); // equal len → returns first
}

#[test]
fn test_shortest() {
    assert_eq!(shortest("hello", "hi"), "hi");
    assert_eq!(shortest("ab", "abc"), "ab");
    assert_eq!(shortest("same", "size"), "same"); // equal → returns first
}

#[test]
fn test_first_word() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("single"), "single");
    assert_eq!(first_word(""), "");
}

#[test]
fn test_first_and_last_word() {
    assert_eq!(first_and_last_word("hello world foo"), Some(("hello", "foo")));
    assert_eq!(first_and_last_word("single"), Some(("single", "single")));
    assert_eq!(first_and_last_word(""), None);
}

#[test]
fn test_after_substring() {
    assert_eq!(after_substring("hello world", "hello "), Some("world"));
    assert_eq!(after_substring("key=value", "="), Some("value"));
    assert_eq!(after_substring("foobar", "baz"), None);
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

#[test]
fn test_strip_matching() {
    assert_eq!(strip_matching("<hello>", "<", ">"), Some("hello"));
    assert_eq!(strip_matching("[test]", "[", "]"), Some("test"));
    assert_eq!(strip_matching("hello>", "<", ">"), None);
    assert_eq!(strip_matching("<hello", "<", ">"), None);
}

#[test]
fn test_trim_to_boundary() {
    assert_eq!(trim_to_boundary("hello world foo", 20), "hello world foo");
    assert_eq!(trim_to_boundary("hello world foo", 11), "hello world");
    assert_eq!(trim_to_boundary("hello world foo", 7), "hello");
    assert_eq!(trim_to_boundary("abcdefg", 5), "abcde");
}

// ===== Topic 2: Structs Holding References =====

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
    assert!(Excerpt::new("short", 0, 100).is_none());
    assert!(Excerpt::new("short", 3, 2).is_none());
}

#[test]
fn test_excerpt_contains() {
    let text = "Hello, world!";
    let exc = Excerpt::new(text, 0, 12).unwrap();
    assert!(exc.contains("world"));
    assert!(!exc.contains("xyz"));
}

#[test]
fn test_excerpt_display() {
    let text = "Hello, world!";
    let exc = Excerpt::new(text, 7, 12).unwrap();
    assert_eq!(exc.to_string(), "[7..12]: world");
}

#[test]
fn test_excerpt_empty() {
    let text = "Hello";
    let exc = Excerpt::new(text, 3, 3).unwrap();
    assert!(exc.is_empty());
    assert_eq!(exc.len(), 0);
}

#[test]
fn test_key_value_parse() {
    let kv = KeyValue::parse("name = Alice").unwrap();
    assert_eq!(kv.key, "name");
    assert_eq!(kv.value, "Alice");
}

#[test]
fn test_key_value_no_equals() {
    assert!(KeyValue::parse("no equals here").is_none());
}

#[test]
fn test_key_value_display() {
    let kv = KeyValue::parse("host = localhost").unwrap();
    assert_eq!(kv.to_string(), "host=localhost");
}

#[test]
fn test_split_pair_new() {
    let sp = SplitPair::new("hello:world", ':').unwrap();
    assert_eq!(sp.left, "hello");
    assert_eq!(sp.right, "world");
    assert!(SplitPair::new("nocolon", ':').is_none());
}

#[test]
fn test_split_pair_rejoin() {
    let sp = SplitPair::new("key=value", '=').unwrap();
    assert_eq!(sp.rejoin(), "key=value");
}

#[test]
fn test_split_pair_swap() {
    let sp = SplitPair::new("left-right", '-').unwrap();
    let swapped = sp.swap();
    assert_eq!(swapped.left, "right");
    assert_eq!(swapped.right, "left");
}

#[test]
fn test_split_pair_display() {
    let sp = SplitPair::new("a:b", ':').unwrap();
    assert_eq!(sp.to_string(), "a ':' b");
}

// ===== Topic 3: Custom Iterators with Lifetimes =====

#[test]
fn test_lines_iter() {
    assert_eq!(lines_iter("a\nb\nc"), vec!["a", "b", "c"]);
    assert_eq!(lines_iter("single"), vec!["single"]);
}

#[test]
fn test_long_words() {
    assert_eq!(
        long_words("I am a test sentence", 3),
        vec!["test", "sentence"]
    );
}

#[test]
fn test_long_words_none() {
    assert_eq!(long_words("hi", 5), Vec::<&str>::new());
}

#[test]
fn test_find_all_matches() {
    assert_eq!(find_all_matches("abcabc", "abc").len(), 2);
    assert_eq!(find_all_matches("aaa", "aa").len(), 2); // overlapping
}

#[test]
fn test_find_all_matches_empty() {
    assert_eq!(find_all_matches("hello", "xyz"), Vec::<&str>::new());
    assert_eq!(find_all_matches("hello", ""), Vec::<&str>::new());
}

#[test]
fn test_take_while_alpha() {
    assert_eq!(take_while_alpha("hello123"), "hello");
    assert_eq!(take_while_alpha("123abc"), "");
    assert_eq!(take_while_alpha("allalpha"), "allalpha");
}

#[test]
fn test_sentences() {
    let sents: Vec<&str> = Sentences::new("Hello world. How are you. Fine.").collect();
    assert_eq!(sents, vec!["Hello world", "How are you", "Fine"]);
}

#[test]
fn test_sentences_empty() {
    let sents: Vec<&str> = Sentences::new("").collect();
    assert!(sents.is_empty());
}

#[test]
fn test_csv_fields() {
    let fields: Vec<&str> = CsvFields::new("a, b, c").collect();
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn test_csv_fields_single() {
    let fields: Vec<&str> = CsvFields::new("only").collect();
    assert_eq!(fields, vec!["only"]);
}

#[test]
fn test_csv_fields_empty() {
    let fields: Vec<&str> = CsvFields::new("").collect();
    assert!(fields.is_empty());
}

#[test]
fn test_windows() {
    let wins: Vec<&str> = Windows::new("hello", 3).collect();
    assert_eq!(wins, vec!["hel", "ell", "llo"]);
}

#[test]
fn test_windows_larger_than_text() {
    let wins: Vec<&str> = Windows::new("hi", 5).collect();
    assert!(wins.is_empty());
}

#[test]
fn test_windows_width_one() {
    let wins: Vec<&str> = Windows::new("abc", 1).collect();
    assert_eq!(wins, vec!["a", "b", "c"]);
}

// ===== Topic 4: Multiple Lifetimes =====

#[test]
fn test_filter_containing() {
    let items = vec!["apple", "banana", "apricot", "cherry"];
    assert_eq!(filter_containing(&items, "ap"), vec!["apple", "apricot"]);
}

#[test]
fn test_filter_containing_none() {
    let items = vec!["hello", "world"];
    assert!(filter_containing(&items, "xyz").is_empty());
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

#[test]
fn test_interleave() {
    let a = vec!["a", "b", "c"];
    let b = vec!["1", "2", "3"];
    assert_eq!(interleave(&a, &b), vec!["a", "1", "b", "2", "c", "3"]);
}

#[test]
fn test_interleave_unequal() {
    let a = vec!["a", "b"];
    let b = vec!["1"];
    assert_eq!(interleave(&a, &b), vec!["a", "1", "b"]);
}

#[test]
fn test_interleave_empty() {
    let a: Vec<&str> = vec![];
    let b: Vec<&str> = vec![];
    assert!(interleave(&a, &b).is_empty());
}

#[test]
fn test_select_values() {
    let pairs = vec![("color", "red"), ("size", "big"), ("color", "blue")];
    assert_eq!(select_values(&pairs, "color"), vec!["red", "blue"]);
}

#[test]
fn test_select_values_missing() {
    let pairs = vec![("a", "1")];
    assert!(select_values(&pairs, "z").is_empty());
}

#[test]
fn test_highlight_render() {
    let h = Highlight::new("important", "b");
    assert_eq!(h.render(), "<b>important</b>");
}

#[test]
fn test_highlight_with_text() {
    let h = Highlight::new("old", "em");
    let h2 = h.with_text("new");
    assert_eq!(h2.render(), "<em>new</em>");
    assert_eq!(h.tag, "em"); // original unchanged
}

#[test]
fn test_highlight_with_tag() {
    let h = Highlight::new("text", "b");
    let h2 = h.with_tag("i");
    assert_eq!(h2.render(), "<i>text</i>");
}

#[test]
fn test_annotated() {
    let a = Annotated::new("hello", "note1", 5);
    assert_eq!(a.format(), "hello [note1@5]");
    assert_eq!(a.text, "hello");
    assert_eq!(a.note, "note1");
}

// ===== Topic 5: 'static & Lifetime Elision =====

#[test]
fn test_greeting() {
    assert_eq!(greeting(true), "Good day");
    assert_eq!(greeting(false), "Hey");
}

#[test]
fn test_status_message() {
    assert_eq!(status_message(200), "OK");
    assert_eq!(status_message(404), "Not Found");
}

#[test]
fn test_status_message_unknown() {
    assert_eq!(status_message(999), "Unknown");
}

#[test]
fn test_day_name() {
    assert_eq!(day_name(0), "Sunday");
    assert_eq!(day_name(6), "Saturday");
}

#[test]
fn test_day_name_invalid() {
    assert_eq!(day_name(7), "Invalid");
}

#[test]
fn test_trim_non_alnum() {
    assert_eq!(trim_non_alnum("---hello"), "hello");
    assert_eq!(trim_non_alnum("  ##title"), "title");
    assert_eq!(trim_non_alnum("already"), "already");
}

#[test]
fn test_split_first_token() {
    assert_eq!(split_first_token("hello world foo"), ("hello", "world foo"));
}

#[test]
fn test_split_first_token_single() {
    assert_eq!(split_first_token("single"), ("single", ""));
}

#[test]
fn test_split_first_token_empty() {
    assert_eq!(split_first_token(""), ("", ""));
    assert_eq!(split_first_token("  "), ("", ""));
}

#[test]
fn test_config_get_value() {
    let config = Config::new("host=localhost\nport=8080");
    assert_eq!(config.get_value("host"), Some("localhost"));
    assert_eq!(config.get_value("port"), Some("8080"));
    assert_eq!(config.get_value("missing"), None);
}

#[test]
fn test_config_keys() {
    let config = Config::new("host=localhost\nport=8080\nname=test");
    assert_eq!(config.keys(), vec!["host", "port", "name"]);
}

#[test]
fn test_config_sections() {
    let config = Config::new("[database]\nhost=localhost\n[server]\nport=8080");
    assert_eq!(config.sections(), vec!["database", "server"]);
}

// ===== Topic 6: Lifetime Bounds on Generics =====

#[test]
fn test_find_first() {
    let nums = vec![1, 5, 3, 8, 2];
    assert_eq!(find_first(&nums, |&n| n > 4), Some(&5));
}

#[test]
fn test_find_first_none() {
    let nums = vec![1, 2, 3];
    assert_eq!(find_first(&nums, |&n| n > 10), None);
}

#[test]
fn test_min_by_key() {
    let words = vec!["hello", "hi", "hey"];
    assert_eq!(min_by_key(&words, |w| w.len()), Some(&"hi"));
}

#[test]
fn test_min_by_key_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(min_by_key(&empty, |&x| x), None);
}

#[test]
fn test_partition_refs() {
    let nums = vec![1, 2, 3, 4, 5];
    let (evens, odds) = partition_refs(&nums, |&n| n % 2 == 0);
    assert_eq!(evens, vec![&2, &4]);
    assert_eq!(odds, vec![&1, &3, &5]);
}

#[test]
fn test_longest_matching() {
    let items = vec!["hello", "help", "world", "helper"];
    assert_eq!(longest_matching(&items, "hel").copied(), Some("helper"));
}

#[test]
fn test_longest_matching_none() {
    let items = vec!["hello", "world"];
    assert!(longest_matching(&items, "xyz").is_none());
}

#[test]
fn test_cached_lookup() {
    let data = [("x", 10), ("y", 20), ("z", 30)];
    let cache = CachedLookup::new(&data);
    assert_eq!(cache.lookup("x"), Some(10));
}

#[test]
fn test_cached_lookup_miss() {
    let data = [("a", 1)];
    let cache = CachedLookup::new(&data);
    assert_eq!(cache.lookup("z"), None);
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

#[test]
fn test_cached_entries_where() {
    let data = [("apple", 10), ("banana", 5), ("avocado", 20)];
    let cache = CachedLookup::new(&data);
    let result = cache.entries_where(|k| k.starts_with('a'));
    assert_eq!(result.len(), 2);
    assert!(result.contains(&("apple", 10)));
    assert!(result.contains(&("avocado", 20)));
}

// ===== Topic 7: Combined — Lifetimes + Traits + Generics =====

#[test]
fn test_ref_new_get() {
    let val = 42;
    let r = Ref::new(&val);
    assert_eq!(*r.get(), 42);
}

#[test]
fn test_ref_map() {
    let val = 10;
    let r = Ref::new(&val);
    assert_eq!(r.map(|&v| v * 2), 20);
}

#[test]
fn test_ref_display() {
    let val = "hello";
    let r = Ref::new(&val);
    assert_eq!(r.to_string(), "Ref(hello)");
}

#[test]
fn test_ref_eq() {
    let a = 42;
    let b = 42;
    let ra = Ref::new(&a);
    let rb = Ref::new(&b);
    assert_eq!(ra, rb);
}

#[test]
fn test_lookup_table_new_empty() {
    let table: LookupTable = LookupTable::new();
    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
}

#[test]
fn test_lookup_table_from_pairs() {
    let table = LookupTable::from_pairs(&[("a", "1"), ("b", "2")]);
    assert_eq!(table.get("a"), Some("1"));
    assert_eq!(table.get("b"), Some("2"));
    assert_eq!(table.len(), 2);
}

#[test]
fn test_lookup_table_insert_get() {
    let mut table = LookupTable::new();
    table.insert("key", "value");
    assert_eq!(table.get("key"), Some("value"));
    table.insert("key", "updated");
    assert_eq!(table.get("key"), Some("updated"));
    assert_eq!(table.len(), 1); // updated, not duplicated
}

#[test]
fn test_lookup_table_contains_key() {
    let table = LookupTable::from_pairs(&[("x", "1")]);
    assert!(table.contains_key("x"));
    assert!(!table.contains_key("y"));
}

#[test]
fn test_lookup_table_keys_values() {
    let table = LookupTable::from_pairs(&[("a", "1"), ("b", "2")]);
    assert_eq!(table.keys(), vec!["a", "b"]);
    assert_eq!(table.values(), vec!["1", "2"]);
}

#[test]
fn test_lookup_table_display() {
    let table = LookupTable::from_pairs(&[("x", "10"), ("y", "20")]);
    assert_eq!(table.to_string(), "{x=10, y=20}");
}

#[test]
fn test_lookup_table_default() {
    let table: LookupTable = LookupTable::default();
    assert!(table.is_empty());
}

#[test]
fn test_parse_pairs_ok() {
    let input = "name=Alice\nage=30";
    let pairs = parse_pairs(input).unwrap();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].key, "name");
    assert_eq!(pairs[0].value, "Alice");
    assert_eq!(pairs[1].key, "age");
    assert_eq!(pairs[1].value, "30");
}

#[test]
fn test_parse_pairs_err() {
    let input = "name=Alice\nbad line\nage=30";
    let result = parse_pairs(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "invalid pair on line 2");
}

#[test]
fn test_parse_pairs_empty_lines() {
    let input = "name=Alice\n\nage=30\n";
    let pairs = parse_pairs(input).unwrap();
    assert_eq!(pairs.len(), 2);
}

#[test]
fn test_format_table() {
    let rows = vec![("name", "Alice"), ("age", "30"), ("city", "NYC")];
    let table = format_table(&rows);
    assert!(table.contains("name | Alice"));
    assert!(table.contains("age  | 30"));
    assert!(table.contains("city | NYC"));
}

#[test]
fn test_format_table_empty() {
    assert_eq!(format_table(&[]), "");
}