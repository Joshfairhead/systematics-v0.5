//! Entry types for the property graph.
//!
//! Layered ontology (from the plan):
//!
//! ```text
//! Order, Position           ← numeric substrate anchors
//! Point, Line               ← topological anchors (1-vertex and 2-vertex)
//! Coordinate, Segment       ← geometric bindings to Point / Line
//! Character                 ← content-only semantic value (no anchor field)
//! ```
//!
//! The `Entry` sum type stores all of these heterogeneously in one collection,
//! enabling unified iteration and queries across all entry kinds.
//!
//! Vocabulary layer (`TopologicalVocabulary`, `GeometricVocabulary`,
//! `SemanticVocabulary`) and Grammar live in their own modules and hold
//! *references* into this substrate.

use serde::{Deserialize, Serialize};

// =============================================================================
// Geometric value type
// =============================================================================

/// 3D vector value used inside `Coordinate` (not a graph entry itself).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

// =============================================================================
// Numeric substrate anchors
// =============================================================================

/// Order: the system level (1-12).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub value: u8,
}

impl Order {
    pub fn new(value: u8) -> Self {
        Self {
            id: format!("order_{}", value),
            value,
        }
    }

    /// Get the standard name for this order.
    pub fn standard_name(&self) -> Option<&'static str> {
        match self.value {
            1 => Some("Monad"),
            2 => Some("Dyad"),
            3 => Some("Triad"),
            4 => Some("Tetrad"),
            5 => Some("Pentad"),
            6 => Some("Hexad"),
            7 => Some("Heptad"),
            8 => Some("Octad"),
            9 => Some("Ennead"),
            10 => Some("Decad"),
            11 => Some("Undecad"),
            12 => Some("Dodecad"),
            _ => None,
        }
    }
}

/// Position: abstract "n-th place" (1-12).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub value: u8,
}

impl Position {
    pub fn new(value: u8) -> Self {
        Self {
            id: format!("position_{}", value),
            value,
        }
    }
}

// =============================================================================
// Topological anchors: Point and Line
// =============================================================================

/// A topological anchor at a single vertex of the K_n system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub id: String,       // "point_{order}_{position}"
    pub order: String,    // "order_{order}"
    pub position: String, // "position_{position}"
}

impl Point {
    pub fn new(order: u8, position: u8) -> Self {
        Self {
            id: format!("point_{}_{}", order, position),
            order: format!("order_{}", order),
            position: format!("position_{}", position),
        }
    }

    pub fn order_value(&self) -> Option<u8> {
        self.order
            .strip_prefix("order_")
            .and_then(|s| s.parse().ok())
    }

    pub fn position_value(&self) -> Option<u8> {
        self.position
            .strip_prefix("position_")
            .and_then(|s| s.parse().ok())
    }
}

/// A topological anchor at an edge (two vertices) of the K_n system.
///
/// Canonical form: `position <= position_secondary`. `Line::new(3, 2, 1)`
/// produces the same ID as `Line::new(3, 1, 2)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line {
    pub id: String,                 // "line_{order}_{p1}_{p2}" with p1 < p2
    pub order: String,              // "order_{order}"
    pub position: String,           // "position_{p1}"
    pub position_secondary: String, // "position_{p2}"
}

impl Line {
    pub fn new(order: u8, p1: u8, p2: u8) -> Self {
        let (lo, hi) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
        Self {
            id: format!("line_{}_{}_{}", order, lo, hi),
            order: format!("order_{}", order),
            position: format!("position_{}", lo),
            position_secondary: format!("position_{}", hi),
        }
    }

    pub fn order_value(&self) -> Option<u8> {
        self.order
            .strip_prefix("order_")
            .and_then(|s| s.parse().ok())
    }

    pub fn position_value(&self) -> Option<u8> {
        self.position
            .strip_prefix("position_")
            .and_then(|s| s.parse().ok())
    }

    pub fn position_secondary_value(&self) -> Option<u8> {
        self.position_secondary
            .strip_prefix("position_")
            .and_then(|s| s.parse().ok())
    }
}

// =============================================================================
// Geometric bindings: Coordinate and Segment
// =============================================================================

/// A geometric point bound to a topological `Point`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    pub id: String,        // "coord_{order}_{position}"
    pub point_ref: String, // "point_{order}_{position}"
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinate {
    pub fn new(order: u8, position: u8, x: f64, y: f64, z: f64) -> Self {
        Self {
            id: format!("coord_{}_{}", order, position),
            point_ref: format!("point_{}_{}", order, position),
            x,
            y,
            z,
        }
    }

    pub fn from_point3d(order: u8, position: u8, p: Point3d) -> Self {
        Self::new(order, position, p.x, p.y, p.z)
    }

    pub fn order_value(&self) -> Option<u8> {
        self.point_ref
            .strip_prefix("point_")
            .and_then(|s| s.split('_').next())
            .and_then(|s| s.parse().ok())
    }

    pub fn position_value(&self) -> Option<u8> {
        self.point_ref
            .strip_prefix("point_")
            .and_then(|s| s.split('_').nth(1))
            .and_then(|s| s.parse().ok())
    }
}

/// A geometric segment bound to a `Line`, referencing its endpoint coordinates.
/// Explicit type gives future rendering metadata (style, curve) a home.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,              // "seg_{order}_{p1}_{p2}"
    pub line_ref: String,        // "line_{order}_{p1}_{p2}"
    pub start_coord_ref: String, // "coord_{order}_{p1}"
    pub end_coord_ref: String,   // "coord_{order}_{p2}"
}

impl Segment {
    pub fn new(order: u8, p1: u8, p2: u8) -> Self {
        let (lo, hi) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
        Self {
            id: format!("seg_{}_{}_{}", order, lo, hi),
            line_ref: format!("line_{}_{}_{}", order, lo, hi),
            start_coord_ref: format!("coord_{}_{}", order, lo),
            end_coord_ref: format!("coord_{}_{}", order, hi),
        }
    }
}

// =============================================================================
// Semantic content: Character (content-only)
// =============================================================================

/// A semantic value. Where a Character "lives" is determined by which
/// SemanticVocabulary slot references its ID, not by any field on the
/// Character itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub id: String,    // "char_{slug}" or user-supplied
    pub kind: String,  // "word" | "hex" | "colour_name" | "number" | ...
    pub value: String, // stringified expression; parsed per kind
}

impl Character {
    pub fn new(id: impl Into<String>, kind: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: kind.into(),
            value: value.into(),
        }
    }

    /// Create a character with an auto-generated ID derived from kind + value.
    pub fn with_auto_id(kind: impl Into<String>, value: impl Into<String>) -> Self {
        let kind = kind.into();
        let value = value.into();
        let slug = value.to_lowercase().replace(' ', "_");
        Self {
            id: format!("char_{}_{}", kind, slug),
            kind,
            value,
        }
    }
}

// =============================================================================
// Entry sum type
// =============================================================================

/// Entry stores heterogeneous entries in a single collection, enabling the
/// Graph to hold all entry types in one `entries` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Entry {
    Order(Order),
    Position(Position),
    Point(Point),
    Line(Line),
    Coordinate(Coordinate),
    Segment(Segment),
    Character(Character),
}

impl Entry {
    /// Get the ID of this entry.
    pub fn id(&self) -> &str {
        match self {
            Entry::Order(e) => &e.id,
            Entry::Position(e) => &e.id,
            Entry::Point(e) => &e.id,
            Entry::Line(e) => &e.id,
            Entry::Coordinate(e) => &e.id,
            Entry::Segment(e) => &e.id,
            Entry::Character(e) => &e.id,
        }
    }

    /// Get the order value of this entry (if applicable).
    pub fn order(&self) -> Option<u8> {
        match self {
            Entry::Order(e) => Some(e.value),
            Entry::Position(_) => None,
            Entry::Point(e) => e.order_value(),
            Entry::Line(e) => e.order_value(),
            Entry::Coordinate(e) => e.order_value(),
            Entry::Segment(_) => None, // derivable via line_ref if needed
            Entry::Character(_) => None,
        }
    }

    /// Get the position value of this entry (if applicable).
    pub fn position(&self) -> Option<u8> {
        match self {
            Entry::Position(e) => Some(e.value),
            Entry::Point(e) => e.position_value(),
            Entry::Line(e) => e.position_value(),
            Entry::Coordinate(e) => e.position_value(),
            _ => None,
        }
    }

    /// True if this is a numeric substrate anchor (Order or Position).
    pub fn is_numeric_anchor(&self) -> bool {
        matches!(self, Entry::Order(_) | Entry::Position(_))
    }

    /// True if this is a topological anchor (Point or Line).
    pub fn is_topological(&self) -> bool {
        matches!(self, Entry::Point(_) | Entry::Line(_))
    }

    /// True if this is a geometric anchor (Coordinate or Segment).
    pub fn is_geometric(&self) -> bool {
        matches!(self, Entry::Coordinate(_) | Entry::Segment(_))
    }

    /// True if this is semantic content (Character).
    pub fn is_semantic(&self) -> bool {
        matches!(self, Entry::Character(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let order = Order::new(3);
        assert_eq!(order.id, "order_3");
        assert_eq!(order.value, 3);
        assert_eq!(order.standard_name(), Some("Triad"));
    }

    #[test]
    fn test_position_creation() {
        let pos = Position::new(1);
        assert_eq!(pos.id, "position_1");
        assert_eq!(pos.value, 1);
    }

    #[test]
    fn test_point_creation() {
        let p = Point::new(3, 1);
        assert_eq!(p.id, "point_3_1");
        assert_eq!(p.order_value(), Some(3));
        assert_eq!(p.position_value(), Some(1));
    }

    #[test]
    fn test_line_canonical_order() {
        let a = Line::new(3, 1, 2);
        let b = Line::new(3, 2, 1);
        assert_eq!(a.id, "line_3_1_2");
        assert_eq!(a, b);
        assert_eq!(a.position_value(), Some(1));
        assert_eq!(a.position_secondary_value(), Some(2));
    }

    #[test]
    fn test_coordinate_from_point3d() {
        let c = Coordinate::from_point3d(3, 1, Point3d::new(0.0, 1.0, 0.0));
        assert_eq!(c.id, "coord_3_1");
        assert_eq!(c.point_ref, "point_3_1");
    }

    #[test]
    fn test_segment_canonical() {
        let s = Segment::new(3, 2, 1);
        assert_eq!(s.id, "seg_3_1_2");
        assert_eq!(s.line_ref, "line_3_1_2");
        assert_eq!(s.start_coord_ref, "coord_3_1");
        assert_eq!(s.end_coord_ref, "coord_3_2");
    }

    #[test]
    fn test_character_content_only() {
        let c = Character::with_auto_id("word", "Will");
        assert_eq!(c.id, "char_word_will");
        assert_eq!(c.kind, "word");
        assert_eq!(c.value, "Will");
    }

    #[test]
    fn test_entry_categorisation() {
        assert!(Entry::Order(Order::new(3)).is_numeric_anchor());
        assert!(Entry::Point(Point::new(3, 1)).is_topological());
        assert!(Entry::Line(Line::new(3, 1, 2)).is_topological());
        assert!(Entry::Coordinate(Coordinate::new(3, 1, 0.0, 0.0, 0.0)).is_geometric());
        assert!(Entry::Segment(Segment::new(3, 1, 2)).is_geometric());
        assert!(Entry::Character(Character::with_auto_id("word", "Will")).is_semantic());
    }
}
