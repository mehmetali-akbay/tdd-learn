// ============================================
// Level 2, Project 3: Traits — Defining & Implementing Traits
// Learn: Trait definition, impl, default methods, bounds, Display
// ============================================

use std::fmt;

// ============================================
// Topic 1: Basic Traits — Describable
// Learn: Defining a trait, implementing it for multiple structs
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
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("{} {} {}", self.year, self.make, self.model)
    }
}

/// A book with title and author
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
        format!(
            "\"{}\" by {} ({} pages)",
            self.title, self.author, self.pages
        )
    }
}

/// A generic function that takes anything Describable
pub fn print_description(item: &dyn Describable) -> String {
    item.describe()
}

// ============================================
// Topic 2: Display & Debug — Standard Library Traits
// Learn: Implementing fmt::Display for custom types
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
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A temperature with unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
}

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
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit {
            TempUnit::Celsius => write!(f, "{}°C", self.value),
            TempUnit::Fahrenheit => write!(f, "{}°F", self.value),
        }
    }
}

// ============================================
// Topic 3: Default Trait
// Learn: Implementing Default, builder-like patterns
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

// ============================================
// Topic 4: From / Into Conversions
// Learn: Implementing From trait for type conversions
// ============================================

/// Celsius temperature value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

/// Fahrenheit temperature value
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

/// An RGB color from a tuple
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

// ============================================
// Topic 5: Trait Bounds
// Learn: Generic functions with trait constraints
// ============================================

/// Print all items (requires Display)
pub fn format_all<T: fmt::Display>(items: &[T]) -> String {
    items
        .iter()
        .map(|item| format!("{}", item))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Find the largest item (requires PartialOrd + Copy)
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    items
        .iter()
        .copied()
        .reduce(|a, b| if a >= b { a } else { b })
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

// ============================================
// Topic 6: Advanced — Trait Objects & Default Methods
// Learn: dyn Trait, default method implementations
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

    /// Default: compare two measurables
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
pub struct Duration(pub f64);

impl Measurable for Duration {
    fn measure(&self) -> f64 {
        self.0
    }

    fn unit(&self) -> &'static str {
        "s"
    }
}

/// Sum the measurements of a slice of trait objects
pub fn total_measurement(items: &[&dyn Measurable]) -> f64 {
    items.iter().map(|item| item.measure()).sum()
}

/// Find the largest measurement in a slice of trait objects
pub fn max_measurement(items: &[&dyn Measurable]) -> Option<f64> {
    items.iter().map(|item| item.measure()).reduce(f64::max)
}

// ============================================
// Topic 7: Extra Practice
// Learn: More trait implementation practice
// ============================================

/// A trait for things that can be tagged
pub trait Taggable {
    fn tags(&self) -> Vec<&str>;
    fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(&tag)
    }
}

pub struct Article { pub title: String, pub article_tags: Vec<String> }
pub struct Photo { pub filename: String, pub photo_tags: Vec<String> }

impl Taggable for Article {
    fn tags(&self) -> Vec<&str> { self.article_tags.iter().map(|s| s.as_str()).collect() }
}

impl Taggable for Photo {
    fn tags(&self) -> Vec<&str> { self.photo_tags.iter().map(|s| s.as_str()).collect() }
}

/// Collect all unique tags from a slice of Taggable items.
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

/// Filter taggable items that have a specific tag.
pub fn filter_by_tag<'a>(items: &[&'a dyn Taggable], tag: &str) -> Vec<&'a dyn Taggable> {
    items.iter().filter(|i| i.has_tag(tag)).copied().collect()
}

