// ============================================
// Level 3, Project 3: Files — File I/O & String Processing
// Learn: Reading/writing strings as "files", parsing, CSV, config, text stats, log parsing, path ops
// Note: We simulate file I/O with String processing to keep tests simple
// ============================================

use std::collections::HashMap;
use std::collections::HashSet;

// ============================================
// Topic 1: Line Processing
// Learn: split lines, trim, filter empty, enumerate, take/skip, dedup
// Reinforces: 05_vecs (iterators, collect), 06_strings (string slices), 11_iterators
// ============================================

/// Count non-empty lines.
pub fn count_lines(content: &str) -> usize {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

/// Return lines that contain a search term (case-insensitive).
pub fn grep(content: &str, pattern: &str) -> Vec<String> {
    let pat = pattern.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&pat))
        .map(|line| line.to_string())
        .collect()
}

/// Return matching lines with 1-based line numbers: vec![(1, "line text"), ...].
pub fn grep_line_numbers(content: &str, pattern: &str) -> Vec<(usize, String)> {
    let pat = pattern.to_lowercase();
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&pat))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}

/// Return lines with line numbers (1-based): "1: first line\n2: second line".
pub fn number_lines(content: &str) -> String {
    content
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{}: {}", i + 1, line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Return the first N lines.
pub fn head(content: &str, n: usize) -> String {
    content.lines().take(n).collect::<Vec<_>>().join("\n")
}

/// Return the last N lines.
pub fn tail(content: &str, n: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}

/// Remove duplicate consecutive lines.
pub fn dedup_lines(content: &str) -> String {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if result.last() != Some(&line) {
            result.push(line);
        }
    }
    result.join("\n")
}

/// Count blank (empty or whitespace-only) lines.
pub fn blank_line_count(content: &str) -> usize {
    content.lines().filter(|line| line.trim().is_empty()).count()
}

/// Count words per line, returning a Vec with one entry per line.
pub fn word_count_per_line(content: &str) -> Vec<usize> {
    content
        .lines()
        .map(|line| line.split_whitespace().count())
        .collect()
}

// ============================================
// Topic 2: CSV Parsing
// Learn: Splitting, parsing structured data, filtering, aggregating
// Reinforces: 02_structs (CsvRecord), 05_vecs (Vec), 08_results (Option)
// ============================================

/// A CSV record.
#[derive(Debug, Clone, PartialEq)]
pub struct CsvRecord {
    pub fields: Vec<String>,
}

impl CsvRecord {
    pub fn new(fields: Vec<String>) -> Self {
        CsvRecord { fields }
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.fields.get(index).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

/// Parse CSV content into records (first line is header).
pub fn parse_csv(content: &str) -> (Vec<String>, Vec<CsvRecord>) {
    let mut lines = content.lines();
    let header = match lines.next() {
        Some(h) => h.split(',').map(|s| s.trim().to_string()).collect(),
        None => return (vec![], vec![]),
    };
    let records = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| CsvRecord::new(line.split(',').map(|s| s.trim().to_string()).collect()))
        .collect();
    (header, records)
}

/// Get a column of values by header name.
pub fn get_column(headers: &[String], records: &[CsvRecord], col_name: &str) -> Vec<String> {
    let idx = headers.iter().position(|h| h == col_name);
    match idx {
        Some(i) => records
            .iter()
            .filter_map(|r| r.get(i).map(|s| s.to_string()))
            .collect(),
        None => vec![],
    }
}

/// Sum a numeric column.
pub fn sum_column(headers: &[String], records: &[CsvRecord], col_name: &str) -> Option<f64> {
    let values = get_column(headers, records, col_name);
    if values.is_empty() {
        return None;
    }
    let sum: f64 = values.iter().filter_map(|s| s.parse::<f64>().ok()).sum();
    Some(sum)
}

/// Average a numeric column.
pub fn average_column(headers: &[String], records: &[CsvRecord], col_name: &str) -> Option<f64> {
    let values = get_column(headers, records, col_name);
    let nums: Vec<f64> = values.iter().filter_map(|s| s.parse::<f64>().ok()).collect();
    if nums.is_empty() {
        return None;
    }
    Some(nums.iter().sum::<f64>() / nums.len() as f64)
}

/// Filter records where a column has a given value.
pub fn filter_csv(
    headers: &[String],
    records: &[CsvRecord],
    col_name: &str,
    value: &str,
) -> Vec<CsvRecord> {
    let idx = match headers.iter().position(|h| h == col_name) {
        Some(i) => i,
        None => return vec![],
    };
    records
        .iter()
        .filter(|r| r.get(idx) == Some(value))
        .cloned()
        .collect()
}

/// Convert records back to CSV string.
pub fn to_csv_string(headers: &[String], records: &[CsvRecord]) -> String {
    let mut result = headers.join(",");
    for record in records {
        result.push('\n');
        result.push_str(&record.fields.join(","));
    }
    result
}

/// Count total records (excluding header).
pub fn csv_record_count(content: &str) -> usize {
    let (_, records) = parse_csv(content);
    records.len()
}

// ============================================
// Topic 3: Config File Parsing
// Learn: Key=value parsing, comments, sections, typed getters, serialization
// Reinforces: 07_hashmaps (HashMap), 08_results (Result), 10_generics (parse)
// ============================================

/// Parse a simple key=value config (ignoring comments # and empty lines).
pub fn parse_config(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(pos) = trimmed.find('=') {
            let key = trimmed[..pos].trim().to_string();
            let value = trimmed[pos + 1..].trim().to_string();
            map.insert(key, value);
        }
    }
    map
}

/// Get a config value, returning a default if not found.
pub fn config_get_or<'a>(
    config: &'a HashMap<String, String>,
    key: &str,
    default: &'a str,
) -> &'a str {
    config.get(key).map(|s| s.as_str()).unwrap_or(default)
}

/// Parse a config value as an integer.
pub fn config_get_int(config: &HashMap<String, String>, key: &str) -> Option<i32> {
    config.get(key)?.parse::<i32>().ok()
}

/// Parse a config value as a float.
pub fn config_get_float(config: &HashMap<String, String>, key: &str) -> Option<f64> {
    config.get(key)?.parse::<f64>().ok()
}

/// Parse a config value as a bool ("true"/"false", case-insensitive).
pub fn config_get_bool(config: &HashMap<String, String>, key: &str) -> Option<bool> {
    let val = config.get(key)?;
    match val.to_lowercase().as_str() {
        "true" | "1" | "yes" => Some(true),
        "false" | "0" | "no" => Some(false),
        _ => None,
    }
}

/// Validate that all required keys are present.
pub fn validate_config(
    config: &HashMap<String, String>,
    required: &[&str],
) -> Result<(), Vec<String>> {
    let missing: Vec<String> = required
        .iter()
        .filter(|&&key| !config.contains_key(key))
        .map(|&key| key.to_string())
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

/// Merge two configs (second overwrites first on conflict).
pub fn merge_configs(
    base: &HashMap<String, String>,
    overrides: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged = base.clone();
    for (k, v) in overrides {
        merged.insert(k.clone(), v.clone());
    }
    merged
}

/// Serialize a config back to key=value string (sorted by key).
pub fn serialize_config(config: &HashMap<String, String>) -> String {
    let mut entries: Vec<_> = config.iter().collect();
    entries.sort_by_key(|(k, _)| (*k).clone());
    entries
        .iter()
        .map(|(k, v)| format!("{} = {}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

// ============================================
// Topic 4: Text Statistics
// Learn: Counting, frequency analysis, aggregation
// Reinforces: 02_structs (TextStats), 07_hashmaps (frequency), 11_lifetimes (refs)
// ============================================

/// Text statistics.
#[derive(Debug, PartialEq)]
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

/// Compute text statistics (like wc).
pub fn text_stats(content: &str) -> TextStats {
    TextStats {
        lines: content.lines().count(),
        words: content.split_whitespace().count(),
        chars: content.chars().count(),
        bytes: content.len(),
    }
}

/// Find the longest line.
pub fn longest_line(content: &str) -> Option<String> {
    content
        .lines()
        .max_by_key(|line| line.len())
        .map(|s| s.to_string())
}

/// Find the shortest non-empty line.
pub fn shortest_line(content: &str) -> Option<String> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .min_by_key(|line| line.len())
        .map(|s| s.to_string())
}

/// Average word length in the text.
pub fn average_word_length(content: &str) -> f64 {
    let words: Vec<&str> = content.split_whitespace().collect();
    if words.is_empty() {
        return 0.0;
    }
    let total_len: usize = words.iter().map(|w| w.len()).sum();
    total_len as f64 / words.len() as f64
}

/// Word frequency map (lowercased).
pub fn word_frequency(content: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in content.split_whitespace() {
        let lower = word.to_lowercase();
        *freq.entry(lower).or_insert(0) += 1;
    }
    freq
}

/// Most common word (lowercased). Returns None if empty.
pub fn most_common_word(content: &str) -> Option<String> {
    let freq = word_frequency(content);
    freq.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, _)| word)
}

/// Count of unique words (case-insensitive).
pub fn unique_word_count(content: &str) -> usize {
    let words: HashSet<String> = content
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();
    words.len()
}

// ============================================
// Topic 5: Text Transformations
// Learn: Line-by-line transforms, sorting, reversing, wrapping
// Reinforces: 06_strings (String ops), 05_vecs (collect), 11_iterators
// ============================================

/// Sort lines alphabetically.
pub fn sort_lines(content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    lines.sort();
    lines.join("\n")
}

/// Reverse the order of lines.
pub fn reverse_lines(content: &str) -> String {
    let lines: Vec<&str> = content.lines().rev().collect();
    lines.join("\n")
}

/// Indent every line by a given number of spaces.
pub fn indent_lines(content: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    content
        .lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Truncate each line to max_width characters, add "..." if truncated.
pub fn truncate_lines(content: &str, max_width: usize) -> String {
    content
        .lines()
        .map(|line| {
            if line.len() > max_width {
                format!("{}...", &line[..max_width])
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Remove common leading whitespace from all lines (dedent).
pub fn dedent_lines(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line.trim()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Convert all lines to uppercase.
pub fn to_uppercase_lines(content: &str) -> String {
    content
        .lines()
        .map(|line| line.to_uppercase())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Center each line within a given width, padding with spaces.
pub fn center_lines(content: &str, width: usize) -> String {
    content
        .lines()
        .map(|line| {
            if line.len() >= width {
                line.to_string()
            } else {
                let padding = width - line.len();
                let left = padding / 2;
                let right = padding - left;
                format!("{}{}{}", " ".repeat(left), line, " ".repeat(right))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ============================================
// Topic 6: Log Parsing
// Learn: Complex multi-field parsing, filtering, aggregation
// Reinforces: 02_structs (LogEntry), 03_patterns (match/if-let), 07_hashmaps
// ============================================

/// A parsed log entry.
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
}

/// Format a log entry back to "[LEVEL] message".
pub fn format_log_entry(entry: &LogEntry) -> String {
    format!("[{}] {}", entry.level, entry.message)
}

/// Parse log lines in format "[LEVEL] message".
pub fn parse_log(content: &str) -> Vec<LogEntry> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('[')
                && let Some(end) = trimmed.find(']')
            {
                let level = trimmed[1..end].to_string();
                let message = trimmed[end + 1..].trim().to_string();
                return Some(LogEntry { level, message });
            }
            None
        })
        .collect()
}

/// Filter log entries by level.
pub fn filter_logs(entries: &[LogEntry], level: &str) -> Vec<LogEntry> {
    entries
        .iter()
        .filter(|e| e.level.eq_ignore_ascii_case(level))
        .cloned()
        .collect()
}

/// Count entries per log level.
pub fn log_level_counts(entries: &[LogEntry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        *counts.entry(entry.level.to_uppercase()).or_insert(0) += 1;
    }
    counts
}

/// Find all unique messages for a given level.
pub fn unique_messages(entries: &[LogEntry], level: &str) -> Vec<String> {
    let mut seen = Vec::new();
    for entry in entries {
        if entry.level.eq_ignore_ascii_case(level) && !seen.contains(&entry.message) {
            seen.push(entry.message.clone());
        }
    }
    seen
}

/// Search logs for entries whose message contains a pattern (case-insensitive).
pub fn search_logs(entries: &[LogEntry], pattern: &str) -> Vec<LogEntry> {
    let pat = pattern.to_lowercase();
    entries
        .iter()
        .filter(|e| e.message.to_lowercase().contains(&pat))
        .cloned()
        .collect()
}

/// Produce a log summary: "INFO: 3, ERROR: 2, WARN: 1" (sorted by level).
pub fn log_summary(entries: &[LogEntry]) -> String {
    let counts = log_level_counts(entries);
    let mut pairs: Vec<_> = counts.into_iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
        .iter()
        .map(|(level, count)| format!("{}: {}", level, count))
        .collect::<Vec<_>>()
        .join(", ")
}

// ============================================
// Topic 7: Path String Operations
// Learn: String-based path manipulation (no actual filesystem)
// Reinforces: 08_results (Option), 06_strings (split/join), 03_patterns (match)
// ============================================

/// Extract the file extension from a path string.
pub fn get_extension(path: &str) -> Option<&str> {
    let filename = get_filename(path)?;
    filename.rsplit_once('.').map(|(_, ext)| ext)
}

/// Return the parent directory from a path string.
pub fn get_parent(path: &str) -> Option<&str> {
    path.rsplit_once('/').map(|(parent, _)| {
        if parent.is_empty() {
            "/"
        } else {
            parent
        }
    })
}

/// Return the filename (last component) from a path.
pub fn get_filename(path: &str) -> Option<&str> {
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.rsplit_once('/').map_or(trimmed, |(_, name)| name))
}

/// Return the file stem (filename without last extension).
pub fn get_stem(path: &str) -> Option<&str> {
    let filename = get_filename(path)?;
    match filename.rsplit_once('.') {
        Some((stem, _)) if !stem.is_empty() => Some(stem),
        _ => Some(filename),
    }
}

/// Replace the extension of a path.
pub fn replace_extension(path: &str, new_ext: &str) -> String {
    match path.rsplit_once('.') {
        Some((base, _)) => format!("{}.{}", base, new_ext),
        None => format!("{}.{}", path, new_ext),
    }
}

/// Join two path segments with '/'.
pub fn join_path(base: &str, child: &str) -> String {
    if child.starts_with('/') {
        return child.to_string();
    }
    let base = base.trim_end_matches('/');
    if base.is_empty() {
        return child.to_string();
    }
    format!("{}/{}", base, child)
}

/// Check if a path is absolute (starts with /).
pub fn is_absolute(path: &str) -> bool {
    path.starts_with('/')
}

/// Check if a filename matches a simple pattern (e.g., "*.rs").
pub fn matches_pattern(filename: &str, pattern: &str) -> bool {
    if let Some(ext_pattern) = pattern.strip_prefix("*.") {
        filename.ends_with(&format!(".{}", ext_pattern))
    } else {
        filename == pattern
    }
}

/// Count the depth (number of components) in a path.
pub fn path_depth(path: &str) -> usize {
    path.split('/')
        .filter(|s| !s.is_empty() && *s != ".")
        .count()
}
