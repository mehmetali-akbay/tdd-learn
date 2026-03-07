use files::*;

// =====================================================================
// Topic 1: Line Processing
// =====================================================================

#[test]
fn test_count_lines() {
    assert_eq!(count_lines("hello\nworld\n"), 2);
    assert_eq!(count_lines("hello\n\nworld\n"), 2); // skip empty
    assert_eq!(count_lines(""), 0);
}

#[test]
fn test_grep() {
    let content = "Hello World\nfoo bar\nhello again";
    let matches = grep(content, "hello");
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0], "Hello World");
}

#[test]
fn test_grep_no_match() {
    assert!(grep("aaa\nbbb", "zzz").is_empty());
}

#[test]
fn test_grep_line_numbers() {
    let content = "apple\nbanana\napricot";
    let hits = grep_line_numbers(content, "ap");
    assert_eq!(hits, vec![(1, "apple".into()), (3, "apricot".into())]);
}

#[test]
fn test_number_lines() {
    assert_eq!(number_lines("a\nb\nc"), "1: a\n2: b\n3: c");
}

#[test]
fn test_head() {
    assert_eq!(head("a\nb\nc\nd\ne", 3), "a\nb\nc");
    assert_eq!(head("a\nb", 5), "a\nb"); // fewer lines than requested
}

#[test]
fn test_tail() {
    assert_eq!(tail("a\nb\nc\nd\ne", 2), "d\ne");
    assert_eq!(tail("a\nb", 5), "a\nb");
}

#[test]
fn test_dedup_lines() {
    assert_eq!(dedup_lines("a\na\nb\nb\nb\na"), "a\nb\na");
    assert_eq!(dedup_lines("x"), "x");
}

#[test]
fn test_blank_line_count() {
    assert_eq!(blank_line_count("hello\n\nworld\n  \nfoo"), 2);
    assert_eq!(blank_line_count("no blanks"), 0);
}

#[test]
fn test_word_count_per_line() {
    assert_eq!(
        word_count_per_line("hello world\nfoo\n\nbar baz qux"),
        vec![2, 1, 0, 3]
    );
}

// =====================================================================
// Topic 2: CSV Parsing
// =====================================================================

#[test]
fn test_parse_csv() {
    let csv = "name,age,city\nAlice,30,NYC\nBob,25,LA";
    let (headers, records) = parse_csv(csv);
    assert_eq!(headers, vec!["name", "age", "city"]);
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].get(0), Some("Alice"));
    assert_eq!(records[1].get(1), Some("25"));
}

#[test]
fn test_csv_record_len() {
    let r = CsvRecord::new(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(r.len(), 2);
    assert!(!r.is_empty());
}

#[test]
fn test_csv_record_empty() {
    let r = CsvRecord::new(vec![]);
    assert!(r.is_empty());
    assert_eq!(r.get(0), None);
}

#[test]
fn test_get_column() {
    let csv = "name,score\nAlice,90\nBob,85";
    let (h, r) = parse_csv(csv);
    assert_eq!(get_column(&h, &r, "name"), vec!["Alice", "Bob"]);
    assert_eq!(get_column(&h, &r, "missing"), Vec::<String>::new());
}

#[test]
fn test_sum_column() {
    let csv = "name,score\nAlice,90\nBob,85";
    let (h, r) = parse_csv(csv);
    assert_eq!(sum_column(&h, &r, "score"), Some(175.0));
    assert_eq!(sum_column(&h, &r, "missing"), None);
}

#[test]
fn test_average_column() {
    let csv = "name,score\nAlice,90\nBob,80\nCharlie,70";
    let (h, r) = parse_csv(csv);
    assert_eq!(average_column(&h, &r, "score"), Some(80.0));
    assert_eq!(average_column(&h, &r, "missing"), None);
}

#[test]
fn test_filter_csv() {
    let csv = "name,city\nAlice,NYC\nBob,LA\nCharlie,NYC";
    let (h, r) = parse_csv(csv);
    let nyc = filter_csv(&h, &r, "city", "NYC");
    assert_eq!(nyc.len(), 2);
    assert_eq!(nyc[0].get(0), Some("Alice"));
    assert_eq!(nyc[1].get(0), Some("Charlie"));
}

#[test]
fn test_filter_csv_no_match() {
    let csv = "name,city\nAlice,NYC";
    let (h, r) = parse_csv(csv);
    assert!(filter_csv(&h, &r, "city", "London").is_empty());
}

#[test]
fn test_to_csv_string() {
    let csv = "name,age\nAlice,30\nBob,25";
    let (h, r) = parse_csv(csv);
    assert_eq!(to_csv_string(&h, &r), csv);
}

#[test]
fn test_csv_roundtrip() {
    let original = "a,b,c\n1,2,3\n4,5,6";
    let (h, r) = parse_csv(original);
    assert_eq!(to_csv_string(&h, &r), original);
}

#[test]
fn test_csv_record_count() {
    assert_eq!(csv_record_count("h1,h2\na,b\nc,d\ne,f"), 3);
    assert_eq!(csv_record_count("h1\n"), 0);
}

// =====================================================================
// Topic 3: Config Parsing
// =====================================================================

#[test]
fn test_parse_config() {
    let content = "# comment\nhost = localhost\nport = 8080\n\n# another comment\ndebug = true";
    let config = parse_config(content);
    assert_eq!(config["host"], "localhost");
    assert_eq!(config["port"], "8080");
    assert_eq!(config["debug"], "true");
    assert_eq!(config.len(), 3);
}

#[test]
fn test_config_get_or() {
    let config = parse_config("host = localhost");
    assert_eq!(config_get_or(&config, "host", "0.0.0.0"), "localhost");
    assert_eq!(config_get_or(&config, "port", "3000"), "3000");
}

#[test]
fn test_config_get_int() {
    let config = parse_config("port = 8080\nname = server");
    assert_eq!(config_get_int(&config, "port"), Some(8080));
    assert_eq!(config_get_int(&config, "name"), None);
    assert_eq!(config_get_int(&config, "missing"), None);
}

#[test]
fn test_config_get_float() {
    let config = parse_config("rate = 3.14\nname = pi");
    assert_eq!(config_get_float(&config, "rate"), Some(3.14));
    assert_eq!(config_get_float(&config, "name"), None);
}

#[test]
fn test_config_get_bool() {
    let config = parse_config("debug = true\nverbose = yes\nsilent = false\nquiet = 0");
    assert_eq!(config_get_bool(&config, "debug"), Some(true));
    assert_eq!(config_get_bool(&config, "verbose"), Some(true));
    assert_eq!(config_get_bool(&config, "silent"), Some(false));
    assert_eq!(config_get_bool(&config, "quiet"), Some(false));
    assert_eq!(config_get_bool(&config, "missing"), None);
}

#[test]
fn test_validate_config() {
    let config = parse_config("host = localhost\nport = 8080");
    assert!(validate_config(&config, &["host", "port"]).is_ok());
    let missing = validate_config(&config, &["host", "port", "db"]);
    assert_eq!(missing, Err(vec!["db".to_string()]));
}

#[test]
fn test_merge_configs() {
    let base = parse_config("host = localhost\nport = 3000");
    let overrides = parse_config("port = 8080\ndebug = true");
    let merged = merge_configs(&base, &overrides);
    assert_eq!(merged["host"], "localhost");
    assert_eq!(merged["port"], "8080"); // overridden
    assert_eq!(merged["debug"], "true"); // added
}

#[test]
fn test_serialize_config() {
    let config = parse_config("port = 8080\nhost = localhost");
    let serialized = serialize_config(&config);
    assert!(serialized.contains("host = localhost"));
    assert!(serialized.contains("port = 8080"));
    // Should be sorted: host before port
    let lines: Vec<&str> = serialized.lines().collect();
    assert!(lines[0].starts_with("host"));
    assert!(lines[1].starts_with("port"));
}

#[test]
fn test_config_roundtrip() {
    let original = "debug = true\nhost = localhost\nport = 8080";
    let config = parse_config(original);
    let serialized = serialize_config(&config);
    let reparsed = parse_config(&serialized);
    assert_eq!(config, reparsed);
}

// =====================================================================
// Topic 4: Text Statistics
// =====================================================================

#[test]
fn test_text_stats() {
    let stats = text_stats("hello world\nfoo bar baz");
    assert_eq!(stats.lines, 2);
    assert_eq!(stats.words, 5);
}

#[test]
fn test_text_stats_empty() {
    let stats = text_stats("");
    assert_eq!(stats.words, 0);
    assert_eq!(stats.chars, 0);
}

#[test]
fn test_longest_line() {
    assert_eq!(
        longest_line("short\na much longer line\nmedium"),
        Some("a much longer line".to_string())
    );
    assert_eq!(longest_line(""), None);
}

#[test]
fn test_shortest_line() {
    assert_eq!(
        shortest_line("short\nhi\na much longer line"),
        Some("hi".to_string())
    );
}

#[test]
fn test_average_word_length() {
    assert!((average_word_length("hi there world") - 4.0).abs() < 0.1);
    // "hi"=2, "there"=5, "world"=5 => avg = 4.0
    assert_eq!(average_word_length(""), 0.0);
}

#[test]
fn test_word_frequency() {
    let freq = word_frequency("the cat sat on the mat");
    assert_eq!(freq["the"], 2);
    assert_eq!(freq["cat"], 1);
    assert_eq!(freq["mat"], 1);
}

#[test]
fn test_word_frequency_case_insensitive() {
    let freq = word_frequency("Hello hello HELLO");
    assert_eq!(freq["hello"], 3);
    assert_eq!(freq.len(), 1);
}

#[test]
fn test_most_common_word() {
    assert_eq!(
        most_common_word("the cat and the dog and the fish"),
        Some("the".to_string())
    );
    assert_eq!(most_common_word(""), None);
}

#[test]
fn test_unique_word_count() {
    assert_eq!(unique_word_count("the cat and the dog"), 4);
    assert_eq!(unique_word_count("Hello hello HELLO"), 1);
    assert_eq!(unique_word_count(""), 0);
}

// =====================================================================
// Topic 5: Text Transformations
// =====================================================================

#[test]
fn test_sort_lines() {
    assert_eq!(sort_lines("banana\napple\ncherry"), "apple\nbanana\ncherry");
}

#[test]
fn test_reverse_lines() {
    assert_eq!(reverse_lines("a\nb\nc"), "c\nb\na");
}

#[test]
fn test_sort_reverse_roundtrip() {
    let content = "c\na\nb";
    let sorted = sort_lines(content);
    let reversed_sorted = reverse_lines(&sorted);
    assert_eq!(reversed_sorted, "c\nb\na");
}

#[test]
fn test_indent_lines() {
    assert_eq!(indent_lines("a\nb\nc", 4), "    a\n    b\n    c");
}

#[test]
fn test_truncate_lines() {
    assert_eq!(
        truncate_lines("hello world\nhi\nabcdefghij", 5),
        "hello...\nhi\nabcde..."
    );
}

#[test]
fn test_dedent_lines() {
    let indented = "    hello\n    world\n    foo";
    assert_eq!(dedent_lines(indented), "hello\nworld\nfoo");
}

#[test]
fn test_dedent_mixed_indent() {
    let content = "    hello\n      world\n    foo";
    assert_eq!(dedent_lines(content), "hello\n  world\nfoo");
}

#[test]
fn test_indent_dedent_roundtrip() {
    let original = "hello\nworld";
    let indented = indent_lines(original, 4);
    let dedented = dedent_lines(&indented);
    assert_eq!(dedented, original);
}

#[test]
fn test_to_uppercase_lines() {
    assert_eq!(to_uppercase_lines("hello\nWorld"), "HELLO\nWORLD");
}

#[test]
fn test_center_lines() {
    let centered = center_lines("hi\nhello", 10);
    let lines: Vec<&str> = centered.lines().collect();
    assert_eq!(lines[0].len(), 10);
    assert!(lines[0].contains("hi"));
    assert_eq!(lines[1].len(), 10);
}

#[test]
fn test_center_lines_already_wide() {
    // Lines wider than width should not be padded
    assert_eq!(center_lines("hello world", 5), "hello world");
}

// =====================================================================
// Topic 6: Log Parsing
// =====================================================================

#[test]
fn test_parse_log() {
    let log = "[INFO] Server started\n[ERROR] Connection failed\n[INFO] Request received";
    let entries = parse_log(log);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].level, "INFO");
    assert_eq!(entries[0].message, "Server started");
    assert_eq!(entries[1].level, "ERROR");
}

#[test]
fn test_parse_log_ignores_invalid() {
    let log = "[INFO] valid\nno brackets\n[WARN] also valid";
    let entries = parse_log(log);
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_format_log_entry() {
    let entry = LogEntry {
        level: "ERROR".into(),
        message: "disk full".into(),
    };
    assert_eq!(format_log_entry(&entry), "[ERROR] disk full");
}

#[test]
fn test_parse_format_roundtrip() {
    let original = "[INFO] hello world";
    let entries = parse_log(original);
    assert_eq!(entries.len(), 1);
    assert_eq!(format_log_entry(&entries[0]), original);
}

#[test]
fn test_filter_logs() {
    let log = "[INFO] A\n[ERROR] B\n[INFO] C";
    let entries = parse_log(log);
    let errors = filter_logs(&entries, "ERROR");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message, "B");
}

#[test]
fn test_log_level_counts() {
    let log = "[INFO] A\n[ERROR] B\n[INFO] C\n[ERROR] D";
    let entries = parse_log(log);
    let counts = log_level_counts(&entries);
    assert_eq!(counts["INFO"], 2);
    assert_eq!(counts["ERROR"], 2);
}

#[test]
fn test_unique_messages() {
    let log = "[INFO] hello\n[INFO] world\n[INFO] hello";
    let entries = parse_log(log);
    let msgs = unique_messages(&entries, "INFO");
    assert_eq!(msgs, vec!["hello", "world"]);
}

#[test]
fn test_search_logs() {
    let log = "[INFO] User logged in\n[ERROR] Disk full\n[INFO] User logged out";
    let entries = parse_log(log);
    let results = search_logs(&entries, "user");
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].message, "User logged in");
}

#[test]
fn test_search_logs_no_match() {
    let entries = parse_log("[INFO] hello");
    assert!(search_logs(&entries, "zzz").is_empty());
}

#[test]
fn test_log_summary() {
    let log = "[INFO] A\n[ERROR] B\n[INFO] C\n[WARN] D";
    let entries = parse_log(log);
    let summary = log_summary(&entries);
    assert!(summary.contains("INFO: 2"));
    assert!(summary.contains("ERROR: 1"));
    assert!(summary.contains("WARN: 1"));
}

// =====================================================================
// Topic 7: Path Operations
// =====================================================================

#[test]
fn test_get_extension() {
    assert_eq!(get_extension("main.rs"), Some("rs"));
    assert_eq!(get_extension("archive.tar.gz"), Some("gz"));
    assert_eq!(get_extension("Makefile"), None);
}

#[test]
fn test_get_extension_with_path() {
    assert_eq!(get_extension("/home/user/file.txt"), Some("txt"));
}

#[test]
fn test_get_parent() {
    assert_eq!(get_parent("/home/user/file.txt"), Some("/home/user"));
    assert_eq!(get_parent("/file.txt"), Some("/"));
}

#[test]
fn test_get_parent_no_slash() {
    assert_eq!(get_parent("file.txt"), None);
}

#[test]
fn test_get_filename() {
    assert_eq!(get_filename("/home/user/file.txt"), Some("file.txt"));
    assert_eq!(get_filename("file.txt"), Some("file.txt"));
    assert_eq!(get_filename("/"), None);
}

#[test]
fn test_get_stem() {
    assert_eq!(get_stem("main.rs"), Some("main"));
    assert_eq!(get_stem("archive.tar.gz"), Some("archive.tar"));
    assert_eq!(get_stem("Makefile"), Some("Makefile"));
}

#[test]
fn test_get_stem_with_path() {
    assert_eq!(get_stem("/src/main.rs"), Some("main"));
}

#[test]
fn test_replace_extension() {
    assert_eq!(replace_extension("main.rs", "txt"), "main.txt");
    assert_eq!(replace_extension("Makefile", "bak"), "Makefile.bak");
}

#[test]
fn test_join_path() {
    assert_eq!(join_path("/home/user", "file.txt"), "/home/user/file.txt");
    assert_eq!(join_path("/home/user/", "file.txt"), "/home/user/file.txt");
    assert_eq!(join_path("", "file.txt"), "file.txt");
}

#[test]
fn test_join_path_absolute_child() {
    // Absolute child replaces entire path
    assert_eq!(join_path("/home/user", "/etc/config"), "/etc/config");
}

#[test]
fn test_is_absolute() {
    assert!(is_absolute("/home/user"));
    assert!(!is_absolute("relative/path"));
    assert!(!is_absolute("file.txt"));
}

#[test]
fn test_matches_pattern() {
    assert!(matches_pattern("main.rs", "*.rs"));
    assert!(!matches_pattern("main.rs", "*.txt"));
    assert!(matches_pattern("Makefile", "Makefile"));
}

#[test]
fn test_path_depth() {
    assert_eq!(path_depth("/home/user/file.txt"), 3);
    assert_eq!(path_depth("file.txt"), 1);
    assert_eq!(path_depth("/"), 0);
    assert_eq!(path_depth("a/b/c"), 3);
}

// =====================================================================
// Additional edge-case & integration tests
// =====================================================================

#[test]
fn test_count_lines_only_blanks() {
    assert_eq!(count_lines("\n\n  \n"), 0);
}

#[test]
fn test_head_empty() {
    assert_eq!(head("", 5), "");
}

#[test]
fn test_tail_empty() {
    assert_eq!(tail("", 5), "");
}

#[test]
fn test_number_lines_single() {
    assert_eq!(number_lines("only"), "1: only");
}

#[test]
fn test_blank_line_count_none() {
    assert_eq!(blank_line_count("all\nlines\nfilled"), 0);
}

#[test]
fn test_word_count_per_line_single() {
    assert_eq!(word_count_per_line("hello world foo"), vec![3]);
}

#[test]
fn test_parse_csv_empty() {
    let (headers, records) = parse_csv("");
    assert!(headers.is_empty());
    assert!(records.is_empty());
}

#[test]
fn test_sum_column_non_numeric() {
    let csv = "name,score\nAlice,abc\nBob,xyz";
    let (h, r) = parse_csv(csv);
    // parse fails on non-numeric so sum is 0.0 (all filtered out)
    assert_eq!(sum_column(&h, &r, "score"), Some(0.0));
}

#[test]
fn test_parse_config_empty() {
    let config = parse_config("");
    assert!(config.is_empty());
}

#[test]
fn test_parse_config_only_comments() {
    let config = parse_config("# this is a comment\n# another one");
    assert!(config.is_empty());
}

#[test]
fn test_validate_config_empty_required() {
    let config = parse_config("key = value");
    assert!(validate_config(&config, &[]).is_ok());
}

#[test]
fn test_text_stats_multiline() {
    let stats = text_stats("one two\nthree four five\nsix");
    assert_eq!(stats.lines, 3);
    assert_eq!(stats.words, 6);
    assert_eq!(stats.chars, 27);
}

#[test]
fn test_longest_line_tie() {
    let result = longest_line("abc\ndef\nghi");
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn test_shortest_line_empty() {
    assert_eq!(shortest_line(""), None);
}

#[test]
fn test_sort_lines_already_sorted() {
    assert_eq!(sort_lines("a\nb\nc"), "a\nb\nc");
}

#[test]
fn test_reverse_lines_single() {
    assert_eq!(reverse_lines("only"), "only");
}

#[test]
fn test_truncate_lines_exact() {
    assert_eq!(truncate_lines("hello", 5), "hello");
}

#[test]
fn test_dedent_lines_no_indent() {
    assert_eq!(dedent_lines("hello\nworld"), "hello\nworld");
}

#[test]
fn test_to_uppercase_lines_empty() {
    assert_eq!(to_uppercase_lines(""), "");
}

#[test]
fn test_log_level_counts_empty() {
    let counts = log_level_counts(&[]);
    assert!(counts.is_empty());
}

#[test]
fn test_unique_messages_none_match() {
    let entries = parse_log("[INFO] hello");
    assert!(unique_messages(&entries, "ERROR").is_empty());
}

#[test]
fn test_filter_logs_case_insensitive() {
    let entries = parse_log("[info] msg1\n[INFO] msg2");
    let results = filter_logs(&entries, "INFO");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_log_summary_empty() {
    assert_eq!(log_summary(&[]), "");
}

#[test]
fn test_get_extension_none() {
    assert_eq!(get_extension("/"), None);
}

#[test]
fn test_replace_extension_path() {
    assert_eq!(replace_extension("/src/main.rs", "o"), "/src/main.o");
}

#[test]
fn test_is_absolute_empty() {
    assert!(!is_absolute(""));
}

#[test]
fn test_matches_pattern_exact() {
    assert!(matches_pattern("README.md", "README.md"));
    assert!(!matches_pattern("README.md", "readme.md"));
}
