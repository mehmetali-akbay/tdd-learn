// ============================================
// Level 4, Project 3: Lifetimes — References & Borrowing
// Learn: Lifetime annotations, structs with refs, lifetime bounds
// ============================================

// ============================================
// Topic 1: Lifetime Basics — Function Annotations
// Learn: 'a syntax, input/output lifetime relationships
// ============================================

/// Return the longer of two string slices
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// Return the first word of a string (up to the first space)
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

/// Return the part of `haystack` after the first occurrence of `needle`
pub fn after_substring<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    haystack
        .find(needle)
        .map(|pos| &haystack[pos + needle.len()..])
}

/// Return the common prefix of two strings
pub fn common_prefix<'a>(a: &'a str, b: &str) -> &'a str {
    let len = a
        .chars()
        .zip(b.chars())
        .take_while(|(ca, cb)| ca == cb)
        .count();
    &a[..a.chars().take(len).map(|c| c.len_utf8()).sum::<usize>()]
}

/// Return the longest string in a slice
pub fn longest_in_slice<'a>(items: &[&'a str]) -> Option<&'a str> {
    items.iter().max_by_key(|s| s.len()).copied()
}

// ============================================
// Topic 2: Structs with Lifetimes
// Learn: Structs holding references, lifetime in impl blocks
// ============================================

/// A text excerpt holding a reference to the original text
#[derive(Debug, PartialEq)]
pub struct Excerpt<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a> Excerpt<'a> {
    pub fn new(source: &'a str, start: usize, end: usize) -> Option<Self> {
        if start <= end && end <= source.len() {
            Some(Excerpt {
                text: &source[start..end],
                start,
                end,
            })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

/// A key-value pair holding references
#[derive(Debug, PartialEq)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> KeyValue<'a> {
    /// Parse "key=value" into a KeyValue
    pub fn parse(input: &'a str) -> Option<Self> {
        let eq_pos = input.find('=')?;
        Some(KeyValue {
            key: input[..eq_pos].trim(),
            value: input[eq_pos + 1..].trim(),
        })
    }
}

// ============================================
// Topic 3: Iterators with Lifetimes
// Learn: Returning iterators over borrowed data
// ============================================

/// Split text into lines, returning borrowed slices
pub fn lines_iter(text: &str) -> Vec<&str> {
    text.lines().collect()
}

/// Return words that are longer than `min_len`
pub fn long_words(text: &str, min_len: usize) -> Vec<&str> {
    text.split_whitespace()
        .filter(|w| w.len() > min_len)
        .collect()
}

/// Find all occurrences of a pattern, returning slices from the haystack
pub fn find_all_matches<'a>(haystack: &'a str, needle: &str) -> Vec<&'a str> {
    let needle_len = needle.len();
    if needle_len == 0 {
        return vec![];
    }
    let mut results = Vec::new();
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs_pos = start + pos;
        results.push(&haystack[abs_pos..abs_pos + needle_len]);
        start = abs_pos + 1;
    }
    results
}

/// A sentence iterator that yields sentences (split by '.')
pub struct Sentences<'a> {
    remaining: &'a str,
}

impl<'a> Sentences<'a> {
    pub fn new(text: &'a str) -> Self {
        Sentences { remaining: text }
    }
}

impl<'a> Iterator for Sentences<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining.trim_start();
        if remaining.is_empty() {
            return None;
        }
        match remaining.find('.') {
            Some(pos) => {
                let sentence = remaining[..pos].trim();
                self.remaining = &remaining[pos + 1..];
                if sentence.is_empty() {
                    self.next()
                } else {
                    Some(sentence)
                }
            }
            None => {
                let sentence = remaining.trim();
                self.remaining = "";
                if sentence.is_empty() {
                    None
                } else {
                    Some(sentence)
                }
            }
        }
    }
}

// ============================================
// Topic 4: Multiple Lifetimes
// Learn: Different lifetimes for different parameters
// ============================================

/// Return elements from `items` that contain `pattern`
pub fn filter_containing<'a>(items: &[&'a str], pattern: &str) -> Vec<&'a str> {
    items
        .iter()
        .filter(|&&s| s.contains(pattern))
        .copied()
        .collect()
}

/// A struct holding references with two different lifetimes
#[derive(Debug)]
pub struct Highlight<'text, 'tag> {
    pub text: &'text str,
    pub tag: &'tag str,
}

impl<'text, 'tag> Highlight<'text, 'tag> {
    pub fn new(text: &'text str, tag: &'tag str) -> Self {
        Highlight { text, tag }
    }

    pub fn render(&self) -> String {
        format!("<{}>{}</{}>", self.tag, self.text, self.tag)
    }
}

/// Merge sorted slices, Returning borrowed elements
pub fn merge_sorted<'a>(a: &[&'a str], b: &[&'a str]) -> Vec<&'a str> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

// ============================================
// Topic 5: Lifetime Elision & Static
// Learn: When lifetimes can be omitted, 'static
// ============================================

/// Return a static string based on a condition
pub fn greeting(formal: bool) -> &'static str {
    if formal {
        "Good day"
    } else {
        "Hey"
    }
}

/// Return the error message for an HTTP status code
pub fn status_message(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

/// Demonstrates that elision works: single reference in, reference out
pub fn trim_prefix(s: &str) -> &str {
    s.trim_start_matches(|c: char| !c.is_alphanumeric())
}

/// Another elision example: method on a struct
pub struct Config {
    data: String,
}

impl Config {
    pub fn new(data: &str) -> Self {
        Config {
            data: data.to_string(),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        for line in self.data.lines() {
            if let Some(rest) = line.strip_prefix(key) {
                if let Some(value) = rest.strip_prefix('=') {
                    return Some(value.trim());
                }
            }
        }
        None
    }
}

// ============================================
// Topic 6: Advanced — Lifetime Bounds on Generics
// Learn: T: 'a, where clauses with lifetimes
// ============================================

/// Find the first item matching a predicate, returning a reference
pub fn find_first<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> Option<&T> {
    items.iter().find(|item| predicate(item))
}

/// A cache that borrows its data source
#[derive(Debug)]
pub struct CachedLookup<'a> {
    data: &'a [(&'a str, i32)],
}

impl<'a> CachedLookup<'a> {
    pub fn new(data: &'a [(&'a str, i32)]) -> Self {
        CachedLookup { data }
    }

    pub fn lookup(&self, key: &str) -> Option<i32> {
        self.data.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
    }

    pub fn keys(&self) -> Vec<&'a str> {
        self.data.iter().map(|(k, _)| *k).collect()
    }

    pub fn values_above(&self, threshold: i32) -> Vec<(&'a str, i32)> {
        self.data
            .iter()
            .filter(|(_, v)| *v > threshold)
            .copied()
            .collect()
    }
}

// ============================================
// Topic 7: Extra Practice
// Learn: More lifetime puzzles
// ============================================

/// Return the longer of two string slices.
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// Find the first word that starts with a prefix.
pub fn find_with_prefix<'a>(words: &'a [&str], prefix: &str) -> Option<&'a str> {
    words.iter().find(|w| w.starts_with(prefix)).copied()
}

/// Return all strings that contain the substring.
pub fn find_containing<'a>(items: &'a [&str], sub: &str) -> Vec<&'a str> {
    items.iter().filter(|s| s.contains(sub)).copied().collect()
}

/// Split a string and return the first and last parts.
pub fn first_and_last_word(s: &str) -> Option<(&str, &str)> {
    let mut words = s.split_whitespace();
    let first = words.next()?;
    let last = words.last().unwrap_or(first);
    Some((first, last))
}
