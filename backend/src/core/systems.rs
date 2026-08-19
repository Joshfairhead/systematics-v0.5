//! System: the reconciler of a Template and a Vocabulary.
//!
//! A `System` holds the system-relevant metadata (name, order, coherence,
//! term/connective designations) and reconciles one `Template` (the K_n
//! structure/rules) with one Vocabulary (`Vocabulary` — a particular
//! set of terms). "The triad of XYZ" is a System. Conceptually a System is a
//! *constrained* perspective: a reconciling association bound by the grammar's
//! validation. This replaces the former `Perspective` (rules-container) type.

use serde::{Deserialize, Serialize};

/// A System over one Order: metadata + refs to a Template and a Vocabulary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub id: String,
    pub name: String,
    pub order: u8,
    pub coherence: String,
    pub term_designation: String,
    pub connective_designation: String,
    /// Reference to the `Template` (structure) this System reconciles.
    pub grammar_ref: String,
    /// Reference to the Vocabulary (`Vocabulary`) filling the grammar.
    pub vocabulary_ref: String,
}

impl System {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        order: u8,
        coherence: impl Into<String>,
        term_designation: impl Into<String>,
        connective_designation: impl Into<String>,
        grammar_ref: impl Into<String>,
        vocabulary_ref: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            order,
            coherence: coherence.into(),
            term_designation: term_designation.into(),
            connective_designation: connective_designation.into(),
            grammar_ref: grammar_ref.into(),
            vocabulary_ref: vocabulary_ref.into(),
        }
    }

    /// Build an id from name and order: `system_{slug}_{order}`.
    #[allow(clippy::too_many_arguments)]
    pub fn with_auto_id(
        name: impl Into<String>,
        order: u8,
        coherence: impl Into<String>,
        term_designation: impl Into<String>,
        connective_designation: impl Into<String>,
        grammar_ref: impl Into<String>,
        vocabulary_ref: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let slug = name.to_lowercase().replace(' ', "_");
        Self::new(
            format!("system_{}_{}", slug, order),
            name,
            order,
            coherence,
            term_designation,
            connective_designation,
            grammar_ref,
            vocabulary_ref,
        )
    }
}
