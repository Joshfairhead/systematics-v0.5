//! Entry types for the property graph.
//!
//! This module implements a category-theoretic foundation with anchor types:
//! - Order: the system level (1-12)
//! - Position: abstract "n-th place" (1-12)
//! - Location: the pullback of Order × Position
//!
//! Other entries map TO these anchors:
//! - Order-level entries (SystemName, CoherenceAttribute, etc.) reference Order
//! - Location-level entries (Term, Coordinate, Colour) reference Location
//!
//! The Entry enum is a sum type for storing heterogeneous entries in a single collection,
//! enabling the Graph to hold all entry types in one `entries` field.

use serde::{Deserialize, Serialize};

use super::language::Language;

// =============================================================================
// Geometric Types
// =============================================================================

/// 3D point for geometric coordinates
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
// Anchor Types - The fundamental objects we map TO
// =============================================================================

/// Order: the system level (1-12)
/// Order-level entries (CoherenceAttribute, TermDesignation, etc.) reference this.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    /// System order value (1-12)
    pub value: u8,
}

impl Order {
    pub fn new(value: u8) -> Self {
        Self {
            id: format!("order_{}", value),
            value,
        }
    }

    /// Get the standard name for this order
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

/// Position: abstract "n-th place" (1-12)
/// Enables queries like "all position-1s across orders".
/// Position is abstract "first-ness", "second-ness", etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    /// Position value (1-12)
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

/// Location: the pullback of Order × Position
/// Location-level entries (Term, Coordinate, Colour) reference this.
/// A Location binds a specific position within a specific order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    /// References Order entry ID
    pub order: String,
    /// References Position entry ID
    pub position: String,
}

impl Location {
    pub fn new(order: u8, position: u8) -> Self {
        Self {
            id: format!("loc_{}_{}", order, position),
            order: format!("order_{}", order),
            position: format!("position_{}", position),
        }
    }

    /// Extract order value from order reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.order
            .strip_prefix("order_")
            .and_then(|s| s.parse().ok())
    }

    /// Extract position value from position reference ID
    pub fn position_value(&self) -> Option<u8> {
        self.position
            .strip_prefix("position_")
            .and_then(|s| s.parse().ok())
    }
}

// =============================================================================
// Semantic Content - Reusable vocabulary elements
// =============================================================================

/// Character is the semantic content, independent of structural position.
/// Same Character can appear as a Term (at a location) or referenced by a Connective (as a link).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    /// The vocabulary language (Canonical, Energy, Values, Society)
    pub language: Language,
    /// The semantic value (e.g., "Will", "Generation")
    pub value: String,
}

impl Character {
    pub fn new(id: impl Into<String>, language: Language, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            language,
            value: value.into(),
        }
    }

    /// Create a character with an auto-generated ID
    pub fn with_auto_id(language: Language, value: impl Into<String>) -> Self {
        let value = value.into();
        let id = format!(
            "char_{}_{}",
            language.to_string().to_lowercase(),
            value.to_lowercase().replace(' ', "_")
        );
        Self {
            id,
            language,
            value,
        }
    }
}

// =============================================================================
// Order-Level Entries - Reference Order anchor
// =============================================================================

/// SystemName provides the human-readable name for a system order.
/// For example, Order 3 is "Triad".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemName {
    pub id: String,
    /// References Order entry ID
    pub order: String,
    /// The system name (e.g., "Monad", "Dyad", "Triad")
    pub value: String,
}

impl SystemName {
    pub fn new(id: impl Into<String>, order: String, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a system name with an auto-generated ID for a given order value
    pub fn with_auto_id(order_value: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("system_{}", order_value),
            order: format!("order_{}", order_value),
            value: value.into(),
        }
    }

    /// Get the standard name for a given order
    pub fn standard_name(order: u8) -> Option<&'static str> {
        match order {
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

    /// Extract order value from order reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.order
            .strip_prefix("order_")
            .and_then(|s| s.parse().ok())
    }
}

/// CoherenceAttribute is a per-order attribute describing the coherence quality.
/// For example, Order 3 has coherence "Dynamism".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoherenceAttribute {
    pub id: String,
    /// References Order entry ID
    pub order: String,
    /// The coherence value (e.g., "Dynamism")
    pub value: String,
}

impl CoherenceAttribute {
    pub fn new(id: impl Into<String>, order: String, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a coherence attribute with an auto-generated ID for a given order value
    pub fn with_auto_id(order_value: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("coherence_{}", order_value),
            order: format!("order_{}", order_value),
            value: value.into(),
        }
    }

    /// Extract order value from order reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.order
            .strip_prefix("order_")
            .and_then(|s| s.parse().ok())
    }
}

/// TermDesignation is a per-order label that applies to all terms in a system.
/// For example, Order 3 terms are called "Impulses".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermDesignation {
    pub id: String,
    /// References Order entry ID
    pub order: String,
    /// The designation value (e.g., "Impulses", "Sources", "Limits")
    pub value: String,
}

impl TermDesignation {
    pub fn new(id: impl Into<String>, order: String, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a term designation with an auto-generated ID for a given order value
    pub fn with_auto_id(order_value: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("term_des_{}", order_value),
            order: format!("order_{}", order_value),
            value: value.into(),
        }
    }

    /// Extract order value from order reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.order
            .strip_prefix("order_")
            .and_then(|s| s.parse().ok())
    }
}

/// ConnectiveDesignation is a per-order label that applies to all connectives in a system.
/// For example, Order 3 connectives are called "Acts".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectiveDesignation {
    pub id: String,
    /// References Order entry ID
    pub order: String,
    /// The designation value (e.g., "Acts", "Interplays", "Steps")
    pub value: String,
}

impl ConnectiveDesignation {
    pub fn new(id: impl Into<String>, order: String, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a connective designation with an auto-generated ID for a given order value
    pub fn with_auto_id(order_value: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("conn_des_{}", order_value),
            order: format!("order_{}", order_value),
            value: value.into(),
        }
    }

    /// Extract order value from order reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.order
            .strip_prefix("order_")
            .and_then(|s| s.parse().ok())
    }
}

// =============================================================================
// Location-Level Entries - Reference Location anchor
// =============================================================================

/// Term is a positional entry referencing a Character.
/// Terms exist at a specific Location (order × position) within a system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Term {
    pub id: String,
    /// References Location entry ID
    pub location: String,
    /// ID of the Character entry this term references
    pub character: String,
}

impl Term {
    pub fn new(
        id: impl Into<String>,
        location: impl Into<String>,
        character: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            location: location.into(),
            character: character.into(),
        }
    }

    /// Create a term with an auto-generated ID for a given order and position
    pub fn with_auto_id(order: u8, position: u8, character: impl Into<String>) -> Self {
        let character = character.into();
        Self {
            id: format!("term_{}_{}", order, position),
            location: format!("loc_{}_{}", order, position),
            character,
        }
    }

    /// Extract order value from location reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.location
            .strip_prefix("loc_")
            .and_then(|s| s.split('_').next())
            .and_then(|s| s.parse().ok())
    }

    /// Extract position value from location reference ID
    pub fn position_value(&self) -> Option<u8> {
        self.location
            .strip_prefix("loc_")
            .and_then(|s| s.split('_').nth(1))
            .and_then(|s| s.parse().ok())
    }
}

/// Coordinate represents a 3D point at a specific Location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    pub id: String,
    /// References Location entry ID
    pub location: String,
    /// 3D coordinate value
    pub value: Point3d,
}

impl Coordinate {
    pub fn new(id: impl Into<String>, location: impl Into<String>, value: Point3d) -> Self {
        Self {
            id: id.into(),
            location: location.into(),
            value,
        }
    }

    /// Create a coordinate with an auto-generated ID for a given order and position
    pub fn with_auto_id(order: u8, position: u8, value: Point3d) -> Self {
        Self {
            id: format!("coord_{}_{}", order, position),
            location: format!("loc_{}_{}", order, position),
            value,
        }
    }

    /// Extract order value from location reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.location
            .strip_prefix("loc_")
            .and_then(|s| s.split('_').next())
            .and_then(|s| s.parse().ok())
    }

    /// Extract position value from location reference ID
    pub fn position_value(&self) -> Option<u8> {
        self.location
            .strip_prefix("loc_")
            .and_then(|s| s.split('_').nth(1))
            .and_then(|s| s.parse().ok())
    }
}

/// Colour represents a color value at a specific Location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Colour {
    pub id: String,
    /// References Location entry ID
    pub location: String,
    /// Representation language (Hex or Name)
    pub language: Language,
    /// The color value (e.g., "#FF0000" or "Red")
    pub value: String,
}

impl Colour {
    pub fn new(
        id: impl Into<String>,
        location: impl Into<String>,
        language: Language,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            location: location.into(),
            language,
            value: value.into(),
        }
    }

    /// Create a colour with an auto-generated ID for a given order and position
    pub fn with_auto_id(
        order: u8,
        position: u8,
        language: Language,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: format!(
                "colour_{}_{}_{}",
                order,
                position,
                language.to_string().to_lowercase()
            ),
            location: format!("loc_{}_{}", order, position),
            language,
            value: value.into(),
        }
    }

    /// Extract order value from location reference ID
    pub fn order_value(&self) -> Option<u8> {
        self.location
            .strip_prefix("loc_")
            .and_then(|s| s.split('_').next())
            .and_then(|s| s.parse().ok())
    }

    /// Extract position value from location reference ID
    pub fn position_value(&self) -> Option<u8> {
        self.location
            .strip_prefix("loc_")
            .and_then(|s| s.split('_').nth(1))
            .and_then(|s| s.parse().ok())
    }
}

// =============================================================================
// Entry Sum Type
// =============================================================================

/// Entry is a sum type for storing heterogeneous entries in a single collection.
/// This enables the Graph to hold all entry types in one `entries` field,
/// allowing unified iteration and queries across all entry kinds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Entry {
    // Anchor types - the fundamental objects we map TO
    Order(Order),
    Position(Position),
    Location(Location),

    // Order-level entries (reference Order)
    SystemName(SystemName),
    CoherenceAttribute(CoherenceAttribute),
    TermDesignation(TermDesignation),
    ConnectiveDesignation(ConnectiveDesignation),

    // Location-level entries (reference Location)
    Term(Term),
    Colour(Colour),
    Coordinate(Coordinate),

    // Semantic content (reusable)
    Character(Character),
}

impl Entry {
    /// Get the ID of this entry
    pub fn id(&self) -> &str {
        match self {
            Entry::Order(e) => &e.id,
            Entry::Position(e) => &e.id,
            Entry::Location(e) => &e.id,
            Entry::SystemName(e) => &e.id,
            Entry::CoherenceAttribute(e) => &e.id,
            Entry::TermDesignation(e) => &e.id,
            Entry::ConnectiveDesignation(e) => &e.id,
            Entry::Term(e) => &e.id,
            Entry::Colour(e) => &e.id,
            Entry::Coordinate(e) => &e.id,
            Entry::Character(e) => &e.id,
        }
    }

    /// Get the order value of this entry (if applicable)
    /// For anchor types, returns the value directly.
    /// For order-level entries, extracts from order reference.
    /// For location-level entries, extracts from location reference.
    pub fn order(&self) -> Option<u8> {
        match self {
            Entry::Order(e) => Some(e.value),
            Entry::Position(_) => None,
            Entry::Location(e) => e.order_value(),
            Entry::SystemName(e) => e.order_value(),
            Entry::CoherenceAttribute(e) => e.order_value(),
            Entry::TermDesignation(e) => e.order_value(),
            Entry::ConnectiveDesignation(e) => e.order_value(),
            Entry::Term(e) => e.order_value(),
            Entry::Colour(e) => e.order_value(),
            Entry::Coordinate(e) => e.order_value(),
            Entry::Character(_) => None,
        }
    }

    /// Get the position value of this entry (if applicable)
    /// Only location-level entries and Location anchor have positions.
    pub fn position(&self) -> Option<u8> {
        match self {
            Entry::Position(e) => Some(e.value),
            Entry::Location(e) => e.position_value(),
            Entry::Term(e) => e.position_value(),
            Entry::Colour(e) => e.position_value(),
            Entry::Coordinate(e) => e.position_value(),
            _ => None,
        }
    }

    /// Check if this entry is an anchor type
    pub fn is_anchor(&self) -> bool {
        matches!(
            self,
            Entry::Order(_) | Entry::Position(_) | Entry::Location(_)
        )
    }

    /// Check if this entry is order-level (references Order, no position)
    pub fn is_order_level(&self) -> bool {
        matches!(
            self,
            Entry::SystemName(_)
                | Entry::CoherenceAttribute(_)
                | Entry::TermDesignation(_)
                | Entry::ConnectiveDesignation(_)
        )
    }

    /// Check if this entry is location-level (references Location)
    pub fn is_location_level(&self) -> bool {
        matches!(
            self,
            Entry::Term(_) | Entry::Colour(_) | Entry::Coordinate(_)
        )
    }

    /// Check if this entry is semantic content
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
    fn test_location_creation() {
        let loc = Location::new(3, 1);
        assert_eq!(loc.id, "loc_3_1");
        assert_eq!(loc.order, "order_3");
        assert_eq!(loc.position, "position_1");
        assert_eq!(loc.order_value(), Some(3));
        assert_eq!(loc.position_value(), Some(1));
    }

    #[test]
    fn test_character_creation() {
        let char = Character::with_auto_id(Language::Canonical, "Will");
        assert_eq!(char.id, "char_canonical_will");
        assert_eq!(char.value, "Will");
        assert_eq!(char.language, Language::Canonical);
    }

    #[test]
    fn test_term_creation() {
        let term = Term::with_auto_id(3, 1, "char_will");
        assert_eq!(term.id, "term_3_1");
        assert_eq!(term.location, "loc_3_1");
        assert_eq!(term.order_value(), Some(3));
        assert_eq!(term.position_value(), Some(1));
    }

    #[test]
    fn test_coordinate_creation() {
        let coord = Coordinate::with_auto_id(3, 1, Point3d::new(0.0, 1.0, 0.0));
        assert_eq!(coord.id, "coord_3_1");
        assert_eq!(coord.location, "loc_3_1");
        assert_eq!(coord.order_value(), Some(3));
        assert_eq!(coord.position_value(), Some(1));
    }

    #[test]
    fn test_colour_creation() {
        let colour = Colour::with_auto_id(3, 1, Language::Hex, "#FF0000");
        assert_eq!(colour.id, "colour_3_1_hex");
        assert_eq!(colour.location, "loc_3_1");
        assert_eq!(colour.order_value(), Some(3));
        assert_eq!(colour.position_value(), Some(1));
    }

    #[test]
    fn test_system_name_with_order_ref() {
        let sn = SystemName::with_auto_id(3, "Triad");
        assert_eq!(sn.id, "system_3");
        assert_eq!(sn.order, "order_3");
        assert_eq!(sn.order_value(), Some(3));
    }

    #[test]
    fn test_coherence_with_order_ref() {
        let coh = CoherenceAttribute::with_auto_id(3, "Dynamism");
        assert_eq!(coh.id, "coherence_3");
        assert_eq!(coh.order, "order_3");
        assert_eq!(coh.order_value(), Some(3));
    }

    #[test]
    fn test_entry_categorization() {
        let order = Entry::Order(Order::new(3));
        let location = Entry::Location(Location::new(3, 1));
        let system_name = Entry::SystemName(SystemName::with_auto_id(3, "Triad"));
        let term = Entry::Term(Term::with_auto_id(3, 1, "char_will"));
        let char = Entry::Character(Character::with_auto_id(Language::Canonical, "Will"));

        assert!(order.is_anchor());
        assert!(location.is_anchor());
        assert!(system_name.is_order_level());
        assert!(term.is_location_level());
        assert!(char.is_semantic());
    }

    #[test]
    fn test_entry_order_extraction() {
        let order = Entry::Order(Order::new(3));
        let location = Entry::Location(Location::new(3, 1));
        let system_name = Entry::SystemName(SystemName::with_auto_id(3, "Triad"));
        let term = Entry::Term(Term::with_auto_id(3, 1, "char_will"));

        assert_eq!(order.order(), Some(3));
        assert_eq!(location.order(), Some(3));
        assert_eq!(system_name.order(), Some(3));
        assert_eq!(term.order(), Some(3));
    }

    #[test]
    fn test_entry_position_extraction() {
        let position = Entry::Position(Position::new(1));
        let location = Entry::Location(Location::new(3, 1));
        let term = Entry::Term(Term::with_auto_id(3, 1, "char_will"));
        let system_name = Entry::SystemName(SystemName::with_auto_id(3, "Triad"));

        assert_eq!(position.position(), Some(1));
        assert_eq!(location.position(), Some(1));
        assert_eq!(term.position(), Some(1));
        assert_eq!(system_name.position(), None); // Order-level has no position
    }
}
