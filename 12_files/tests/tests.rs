use files::*;

// ===== Topic 1: Line Processing =====

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

// ===== Topic 2: CSV Parsing =====

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
fn test_to_csv_string() {
    let csv = "name,age\nAlice,30\nBob,25";
    let (h, r) = parse_csv(csv);
    assert_eq!(to_csv_string(&h, &r), csv);
}

// ===== Topic 3: Config Parsing =====

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
fn test_validate_config() {
    let config = parse_config("host = localhost\nport = 8080");
    assert!(validate_config(&config, &["host", "port"]).is_ok());
    let missing = validate_config(&config, &["host", "port", "db"]);
    assert_eq!(missing, Err(vec!["db".to_string()]));
}

// ===== Topic 4: Text Statistics =====

#[test]
fn test_text_stats() {
    let stats = text_stats("hello world\nfoo bar baz");
    assert_eq!(stats.lines, 2);
    assert_eq!(stats.words, 5);
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

// ===== Topic 5: Text Transformations =====

#[test]
fn test_sort_lines() {
    assert_eq!(sort_lines("banana\napple\ncherry"), "apple\nbanana\ncherry");
}

#[test]
fn test_reverse_lines() {
    assert_eq!(reverse_lines("a\nb\nc"), "c\nb\na");
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

// ===== Topic 6: Log Parsing =====

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


// ===== Topic 7: Extra Practice =====

#[test]
fn test_get_extension() {
    assert_eq!(get_extension("main.rs"), Some("rs"));
    assert_eq!(get_extension("archive.tar.gz"), Some("gz"));
    assert_eq!(get_extension("Makefile"), None);
}

#[test]
fn test_get_parent() {
    assert_eq!(get_parent("/home/user/file.txt"), Some("/home/user"));
    assert_eq!(get_parent("/file.txt"), Some("/"));
}

#[test]
fn test_replace_extension() {
    assert_eq!(replace_extension("main.rs", "txt"), "main.txt");
    assert_eq!(replace_extension("Makefile", "bak"), "Makefile.bak");
}

#[test]
fn test_matches_pattern() {
    assert!(matches_pattern("main.rs", "*.rs"));
    assert!(!matches_pattern("main.rs", "*.txt"));
    assert!(matches_pattern("Makefile", "Makefile"));
}
