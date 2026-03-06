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
        Pet {
            name: name.to_string(),
            species: species.to_string(),
        }
    }
}

impl Describable for Pet {
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
        Car {
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
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("\"{}\" by {} ({} pages)", self.title, self.author, self.pages)
    }
}

/// A generic function that takes anything Describable (trait object parameter)
pub fn print_description(item: &dyn Describable) -> String {
    item.describe()
}

/// Collect descriptions from a slice of trait objects into a Vec
/// Reinforces: slice borrowing, Vec collection, iteration
pub fn describe_all(items: &[&dyn Describable]) -> Vec<String> {
    items.iter().map(|item| item.describe()).collect()
}

/// Find the longest description among a slice of Describable items
/// Reinforces: Option<String>, iteration, max_by_key
pub fn longest_description(items: &[&dyn Describable]) -> Option<String> {
    items
        .iter()
        .map(|item| item.describe())
        .max_by_key(|s| s.len())
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    pub fn to_celsius(&self) -> Temperature {
        match self.unit {
            TempUnit::Celsius => *self,
            TempUnit::Fahrenheit => Temperature::celsius((self.value - 32.0) * 5.0 / 9.0),
        }
    }

    /// Convert to Fahrenheit (returns a new Temperature)
    /// Reinforces: match on enum, arithmetic
    pub fn to_fahrenheit(&self) -> Temperature {
        match self.unit {
            TempUnit::Fahrenheit => *self,
            TempUnit::Celsius => Temperature::fahrenheit(self.value * 9.0 / 5.0 + 32.0),
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit {
            TempUnit::Celsius => write!(f, "{:.1}°C", self.value),
            TempUnit::Fahrenheit => write!(f, "{:.1}°F", self.value),
        }
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
        self.r == self.g && self.g == self.b
    }

    /// Calculate brightness using the weighted formula (0.299R + 0.587G + 0.114B)
    /// Reinforces: arithmetic, f64 casting
    pub fn brightness(&self) -> f64 {
        0.299 * self.r as f64 + 0.587 * self.g as f64 + 0.114 * self.b as f64
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

// ============================================
// Topic 3: Default Trait & Builder Pattern
// Learn: Implementing Default, builder-like patterns with consuming self
// Reinforces: struct construction, String ownership, method chaining
// ============================================

/// Application configuration with defaults
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub max_connections: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
            max_connections: 100,
        }
    }
}

impl Config {
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Generate a human-readable summary of the config
    /// Reinforces: String formatting, conditional logic
    pub fn summary(&self) -> String {
        let mode = if self.debug { "debug" } else { "release" };
        format!(
            "{}:{} ({}, max_conn={})",
            self.host, self.port, mode, self.max_connections
        )
    }

    /// Check if this config uses all default values
    /// Reinforces: PartialEq comparison
    pub fn is_default(&self) -> bool {
        *self == Config::default()
    }
}

/// Game settings with defaults
#[derive(Debug, Clone, PartialEq)]
pub struct GameSettings {
    pub difficulty: String,
    pub sound: bool,
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            difficulty: "normal".to_string(),
            sound: true,
            fullscreen: false,
        }
    }
}

impl GameSettings {
    /// Builder method to set difficulty
    /// Reinforces: consuming self pattern, String ownership
    pub fn with_difficulty(mut self, difficulty: &str) -> Self {
        self.difficulty = difficulty.to_string();
        self
    }

    /// Builder method to toggle sound
    pub fn with_sound(mut self, sound: bool) -> Self {
        self.sound = sound;
        self
    }

    /// Builder method to toggle fullscreen
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
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
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculate the dot product with another vector
    /// Reinforces: borrowing, arithmetic
    pub fn dot(&self, other: &Vec2d) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2d {
    type Output = Vec2d;
    fn add(self, rhs: Vec2d) -> Vec2d {
        Vec2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;
    fn sub(self, rhs: Vec2d) -> Vec2d {
        Vec2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;
    fn neg(self) -> Vec2d {
        Vec2d::new(-self.x, -self.y)
    }
}

impl PartialEq for Vec2d {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 1e-9 && (self.y - other.y).abs() < 1e-9
    }
}

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A monetary amount in cents (avoids floating point issues)
/// Reinforces: tuple struct, arithmetic, Display formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(pub i64);

impl Money {
    /// Create from dollars and cents
    pub fn new(dollars: i64, cents: i64) -> Self {
        Money(dollars * 100 + cents)
    }

    /// Get the dollar part
    pub fn dollars(&self) -> i64 {
        self.0 / 100
    }

    /// Get the cents part (always 0-99)
    pub fn cents(&self) -> i64 {
        (self.0 % 100).abs()
    }
}

impl Add for Money {
    type Output = Money;
    fn add(self, rhs: Money) -> Money {
        Money(self.0 + rhs.0)
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, rhs: Money) -> Money {
        Money(self.0 - rhs.0)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 0 {
            write!(f, "-${}.{:02}", (-self.0) / 100, (-self.0) % 100)
        } else {
            write!(f, "${}.{:02}", self.0 / 100, self.0 % 100)
        }
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
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

/// A URL-friendly slug
#[derive(Debug, Clone, PartialEq)]
pub struct Slug(pub String);

impl From<&str> for Slug {
    fn from(s: &str) -> Self {
        Slug(s.trim().to_lowercase().replace(' ', "-"))
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
        Rgb { r, g, b }
    }
}

impl From<Rgb> for (u8, u8, u8) {
    fn from(c: Rgb) -> Self {
        (c.r, c.g, c.b)
    }
}

/// Convert an array [f64; 2] into a Coord
/// Reinforces: arrays, From for existing types
impl From<[f64; 2]> for Coord {
    fn from(arr: [f64; 2]) -> Self {
        Coord::new(arr[0], arr[1])
    }
}

/// Convert a (title, author, pages) tuple into a Book
/// Reinforces: tuples, destructuring, String ownership
impl From<(&str, &str, u32)> for Book {
    fn from((title, author, pages): (&str, &str, u32)) -> Self {
        Book::new(title, author, pages)
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
    /// Calculate the average as f64
    pub fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum as f64 / self.count as f64
        }
    }
}

impl From<&[i32]> for Stats {
    fn from(slice: &[i32]) -> Self {
        if slice.is_empty() {
            return Stats {
                count: 0,
                sum: 0,
                min: 0,
                max: 0,
            };
        }
        let count = slice.len();
        let sum: i64 = slice.iter().map(|&x| x as i64).sum();
        let min = *slice.iter().min().unwrap() as i64;
        let max = *slice.iter().max().unwrap() as i64;
        Stats {
            count,
            sum,
            min,
            max,
        }
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
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "low" => Priority::Low,
            "medium" | "med" => Priority::Medium,
            "high" => Priority::High,
            "critical" | "crit" => Priority::Critical,
            _ => Priority::Medium, // default fallback
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Critical => write!(f, "critical"),
        }
    }
}

// ============================================
// Topic 6: Trait Bounds & Generic Functions
// Learn: Generic functions with trait bounds, where clauses,
//        multiple bounds with +, impl Trait syntax
// Reinforces: slices, Vec, Option, generic programming
// ============================================

/// Format all items using Display, joined by a separator
pub fn format_all<T: fmt::Display>(items: &[T]) -> String {
    items
        .iter()
        .map(|item| format!("{}", item))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Find the largest item in a slice (requires PartialOrd + Copy)
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    items.iter().copied().reduce(|a, b| if a >= b { a } else { b })
}

/// Find the smallest item in a slice (requires PartialOrd + Copy)
/// Reinforces: mirrors largest, solidifies trait bound understanding
pub fn smallest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    items.iter().copied().reduce(|a, b| if a <= b { a } else { b })
}

/// Find the first item matching a target (requires PartialEq)
pub fn find_match<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    items.iter().position(|item| item == target)
}

/// Count items matching a predicate
pub fn count_where<T>(items: &[T], predicate: fn(&T) -> bool) -> usize {
    items.iter().filter(|item| predicate(item)).count()
}

/// Check if all items satisfy a predicate
pub fn all_match<T>(items: &[T], predicate: fn(&T) -> bool) -> bool {
    items.iter().all(predicate)
}

/// Check if any item satisfies a predicate
/// Reinforces: mirrors all_match
pub fn any_match<T>(items: &[T], predicate: fn(&T) -> bool) -> bool {
    items.iter().any(predicate)
}

/// Sort and deduplicate a slice, returning a new Vec
/// Uses a `where` clause for trait bounds (Rust Book §10.2 style)
/// Reinforces: Vec operations, Clone + Ord bounds
pub fn sort_and_dedup<T>(items: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut result: Vec<T> = items.to_vec();
    result.sort();
    result.dedup();
    result
}

/// Collect unique items from a slice into a Vec (preserving first-seen order)
/// Uses multiple bounds with + syntax: Eq + std::hash::Hash + Clone
/// Reinforces: HashMap for uniqueness tracking, Vec, generics
pub fn unique_ordered<T>(items: &[T]) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for item in items {
        if seen.insert(item.clone()) {
            result.push(item.clone());
        }
    }
    result
}

/// Build a frequency map from a slice of items
/// Reinforces: HashMap, trait bounds (Eq + Hash + Clone), iteration
pub fn frequency_map<T>(items: &[T]) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut map = HashMap::new();
    for item in items {
        *map.entry(item.clone()).or_insert(0) += 1;
    }
    map
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

    /// Default: format as "value unit"
    fn display(&self) -> String {
        format!("{:.1} {}", self.measure(), self.unit())
    }

    /// Default: compare two measurables by value
    fn is_greater_than(&self, other: &dyn Measurable) -> bool {
        self.measure() > other.measure()
    }
}

/// Area in square meters
#[derive(Debug)]
pub struct Area(pub f64);

impl Measurable for Area {
    fn measure(&self) -> f64 {
        self.0
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
        self.0
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
        self.0
    }
    fn unit(&self) -> &'static str {
        "s"
    }
}

/// Sum the measurements of a slice of trait objects
/// Reinforces: slices of &dyn Trait, iteration
pub fn total_measurement(items: &[&dyn Measurable]) -> f64 {
    items.iter().map(|item| item.measure()).sum()
}

/// Find the largest measurement in a slice of trait objects
pub fn max_measurement(items: &[&dyn Measurable]) -> Option<f64> {
    items.iter().map(|item| item.measure()).reduce(f64::max)
}

/// A trait for content that can be summarized (from Rust Book §10.2)
/// Demonstrates: required method + default method calling required method
pub trait Summarizable {
    /// Required: return the author/source
    fn summarize_author(&self) -> String;

    /// Default: builds on summarize_author
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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
        NewsArticle {
            headline: headline.to_string(),
            author: author.to_string(),
            location: location.to_string(),
        }
    }
}

impl Summarizable for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    /// Override default: use headline + author + location
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
        SocialPost {
            username: username.to_string(),
            content: content.to_string(),
        }
    }
}

impl Summarizable for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses the default summarize() implementation
}

/// Collect summaries from a slice of Summarizable trait objects
/// Reinforces: Vec, trait objects, iteration
pub fn collect_summaries(items: &[&dyn Summarizable]) -> Vec<String> {
    items.iter().map(|item| item.summarize()).collect()
}

/// Create a social post and return it as `impl Summarizable`
/// Demonstrates returning impl Trait (Rust Book §10.2)
pub fn make_social_post(username: &str, content: &str) -> impl Summarizable {
    SocialPost::new(username, content)
}

/// A trait for taggable content — demonstrates default methods
pub trait Taggable {
    /// Required: return all tags
    fn tags(&self) -> Vec<&str>;

    /// Default: check if a specific tag exists
    fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(&tag)
    }

    /// Default: count the tags
    fn tag_count(&self) -> usize {
        self.tags().len()
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
        self.article_tags.iter().map(|s| s.as_str()).collect()
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
        self.photo_tags.iter().map(|s| s.as_str()).collect()
    }
}

/// Collect all unique tags from a slice of Taggable items
/// Reinforces: HashSet-based dedup, Vec, trait objects
pub fn all_tags(items: &[&dyn Taggable]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for item in items {
        for tag in item.tags() {
            if seen.insert(tag.to_string()) {
                result.push(tag.to_string());
            }
        }
    }
    result
}

/// Filter taggable items that have a specific tag
/// Reinforces: slice filtering, trait object references with lifetimes
pub fn filter_by_tag<'a>(items: &[&'a dyn Taggable], tag: &str) -> Vec<&'a dyn Taggable> {
    items.iter().filter(|i| i.has_tag(tag)).copied().collect()
}

/// Count how many items have a specific tag
/// Reinforces: counting with trait objects
pub fn count_with_tag(items: &[&dyn Taggable], tag: &str) -> usize {
    items.iter().filter(|i| i.has_tag(tag)).count()
}