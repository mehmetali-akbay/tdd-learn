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
        todo!()
}

/// Return the first word of a string (up to the first space)
pub fn first_word(s: &str) -> &str {
        todo!()
}

/// Return the part of `haystack` after the first occurrence of `needle`
pub fn after_substring<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
        todo!()
}

/// Return the common prefix of two strings
pub fn common_prefix<'a>(a: &'a str, b: &str) -> &'a str {
        todo!()
}

/// Return the longest string in a slice
pub fn longest_in_slice<'a>(items: &[&'a str]) -> Option<&'a str> {
        todo!()
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
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
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
        todo!()
    }
}

// ============================================
// Topic 3: Iterators with Lifetimes
// Learn: Returning iterators over borrowed data
// ============================================

/// Split text into lines, returning borrowed slices
pub fn lines_iter(text: &str) -> Vec<&str> {
        todo!()
}

/// Return words that are longer than `min_len`
pub fn long_words(text: &str, min_len: usize) -> Vec<&str> {
        todo!()
}

/// Find all occurrences of a pattern, returning slices from the haystack
pub fn find_all_matches<'a>(haystack: &'a str, needle: &str) -> Vec<&'a str> {
        todo!()
}

/// A sentence iterator that yields sentences (split by '.')
pub struct Sentences<'a> {
    remaining: &'a str,
}

impl<'a> Sentences<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!()
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
        todo!()
}

/// A struct holding references with two different lifetimes
#[derive(Debug)]
pub struct Highlight<'text, 'tag> {
    pub text: &'text str,
    pub tag: &'tag str,
}

impl<'text, 'tag> Highlight<'text, 'tag> {
    pub fn new(text: &'text str, tag: &'tag str) -> Self {
        todo!()
    }

    pub fn render(&self) -> String {
        todo!()
    }
}

/// Merge sorted slices, Returning borrowed elements
pub fn merge_sorted<'a>(a: &[&'a str], b: &[&'a str]) -> Vec<&'a str> {
        todo!()
}

// ============================================
// Topic 5: Lifetime Elision & Static
// Learn: When lifetimes can be omitted, 'static
// ============================================

/// Return a static string based on a condition
pub fn greeting(formal: bool) -> &'static str {
        todo!()
}

/// Return the error message for an HTTP status code
pub fn status_message(code: u16) -> &'static str {
        todo!()
}

/// Demonstrates that elision works: single reference in, reference out
pub fn trim_prefix(s: &str) -> &str {
        todo!()
}

/// Another elision example: method on a struct
pub struct Config {
    data: String,
}

impl Config {
    pub fn new(data: &str) -> Self {
        todo!()
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        todo!()
    }
}

// ============================================
// Topic 6: Advanced — Lifetime Bounds on Generics
// Learn: T: 'a, where clauses with lifetimes
// ============================================

/// Find the first item matching a predicate, returning a reference
pub fn find_first<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> Option<&T> {
        todo!()
}

/// A cache that borrows its data source
#[derive(Debug)]
pub struct CachedLookup<'a> {
    data: &'a [(&'a str, i32)],
}

impl<'a> CachedLookup<'a> {
    pub fn new(data: &'a [(&'a str, i32)]) -> Self {
        todo!()
    }

    pub fn lookup(&self, key: &str) -> Option<i32> {
        todo!()
    }

    pub fn keys(&self) -> Vec<&'a str> {
        todo!()
    }

    pub fn values_above(&self, threshold: i32) -> Vec<(&'a str, i32)> {
        todo!()
    }
}

// ============================================
// Topic 7: Extra Practice
// Learn: More lifetime puzzles
// ============================================

/// Return the longer of two string slices.
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
        todo!()
}

/// Find the first word that starts with a prefix.
pub fn find_with_prefix<'a>(words: &'a [&str], prefix: &str) -> Option<&'a str> {
        todo!()
}

/// Return all strings that contain the substring.
pub fn find_containing<'a>(items: &'a [&str], sub: &str) -> Vec<&'a str> {
        todo!()
}

/// Split a string and return the first and last parts.
pub fn first_and_last_word(s: &str) -> Option<(&str, &str)> {
        todo!()
}
