//! GraphTemplate: the complete graph `K_n` — structural skeleton + validation rules.
//!
//! A `GraphTemplate` is the *syntax* of a system: the invariant complete-graph
//! structure for one Order (it references the topological + geometric
//! substrate vocabularies) plus the arity rules a Vocabulary must satisfy to
//! fill it — a triad (`K_3`) has `order` terms and `C(order,2)` connectives
//! ("three impulses, three acts"). Grammars are deterministic per Order and are
//! seeded in code (like the substrate vocabularies), not persisted as content.

use serde::{Deserialize, Serialize};

use super::vocabularies::{Geometry, Vocabulary, Topology};

/// The complete graph `K_n` for one Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphTemplate {
    pub id: String,
    pub order: u8,
    pub topological_vocab_ref: String,
    pub geometric_vocab_ref: String,
}

impl GraphTemplate {
    pub fn new(
        id: impl Into<String>,
        order: u8,
        topological_vocab_ref: impl Into<String>,
        geometric_vocab_ref: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            order,
            topological_vocab_ref: topological_vocab_ref.into(),
            geometric_vocab_ref: geometric_vocab_ref.into(),
        }
    }

    /// The canonical GraphTemplate for an Order: `grammar_{order}`, wired to the
    /// canonical `topvocab_{order}` / `geovocab_{order}` substrate.
    pub fn for_order(order: u8) -> Self {
        Self::new(
            format!("grammar_{}", order),
            order,
            format!("topvocab_{}", order),
            format!("geovocab_{}", order),
        )
    }

    /// Number of terms (nodes) a Vocabulary must supply: one per vertex.
    pub fn expected_terms(&self) -> usize {
        self.order as usize
    }

    /// Number of connectives (edges) a Vocabulary must supply: `C(order, 2)`.
    pub fn expected_connectives(&self) -> usize {
        let n = self.order as usize;
        n * n.saturating_sub(1) / 2
    }

    /// Validate that a Vocabulary satisfies this GraphTemplate's rules (order + arity).
    pub fn validate(&self, vocab: &Vocabulary) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        if vocab.order != self.order {
            errs.push(format!(
                "GraphTemplate {}: vocabulary '{}' order {} doesn't match grammar order {}",
                self.id, vocab.id, vocab.order, self.order
            ));
        }
        if vocab.terms.len() != self.expected_terms() {
            errs.push(format!(
                "GraphTemplate {}: vocabulary '{}' has {} terms, expected {}",
                self.id,
                vocab.id,
                vocab.terms.len(),
                self.expected_terms()
            ));
        }
        if vocab.connectives.len() != self.expected_connectives() {
            errs.push(format!(
                "GraphTemplate {}: vocabulary '{}' has {} connectives, expected {}",
                self.id,
                vocab.id,
                vocab.connectives.len(),
                self.expected_connectives()
            ));
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }

    /// Full structural validation against the resolved substrate + a Vocabulary:
    /// checks the referenced substrate matches by id/order and delegates arity
    /// to the substrate vocabularies and this GraphTemplate's rules.
    pub fn validate_with(
        &self,
        topology: &Topology,
        geometry: &Geometry,
        vocab: &Vocabulary,
    ) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();

        if topology.id != self.topological_vocab_ref {
            errs.push(format!(
                "GraphTemplate {}: topological_vocab_ref '{}' doesn't match passed topology '{}'",
                self.id, self.topological_vocab_ref, topology.id
            ));
        }
        if geometry.id != self.geometric_vocab_ref {
            errs.push(format!(
                "GraphTemplate {}: geometric_vocab_ref '{}' doesn't match passed geometry '{}'",
                self.id, self.geometric_vocab_ref, geometry.id
            ));
        }
        if topology.order != self.order {
            errs.push(format!(
                "GraphTemplate {}: order {} doesn't match topology order {}",
                self.id, self.order, topology.order
            ));
        }
        if geometry.order != self.order {
            errs.push(format!(
                "GraphTemplate {}: order {} doesn't match geometry order {}",
                self.id, self.order, geometry.order
            ));
        }

        if let Err(e) = topology.validate() {
            errs.extend(e);
        }
        if let Err(e) = geometry.validate() {
            errs.extend(e);
        }
        if let Err(e) = vocab.validate_against(topology) {
            errs.extend(e);
        }
        if let Err(e) = self.validate(vocab) {
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

    #[test]
    fn test_for_order_arity() {
        let g = GraphTemplate::for_order(3);
        assert_eq!(g.id, "grammar_3");
        assert_eq!(g.expected_terms(), 3);
        assert_eq!(g.expected_connectives(), 3);
        let tetrad = GraphTemplate::for_order(4);
        assert_eq!(tetrad.expected_connectives(), 6);
    }

    #[test]
    fn test_validate_arity() {
        let g = GraphTemplate::for_order(3);
        let ok = Vocabulary::new(
            "vocab_x_3",
            "X",
            3,
            vec!["a".into(), "b".into(), "c".into()],
            vec!["d".into(), "e".into(), "f".into()],
        );
        assert!(g.validate(&ok).is_ok());
        let bad = Vocabulary::new("vocab_y_3", "Y", 3, vec!["a".into()], vec![]);
        assert!(g.validate(&bad).is_err());
    }
}
