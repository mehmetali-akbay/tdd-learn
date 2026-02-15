use advanced_traits::*;

#[test]
fn test_named_list_container() {
    let l = NamedList::new("fruits", vec!["apple".into(), "banana".into()]);
    assert_eq!(l.len(), 2);
    assert!(!l.is_empty());
}
#[test]
fn test_named_list_items() {
    let l = NamedList::new("test", vec!["a".into()]);
    assert_eq!(l.items(), &["a".to_string()]);
}
#[test]
fn test_square_iter() {
    let s: Vec<u32> = SquareIter::new(5).collect();
    assert_eq!(s, vec![1, 4, 9, 16, 25]);
}
#[test]
fn test_square_iter_zero() {
    let s: Vec<u32> = SquareIter::new(0).collect();
    assert!(s.is_empty());
}
#[test]
fn test_vec2_add() {
    assert_eq!(
        Vec2::new(1.0, 2.0) + Vec2::new(3.0, 4.0),
        Vec2::new(4.0, 6.0)
    );
}
#[test]
fn test_vec2_sub() {
    assert_eq!(
        Vec2::new(5.0, 3.0) - Vec2::new(1.0, 1.0),
        Vec2::new(4.0, 2.0)
    );
}
#[test]
fn test_vec2_mul_scalar() {
    assert_eq!(Vec2::new(2.0, 3.0) * 2.0, Vec2::new(4.0, 6.0));
}
#[test]
fn test_vec2_neg() {
    assert_eq!(-Vec2::new(1.0, -2.0), Vec2::new(-1.0, 2.0));
}
#[test]
fn test_vec2_magnitude() {
    assert!((Vec2::new(3.0, 4.0).magnitude() - 5.0).abs() < 1e-6);
}
#[test]
fn test_matrix_index() {
    let m = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(1, 1)], 4.0);
}
#[test]
fn test_labeled_value_display() {
    let lv = LabeledValue::new("temp", 36.6);
    let d = lv.print_display();
    assert!(d.contains("temp"));
    assert!(d.contains("36.6"));
}
#[test]
fn test_labeled_value_debug() {
    let lv = LabeledValue::new("score", 100.0);
    assert!(lv.print_debug().contains("LabeledValue"));
}
#[test]
fn test_wrapper_display() {
    let w = Wrapper(vec!["hello".into(), "world".into()]);
    let d = format!("{}", w);
    assert!(d.contains("hello"));
    assert!(d.contains("world"));
}
#[test]
fn test_email_valid() {
    let e = Email::new("test@example.com").unwrap();
    assert_eq!(e.as_str(), "test@example.com");
    assert_eq!(e.domain(), "example.com");
}
#[test]
fn test_email_invalid() {
    assert!(Email::new("not-email").is_err());
}
#[test]
fn test_meters_from_km() {
    let m: Meters = Kilometers(1.0).into();
    assert!((m.0 - 1000.0).abs() < 1e-6);
}
#[test]
fn test_km_from_meters() {
    let km: Kilometers = Meters(500.0).into();
    assert!((km.0 - 0.5).abs() < 1e-6);
}
#[test]
fn test_meters_add() {
    assert!((Meters(100.0) + Meters(200.0)).0 - 300.0 < 1e-6);
}
#[test]
fn test_human_fly() {
    let h = Human;
    assert!(h.fly().contains("waving"));
}
#[test]
fn test_pilot_fly() {
    let h = Human;
    assert!(Pilot::fly(&h).contains("cockpit"));
}
#[test]
fn test_wizard_fly() {
    let h = Human;
    assert!(Wizard::fly(&h).contains("magic"));
}
#[test]
fn test_dog_name_inherent() {
    assert!(Dog::name().contains("dog"));
}
#[test]
fn test_dog_name_animal() {
    assert!(<Dog as Animal>::name().contains("animal"));
}
#[test]
fn test_summarize_blanket() {
    assert_eq!(42.summarize(), "Summary: 42");
}
#[test]
fn test_summarize_string() {
    assert_eq!("hello".to_string().summarize(), "Summary: hello");
}
#[test]
fn test_point_display() {
    let p = Point::new(1.0, 2.0);
    assert!(p.summarize().contains("1") && p.summarize().contains("2"));
}
#[test]
fn test_pretty_print_blanket() {
    let p = Point::new(1.0, 2.0);
    assert!(p.pretty().contains("Point"));
}
