// ============================================
// Topic 1: String Basics
// Learn: Creating strings, appending, basic ops
// ============================================

/// Create and return a new empty String
pub fn create_empty_string() -> String {
    String::from("")
}

/// Create a String from the literal "hello, rust!"
pub fn create_greeting() -> String {
    String::from("hello, rust!")
}

/// Append the suffix to the base string and return a new String
/// Example: append("hello", " world") => "hello world"
pub fn append(base: &str, suffix: &str) -> String {
    let mut base = String::from(base);
    base.push_str(suffix);
    base
}

/// Push the character '!' to the end of the string and return it
/// Example: add_excitement("wow") => "wow!"
pub fn add_excitement(s: &str) -> String {
    let mut result = String::from(s);
    result.push('!');
    result
}

/// Return the length (in bytes) of the given string
/// Note: for ASCII strings, bytes == characters
pub fn byte_length(s: &str) -> usize {
    s.len()
}

/// Return the number of characters in the string
/// Hint: this differs from byte_length for multi-byte characters (e.g. emoji)
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Return true if the string is empty
pub fn is_blank(s: &str) -> bool {
    if s.is_empty() { true } else { false }
}

// ============================================
// Topic 2: String Slices & Searching
// Learn: &str, slicing, searching within strings
// ============================================

/// Return the first `n` characters of the string
/// If n > string length, return the whole string
pub fn first_n_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// Return the last `n` characters of the string
/// If n > string length, return the whole string
pub fn last_n_chars(s: &str, n: usize) -> String {
    let length = s.chars().count();
    let final_length = length.saturating_sub(n);
    s.chars().skip(final_length).collect()
}

/// Return true if the string contains the given substring
pub fn contains_substring(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

/// Return true if the string starts with the given prefix
pub fn has_prefix(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// Return true if the string ends with the given suffix
pub fn has_suffix(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Find the byte index of the first occurrence of `target` in `s`
/// Return None if not found
pub fn find_position(s: &str, target: &str) -> Option<usize> {
    s.find(target)
}

// ============================================
// Topic 3: String Transformation Methods
// Learn: trim, case, replace, split, repeat
// ============================================

/// Remove leading and trailing whitespace
pub fn clean_whitespace(s: &str) -> String {
    s.trim().to_string()
}

/// Convert the string to UPPERCASE
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

/// Convert the string to lowercase
pub fn whisper(s: &str) -> String {
    s.to_lowercase()
}

/// Replace all occurrences of `from` with `to` in the string
pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// Split the string by the given delimiter and return a Vec of Strings
pub fn split_by(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter).map(String::from).collect()
}

/// Repeat the string `n` times
/// Example: repeat_string("ha", 3) => "hahaha"
pub fn repeat_string(s: &str, n: usize) -> String {
    s.repeat(n)
}

// ============================================
// Topic 4: String Conversion & Parsing
// Learn: parse, to_string, format!
// ============================================

/// Convert an integer to its string representation
pub fn int_to_string(n: i32) -> String {
    n.to_string()
}

/// Parse a string into an i32, return None if parsing fails
pub fn parse_int(s: &str) -> Option<i32> {
    let result = s.parse().ok();
    result
}

/// Parse a string into an f64, return None if parsing fails
pub fn parse_float(s: &str) -> Option<f64> {
    let result: Option<f64> = s.parse().ok();
    result
}

/// Format a greeting: "Hello, {name}! You are {age} years old."
pub fn format_greeting(name: &str, age: u32) -> String {
    format!("Hello, {name}! You are {age} years old.")
}

/// Convert a boolean to "yes" or "no"
pub fn bool_to_word(b: bool) -> String {
    if b { "yes" } else { "no" }.to_string()
}

// ============================================
// Topic 5: String Building & Advanced
// Learn: join, reverse, count, acronyms, palindromes
// ============================================

/// Join a slice of string slices with a comma and space
/// Example: ["a", "b", "c"] => "a, b, c"
pub fn comma_join(parts: &[&str]) -> String {
    parts.join(", ")
}

/// Reverse a string
/// Example: "hello" => "olleh"
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Count how many times `target` character appears in the string
pub fn count_char(s: &str, target: char) -> usize {
    s.chars().filter(|&c| c == target).count()
}

/// Extract all words from a sentence (split by whitespace)
pub fn extract_words(s: &str) -> Vec<String> {
    s.trim().split_whitespace().map(|e| e.to_string()).collect()
}

/// Create an acronym from a phrase
/// Example: "Test Driven Development" => "TDD"
pub fn make_acronym(phrase: &str) -> String {
    phrase
        .trim()
        .split_whitespace()
        .filter_map(|e| e.chars().next())
        .map(|e| e.to_uppercase().to_string())
        .collect()
}

/// Check if a string is a palindrome (case-insensitive)
/// Example: "Racecar" => true, "hello" => false
pub fn is_palindrome(s: &str) -> bool {
    let lower = s.to_lowercase();
    let rev: String = s.to_lowercase().chars().rev().collect();
    lower == rev
}

/// Capitalize the first letter of each word
/// Example: "hello world" => "Hello World"
pub fn title_case(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .split_whitespace()
        .map(|e| {
            let mut ch = e.chars();
            if let Some(first_ch) = ch.next() {
                let remain = ch.as_str();
                return format!("{}{}",first_ch.to_uppercase(), remain);
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// ============================================
// Topic 6: Advanced String Challenges
// Learn: Encoding, validation, algorithms
// ============================================

/// Caesar cipher: shift each letter by `shift` positions in the alphabet
/// Wraps around: 'z' shifted by 1 => 'a'. Only shifts letters, keeps other chars.
/// Example: caesar_cipher("hello", 3) => "khoor"
pub fn caesar_cipher(s: &str, shift: u8) -> String {
    todo!()
}

/// Run-length encoding: compress consecutive repeated characters
/// Example: "aaabbc" => "3a2b1c"
pub fn run_length_encode(s: &str) -> String {
    todo!()
}

/// Run-length decoding: decompress an encoded string
/// Example: "3a2b1c" => "aaabbc"
pub fn run_length_decode(s: &str) -> String {
    todo!()
}

/// Count vowels and consonants, return (vowels, consonants)
/// Only count ASCII letters
pub fn count_vowels_consonants(s: &str) -> (usize, usize) {
    todo!()
}

/// Return the longest word in a sentence
/// If multiple words have the same max length, return the first one
pub fn longest_word(s: &str) -> String {
    todo!()
}

/// Remove consecutive duplicate characters
/// Example: "aabbccaab" => "abcab"
pub fn remove_consecutive_dupes(s: &str) -> String {
    todo!()
}

/// Interleave two strings character by character
/// Example: interleave("abc", "123") => "a1b2c3"
/// If one is longer, append remaining chars
pub fn interleave(a: &str, b: &str) -> String {
    todo!()
}

/// Check if two strings are anagrams (same letters, different order)
/// Case-insensitive, ignores spaces
pub fn are_anagrams(a: &str, b: &str) -> bool {
    todo!()
}

/// Pig Latin: move first consonant cluster to end and add "ay"
/// If starts with vowel, just add "hay" to the end
/// Example: "hello" => "ellohay", "apple" => "applehay", "string" => "ingstray"
pub fn pig_latin(word: &str) -> String {
    todo!()
}

/// Validate a simple email: must have exactly one '@', at least one '.' after '@',
/// non-empty parts before '@' and after '.'
pub fn is_valid_email(email: &str) -> bool {
    todo!()
}

/// Truncate a string to `max_len` chars, adding "..." if truncated
/// Example: truncate("hello world", 5) => "hello..."
/// If string is shorter or equal to max_len, return as-is
pub fn truncate(s: &str, max_len: usize) -> String {
    todo!()
}

/// Convert a camelCase or PascalCase string to snake_case
/// Example: "HelloWorld" => "hello_world", "helloWorld" => "hello_world"
pub fn to_snake_case(s: &str) -> String {
    todo!()
}

// ============================================
// Topic 7: Advanced String Manipulation
// Learn: In-place mutation, split_once, lines, trim_matches, char_indices
// ============================================

/// Remove the last character from the string and return it.
/// Returns None if the string is empty.
pub fn remove_last_char(s: &mut String) -> Option<char> {
    todo!()
}

/// Split the string at the first occurrence of the delimiter.
/// Returns (before, after) if the delimiter is found, otherwise None.
pub fn split_once_at<'a>(s: &'a str, delim: char) -> Option<(&'a str, &'a str)> {
    todo!()
}

/// Count the number of non-empty lines in a string.
pub fn count_non_empty_lines(s: &str) -> usize {
    todo!()
}

/// Remove surrounding single or double quotes from the string.
pub fn remove_surrounding_quotes(s: &str) -> &str {
    todo!()
}

/// Return the byte index of the character at `char_index`.
/// Emojis and special characters can take multiple bytes, meaning char_index != byte_index.
pub fn find_char_byte_index(s: &str, char_index: usize) -> Option<usize> {
    todo!()
}

/// Clear the string contents without reallocating its capacity.
pub fn clear_string(s: &mut String) {
    todo!()
}

// ============================================
// Topic 8: Extra Practice
// Learn: More string algorithms for additional practice
// ============================================

/// Check if a string is a pangram (contains every letter a-z at least once).
pub fn is_pangram(text: &str) -> bool {
    todo!()
}

/// Count distinct words in a string (case-insensitive).
pub fn count_distinct_words(s: &str) -> usize {
    todo!()
}

/// Find the most common character in a string (excluding spaces).
/// Returns None for empty strings.
pub fn most_common_char(s: &str) -> Option<char> {
    todo!()
}

/// Wrap text at `width` characters, inserting newlines.
pub fn word_wrap(text: &str, width: usize) -> String {
    todo!()
}
