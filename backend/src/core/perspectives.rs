//! Perspective: the rules of a K_n system.
//!
//! A `Perspective` is not a container of content — it's the *validator + inline
//! metadata* for a system. It references the three parallel vocabularies
//! (`TopologicalVocabulary`, `GeometricVocabulary`, `SemanticVocabulary`) and
//! provides the human-readable metadata (name, coherence, term/connective
//! designations).
//!
//! Creating a Perspective with valid references *is* the join — there's no
//! separate `apply` step. Queries walk the references to reconstruct
//! populated views (see `Graph::perspective_view`).

use serde::{Deserialize, Serialize};

use super::vocabularies::{GeometricVocabulary, SemanticVocabulary, TopologicalVocabulary};

/// A Perspective over one Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Perspective {
    pub id: String,
    /// Same as SystemName in the old model — "Triad", "Tetrad", ...
    pub name: String,
    /// The K_n size this Perspective dictates.
    pub order: u8,
    /// Inline metadata (formerly separate SystemName / CoherenceAttribute /
    /// TermDesignation / ConnectiveDesignation entries).
    pub coherence: String,
    pub term_designation: String,
    pub connective_designation: String,
    /// References to the three vocabularies this Perspective reconciles.
    pub topological_vocab_ref: String,
    pub geometric_vocab_ref: String,
    pub semantic_vocab_ref: String,
}

impl Perspective {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        order: u8,
        coherence: impl Into<String>,
        term_designation: impl Into<String>,
        connective_designation: impl Into<String>,
        topological_vocab_ref: impl Into<String>,
        geometric_vocab_ref: impl Into<String>,
        semantic_vocab_ref: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            order,
            coherence: coherence.into(),
            term_designation: term_designation.into(),
            connective_designation: connective_designation.into(),
            topological_vocab_ref: topological_vocab_ref.into(),
            geometric_vocab_ref: geometric_vocab_ref.into(),
            semantic_vocab_ref: semantic_vocab_ref.into(),
        }
    }

    /// Convenience: build an ID from name and order.
    pub fn with_auto_id(
        name: impl Into<String>,
        order: u8,
        coherence: impl Into<String>,
        term_designation: impl Into<String>,
        connective_designation: impl Into<String>,
        topological_vocab_ref: impl Into<String>,
        geometric_vocab_ref: impl Into<String>,
        semantic_vocab_ref: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let slug = name.to_lowercase().replace(' ', "_");
        Self::new(
            format!("perspective_{}_{}", slug, order),
            name,
            order,
            coherence,
            term_designation,
            connective_designation,
            topological_vocab_ref,
            geometric_vocab_ref,
            semantic_vocab_ref,
        )
    }

    /// Validate against resolved vocabularies. Callers pass the three referenced
    /// vocabularies; this checks order alignment and arity across the stack.
    pub fn validate_with(
        &self,
        topology: &TopologicalVocabulary,
        geometry: &GeometricVocabulary,
        semantics: &SemanticVocabulary,
    ) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();

        // Check refs match the passed vocabularies by ID.
        if topology.id != self.topological_vocab_ref {
            errs.push(format!(
                "Perspective {}: topological_vocab_ref '{}' doesn't match passed topology '{}'",
                self.id, self.topological_vocab_ref, topology.id
            ));
        }
        if geometry.id != self.geometric_vocab_ref {
            errs.push(format!(
                "Perspective {}: geometric_vocab_ref '{}' doesn't match passed geometry '{}'",
                self.id, self.geometric_vocab_ref, geometry.id
            ));
        }
        if semantics.id != self.semantic_vocab_ref {
            errs.push(format!(
                "Perspective {}: semantic_vocab_ref '{}' doesn't match passed semantics '{}'",
                self.id, self.semantic_vocab_ref, semantics.id
            ));
        }

        // Order alignment.
        if topology.order != self.order {
            errs.push(format!(
                "Perspective {}: order {} doesn't match topology order {}",
                self.id, self.order, topology.order
            ));
        }
        if geometry.order != self.order {
            errs.push(format!(
                "Perspective {}: order {} doesn't match geometry order {}",
                self.id, self.order, geometry.order
            ));
        }
        if semantics.order != self.order {
            errs.push(format!(
                "Perspective {}: order {} doesn't match semantics order {}",
                self.id, self.order, semantics.order
            ));
        }

        // Delegate arity checks to the vocabularies themselves.
        if let Err(e) = topology.validate() {
            errs.extend(e);
        }
        if let Err(e) = geometry.validate() {
            errs.extend(e);
        }
        if let Err(e) = semantics.validate_against(topology) {
            errs.extend(e);
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triad_vocabs() -> (TopologicalVocabulary, GeometricVocabulary, SemanticVocabulary) {
        let t = TopologicalVocabulary::canonical_for(3);
        let g = GeometricVocabulary::canonical_for(3);
        let s = SemanticVocabulary::with_auto_id(
            "Canonical Triad",
            3,
            vec![
                "char_word_will".into(),
                "char_word_function".into(),
                "char_word_being".into(),
            ],
            vec![
                "char_word_generation".into(),
                "char_word_consent".into(),
                "char_word_decision".into(),
            ],
        );
        (t, g, s)
    }

    #[test]
    fn test_perspective_creation_with_auto_id() {
        let (t, g, s) = triad_vocabs();
        let gram = Perspective::with_auto_id(
            "Triad",
            3,
            "Dynamism",
            "Impulses",
            "Acts",
            &t.id,
            &g.id,
            &s.id,
        );
        assert_eq!(gram.id, "perspective_triad_3");
        assert_eq!(gram.name, "Triad");
        assert_eq!(gram.coherence, "Dynamism");
    }

    #[test]
    fn test_perspective_validate_ok() {
        let (t, g, s) = triad_vocabs();
        let gram = Perspective::with_auto_id(
            "Triad",
            3,
            "Dynamism",
            "Impulses",
            "Acts",
            &t.id,
            &g.id,
            &s.id,
        );
        assert!(gram.validate_with(&t, &g, &s).is_ok());
    }

    #[test]
    fn test_perspective_validate_catches_order_mismatch() {
        let (t, g, _) = triad_vocabs();
        let tetrad_sem = SemanticVocabulary::with_auto_id(
            "Bad Sem",
            4,
            vec!["a".into(), "b".into(), "c".into(), "d".into()],
            vec!["e".into(), "f".into(), "g".into(), "h".into(), "i".into(), "j".into()],
        );
        let gram = Perspective::with_auto_id(
            "Confused",
            3,
            "?",
            "?",
            "?",
            &t.id,
            &g.id,
            &tetrad_sem.id,
        );
        let errs = gram.validate_with(&t, &g, &tetrad_sem).unwrap_err();
        assert!(errs.iter().any(|e| e.contains("order")));
    }

    #[test]
    fn test_perspective_validate_catches_wrong_ref() {
        let (t, g, s) = triad_vocabs();
        let other_topology = TopologicalVocabulary::canonical_for(4);
        let gram = Perspective::with_auto_id(
            "Triad",
            3,
            "Dynamism",
            "Impulses",
            "Acts",
            "topvocab_3",
            &g.id,
            &s.id,
        );
        // Passing the wrong topology object triggers a ref-mismatch error.
        let errs = gram.validate_with(&other_topology, &g, &s).unwrap_err();
        assert!(errs
            .iter()
            .any(|e| e.contains("topological_vocab_ref")));
        // t is unused warning avoided:
        drop(t);
    }
}
