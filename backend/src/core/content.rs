//! GraphContent — the serialisable data layer.
//!
//! The combinatoric substrate (Order, Position, Point, Line, Segment, and the
//! topological/geometric vocabulary ref-lists) is deterministic from the Order
//! and lives in code. Everything *data-like* — coordinates, characters,
//! semantic vocabularies, and perspectives — is content, and it round-trips through
//! this one struct. Both the canonical seed (`data/canonical.json`) and the
//! writable user store use this shape.

use serde::{Deserialize, Serialize};

use super::entries::{Character, Coordinate};
use super::perspectives::Perspective;
use super::vocabularies::SemanticVocabulary;

/// A portable slice of the data layer. Applied onto a substrate to populate it;
/// snapshotted back out to persist.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GraphContent {
    #[serde(default)]
    pub characters: Vec<Character>,
    #[serde(default)]
    pub coordinates: Vec<Coordinate>,
    #[serde(default, rename = "semanticVocabs")]
    pub semantic_vocabs: Vec<SemanticVocabulary>,
    #[serde(default)]
    pub perspectives: Vec<Perspective>,
}

impl GraphContent {
    pub fn is_empty(&self) -> bool {
        self.characters.is_empty()
            && self.coordinates.is_empty()
            && self.semantic_vocabs.is_empty()
            && self.perspectives.is_empty()
    }

    /// All entry IDs carried by this content (across every kind).
    pub fn ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        ids.extend(self.characters.iter().map(|c| c.id.clone()));
        ids.extend(self.coordinates.iter().map(|c| c.id.clone()));
        ids.extend(self.semantic_vocabs.iter().map(|v| v.id.clone()));
        ids.extend(self.perspectives.iter().map(|g| g.id.clone()));
        ids
    }
}
