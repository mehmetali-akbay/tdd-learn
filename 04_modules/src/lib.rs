// ============================================
// Level 4, Project 2: Modules — Crates & Visibility
// ============================================

mod math {
    /// Add two numbers (public function in a module).
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    /// Multiply two numbers (public).
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    fn _square(x: i32) -> i32 {
        x * x
    }
    /// Uses the private helper internally.
    pub fn sum_of_squares(a: i32, b: i32) -> i32 {
        _square(a) + _square(b)
    }
}

mod geometry {
    pub mod shapes {
        /// Calculate circle area.
        pub fn circle_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
        /// Calculate rectangle area.
        pub fn rectangle_area(width: f64, height: f64) -> f64 {
            width * height
        }
    }
    pub mod utils {
        /// Use super to access sibling module's function.
        /// Returns the area of a square (uses rectangle_area from shapes).
        pub fn square_area(side: f64) -> f64 {
            super::shapes::rectangle_area(side, side)
        }
        /// Format the area with 2 decimal places.
        pub fn format_area(area: f64) -> String {
            format!("{:.2}", area)
        }
    }
}

mod converters {
    /// Celsius to Fahrenheit.
    pub fn celsius_to_fahrenheit(c: f64) -> f64 {
        c * 9.0 / 5.0 + 32.0
    }
    /// Fahrenheit to Celsius.
    pub fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - 32.0) * 5.0 / 9.0
    }
    /// Kilometers to miles.
    pub fn km_to_miles(km: f64) -> f64 {
        km * 0.621371
    }
    /// Miles to kilometers.
    pub fn miles_to_km(miles: f64) -> f64 {
        miles / 0.621371
    }
}

/// Demonstrate use with alias — convert temperature and format.
pub fn describe_temperature(celsius: f64) -> String {
    let f = converters::celsius_to_fahrenheit(celsius);
    format!("{:.0}°C = {:.0}°F", celsius, f)
}

/// Convert distance using the converters module.
pub fn describe_distance_km(km: f64) -> String {
    let miles = converters::km_to_miles(km);
    format!("{:.0} km = {:.2} miles", km, miles)
}

mod database {
    /// A record with mixed visibility fields.
    pub struct Record {
        pub id: u32,
        pub name: String,
        pub(crate) internal_score: f64,
        created_at: String,
    }
    impl Record {
        /// Constructor — only way to set private fields.
        pub fn new(id: u32, name: String, score: f64) -> Self {
            Self {
                id,
                name,
                internal_score: score,
                created_at: "2024-01-01".to_string(),
            }
        }
        /// Public getter for the private created_at field.
        pub fn created_at(&self) -> &str {
            &self.created_at
        }
        /// Only accessible within the crate.
        pub(crate) fn update_score(&mut self, new_score: f64) {
            self.internal_score = new_score;
        }
    }
    pub(super) fn create_default_record() -> Record {
        Record::new(0, "default".to_string(), 0.0)
    }
}

/// Demonstrates access to pub and pub(crate) fields.
pub fn get_record_summary(id: u32, name: &str, score: f64) -> String {
    let r = database::Record::new(id, name.to_string(), score);
    format!(
        "Record #{}: {} (score: {:.1})",
        r.id, r.name, r.internal_score
    )
}

mod engine {
    mod parser {
        pub fn parse(input: &str) -> Vec<&str> {
            input.split_whitespace().collect()
        }
    }
    mod evaluator {
        pub fn evaluate(tokens: &[&str]) -> String {
            tokens.join(" ").to_uppercase()
        }
    }
    pub use evaluator::evaluate;
    pub use parser::parse;
    /// Process input by parsing and evaluating.
    pub fn process(input: &str) -> String {
        let tokens = parse(input);
        evaluate(&tokens)
    }
}

/// Uses the re-exported functions from engine.
pub fn run_engine(input: &str) -> String {
    engine::process(input)
}

/// A trait that plugins implement.
pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

mod plugins {
    use super::Plugin;
    pub struct UppercasePlugin;
    impl Plugin for UppercasePlugin {
        fn name(&self) -> &str {
            "uppercase"
        }
        fn execute(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }
    pub struct ReversePlugin;
    impl Plugin for ReversePlugin {
        fn name(&self) -> &str {
            "reverse"
        }
        fn execute(&self, input: &str) -> String {
            input.chars().rev().collect()
        }
    }
    pub struct RepeatPlugin {
        pub times: usize,
    }
    impl Plugin for RepeatPlugin {
        fn name(&self) -> &str {
            "repeat"
        }
        fn execute(&self, input: &str) -> String {
            input.repeat(self.times)
        }
    }
}

pub use plugins::{RepeatPlugin, ReversePlugin, UppercasePlugin};

/// A plugin runner that executes all registered plugins in order.
pub struct PluginRunner {
    plugins: Vec<Box<dyn Plugin>>,
}
impl PluginRunner {
        /// Constructor — only way to set private fields.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    /// Run all plugins sequentially, each receiving the output of the previous.
    pub fn run(&self, input: &str) -> String {
        self.plugins
            .iter()
            .fold(input.to_string(), |acc, p| p.execute(&acc))
    }
    /// List all registered plugin names.
    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_add() {
        assert_eq!(math::add(2, 3), 5);
    }

    #[test]
    fn test_math_multiply() {
        assert_eq!(math::multiply(3, 4), 12);
    }

    #[test]
    fn test_math_sum_of_squares() {
        assert_eq!(math::sum_of_squares(3, 4), 25);
    }

    #[test]
    fn test_circle_area() {
        assert!((geometry::shapes::circle_area(5.0) - 78.53981633974483).abs() < 1e-6);
    }

    #[test]
    fn test_rectangle_area() {
        assert!((geometry::shapes::rectangle_area(3.0, 4.0) - 12.0).abs() < 1e-6);
    }

    #[test]
    fn test_square_area_via_utils() {
        assert!((geometry::utils::square_area(5.0) - 25.0).abs() < 1e-6);
    }

    #[test]
    fn test_format_area() {
        assert_eq!(geometry::utils::format_area(12.3456), "12.35");
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!((converters::celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-6);
        assert!((converters::celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-6);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert!((converters::fahrenheit_to_celsius(32.0) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_describe_temperature() {
        let desc = describe_temperature(100.0);
        assert!(desc.contains("100"));
        assert!(desc.contains("212"));
    }

    #[test]
    fn test_describe_distance() {
        let desc = describe_distance_km(10.0);
        assert!(desc.contains("10"));
        assert!(desc.contains("6.21"));
    }

    #[test]
    fn test_record_creation() {
        let r = database::Record::new(1, "test".into(), 9.5);
        assert_eq!(r.id, 1);
        assert_eq!(r.name, "test");
    }

    #[test]
    fn test_record_pub_crate_field() {
        let r = database::Record::new(1, "test".into(), 9.5);
        assert!((r.internal_score - 9.5).abs() < 1e-6);
    }

    #[test]
    fn test_record_created_at() {
        let r = database::Record::new(1, "test".into(), 9.5);
        assert!(!r.created_at().is_empty());
    }

    #[test]
    fn test_update_score() {
        let mut r = database::Record::new(1, "test".into(), 5.0);
        r.update_score(10.0);
        assert!((r.internal_score - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_record_summary() {
        let summary = get_record_summary(1, "Alice", 9.5);
        assert!(summary.contains("Alice"));
    }

    #[test]
    fn test_engine_parse() {
        let tokens = engine::parse("hello world");
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_engine_evaluate() {
        let result = engine::evaluate(&["hello", "world"]);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_engine_process() {
        let result = engine::process("hello world");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_run_engine() {
        let result = run_engine("hello world");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_uppercase_plugin() {
        let p = UppercasePlugin;
        assert_eq!(p.execute("hello"), "HELLO");
    }

    #[test]
    fn test_reverse_plugin() {
        let p = ReversePlugin;
        assert_eq!(p.execute("hello"), "olleh");
    }

    #[test]
    fn test_repeat_plugin() {
        let p = RepeatPlugin { times: 3 };
        assert_eq!(p.execute("ha"), "hahaha");
    }

    #[test]
    fn test_plugin_runner_empty() {
        let runner = PluginRunner::new();
        assert_eq!(runner.run("hello"), "hello");
    }

    #[test]
    fn test_plugin_runner_chain() {
        let mut runner = PluginRunner::new();
        runner.register(Box::new(UppercasePlugin));
        runner.register(Box::new(ReversePlugin));
        assert_eq!(runner.run("hello"), "OLLEH");
    }

    #[test]
    fn test_plugin_names() {
        let mut runner = PluginRunner::new();
        runner.register(Box::new(UppercasePlugin));
        runner.register(Box::new(ReversePlugin));
        assert_eq!(runner.plugin_names(), vec!["uppercase", "reverse"]);
    }
}
