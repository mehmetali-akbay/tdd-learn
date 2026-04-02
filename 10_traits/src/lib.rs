// ============================================
// Module 9: Traits — Defining & Implementing Traits
// Learn: Trait definition, impl, default methods, operator overloading,
//        From/Into, trait bounds, Display, impl Trait, where clauses
// Reinforces: ownership, borrowing, structs, enums, pattern matching,
//             Vec, String, HashMap, Result
// ============================================

use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Neg, Sub};

// ============================================
// Topic 1: Basic Traits — Defining & Implementing
// Learn: Define a trait, implement for multiple types, &dyn Trait
// Reinforces: structs with owned fields, Vec<String>, Option, slices
// ============================================

/// A trait for things that can describe themselves
pub trait Describable {
    fn describe(&self) -> String;
}

/// A pet with name and species
#[derive(Debug, Clone)]
pub struct Pet {
    pub name: String,
    pub species: String,
}

impl Pet {
    pub fn new(name: &str, species: &str) -> Self {
        Self {
            name: name.to_string(),
            species: species.to_string(),
        }
    }
}

impl Describable for Pet {
    /// Return "{name} the {species}"
    fn describe(&self) -> String {
        format!("{} the {}", self.name, self.species)
    }
}

/// A car with make, model, and year
#[derive(Debug, Clone)]
pub struct Car {
    pub make: String,
    pub model: String,
    pub year: u32,
}

impl Car {
    pub fn new(make: &str, model: &str, year: u32) -> Self {
        Self {
            make: make.to_string(),
            model: model.to_string(),
            year,
        }
    }

    /// Check if the car is vintage (before 1990)
    /// Reinforces: simple method, comparison
    pub fn is_vintage(&self) -> bool {
        self.year < 1990
    }
}

impl Describable for Car {
    /// Return "{year} {make} {model}"
    fn describe(&self) -> String {
        format!("{} {} {}", self.year, self.make, self.model)
    }
}

/// A book with title, author, and pages
#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, pages: u32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }
}

impl Describable for Book {
    /// Return "\"<title>\" by <author> (<pages> pages)"
    fn describe(&self) -> String {
        format!(
            "\"{}\" by {} ({} pages)",
            self.title, self.author, self.pages
        )
    }
}

/// A generic function that takes anything Describable (trait object parameter)
pub fn print_description(item: &dyn Describable) -> String {
    item.describe()
}

/// Collect descriptions from a slice of trait objects into a Vec
/// Reinforces: slice borrowing, Vec collection, iteration
pub fn describe_all(items: &[&dyn Describable]) -> Vec<String> {
    items.iter().map(|e| e.describe()).collect()
}

/// Find the longest description among a slice of Describable items
/// Reinforces: Option<String>, iteration, max_by_key
pub fn longest_description(items: &[&dyn Describable]) -> Option<String> {
    items.iter().map(|e| e.describe()).max_by_key(|e| e.len())
}

// ============================================
// Topic 2: Display & Debug — Standard Library Traits
// Learn: Implementing fmt::Display, derive(Debug), formatted output
// Reinforces: enums with match, borrowing, arithmetic
// ============================================

/// A 2D coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Self {
        Coord { x, y }
    }

    /// Calculate the Euclidean distance to another coordinate
    /// Reinforces: borrowing (&self, &Coord), arithmetic
    pub fn distance_to(&self, other: &Coord) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl fmt::Display for Coord {
    /// Display as "(x, y)"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})",self.x,self.y)
    }
}

/// A temperature unit enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
}

/// A temperature with value and unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {
    pub value: f64,
    pub unit: TempUnit,
}

impl Temperature {
    pub fn celsius(value: f64) -> Self {
        Temperature {
            value,
            unit: TempUnit::Celsius,
        }
    }

    pub fn fahrenheit(value: f64) -> Self {
        Temperature {
            value,
            unit: TempUnit::Fahrenheit,
        }
    }

    /// Convert to Celsius (returns a new Temperature)
    /// Reinforces: match on enum, arithmetic
    /// Formula: (F - 32) * 5/9
    pub fn to_celsius(&self) -> Temperature {
        todo!()
    }

    /// Convert to Fahrenheit (returns a new Temperature)
    /// Reinforces: match on enum, arithmetic
    /// Formula: C * 9/5 + 32
    pub fn to_fahrenheit(&self) -> Temperature {
        todo!()
    }
}

impl fmt::Display for Temperature {
    /// Display as "{value:.1}°C" or "{value:.1}°F"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// An RGB color
/// Reinforces: struct with methods, formatted output
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Check if the color is grayscale (r == g == b)
    pub fn is_grayscale(&self) -> bool {
        todo!()
    }

    /// Calculate brightness using the weighted formula (0.299R + 0.587G + 0.114B)
    /// Reinforces: arithmetic, f64 casting
    pub fn brightness(&self) -> f64 {
        todo!()
    }
}

impl fmt::Display for Color {
    /// Display as "#RRGGBB" in uppercase hex (e.g. "#FF8000")
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// ============================================
// Topic 3: Default Trait & Builder Pattern
// Learn: Implementing Default, builder-like patterns with consuming self
// Reinforces: struct construction, String ownership, method chaining
// ============================================

/// Application configuration with defaults
/// Defaults: host="localhost", port=8080, debug=false, max_connections=100
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub max_connections: u32,
}

impl Default for Config {
    fn default() -> Self {
        todo!()
    }
}

impl Config {
    pub fn with_host(mut self, host: &str) -> Self {
        todo!()
    }

    pub fn with_port(mut self, port: u16) -> Self {
        todo!()
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        todo!()
    }

    pub fn with_max_connections(mut self, max: u32) -> Self {
        todo!()
    }

    /// Generate a human-readable summary of the config
    /// Format: "{host}:{port} ({mode}, max_conn={max_connections})"
    /// where mode is "debug" if debug is true, "release" otherwise
    /// Reinforces: String formatting, conditional logic
    pub fn summary(&self) -> String {
        todo!()
    }

    /// Check if this config uses all default values
    /// Reinforces: PartialEq comparison
    pub fn is_default(&self) -> bool {
        todo!()
    }
}

/// Game settings with defaults
/// Defaults: difficulty="normal", sound=true, fullscreen=false
#[derive(Debug, Clone, PartialEq)]
pub struct GameSettings {
    pub difficulty: String,
    pub sound: bool,
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        todo!()
    }
}

impl GameSettings {
    /// Builder method to set difficulty
    /// Reinforces: consuming self pattern, String ownership
    pub fn with_difficulty(mut self, difficulty: &str) -> Self {
        todo!()
    }

    /// Builder method to toggle sound
    pub fn with_sound(mut self, sound: bool) -> Self {
        todo!()
    }

    /// Builder method to toggle fullscreen
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        todo!()
    }
}

// ============================================
// Topic 4: Operator Overloading — PartialEq, PartialOrd, Add
// Learn: Implementing operator traits, derive vs manual impl
// Reinforces: structs, arithmetic, pattern matching, Display
// ============================================

/// A 2D vector that supports arithmetic operations
#[derive(Debug, Clone, Copy)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }

    /// Calculate the magnitude (length) of the vector
    pub fn magnitude(&self) -> f64 {
        todo!()
    }

    /// Calculate the dot product with another vector
    /// Reinforces: borrowing, arithmetic
    pub fn dot(&self, other: &Vec2d) -> f64 {
        todo!()
    }
}

impl Add for Vec2d {
    type Output = Vec2d;
    fn add(self, rhs: Vec2d) -> Vec2d {
        todo!()
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;
    fn sub(self, rhs: Vec2d) -> Vec2d {
        todo!()
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;
    fn neg(self) -> Vec2d {
        todo!()
    }
}

impl PartialEq for Vec2d {
    /// Compare with epsilon tolerance (1e-9)
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl fmt::Display for Vec2d {
    /// Display as "(x, y)"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// A monetary amount in cents (avoids floating point issues)
/// The inner value is cents: Money(575) = $5.75
/// Reinforces: tuple struct, arithmetic, Display formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(pub i64);

impl Money {
    /// Create from dollars and cents: Money::new(5, 75) = Money(575)
    pub fn new(dollars: i64, cents: i64) -> Self {
        todo!()
    }

    /// Get the dollar part
    pub fn dollars(&self) -> i64 {
        todo!()
    }

    /// Get the cents part (always 0-99)
    pub fn cents(&self) -> i64 {
        todo!()
    }
}

impl Add for Money {
    type Output = Money;
    fn add(self, rhs: Money) -> Money {
        todo!()
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, rhs: Money) -> Money {
        todo!()
    }
}

impl fmt::Display for Money {
    /// Display as "$D.CC" for positive, "-$D.CC" for negative
    /// E.g. Money(575) -> "$5.75", Money(-200) -> "-$2.00"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// ============================================
// Topic 5: From / Into Conversions
// Learn: Implementing From trait, automatic Into, type conversions
// Reinforces: tuples, enums, pattern matching, slices, Vec operations
// ============================================

/// Celsius temperature value (tuple struct)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

/// Fahrenheit temperature value (tuple struct)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Celsius> for Fahrenheit {
    /// Formula: C * 9/5 + 32
    fn from(c: Celsius) -> Self {
        todo!()
    }
}

impl From<Fahrenheit> for Celsius {
    /// Formula: (F - 32) * 5/9
    fn from(f: Fahrenheit) -> Self {
        todo!()
    }
}

/// A URL-friendly slug
#[derive(Debug, Clone, PartialEq)]
pub struct Slug(pub String);

impl From<&str> for Slug {
    /// Convert: trim, lowercase, replace spaces with hyphens
    fn from(s: &str) -> Self {
        todo!()
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// An RGB color struct for From/Into conversions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        todo!()
    }
}

impl From<Rgb> for (u8, u8, u8) {
    fn from(c: Rgb) -> Self {
        todo!()
    }
}

/// Convert an array [f64; 2] into a Coord
/// Reinforces: arrays, From for existing types
impl From<[f64; 2]> for Coord {
    fn from(arr: [f64; 2]) -> Self {
        todo!()
    }
}

/// Convert a (title, author, pages) tuple into a Book
/// Reinforces: tuples, destructuring, String ownership
impl From<(&str, &str, u32)> for Book {
    fn from((title, author, pages): (&str, &str, u32)) -> Self {
        todo!()
    }
}

/// Statistics computed from a slice of numbers
/// Reinforces: slices, Vec operations (min, max, sum, len)
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub count: usize,
    pub sum: i64,
    pub min: i64,
    pub max: i64,
}

impl Stats {
    /// Calculate the average as f64 (return 0.0 if count is 0)
    pub fn average(&self) -> f64 {
        todo!()
    }
}

impl From<&[i32]> for Stats {
    /// Build Stats from a slice. For empty slices, all fields are 0.
    fn from(slice: &[i32]) -> Self {
        todo!()
    }
}

/// A priority level enum with From<&str> conversion
/// Reinforces: enums, match on string slices, default handling
#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl From<&str> for Priority {
    /// "low" -> Low, "medium"/"med" -> Medium, "high" -> High,
    /// "critical"/"crit" -> Critical, anything else -> Medium (default)
    /// Case-insensitive
    fn from(s: &str) -> Self {
        todo!()
    }
}

impl fmt::Display for Priority {
    /// Display as lowercase: "low", "medium", "high", "critical"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// ============================================
// Topic 6: Trait Bounds & Generic Functions
// Learn: Generic functions with trait bounds, where clauses,
//        multiple bounds with +, impl Trait syntax
// Reinforces: slices, Vec, Option, generic programming
// ============================================

/// Format all items using Display, joined by ", "
pub fn format_all<T: fmt::Display>(items: &[T]) -> String {
    todo!()
}

/// Find the largest item in a slice (requires PartialOrd + Copy)
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    todo!()
}

/// Find the smallest item in a slice (requires PartialOrd + Copy)
/// Reinforces: mirrors largest, solidifies trait bound understanding
pub fn smallest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    todo!()
}

/// Find the first item matching a target (requires PartialEq)
/// Return Some(index) or None
pub fn find_match<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    todo!()
}

/// Count items matching a predicate
pub fn count_where<T>(items: &[T], predicate: fn(&T) -> bool) -> usize {
    todo!()
}

/// Check if all items satisfy a predicate
pub fn all_match<T>(items: &[T], predicate: fn(&T) -> bool) -> bool {
    todo!()
}

/// Check if any item satisfies a predicate
/// Reinforces: mirrors all_match
pub fn any_match<T>(items: &[T], predicate: fn(&T) -> bool) -> bool {
    todo!()
}

/// Sort and deduplicate a slice, returning a new Vec
/// Uses a `where` clause for trait bounds (Rust Book §10.2 style)
/// Reinforces: Vec operations, Clone + Ord bounds
pub fn sort_and_dedup<T>(items: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    todo!()
}

/// Collect unique items from a slice into a Vec (preserving first-seen order)
/// Uses multiple bounds with + syntax: Eq + Hash + Clone
/// Hint: use a HashSet to track seen items
/// Reinforces: HashMap for uniqueness tracking, Vec, generics
pub fn unique_ordered<T>(items: &[T]) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    todo!()
}

/// Build a frequency map from a slice of items
/// Reinforces: HashMap, trait bounds (Eq + Hash + Clone), iteration
pub fn frequency_map<T>(items: &[T]) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash + Clone,
{
    todo!()
}

// ============================================
// Topic 7: Default Methods, Trait Objects & impl Trait
// Learn: Default method implementations, dyn Trait, impl Trait returns
// Reinforces: Vec collection, Option, enum matching, slices
// ============================================

/// A trait for things that have a measurable size
pub trait Measurable {
    /// Required: return the measurement value
    fn measure(&self) -> f64;

    /// Required: return the unit name
    fn unit(&self) -> &'static str;

    /// Default: format as "{value:.1} {unit}"
    fn display(&self) -> String {
        todo!()
    }

    /// Default: compare two measurables by value
    fn is_greater_than(&self, other: &dyn Measurable) -> bool {
        todo!()
    }
}

/// Area in square meters
#[derive(Debug)]
pub struct Area(pub f64);

impl Measurable for Area {
    fn measure(&self) -> f64 {
        todo!()
    }
    fn unit(&self) -> &'static str {
        "m²"
    }
}

/// Distance in meters
#[derive(Debug)]
pub struct Distance(pub f64);

impl Measurable for Distance {
    fn measure(&self) -> f64 {
        todo!()
    }
    fn unit(&self) -> &'static str {
        "m"
    }
}

/// Duration in seconds
#[derive(Debug)]
pub struct TimeDuration(pub f64);

impl Measurable for TimeDuration {
    fn measure(&self) -> f64 {
        todo!()
    }
    fn unit(&self) -> &'static str {
        "s"
    }
}

/// Sum the measurements of a slice of trait objects
/// Reinforces: slices of &dyn Trait, iteration
pub fn total_measurement(items: &[&dyn Measurable]) -> f64 {
    todo!()
}

/// Find the largest measurement in a slice of trait objects
pub fn max_measurement(items: &[&dyn Measurable]) -> Option<f64> {
    todo!()
}

/// A trait for content that can be summarized (from Rust Book §10.2)
/// Demonstrates: required method + default method calling required method
pub trait Summarizable {
    /// Required: return the author/source
    fn summarize_author(&self) -> String;

    /// Default: builds on summarize_author
    /// Returns "(Read more from {author}...)"
    fn summarize(&self) -> String {
        todo!()
    }
}

/// A news article
/// Reinforces: struct with owned fields
#[derive(Debug, Clone)]
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
}

impl NewsArticle {
    pub fn new(headline: &str, author: &str, location: &str) -> Self {
        todo!()
    }
}

impl Summarizable for NewsArticle {
    fn summarize_author(&self) -> String {
        todo!()
    }

    /// Override default: return "{headline}, by {author} ({location})"
    fn summarize(&self) -> String {
        todo!()
    }
}

/// A social media post
/// Reinforces: struct construction
#[derive(Debug, Clone)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
}

impl SocialPost {
    pub fn new(username: &str, content: &str) -> Self {
        todo!()
    }
}

impl Summarizable for SocialPost {
    /// Return "@{username}"
    fn summarize_author(&self) -> String {
        todo!()
    }
    // Uses the default summarize() implementation — do NOT override
}

/// Collect summaries from a slice of Summarizable trait objects
/// Reinforces: Vec, trait objects, iteration
pub fn collect_summaries(items: &[&dyn Summarizable]) -> Vec<String> {
    todo!()
}

/// Create a social post and return it as `impl Summarizable`
/// Demonstrates returning impl Trait (Rust Book §10.2)
pub fn make_social_post(username: &str, content: &str) -> impl Summarizable {
    todo!();
    // Hint: construct and return a SocialPost
    #[allow(unreachable_code)]
    SocialPost::new("", "")
}

/// A trait for taggable content — demonstrates default methods
pub trait Taggable {
    /// Required: return all tags
    fn tags(&self) -> Vec<&str>;

    /// Default: check if a specific tag exists
    fn has_tag(&self, tag: &str) -> bool {
        todo!()
    }

    /// Default: count the tags
    fn tag_count(&self) -> usize {
        todo!()
    }
}

/// An article with tags
#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub article_tags: Vec<String>,
}

impl Taggable for Article {
    fn tags(&self) -> Vec<&str> {
        todo!()
    }
}

/// A photo with tags
#[derive(Debug)]
pub struct Photo {
    pub filename: String,
    pub photo_tags: Vec<String>,
}

impl Taggable for Photo {
    fn tags(&self) -> Vec<&str> {
        todo!()
    }
}

/// Collect all unique tags from a slice of Taggable items
/// Reinforces: HashSet-based dedup, Vec, trait objects
pub fn all_tags(items: &[&dyn Taggable]) -> Vec<String> {
    todo!()
}

/// Filter taggable items that have a specific tag
/// Reinforces: slice filtering, trait object references with lifetimes
pub fn filter_by_tag<'a>(items: &[&'a dyn Taggable], tag: &str) -> Vec<&'a dyn Taggable> {
    todo!()
}

/// Count how many items have a specific tag
/// Reinforces: counting with trait objects
pub fn count_with_tag(items: &[&dyn Taggable], tag: &str) -> usize {
    todo!()
}
