//! Core types for the Systematics property graph.
//!
//! Layered ontology:
//! - `entries` — the substrate (Order, Position, Point, Line, Coordinate,
//!   Segment, Character) plus the `Entry` sum type.
//! - `links` — Link entries (currently `Line` for coordinate-to-coordinate
//!   rendering; `Connective` shim retained during frontend migration).
//! - `vocabularies` — `TopologicalVocabulary`, `GeometricVocabulary`,
//!   `SemanticVocabulary` — ordered per-Order references into the substrate.
//! - `grammars` — `Grammar`: validation rules + inline metadata that
//!   reconcile the three vocabularies.
//! - `graph` — the container plus queries and mutations.

pub mod entries;
pub mod grammars;
pub mod graph;
pub mod links;
pub mod vocabularies;

pub use entries::{
    Character, Coordinate, Entry, Line, Order, Point, Point3d, Position, Segment,
};

pub use links::{Link, LinkType};

pub use vocabularies::{GeometricVocabulary, SemanticVocabulary, TopologicalVocabulary};

pub use grammars::Grammar;

pub use graph::Graph;
