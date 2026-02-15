// ============================================
// Level 2, Project 1: Shapes — Structs & Methods
// Learn: Defining structs, impl blocks, methods
// ============================================

// ============================================
// Topic 1: Point — Struct Basics
// Learn: Struct definition, new(), methods, associated functions
// ============================================

/// A 2D point with x and y coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a point at the origin (0, 0)
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Calculate the distance from this point to the origin
    pub fn distance_to_origin(&self) -> f64 {
        self.x.hypot(self.y)
    }

    /// Calculate the distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Return a new point shifted by dx and dy
    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    /// Return the midpoint between this point and another
    pub fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    /// Return a new point reflected across the origin (negate both coordinates)
    pub fn reflect(&self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ============================================
// Topic 2: Rectangle
// Learn: Methods with &self, constructor invariants
// ============================================

/// A rectangle defined by width and height
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    /// Create a new rectangle.
    /// Panics if width <= 0.0 or height <= 0.0.
    pub fn new(width: f64, height: f64) -> Self {
        assert!(width > 0.0, "width must be > 0.0");
        assert!(height > 0.0, "height must be > 0.0");
        Self { width, height }
    }

    /// Create a square with the given side length
    pub fn square(side: f64) -> Self {
        Self::new(side, side)
    }

    /// Calculate the area
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Calculate the perimeter
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    /// Check if this rectangle is a square
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    /// Check if a point lies inside this rectangle (origin at 0,0)
    pub fn contains_point(&self, p: &Point) -> bool {
        p.x >= 0.0 && p.x <= self.width && p.y >= 0.0 && p.y <= self.height
    }

    /// Return a new rectangle scaled by a factor.
    /// Panics if factor <= 0.0.
    pub fn scale(&self, factor: f64) -> Rectangle {
        assert!(factor > 0.0, "factor must be > 0.0");
        Self::new(self.width * factor, self.height * factor)
    }

    /// Check if this rectangle can fit inside another
    pub fn fits_inside(&self, other: &Rectangle) -> bool {
        self.width <= other.width && self.height <= other.height
    }

    /// Calculate the diagonal length of the rectangle
    pub fn diagonal(&self) -> f64 {
        (self.width * self.width + self.height * self.height).sqrt()
    }
}

// ============================================
// Topic 3: Circle
// Learn: std::f64::consts::PI, floating-point comparisons
// ============================================

use std::f64::consts::PI;

/// A circle with center and radius
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    /// Create a new circle.
    /// Panics if radius <= 0.0.
    pub fn new(center: Point, radius: f64) -> Self {
        assert!(radius > 0.0, "radius must be > 0.0");
        Self { center, radius }
    }

    /// Create a unit circle at the origin
    pub fn unit() -> Self {
        Self::new(Point::origin(), 1.0)
    }

    /// Calculate the area
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// Calculate the circumference
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    /// Check if a point is inside the circle
    pub fn contains_point(&self, p: &Point) -> bool {
        self.center.distance_to(p) <= self.radius
    }

    /// Check if this is a unit circle (radius 1, centered at origin)
    pub fn is_unit_circle(&self) -> bool {
        (self.radius - 1.0).abs() < f64::EPSILON
            && self.center.x.abs() < f64::EPSILON
            && self.center.y.abs() < f64::EPSILON
    }

    /// Return the diameter
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    /// Return a new circle scaled by the given factor (same center, scaled radius).
    /// Panics if factor <= 0.0.
    pub fn scale(&self, factor: f64) -> Circle {
        assert!(factor > 0.0, "factor must be > 0.0");
        Self::new(self.center, self.radius * factor)
    }
}

// ============================================
// Topic 4: Triangle
// Learn: Validation logic, Heron's formula, classification
// ============================================

/// Triangle classification
#[derive(Debug, PartialEq)]
pub enum TriangleKind {
    Equilateral,
    Isosceles,
    Scalene,
}

/// A triangle defined by three side lengths
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Triangle {
    /// Create a new triangle (does NOT validate)
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Triangle { a, b, c }
    }

    /// Check if the three sides form a valid triangle (triangle inequality)
    pub fn is_valid(&self) -> bool {
        self.a > 0.0
            && self.b > 0.0
            && self.c > 0.0
            && self.a + self.b > self.c
            && self.a + self.c > self.b
            && self.b + self.c > self.a
    }

    /// Calculate the perimeter
    pub fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }

    /// Calculate the area using Heron's formula
    pub fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    /// Classify the triangle
    pub fn classify(&self) -> TriangleKind {
        if (self.a - self.b).abs() < f64::EPSILON && (self.b - self.c).abs() < f64::EPSILON {
            TriangleKind::Equilateral
        } else if (self.a - self.b).abs() < f64::EPSILON
            || (self.b - self.c).abs() < f64::EPSILON
            || (self.a - self.c).abs() < f64::EPSILON
        {
            TriangleKind::Isosceles
        } else {
            TriangleKind::Scalene
        }
    }

    /// Check if the triangle is a right triangle (a² + b² = c²) for any arrangement
    pub fn is_right(&self) -> bool {
        let mut sides = [self.a, self.b, self.c];
        sides.sort_by(|x, y| x.partial_cmp(y).unwrap());
        (sides[0].powi(2) + sides[1].powi(2) - sides[2].powi(2)).abs() < 1e-9
    }
}

// ============================================
// Topic 5: Color RGB
// Learn: Tuple-like struct, formatting, methods
// ============================================

/// An RGB color with values 0–255
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Black (0, 0, 0)
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    /// White (255, 255, 255)
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    /// Invert the color
    pub fn invert(&self) -> Color {
        Color {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
        }
    }

    /// Convert to hex string: "#RRGGBB"
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Calculate brightness (0.0 - 1.0) using perceived luminance
    /// Formula: (0.299*R + 0.587*G + 0.114*B) / 255
    pub fn brightness(&self) -> f64 {
        (0.299 * self.r as f64 + 0.587 * self.g as f64 + 0.114 * self.b as f64) / 255.0
    }

    /// Check if the color is dark (brightness < 0.5)
    pub fn is_dark(&self) -> bool {
        self.brightness() < 0.5
    }

    /// Blend two colors by averaging their components
    pub fn mix(&self, other: &Color) -> Color {
        Color {
            r: ((self.r as u16 + other.r as u16) / 2) as u8,
            g: ((self.g as u16 + other.g as u16) / 2) as u8,
            b: ((self.b as u16 + other.b as u16) / 2) as u8,
        }
    }

    /// Create a grayscale version using brightness
    pub fn grayscale(&self) -> Color {
        let gray = (0.299 * self.r as f64 + 0.587 * self.g as f64 + 0.114 * self.b as f64) as u8;
        Color {
            r: gray,
            g: gray,
            b: gray,
        }
    }

    /// Parse a hex color string like "#FF8000" or "#ff8000" into a Color
    /// Returns None if the format is invalid
    pub fn from_hex(hex: &str) -> Option<Color> {
        let hex = hex.strip_prefix('#')?;
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color { r, g, b })
    }
}

// ============================================
// Topic 6: BoundingBox — Advanced Struct Composition
// Learn: Struct composition, iterator-based constructors
// ============================================

/// An axis-aligned bounding box
#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    /// Create a bounding box from two corners.
    /// Normalizes the points so min.x <= max.x and min.y <= max.y.
    pub fn new(min: Point, max: Point) -> Self {
        Self {
            min: Point::new(min.x.min(max.x), min.y.min(max.y)),
            max: Point::new(min.x.max(max.x), min.y.max(max.y)),
        }
    }

    /// Create the smallest bounding box containing all given points
    /// Returns None if the slice is empty
    pub fn from_points(points: &[Point]) -> Option<BoundingBox> {
        let first = *points.first()?;
        let (min, max) = points.iter().copied().skip(1).fold((first, first), |(min, max), p| {
            (
                Point::new(min.x.min(p.x), min.y.min(p.y)),
                Point::new(max.x.max(p.x), max.y.max(p.y)),
            )
        });
        Some(Self::new(min, max))
    }

    /// Width of the bounding box
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    /// Height of the bounding box
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    /// Area of the bounding box
    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }

    /// Center point of the bounding box
    pub fn center(&self) -> Point {
        self.min.midpoint(&self.max)
    }

    /// Check if a point is inside the bounding box
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Check if two bounding boxes overlap
    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Merge two bounding boxes into one that contains both
    pub fn merge(&self, other: &BoundingBox) -> BoundingBox {
        Self::new(
            Point::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            Point::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        )
    }
}
