// ============================================
// Level 8, Project 2: Serde — Serialization & Deserialization
// ============================================
use serde::{Deserialize, Serialize};

// Topic 1: JSON Basics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}
/// Serialize a Person to JSON string.
pub fn person_to_json(person: &Person) -> String {
    serde_json::to_string(person).unwrap()
}
/// Deserialize a Person from JSON string.
pub fn json_to_person(json: &str) -> Result<Person, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}
/// Serialize a vector of persons to JSON.
pub fn people_to_json(people: &[Person]) -> String {
    serde_json::to_string(people).unwrap()
}
/// Deserialize a vector of persons from JSON.
pub fn json_to_people(json: &str) -> Result<Vec<Person>, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// Topic 2: Custom Field Names & Defaults
fn default_port() -> u16 {
    8080
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub debug: bool,
    #[serde(skip, default)]
    pub internal_id: String,
}
/// Serialize Config to JSON.
pub fn config_to_json(config: &Config) -> String {
    serde_json::to_string(config).unwrap()
}
/// Deserialize Config from JSON (missing fields use defaults).
pub fn json_to_config(json: &str) -> Result<Config, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// Topic 3: Enums
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}
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
    serde_json::to_string(shape).unwrap()
}
/// Deserialize a shape from JSON.
pub fn json_to_shape(json: &str) -> Result<Shape, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}
/// Serialize an event to JSON.
pub fn event_to_json(event: &Event) -> String {
    serde_json::to_string(event).unwrap()
}
/// Deserialize an event from JSON.
pub fn json_to_event(json: &str) -> Result<Event, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// Topic 4: Nested & Optional
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: String,
    pub version: u32,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub address: Option<Address>,
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub metadata: Metadata,
}
/// Serialize a UserProfile to JSON.
pub fn profile_to_json(profile: &UserProfile) -> String {
    serde_json::to_string(profile).unwrap()
}
/// Deserialize a UserProfile from JSON.
pub fn json_to_profile(json: &str) -> Result<UserProfile, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// Topic 5: Custom Serializers
pub mod custom_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};
    /// Serialize a bool as "yes"/"no".
    pub fn serialize_yes_no<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(if *value { "yes" } else { "no" })
    }
    /// Deserialize "yes"/"no" as bool.
    pub fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "yes" => Ok(true),
            "no" => Ok(false),
            _ => Err(serde::de::Error::custom("expected yes or no")),
        }
    }
    /// Serialize a Vec<String> as a comma-separated string.
    pub fn serialize_csv<S>(values: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&values.join(","))
    }
    /// Deserialize a comma-separated string into Vec<String>.
    pub fn deserialize_csv<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(vec![])
        } else {
            Ok(s.split(',').map(|s| s.to_string()).collect())
        }
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
    serde_json::to_string(settings).unwrap()
}
/// Deserialize Settings from JSON.
pub fn json_to_settings(json: &str) -> Result<Settings, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// Topic 6: Multi-Format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub title: String,
    pub port: u16,
    pub features: Vec<String>,
}
/// Serialize AppConfig to JSON.
pub fn app_config_to_json(config: &AppConfig) -> String {
    serde_json::to_string(config).unwrap()
}
/// Serialize AppConfig to TOML.
pub fn app_config_to_toml(config: &AppConfig) -> String {
    toml::to_string(config).unwrap()
}
/// Deserialize AppConfig from JSON.
pub fn json_to_app_config(json: &str) -> Result<AppConfig, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}
/// Deserialize AppConfig from TOML.
pub fn toml_to_app_config(toml_str: &str) -> Result<AppConfig, String> {
    toml::from_str(toml_str).map_err(|e| e.to_string())
}
