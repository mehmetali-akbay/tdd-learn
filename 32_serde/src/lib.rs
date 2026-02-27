// ============================================
// Level 8, Project 2: Serde — Serialization & Deserialization
// Learn: serde, serde_json, toml, derive, custom serialize
// ============================================

use serde::{Deserialize, Serialize};

// ============================================
// Topic 1: JSON Basics
// Learn: Serialize/Deserialize derive, serde_json
// ============================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

/// Serialize a Person to JSON string.
pub fn person_to_json(person: &Person) -> String {
    todo!()
}

/// Deserialize a Person from JSON string.
pub fn json_to_person(json: &str) -> Result<Person, String> {
    todo!()
}

/// Serialize a vector of persons to JSON.
pub fn people_to_json(people: &[Person]) -> String {
    todo!()
}

/// Deserialize a vector of persons from JSON.
pub fn json_to_people(json: &str) -> Result<Vec<Person>, String> {
    todo!()
}

// ============================================
// Topic 2: Custom Field Names & Defaults
// Learn: #[serde(rename)], #[serde(default)], skip
// ============================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "appName")]
    pub app_name: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default)]
    pub debug: bool,

    #[serde(skip_serializing)]
    pub internal_id: String,
}

fn default_port() -> u16 {
    8080
}

/// Serialize Config to JSON.
pub fn config_to_json(config: &Config) -> String {
    todo!()
}

/// Deserialize Config from JSON (missing fields use defaults).
pub fn json_to_config(json: &str) -> Result<Config, String> {
    todo!()
}

// ============================================
// Topic 3: Enums in Serde
// Learn: Tagged, untagged, adjacently tagged
// ============================================

/// Default (externally tagged) enum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

/// Internally tagged enum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "click")]
    Click { x: i32, y: i32 },
    #[serde(rename = "keypress")]
    KeyPress { key: String },
    #[serde(rename = "scroll")]
    Scroll { delta: i32 },
}

/// Untagged enum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
}

/// Serialize a shape to JSON.
pub fn shape_to_json(shape: &Shape) -> String {
    todo!()
}

/// Deserialize a shape from JSON.
pub fn json_to_shape(json: &str) -> Result<Shape, String> {
    todo!()
}

/// Serialize an event to JSON.
pub fn event_to_json(event: &Event) -> String {
    todo!()
}

/// Deserialize an event from JSON.
pub fn json_to_event(json: &str) -> Result<Event, String> {
    todo!()
}

// ============================================
// Topic 4: Nested & Optional Fields
// Learn: Option<T>, Vec<T>, flatten
// ============================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub address: Option<Address>,
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: String,
    pub version: u32,
}

/// Serialize a UserProfile to JSON.
pub fn profile_to_json(profile: &UserProfile) -> String {
    todo!()
}

/// Deserialize a UserProfile from JSON.
pub fn json_to_profile(json: &str) -> Result<UserProfile, String> {
    todo!()
}

// ============================================
// Topic 5: Custom Serializers
// Learn: serialize_with, deserialize_with
// ============================================

pub mod custom_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serialize a bool as "yes"/"no".
    pub fn serialize_yes_no<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }

    /// Deserialize "yes"/"no" as bool.
    pub fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }

    /// Serialize a Vec<String> as a comma-separated string.
    pub fn serialize_csv<S>(values: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }

    /// Deserialize a comma-separated string into Vec<String>.
    pub fn deserialize_csv<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub name: String,

    #[serde(
        serialize_with = "custom_serde::serialize_yes_no",
        deserialize_with = "custom_serde::deserialize_yes_no"
    )]
    pub active: bool,

    #[serde(
        serialize_with = "custom_serde::serialize_csv",
        deserialize_with = "custom_serde::deserialize_csv"
    )]
    pub features: Vec<String>,
}

/// Serialize Settings to JSON.
pub fn settings_to_json(settings: &Settings) -> String {
    todo!()
}

/// Deserialize Settings from JSON.
pub fn json_to_settings(json: &str) -> Result<Settings, String> {
    todo!()
}

// ============================================
// Topic 6: Advanced — Multiple Formats (TOML)
// Learn: TOML via serde; format-agnostic code
// ============================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub title: String,
    pub port: u16,
    pub features: Vec<String>,
}

/// Serialize AppConfig to JSON.
pub fn app_config_to_json(config: &AppConfig) -> String {
    todo!()
}

/// Serialize AppConfig to TOML.
pub fn app_config_to_toml(config: &AppConfig) -> String {
    todo!()
}

/// Deserialize AppConfig from JSON.
pub fn json_to_app_config(json: &str) -> Result<AppConfig, String> {
    todo!()
}

/// Deserialize AppConfig from TOML.
pub fn toml_to_app_config(toml_str: &str) -> Result<AppConfig, String> {
    todo!()
}

// ============================================
// Tests
// ============================================
