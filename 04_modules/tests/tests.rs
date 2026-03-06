use modules_intro::*;

// ===== Topic 1: Basic Modules =====

#[test]
fn test_describe_temperature() {
    let desc = describe_temperature(100.0);
    assert!(desc.contains("100"));
    assert!(desc.contains("212"));
}

#[test]
fn test_describe_temperature_zero() {
    let desc = describe_temperature(0.0);
    assert!(desc.contains("0"));
    assert!(desc.contains("32"));
}

// ===== Topic 2: Nested Modules =====

#[test]
fn test_describe_distance() {
    let desc = describe_distance_km(10.0);
    assert!(desc.contains("10"));
    assert!(desc.contains("6.21"));
}

#[test]
fn test_describe_distance_zero() {
    assert!(describe_distance_km(0.0).contains("0.00"));
}

// ===== Topic 3: Visibility =====

#[test]
fn test_record_summary() {
    let summary = get_record_summary(1, "Alice", 9.5);
    assert!(summary.contains("Alice"));
    assert!(summary.contains("9.5"));
}

#[test]
fn test_record_summary_special_chars() {
    let summary = get_record_summary(42, "Bob O'Neil", 7.0);
    assert!(summary.contains("Bob O'Neil"));
    assert!(summary.contains("#42"));
}

// ===== Topic 4: Re-exports =====

#[test]
fn test_run_engine() {
    let result = run_engine("hello world");
    assert_eq!(result, "HELLO WORLD");
}

#[test]
fn test_run_engine_empty() {
    let result = run_engine("");
    assert_eq!(result, "");
}

// ===== Topic 5: Plugin Pattern =====

#[test]
fn test_uppercase_plugin() {
    let p = UppercasePlugin;
    assert_eq!(p.execute("hello"), "HELLO");
    assert_eq!(p.name(), "uppercase");
}

#[test]
fn test_reverse_plugin() {
    let p = ReversePlugin;
    assert_eq!(p.execute("hello"), "olleh");
    assert_eq!(p.name(), "reverse");
}

#[test]
fn test_repeat_plugin() {
    let p = RepeatPlugin { times: 3 };
    assert_eq!(p.execute("ha"), "hahaha");
    assert_eq!(p.name(), "repeat");
}

// ===== Topic 6: Plugin Runner =====

#[test]
fn test_plugin_runner_empty() {
    let runner = PluginRunner::new();
    assert_eq!(runner.run("hello"), "hello");
    assert!(runner.plugin_names().is_empty());
}

#[test]
fn test_plugin_runner_chain() {
    let mut runner = PluginRunner::new();
    runner.register(Box::new(UppercasePlugin));
    runner.register(Box::new(ReversePlugin));
    assert_eq!(runner.run("hello"), "OLLEH");
}

#[test]
fn test_plugin_runner_names() {
    let mut runner = PluginRunner::new();
    runner.register(Box::new(UppercasePlugin));
    runner.register(Box::new(RepeatPlugin { times: 2 }));
    assert_eq!(runner.plugin_names(), vec!["uppercase", "repeat"]);
}

#[test]
fn test_plugin_runner_single() {
    let mut runner = PluginRunner::new();
    runner.register(Box::new(RepeatPlugin { times: 2 }));
    assert_eq!(runner.run("ab"), "abab");
}
