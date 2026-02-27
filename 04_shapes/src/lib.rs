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
        Self {x: 0.0, y: 0.0}
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
        todo!()
    }

    /// Return the midpoint between this point and another
    pub fn midpoint(&self, other: &Point) -> Point {
        todo!()
    }

    /// Return a new point reflected across the origin (negate both coordinates)
    pub fn reflect(&self) -> Point {
        todo!()
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
        todo!()
    }

    /// Create a square with the given side length
    pub fn square(side: f64) -> Self {
        todo!()
    }

    /// Calculate the area
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// Calculate the perimeter
    pub fn perimeter(&self) -> f64 {
        todo!()
    }

    /// Check if this rectangle is a square
    pub fn is_square(&self) -> bool {
        todo!()
    }

    /// Check if a point lies inside this rectangle (origin at 0,0)
    pub fn contains_point(&self, p: &Point) -> bool {
        todo!()
    }

    /// Return a new rectangle scaled by a factor.
    /// Panics if factor <= 0.0.
    pub fn scale(&self, factor: f64) -> Rectangle {
        todo!()
    }

    /// Check if this rectangle can fit inside another
    pub fn fits_inside(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// Calculate the diagonal length of the rectangle
    pub fn diagonal(&self) -> f64 {
        todo!()
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
        todo!()
    }

    /// Create a unit circle at the origin
    pub fn unit() -> Self {
        todo!()
    }

    /// Calculate the area
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// Calculate the circumference
    pub fn circumference(&self) -> f64 {
        todo!()
    }

    /// Check if a point is inside the circle
    pub fn contains_point(&self, p: &Point) -> bool {
        todo!()
    }

    /// Check if this is a unit circle (radius 1, centered at origin)
    pub fn is_unit_circle(&self) -> bool {
        todo!()
    }

    /// Return the diameter
    pub fn diameter(&self) -> f64 {
        todo!()
    }

    /// Return a new circle scaled by the given factor (same center, scaled radius).
    /// Panics if factor <= 0.0.
    pub fn scale(&self, factor: f64) -> Circle {
        todo!()
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
        todo!()
    }

    /// Check if the three sides form a valid triangle (triangle inequality)
    pub fn is_valid(&self) -> bool {
        todo!()
    }

    /// Calculate the perimeter
    pub fn perimeter(&self) -> f64 {
        todo!()
    }

    /// Calculate the area using Heron's formula
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// Classify the triangle
    pub fn classify(&self) -> TriangleKind {
        todo!()
    }

    /// Check if the triangle is a right triangle (a² + b² = c²) for any arrangement
    pub fn is_right(&self) -> bool {
        todo!()
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
        todo!()
    }

    /// Black (0, 0, 0)
    pub fn black() -> Self {
        todo!()
    }

    /// White (255, 255, 255)
    pub fn white() -> Self {
        todo!()
    }

    /// Invert the color
    pub fn invert(&self) -> Color {
        todo!()
    }

    /// Convert to hex string: "#RRGGBB"
    pub fn to_hex(&self) -> String {
        todo!()
    }

    /// Calculate brightness (0.0 - 1.0) using perceived luminance
    /// Formula: (0.299*R + 0.587*G + 0.114*B) / 255
    pub fn brightness(&self) -> f64 {
        todo!()
    }

    /// Check if the color is dark (brightness < 0.5)
    pub fn is_dark(&self) -> bool {
        todo!()
    }

    /// Blend two colors by averaging their components
    pub fn mix(&self, other: &Color) -> Color {
        todo!()
    }

    /// Create a grayscale version using brightness
    pub fn grayscale(&self) -> Color {
        todo!()
    }

    /// Parse a hex color string like "#FF8000" or "#ff8000" into a Color
    /// Returns None if the format is invalid
    pub fn from_hex(hex: &str) -> Option<Color> {
        todo!()
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
        todo!()
    }

    /// Create the smallest bounding box containing all given points
    /// Returns None if the slice is empty
    pub fn from_points(points: &[Point]) -> Option<BoundingBox> {
        todo!()
    }

    /// Width of the bounding box
    pub fn width(&self) -> f64 {
        todo!()
    }

    /// Height of the bounding box
    pub fn height(&self) -> f64 {
        todo!()
    }

    /// Area of the bounding box
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// Center point of the bounding box
    pub fn center(&self) -> Point {
        todo!()
    }

    /// Check if a point is inside the bounding box
    pub fn contains(&self, p: &Point) -> bool {
        todo!()
    }

    /// Check if two bounding boxes overlap
    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        todo!()
    }

    /// Merge two bounding boxes into one that contains both
    pub fn merge(&self, other: &BoundingBox) -> BoundingBox {
        todo!()
    }
}
