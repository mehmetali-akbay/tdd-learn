use serde_learn::*;

#[test]
fn test_person_roundtrip() {
    let p = Person {
        name: "Alice".into(),
        age: 30,
        email: "alice@example.com".into(),
    };
    let j = person_to_json(&p);
    assert_eq!(json_to_person(&j).unwrap(), p);
}
#[test]
fn test_people_roundtrip() {
    let ps = vec![
        Person {
            name: "Alice".into(),
            age: 30,
            email: "a@b.com".into(),
        },
        Person {
            name: "Bob".into(),
            age: 25,
            email: "b@c.com".into(),
        },
    ];
    assert_eq!(json_to_people(&people_to_json(&ps)).unwrap(), ps);
}
#[test]
fn test_invalid_json() {
    assert!(json_to_person("not json").is_err());
}
#[test]
fn test_config_rename() {
    let c = Config {
        app_name: "MyApp".into(),
        port: 3000,
        debug: true,
        internal_id: "secret".into(),
    };
    let j = config_to_json(&c);
    assert!(j.contains("appName"));
    assert!(!j.contains("app_name"));
    assert!(!j.contains("secret"));
}
#[test]
fn test_config_defaults() {
    let c = json_to_config(r#"{"appName": "Test"}"#).unwrap();
    assert_eq!(c.port, 8080);
    assert!(!c.debug);
}
#[test]
fn test_shape_circle() {
    let s = Shape::Circle { radius: 5.0 };
    assert_eq!(json_to_shape(&shape_to_json(&s)).unwrap(), s);
}
#[test]
fn test_shape_rectangle() {
    let s = Shape::Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(json_to_shape(&shape_to_json(&s)).unwrap(), s);
}
#[test]
fn test_event_internally_tagged() {
    let e = Event::Click { x: 10, y: 20 };
    let j = event_to_json(&e);
    assert!(j.contains(r#""type":"click"#));
    assert_eq!(json_to_event(&j).unwrap(), e);
}
#[test]
fn test_value_untagged_int() {
    let v: Value = serde_json::from_str("42").unwrap();
    assert_eq!(v, Value::Integer(42));
}
#[test]
fn test_value_untagged_string() {
    let v: Value = serde_json::from_str(r#""hello""#).unwrap();
    assert_eq!(v, Value::Text("hello".into()));
}
#[test]
fn test_profile_with_address() {
    let p = UserProfile {
        username: "alice".into(),
        address: Some(Address {
            street: "123 Main".into(),
            city: "Springfield".into(),
            country: "US".into(),
        }),
        tags: vec!["admin".into(), "active".into()],
        metadata: Metadata {
            created_at: "2024-01-01".into(),
            version: 1,
        },
    };
    assert_eq!(json_to_profile(&profile_to_json(&p)).unwrap(), p);
}
#[test]
fn test_profile_without_address() {
    let p = UserProfile {
        username: "bob".into(),
        address: None,
        tags: vec![],
        metadata: Metadata {
            created_at: "2024-01-01".into(),
            version: 1,
        },
    };
    assert_eq!(json_to_profile(&profile_to_json(&p)).unwrap(), p);
}
#[test]
fn test_profile_flatten() {
    let p = UserProfile {
        username: "charlie".into(),
        address: None,
        tags: vec![],
        metadata: Metadata {
            created_at: "2024-01-01".into(),
            version: 2,
        },
    };
    let j = profile_to_json(&p);
    assert!(j.contains("created_at"));
    assert!(j.contains("version"));
}
#[test]
fn test_settings_custom() {
    let s = Settings {
        name: "test".into(),
        active: true,
        features: vec!["a".into(), "b".into(), "c".into()],
    };
    let j = settings_to_json(&s);
    assert!(j.contains("yes"));
    assert!(j.contains("a,b,c"));
}
#[test]
fn test_settings_roundtrip() {
    let s = Settings {
        name: "app".into(),
        active: false,
        features: vec!["x".into(), "y".into()],
    };
    assert_eq!(json_to_settings(&settings_to_json(&s)).unwrap(), s);
}
#[test]
fn test_app_config_json_roundtrip() {
    let c = AppConfig {
        title: "MyApp".into(),
        port: 8080,
        features: vec!["auth".into(), "logging".into()],
    };
    assert_eq!(json_to_app_config(&app_config_to_json(&c)).unwrap(), c);
}
#[test]
fn test_app_config_toml_roundtrip() {
    let c = AppConfig {
        title: "MyApp".into(),
        port: 3000,
        features: vec!["cache".into()],
    };
    assert_eq!(toml_to_app_config(&app_config_to_toml(&c)).unwrap(), c);
}
#[test]
fn test_json_and_toml_same_struct() {
    let c = AppConfig {
        title: "Test".into(),
        port: 9090,
        features: vec![],
    };
    assert_eq!(
        json_to_app_config(&app_config_to_json(&c)).unwrap(),
        toml_to_app_config(&app_config_to_toml(&c)).unwrap()
    );
}
