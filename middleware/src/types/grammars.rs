//! Grammar wire type.

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// The rules for a K_n system: validation logic + inline metadata,
/// plus references to the three vocabularies it reconciles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Grammar {
    pub id: String,
    pub name: String,
    pub order: i32,
    pub coherence: String,
    #[serde(rename = "termDesignation")]
    pub term_designation: String,
    #[serde(rename = "connectiveDesignation")]
    pub connective_designation: String,
    #[serde(rename = "topologicalVocabRef")]
    pub topological_vocab_ref: String,
    #[serde(rename = "geometricVocabRef")]
    pub geometric_vocab_ref: String,
    #[serde(rename = "semanticVocabRef")]
    pub semantic_vocab_ref: String,
}
