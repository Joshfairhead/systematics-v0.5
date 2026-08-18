//! GraphContent — the serialisable data layer.
//!
//! The combinatoric substrate (Order, Position, Point, Line, Segment, and the
//! topological/geometric vocabulary ref-lists) is deterministic from the Order
//! and lives in code. Everything *data-like* — coordinates, characters,
//! semantic vocabularies, and perspectives — is content, and it round-trips through
//! this one struct. Both the canonical seed (`data/canonical.json`) and the
//! writable user store use this shape.

use serde::{Deserialize, Serialize};

use super::citations::{Artefact, Lookup, Reference, Source};
use super::entries::{Character, Coordinate};
use super::functors::Functor;
use super::perspectives::Perspective;
use super::sequences::Sequence;
use super::systems::System;
use super::vocabularies::Vocabulary;

/// A portable slice of the data layer. Applied onto a substrate to populate it;
/// snapshotted back out to persist. (Grammars are deterministic per Order and
/// seeded in code, so they are not part of persisted content.)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GraphContent {
    #[serde(default)]
    pub characters: Vec<Character>,
    #[serde(default)]
    pub coordinates: Vec<Coordinate>,
    #[serde(default, rename = "vocabularies")]
    pub vocabularies: Vec<Vocabulary>,
    #[serde(default)]
    pub systems: Vec<System>,
    // -------- referencing layer (AD4M perspectives + citation triad) --------
    #[serde(default)]
    pub perspectives: Vec<Perspective>,
    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(default)]
    pub artefacts: Vec<Artefact>,
    #[serde(default)]
    pub lookups: Vec<Lookup>,
    #[serde(default)]
    pub references: Vec<Reference>,
    /// Same-grammar functors (position permutations between systems of one
    /// Order). User-created transforms, persisted like systems/references.
    #[serde(default)]
    pub functors: Vec<Functor>,
    /// Ordered series of member addresses (the container triad's `+` pole).
    #[serde(default)]
    pub sequences: Vec<Sequence>,
    /// External addresses this content depends on but does not own (canonical
    /// systems, sibling perspectives, shared citation entities). Advisory:
    /// populated by `Graph::export_perspective` so a module records what must be
    /// loaded elsewhere for its address-links to resolve. Omitted when empty, so
    /// full snapshots and pre-existing module files are unaffected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manifest: Vec<String>,
}

impl GraphContent {
    pub fn is_empty(&self) -> bool {
        self.characters.is_empty()
            && self.coordinates.is_empty()
            && self.vocabularies.is_empty()
            && self.systems.is_empty()
            && self.perspectives.is_empty()
            && self.sources.is_empty()
            && self.artefacts.is_empty()
            && self.lookups.is_empty()
            && self.references.is_empty()
    }

    /// All entry IDs carried by this content (across every kind).
    pub fn ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        ids.extend(self.characters.iter().map(|c| c.id.clone()));
        ids.extend(self.coordinates.iter().map(|c| c.id.clone()));
        ids.extend(self.vocabularies.iter().map(|v| v.id.clone()));
        ids.extend(self.systems.iter().map(|s| s.id.clone()));
        ids.extend(self.perspectives.iter().map(|p| p.id.clone()));
        ids.extend(self.sources.iter().map(|s| s.id.clone()));
        ids.extend(self.artefacts.iter().map(|a| a.id.clone()));
        ids.extend(self.lookups.iter().map(|l| l.id.clone()));
        ids.extend(self.references.iter().map(|r| r.id.clone()));
        ids.extend(self.functors.iter().map(|f| f.id.clone()));
        ids.extend(self.sequences.iter().map(|s| s.id.clone()));
        ids
    }
}
