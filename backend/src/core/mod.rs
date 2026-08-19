//! Core types for the Systematics property graph.
//!
//! Layered ontology:
//! - `entries` — the substrate (Order, Position, Point, Line, Coordinate,
//!   Segment, Character) plus the `Entry` sum type.
//! - `links` — Link entries (currently `Line` for coordinate-to-coordinate
//!   rendering; `Connective` shim retained during frontend migration).
//! - `vocabularies` — `Topology`, `Geometry`,
//!   `Vocabulary` — ordered per-Order references into the substrate.
//! - `grammar` — `GraphTemplate`: the K_n structure + arity validation rules.
//! - `systems` — `System`: metadata reconciling a GraphTemplate with a Vocabulary.
//! - `perspectives` — `Perspective`/`Link`: AD4M-style directed webs.
//! - `citations` — `Source`/`Artefact`/`Lookup`/`Reference`: the citation triad.
//! - `graph` — the container plus queries and mutations.

pub mod citations;
pub mod content;
pub mod entries;
pub mod functors;
pub mod grammar;
pub mod graph;
pub mod links;
pub mod perspectives;
pub mod sequences;
pub mod systems;
pub mod vocabularies;

pub use entries::{
    Character, Coordinate, Entry, Line, Order, Point, Point3d, Position, Segment,
};

pub use links::{Link, LinkType};

pub use vocabularies::{Geometry, Vocabulary, Topology};

pub use grammar::GraphTemplate;

pub use systems::System;

pub use functors::Functor;

pub use sequences::Sequence;

pub use perspectives::{Link as PerspectiveLink, Perspective};

pub use citations::{Artefact, Lookup, Reference, Source};

pub use content::GraphContent;

pub use graph::Graph;
