use strings::*;

// ===== Topic 1: String Basics =====

#[test]
fn test_create_empty_string() {
    let s = create_empty_string();
    assert_eq!(s, "");
    assert!(s.is_empty());
}

#[test]
fn test_create_greeting() {
    assert_eq!(create_greeting(), "hello, rust!");
}

#[test]
fn test_append() {
    assert_eq!(append("hello", " world"), "hello world");
    assert_eq!(append("", "rust"), "rust");
    assert_eq!(append("foo", ""), "foo");
}

#[test]
fn test_add_excitement() {
    assert_eq!(add_excitement("hello"), "hello!");
    assert_eq!(add_excitement(""), "!");
}

#[test]
fn test_byte_length() {
    assert_eq!(byte_length("hello"), 5);
    assert_eq!(byte_length(""), 0);
    assert_eq!(byte_length("🦀"), 4); // emoji is 4 bytes!
}

#[test]
fn test_char_count() {
    assert_eq!(char_count("hello"), 5);
    assert_eq!(char_count(""), 0);
    assert_eq!(char_count("🦀"), 1); // emoji is 1 char
    assert_eq!(char_count("hello🦀"), 6);
}

#[test]
fn test_is_blank() {
    assert!(is_blank(""));
    assert!(!is_blank("hello"));
    assert!(!is_blank(" ")); // space is NOT blank
}

// ===== Topic 2: String Slices & Searching =====

#[test]
fn test_first_n_chars() {
    assert_eq!(first_n_chars("hello", 3), "hel");
    assert_eq!(first_n_chars("hi", 10), "hi");
    assert_eq!(first_n_chars("", 5), "");
}

#[test]
fn test_last_n_chars() {
    assert_eq!(last_n_chars("hello", 3), "llo");
    assert_eq!(last_n_chars("hi", 10), "hi");
    assert_eq!(last_n_chars("rust", 1), "t");
}

#[test]
fn test_contains_substring() {
    assert!(contains_substring("hello world", "world"));
    assert!(contains_substring("hello", "hello"));
    assert!(!contains_substring("hello", "xyz"));
}

#[test]
fn test_has_prefix() {
    assert!(has_prefix("hello world", "hello"));
    assert!(!has_prefix("hello world", "world"));
    assert!(has_prefix("rust", ""));
}

#[test]
fn test_has_suffix() {
    assert!(has_suffix("hello world", "world"));
    assert!(!has_suffix("hello world", "hello"));
    assert!(has_suffix("rust", ""));
}

#[test]
fn test_find_position() {
    assert_eq!(find_position("hello world", "world"), Some(6));
    assert_eq!(find_position("hello", "xyz"), None);
    assert_eq!(find_position("hello hello", "hello"), Some(0));
}

// ===== Topic 3: String Transformation Methods =====

#[test]
fn test_clean_whitespace() {
    assert_eq!(clean_whitespace("  hello  "), "hello");
    assert_eq!(clean_whitespace("no spaces"), "no spaces");
    assert_eq!(clean_whitespace("  "), "");
}

#[test]
fn test_shout() {
    assert_eq!(shout("hello"), "HELLO");
    assert_eq!(shout("Rust"), "RUST");
}

#[test]
fn test_whisper() {
    assert_eq!(whisper("HELLO"), "hello");
    assert_eq!(whisper("Rust"), "rust");
}

#[test]
fn test_replace_all() {
    assert_eq!(replace_all("hello world", "world", "rust"), "hello rust");
    assert_eq!(replace_all("aaa", "a", "b"), "bbb");
}

#[test]
fn test_split_by() {
    assert_eq!(split_by("a,b,c", ','), vec!["a", "b", "c"]);
    assert_eq!(split_by("hello", ','), vec!["hello"]);
}

#[test]
fn test_repeat_string() {
    assert_eq!(repeat_string("ha", 3), "hahaha");
    assert_eq!(repeat_string("x", 0), "");
    assert_eq!(repeat_string("ab", 2), "abab");
}

// ===== Topic 4: String Conversion & Parsing =====

#[test]
fn test_int_to_string() {
    assert_eq!(int_to_string(42), "42");
    assert_eq!(int_to_string(-1), "-1");
    assert_eq!(int_to_string(0), "0");
}

#[test]
fn test_parse_int() {
    assert_eq!(parse_int("42"), Some(42));
    assert_eq!(parse_int("-1"), Some(-1));
    assert_eq!(parse_int("abc"), None);
    assert_eq!(parse_int(""), None);
}

#[test]
fn test_parse_float() {
    assert_eq!(parse_float("3.15"), Some(3.15));
    assert_eq!(parse_float("42"), Some(42.0));
    assert_eq!(parse_float("abc"), None);
}

#[test]
fn test_format_greeting() {
    assert_eq!(
        format_greeting("Alice", 30),
        "Hello, Alice! You are 30 years old."
    );
}

#[test]
fn test_bool_to_word() {
    assert_eq!(bool_to_word(true), "yes");
    assert_eq!(bool_to_word(false), "no");
}

// ===== Topic 5: String Building & Advanced =====

#[test]
fn test_comma_join() {
    assert_eq!(comma_join(&["a", "b", "c"]), "a, b, c");
    assert_eq!(comma_join(&["solo"]), "solo");
    assert_eq!(comma_join(&[]), "");
}

#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("a"), "a");
    assert_eq!(reverse_string(""), "");
}

#[test]
fn test_count_char() {
    assert_eq!(count_char("hello", 'l'), 2);
    assert_eq!(count_char("hello", 'z'), 0);
    assert_eq!(count_char("aaa", 'a'), 3);
}

#[test]
fn test_extract_words() {
    assert_eq!(extract_words("hello world"), vec!["hello", "world"]);
    assert_eq!(extract_words("  spaced  out  "), vec!["spaced", "out"]);
    assert_eq!(extract_words("single"), vec!["single"]);
}

#[test]
fn test_make_acronym() {
    assert_eq!(make_acronym("Test Driven Development"), "TDD");
    assert_eq!(make_acronym("Rust Programming Language"), "RPL");
    assert_eq!(make_acronym("hello"), "H");
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("Racecar"));
    assert!(!is_palindrome("hello"));
    assert!(is_palindrome("a"));
    assert!(is_palindrome(""));
}

#[test]
fn test_title_case() {
    assert_eq!(title_case("hello world"), "Hello World");
    assert_eq!(title_case("rust is great"), "Rust Is Great");
    assert_eq!(title_case("HELLO"), "Hello");
}

// ===== Topic 6: Advanced String Challenges =====

#[test]
fn test_caesar_cipher() {
    assert_eq!(caesar_cipher("hello", 3), "khoor");
    assert_eq!(caesar_cipher("abc", 1), "bcd");
    assert_eq!(caesar_cipher("xyz", 3), "abc"); // wraps around
    assert_eq!(caesar_cipher("Hello!", 1), "Ifmmp!");
    assert_eq!(caesar_cipher("a b c", 0), "a b c");
}

#[test]
fn test_run_length_encode() {
    assert_eq!(run_length_encode("aaabbc"), "3a2b1c");
    assert_eq!(run_length_encode("aaa"), "3a");
    assert_eq!(run_length_encode("abc"), "1a1b1c");
    assert_eq!(run_length_encode(""), "");
}

#[test]
fn test_run_length_decode() {
    assert_eq!(run_length_decode("3a2b1c"), "aaabbc");
    assert_eq!(run_length_decode("3a"), "aaa");
    assert_eq!(run_length_decode("1a1b1c"), "abc");
    assert_eq!(run_length_decode(""), "");
}

#[test]
fn test_roundtrip_rle() {
    let original = "aaabbbccddddee";
    assert_eq!(run_length_decode(&run_length_encode(original)), original);
}

#[test]
fn test_count_vowels_consonants() {
    assert_eq!(count_vowels_consonants("hello"), (2, 3));
    assert_eq!(count_vowels_consonants("aeiou"), (5, 0));
    assert_eq!(count_vowels_consonants("xyz"), (0, 3));
    assert_eq!(count_vowels_consonants("hello 123!"), (2, 3));
}

#[test]
fn test_longest_word() {
    assert_eq!(longest_word("the quick brown fox jumps"), "jumps");
    assert_eq!(longest_word("hello"), "hello");
    assert_eq!(longest_word("a bb ccc"), "ccc");
    assert_eq!(longest_word(""), "");
}

#[test]
fn test_remove_consecutive_dupes_str() {
    assert_eq!(remove_consecutive_dupes("aabbccaab"), "abcab");
    assert_eq!(remove_consecutive_dupes("aaa"), "a");
    assert_eq!(remove_consecutive_dupes("abc"), "abc");
    assert_eq!(remove_consecutive_dupes(""), "");
}

#[test]
fn test_interleave() {
    assert_eq!(interleave("abc", "123"), "a1b2c3");
    assert_eq!(interleave("abcd", "12"), "a1b2cd");
    assert_eq!(interleave("ab", "1234"), "a1b234");
    assert_eq!(interleave("", "abc"), "abc");
    assert_eq!(interleave("abc", ""), "abc");
}

#[test]
fn test_are_anagrams() {
    assert!(are_anagrams("listen", "silent"));
    assert!(are_anagrams("Astronomer", "Moon starer"));
    assert!(!are_anagrams("hello", "world"));
    assert!(are_anagrams("aab", "aba"));
}

#[test]
fn test_pig_latin() {
    assert_eq!(pig_latin("hello"), "ellohay");
    assert_eq!(pig_latin("apple"), "applehay");
    assert_eq!(pig_latin("string"), "ingstray");
    assert_eq!(pig_latin("chair"), "airchay");
}

#[test]
fn test_is_valid_email() {
    assert!(is_valid_email("user@example.com"));
    assert!(is_valid_email("a@b.c"));
    assert!(!is_valid_email("no-at-sign.com"));
    assert!(!is_valid_email("@example.com"));
    assert!(!is_valid_email("user@"));
    assert!(!is_valid_email("user@nodot"));
    assert!(!is_valid_email("user@@double.com"));
}

#[test]
fn test_truncate() {
    assert_eq!(truncate("hello world", 5), "hello...");
    assert_eq!(truncate("hi", 5), "hi");
    assert_eq!(truncate("hello", 5), "hello");
    assert_eq!(truncate("abcdef", 3), "abc...");
}

#[test]
fn test_to_snake_case() {
    assert_eq!(to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(to_snake_case("helloWorld"), "hello_world");
    assert_eq!(to_snake_case("Hello"), "hello");
    assert_eq!(to_snake_case("already"), "already");
}

// ===== Topic 7: Advanced String Manipulation =====

#[test]
fn test_remove_last_char() {
    let mut s = String::from("hello");
    assert_eq!(remove_last_char(&mut s), Some('o'));
    assert_eq!(s, "hell");

    let mut empty = String::new();
    assert_eq!(remove_last_char(&mut empty), None);
}

#[test]
fn test_split_once_at() {
    assert_eq!(split_once_at("key=value", '='), Some(("key", "value")));
    assert_eq!(split_once_at("a,b,c", ','), Some(("a", "b,c")));
    assert_eq!(split_once_at("no delimiter", ':'), None);
}

#[test]
fn test_count_non_empty_lines() {
    let text = "Line 1\n\nLine 2\n   \nLine 3";
    assert_eq!(count_non_empty_lines(text), 3);
    assert_eq!(count_non_empty_lines(""), 0);
}

#[test]
fn test_remove_surrounding_quotes() {
    assert_eq!(remove_surrounding_quotes("\"hello\""), "hello");
    assert_eq!(remove_surrounding_quotes("'world'"), "world");
    assert_eq!(remove_surrounding_quotes("\"'nested'\""), "nested");
    assert_eq!(remove_surrounding_quotes("no quotes"), "no quotes");
}

#[test]
fn test_find_char_byte_index() {
    // '🦀' is 4 bytes.
    let s = "a🦀b";
    assert_eq!(find_char_byte_index(s, 0), Some(0)); // 'a'
    assert_eq!(find_char_byte_index(s, 1), Some(1)); // '🦀'
    assert_eq!(find_char_byte_index(s, 2), Some(5)); // 'b' starts at byte 5
    assert_eq!(find_char_byte_index(s, 3), None);
}

#[test]
fn test_clear_string() {
    let mut s = String::from("hello");
    let capacity_before = s.capacity();
    clear_string(&mut s);
    assert_eq!(s, "");
    assert_eq!(s.capacity(), capacity_before);
}

// ===== Topic 8: Extra Practice =====

#[test]
fn test_is_pangram() {
    assert!(is_pangram("The quick brown fox jumps over the lazy dog"));
    assert!(!is_pangram("Hello World"));
    assert!(!is_pangram(""));
}

#[test]
fn test_count_distinct_words() {
    assert_eq!(count_distinct_words("hello Hello HELLO"), 1);
    assert_eq!(count_distinct_words("the cat sat on the mat"), 5);
    assert_eq!(count_distinct_words(""), 0);
}

#[test]
fn test_most_common_char() {
    assert_eq!(most_common_char("aabbbcc"), Some('b'));
    assert_eq!(most_common_char(""), None);
}

#[test]
fn test_word_wrap() {
    let result = word_wrap("the quick brown fox jumps over the lazy dog", 15);
    assert!(result.contains('\n'));
    // Each line should be <= 15 chars
    for line in result.lines() {
        assert!(line.len() <= 15, "line too long: '{}'", line);
    }
}

