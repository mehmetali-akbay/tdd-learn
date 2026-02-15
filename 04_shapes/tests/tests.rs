use shapes::*;

const EPS: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPS,
        "expected {expected}, got {actual}"
    );
}

// ===== Topic 1: Point =====

#[test]
fn point_new_and_origin_store_coordinates() {
    let p = Point::new(3.0, -4.0);
    assert_eq!(p, Point { x: 3.0, y: -4.0 });
    assert_eq!(Point::origin(), Point::new(0.0, 0.0));
}

#[test]
fn point_distance_methods_are_consistent() {
    let p = Point::new(3.0, 4.0);
    assert_close(p.distance_to_origin(), 5.0);
    assert_close(Point::origin().distance_to(&p), 5.0);
}

#[test]
fn point_translate_returns_new_value_without_mutating_original() {
    let original = Point::new(1.0, 2.0);
    let moved = original.translate(5.0, -3.0);

    assert_eq!(original, Point::new(1.0, 2.0));
    assert_eq!(moved, Point::new(6.0, -1.0));
}

#[test]
fn point_midpoint_is_symmetric() {
    let a = Point::new(-2.0, 1.0);
    let b = Point::new(4.0, 7.0);

    assert_eq!(a.midpoint(&b), Point::new(1.0, 4.0));
    assert_eq!(a.midpoint(&b), b.midpoint(&a));
}

#[test]
fn point_reflect_is_involution() {
    let p = Point::new(5.5, -9.0);
    assert_eq!(p.reflect(), Point::new(-5.5, 9.0));
    assert_eq!(p.reflect().reflect(), p);
}

#[test]
fn point_display_is_readable() {
    let p = Point::new(1.25, -2.5);
    assert_eq!(format!("{p}"), "(1.25, -2.5)");
}

// ===== Topic 2: Rectangle =====

#[test]
#[should_panic]
fn rectangle_new_rejects_non_positive_width() {
    let _ = Rectangle::new(0.0, 2.0);
}

#[test]
#[should_panic]
fn rectangle_new_rejects_non_positive_height() {
    let _ = Rectangle::new(2.0, -1.0);
}

#[test]
fn rectangle_square_uses_same_side_for_both_fields() {
    let sq = Rectangle::square(4.0);
    assert_eq!(sq.width, 4.0);
    assert_eq!(sq.height, 4.0);
    assert!(sq.is_square());
}

#[test]
fn rectangle_area_perimeter_and_diagonal() {
    let rect = Rectangle::new(3.0, 4.0);
    assert_close(rect.area(), 12.0);
    assert_close(rect.perimeter(), 14.0);
    assert_close(rect.diagonal(), 5.0);
}

#[test]
fn rectangle_contains_point_includes_edges() {
    let rect = Rectangle::new(10.0, 8.0);
    assert!(rect.contains_point(&Point::new(0.0, 0.0)));
    assert!(rect.contains_point(&Point::new(10.0, 8.0)));
    assert!(!rect.contains_point(&Point::new(10.1, 8.0)));
    assert!(!rect.contains_point(&Point::new(-0.1, 0.0)));
}

#[test]
fn rectangle_scale_returns_new_instance() {
    let original = Rectangle::new(2.0, 3.0);
    let scaled = original.scale(2.5);

    assert_eq!(original, Rectangle::new(2.0, 3.0));
    assert_close(scaled.width, 5.0);
    assert_close(scaled.height, 7.5);
}

#[test]
#[should_panic]
fn rectangle_scale_rejects_non_positive_factor() {
    let _ = Rectangle::new(2.0, 3.0).scale(0.0);
}

#[test]
fn rectangle_fits_inside_checks_both_dimensions() {
    let small = Rectangle::new(3.0, 2.0);
    let large = Rectangle::new(5.0, 5.0);
    let too_tall = Rectangle::new(5.0, 6.0);

    assert!(small.fits_inside(&large));
    assert!(!too_tall.fits_inside(&large));
}

// ===== Topic 3: Circle =====

#[test]
#[should_panic]
fn circle_new_rejects_non_positive_radius() {
    let _ = Circle::new(Point::origin(), 0.0);
}

#[test]
fn circle_unit_is_centered_and_has_radius_one() {
    let unit = Circle::unit();
    assert_eq!(unit.center, Point::origin());
    assert_close(unit.radius, 1.0);
    assert!(unit.is_unit_circle());
}

#[test]
fn circle_area_circumference_and_diameter() {
    let c = Circle::new(Point::origin(), 2.0);
    assert_close(c.area(), 4.0 * std::f64::consts::PI);
    assert_close(c.circumference(), 4.0 * std::f64::consts::PI);
    assert_close(c.diameter(), 4.0);
}

#[test]
fn circle_contains_point_respects_center_offset() {
    let c = Circle::new(Point::new(5.0, -2.0), 3.0);
    assert!(c.contains_point(&Point::new(8.0, -2.0)));
    assert!(!c.contains_point(&Point::new(8.1, -2.0)));
}

#[test]
fn circle_scale_keeps_center_and_returns_new_value() {
    let original = Circle::new(Point::new(1.0, 2.0), 4.0);
    let scaled = original.scale(0.5);

    assert_eq!(scaled.center, original.center);
    assert_close(scaled.radius, 2.0);
    assert_close(original.radius, 4.0);
}

#[test]
#[should_panic]
fn circle_scale_rejects_non_positive_factor() {
    let _ = Circle::new(Point::origin(), 1.0).scale(-1.0);
}

// ===== Topic 4: Triangle =====

#[test]
fn triangle_validity_covers_basic_invalid_cases() {
    assert!(Triangle::new(3.0, 4.0, 5.0).is_valid());
    assert!(!Triangle::new(1.0, 2.0, 10.0).is_valid());
    assert!(!Triangle::new(0.0, 4.0, 5.0).is_valid());
}

#[test]
fn triangle_perimeter_and_area_for_three_four_five() {
    let t = Triangle::new(3.0, 4.0, 5.0);
    assert_close(t.perimeter(), 12.0);
    assert_close(t.area(), 6.0);
}

#[test]
fn triangle_classification() {
    assert_eq!(Triangle::new(5.0, 5.0, 5.0).classify(), TriangleKind::Equilateral);
    assert_eq!(Triangle::new(5.0, 5.0, 3.0).classify(), TriangleKind::Isosceles);
    assert_eq!(Triangle::new(3.0, 4.0, 5.0).classify(), TriangleKind::Scalene);
}

#[test]
fn triangle_right_check_is_order_independent() {
    assert!(Triangle::new(3.0, 4.0, 5.0).is_right());
    assert!(Triangle::new(5.0, 3.0, 4.0).is_right());
    assert!(!Triangle::new(4.0, 4.0, 4.0).is_right());
}

// ===== Topic 5: Color RGB =====

#[test]
fn color_constructors_and_invert_round_trip() {
    let black = Color::black();
    let white = Color::white();

    assert_eq!(black, Color::new(0, 0, 0));
    assert_eq!(white, Color::new(255, 255, 255));
    assert_eq!(black.invert(), white);
    assert_eq!(white.invert(), black);
}

#[test]
fn color_hex_round_trip_supports_upper_and_lower_case() {
    let color = Color::new(255, 128, 0);
    let parsed_upper = Color::from_hex("#FF8000");
    let parsed_lower = Color::from_hex("#ff8000");

    assert_eq!(color.to_hex(), "#FF8000");
    assert_eq!(parsed_upper, Some(color));
    assert_eq!(parsed_lower, Some(color));
}

#[test]
fn color_from_hex_rejects_invalid_inputs() {
    assert_eq!(Color::from_hex("FF8000"), None);
    assert_eq!(Color::from_hex("#FFF"), None);
    assert_eq!(Color::from_hex("#GG0000"), None);
}

#[test]
fn color_brightness_and_is_dark() {
    assert_close(Color::white().brightness(), 1.0);
    assert_close(Color::black().brightness(), 0.0);
    assert!(Color::new(10, 10, 10).is_dark());
    assert!(!Color::new(240, 240, 240).is_dark());
}

#[test]
fn color_mix_is_symmetric() {
    let a = Color::new(30, 80, 130);
    let b = Color::new(50, 120, 200);
    assert_eq!(a.mix(&b), b.mix(&a));
}

#[test]
fn color_grayscale_has_equal_channels() {
    let gray = Color::new(120, 40, 200).grayscale();
    assert_eq!(gray.r, gray.g);
    assert_eq!(gray.g, gray.b);
}

// ===== Topic 6: BoundingBox =====

#[test]
fn bbox_new_normalizes_min_and_max_points() {
    let bbox = BoundingBox::new(Point::new(8.0, -1.0), Point::new(2.0, 5.0));
    assert_eq!(bbox.min, Point::new(2.0, -1.0));
    assert_eq!(bbox.max, Point::new(8.0, 5.0));
}

#[test]
fn bbox_from_points_handles_empty_and_non_empty() {
    assert_eq!(BoundingBox::from_points(&[]), None);

    let points = [Point::new(1.0, 5.0), Point::new(-2.0, 3.0), Point::new(4.0, -1.0)];
    let bbox = BoundingBox::from_points(&points).unwrap();
    assert_eq!(bbox.min, Point::new(-2.0, -1.0));
    assert_eq!(bbox.max, Point::new(4.0, 5.0));
}

#[test]
fn bbox_width_height_area_and_center() {
    let bbox = BoundingBox::new(Point::new(1.0, 2.0), Point::new(5.0, 7.0));
    assert_close(bbox.width(), 4.0);
    assert_close(bbox.height(), 5.0);
    assert_close(bbox.area(), 20.0);
    assert_eq!(bbox.center(), Point::new(3.0, 4.5));
}

#[test]
fn bbox_contains_is_inclusive_on_edges() {
    let bbox = BoundingBox::new(Point::new(0.0, 0.0), Point::new(10.0, 10.0));
    assert!(bbox.contains(&Point::new(0.0, 0.0)));
    assert!(bbox.contains(&Point::new(10.0, 10.0)));
    assert!(!bbox.contains(&Point::new(10.1, 10.0)));
}

#[test]
fn bbox_overlaps_when_touching_edges() {
    let a = BoundingBox::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    let b = BoundingBox::new(Point::new(5.0, 2.0), Point::new(8.0, 4.0));
    let c = BoundingBox::new(Point::new(6.0, 6.0), Point::new(9.0, 9.0));

    assert!(a.overlaps(&b));
    assert!(!a.overlaps(&c));
}

#[test]
fn bbox_merge_spans_both_boxes() {
    let a = BoundingBox::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    let b = BoundingBox::new(Point::new(3.0, -2.0), Point::new(8.0, 8.0));
    let merged = a.merge(&b);

    assert_eq!(merged.min, Point::new(0.0, -2.0));
    assert_eq!(merged.max, Point::new(8.0, 8.0));
}
