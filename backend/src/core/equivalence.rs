//! **Archetype equivalence** — the book-matching of a system's two hexads: its **topology**
//! face ([`TopologyHexad`]) and its **vocabulary** face ([`SystematicsHexad`]). Both are
//! derived from the single cardinality `n`; the equivalence pairs them dimension-for-dimension
//! and lets us validate that a concrete instance's two faces agree. See
//! `docs/v0.6-rebuild/validation.md`.
//!
//! | topology (TopologyHexad) | vocabulary (SystematicsHexad) | example (n = 3)  |
//! |--------------------------|-------------------------------|------------------|
//! | Cardinality              | System (name)                 | (3,3) = Triad    |
//! | Eigenvalue *(proposed)*  | Coherence                     | 0,3,3 = Dynamism |
//! | Order                    | Term designation              | 3 = Impulses     |
//! | Size                     | Connective designation        | 3 = Acts         |
//! | Vertex ordinality        | Term ordinality               | 1 = term1        |
//! | Edge ordinality          | Connective ordinality         | 1 = connective1  |
//!
//! Topology and vocabulary are two faces of one archetype: validating an instance is
//! checking they agree (all derived from the one cardinality) and that terms anchor to
//! vertices and connectives to edges (ordinality = placement). A K_n *is* its cardinality
//! (`(4,6)` = K4 = Tetrad), so cardinality book-matches the system name; the graph's
//! spectral *quality* (eigenvalue) book-matches coherence. **Seriality** is not a facet here —
//! it is the six-laws *arrangement* of the ordinalities (123, 132, …); see the Controller.

use super::hexadicsystems::{
    edge_cardinality, systematics_hexad, topology_hexad, SystematicsHexad, TopologyHexad,
};

/// One paired dimension of the archetype, with the value each face takes at order `n`.
#[derive(Debug, Clone, PartialEq)]
pub struct EquivalencePair {
    /// The topology-face name: Cardinality, Eigenvalue, Order, Size, VertexOrdinality, EdgeOrdinality.
    pub topology: &'static str,
    /// The vocabulary/system-face name: System, Coherence, TermDesignation, …, ConnectiveOrdinality.
    pub system: &'static str,
    /// The topology value at this order (e.g. "K3", "(3,3)", "3", "1..3").
    pub topology_value: String,
    /// The system value at this order (e.g. "Triad", "Dynamism", "Impulses", "term1..term3").
    pub system_value: String,
}

fn serial(range: &[u8]) -> String {
    match (range.first(), range.last()) {
        (Some(a), Some(b)) => format!("{a}..{b}"),
        _ => "—".to_string(),
    }
}
fn labelled(base: &str, count: u8) -> String {
    if count == 0 {
        "—".to_string()
    } else {
        format!("{base}1..{base}{count}")
    }
}

/// The archetype at order `n` as a **book-matched pair of hexads** — the topology face and
/// the vocabulary face, held together so their six dimensions can be paired and checked.
#[derive(Debug, Clone, PartialEq)]
pub struct EquivalenceHexad {
    pub topology: TopologyHexad,
    pub system: SystematicsHexad,
}

impl EquivalenceHexad {
    /// Both faces of the archetype at cardinality `n`, each from its own source of truth.
    pub fn for_order(order: u8) -> Self {
        Self {
            topology: topology_hexad(order),
            system: systematics_hexad(order),
        }
    }

    /// The six equivalence pairs, book-matching the two hexads dimension-for-dimension.
    /// (The term/connective ordinalities line up with the topology's vertex/edge ordinalities
    /// because both faces are derived from the one `n`.)
    pub fn pairs(&self) -> Vec<EquivalencePair> {
        let t = &self.topology;
        let s = &self.system;
        vec![
            EquivalencePair {
                topology: "Cardinality",
                system: "System",
                topology_value: t.cardinality.clone(),
                system_value: s.name.clone(),
            },
            EquivalencePair {
                topology: "Eigenvalue",
                system: "Coherence",
                topology_value: t.eigenvalue.clone(),
                system_value: s.coherence.clone(),
            },
            EquivalencePair {
                topology: "Order",
                system: "TermDesignation",
                topology_value: t.order.to_string(),
                system_value: s.term_designation.clone(),
            },
            EquivalencePair {
                topology: "Size",
                system: "ConnectiveDesignation",
                topology_value: t.size.to_string(),
                system_value: s.connective_designation.clone(),
            },
            EquivalencePair {
                topology: "VertexOrdinality",
                system: "TermOrdinality",
                topology_value: serial(&t.vertex_ordinality),
                system_value: labelled("term", s.term_ordinality.len() as u8),
            },
            EquivalencePair {
                topology: "EdgeOrdinality",
                system: "ConnectiveOrdinality",
                topology_value: serial(&t.edge_ordinality),
                system_value: labelled("connective", s.connective_ordinality.len() as u8),
            },
        ]
    }

    /// Validate that the two faces **book-match** — the bridge that guarantees the equivalence:
    /// the topology's order/size equal the vocabulary's term/connective counts, and the
    /// vertex/edge ordinalities equal the term/connective ordinalities (terms anchor to vertices
    /// `1..n`, connectives to edges `1..size`). Empty ⇒ the hexads are a consistent archetype.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let t = &self.topology;
        let s = &self.system;
        let mut errs = Vec::new();
        if t.order as usize != s.term_ordinality.len() {
            errs.push(format!(
                "Order↔TermOrdinality: topology order {} ≠ {} term ordinalities",
                t.order,
                s.term_ordinality.len()
            ));
        }
        if t.size as usize != s.connective_ordinality.len() {
            errs.push(format!(
                "Size↔ConnectiveOrdinality: topology size {} ≠ {} connective ordinalities",
                t.size,
                s.connective_ordinality.len()
            ));
        }
        if t.vertex_ordinality != s.term_ordinality {
            errs.push(format!(
                "VertexOrdinality↔TermOrdinality: {:?} ≠ {:?}",
                t.vertex_ordinality, s.term_ordinality
            ));
        }
        if t.edge_ordinality != s.connective_ordinality {
            errs.push(format!(
                "EdgeOrdinality↔ConnectiveOrdinality: {:?} ≠ {:?}",
                t.edge_ordinality, s.connective_ordinality
            ));
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

/// The six equivalence pairs at order `n` — the topology ↔ vocabulary mapping, made
/// first-class. Thin accessor over [`EquivalenceHexad::pairs`].
pub fn equivalence(order: u8) -> Vec<EquivalencePair> {
    EquivalenceHexad::for_order(order).pairs()
}

/// Validate a system **instance** against the archetype for its order — the topology ↔
/// vocabulary equivalence, checked:
/// - Eigenvalue ↔ Coherence, Order ↔ TermDesignation, Size ↔ ConnectiveDesignation
///   (the vocabulary face must match the canonical value for `order`);
/// - VertexOrdinality ↔ TermOrdinality, EdgeOrdinality ↔ ConnectiveOrdinality (the term
///   ordinalities must run `1..n` over the vertices, the connective ordinalities `1..C(n,2)`
///   over the edges — i.e. terms anchor to vertices and connectives to edges).
///
/// Returns the mismatches (empty ⇒ the instance conforms to its archetype).
pub fn validate_instance(
    order: u8,
    coherence: &str,
    term_designation: &str,
    connective_designation: &str,
    term_ordinalities: &[i32],
    connective_ordinalities: &[i32],
) -> Result<(), Vec<String>> {
    let h = systematics_hexad(order);
    let size = edge_cardinality(order);
    let mut errs = Vec::new();

    if !coherence.eq_ignore_ascii_case(&h.coherence) {
        errs.push(format!(
            "Eigenvalue↔Coherence: '{coherence}' ≠ '{}' for K{order}",
            h.coherence
        ));
    }
    if !term_designation.eq_ignore_ascii_case(&h.term_designation) {
        errs.push(format!(
            "Order↔TermDesignation: '{term_designation}' ≠ '{}' for order {order}",
            h.term_designation
        ));
    }
    if !connective_designation.eq_ignore_ascii_case(&h.connective_designation) {
        errs.push(format!(
            "Size↔ConnectiveDesignation: '{connective_designation}' ≠ '{}' for size {size}",
            h.connective_designation
        ));
    }

    let want_terms: Vec<i32> = (1..=order as i32).collect();
    if term_ordinalities != want_terms.as_slice() {
        errs.push(format!(
            "VertexOrdinality↔TermOrdinality: term ordinalities {term_ordinalities:?} ≠ vertex ordinalities {want_terms:?}"
        ));
    }
    let want_conns: Vec<i32> = (1..=size as i32).collect();
    if connective_ordinalities != want_conns.as_slice() {
        errs.push(format!(
            "EdgeOrdinality↔ConnectiveOrdinality: connective ordinalities {connective_ordinalities:?} ≠ edge ordinalities {want_conns:?}"
        ));
    }

    if errs.is_empty() {
        Ok(())
    } else {
        Err(errs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair<'a>(pairs: &'a [EquivalencePair], topology: &str) -> &'a EquivalencePair {
        pairs.iter().find(|p| p.topology == topology).expect("dimension present")
    }

    #[test]
    fn triad_equivalence_pairs() {
        let e = equivalence(3);
        assert_eq!(e.len(), 6, "six paired dimensions");
        // Cardinality (3,3) ↔ System (Triad).
        let c = pair(&e, "Cardinality");
        assert_eq!((c.system, c.topology_value.as_str(), c.system_value.as_str()), ("System", "(3,3)", "Triad"));
        // Eigenvalue (Laplacian spectrum) ↔ Coherence (Dynamism).
        let ev = pair(&e, "Eigenvalue");
        assert_eq!((ev.system, ev.system_value.as_str()), ("Coherence", "Dynamism"));
        assert_eq!(ev.topology_value, "0 (×1), 3 (×2)");
        assert_eq!(pair(&e, "Order").system_value, "Impulses");       // 3 → Impulses
        assert_eq!(pair(&e, "Size").system_value, "Acts");            // 3 → Acts
        // ordinalities book-match: vertex ordinalities 1..3 ↔ term1..term3.
        assert_eq!(pair(&e, "VertexOrdinality").system, "TermOrdinality");
        assert_eq!(pair(&e, "VertexOrdinality").topology_value, "1..3");
        assert_eq!(pair(&e, "VertexOrdinality").system_value, "term1..term3");
    }

    #[test]
    fn tetrad_cardinality_is_four_six() {
        let e = equivalence(4);
        // Cardinality (4,6) ↔ System (Tetrad).
        assert_eq!(pair(&e, "Cardinality").topology_value, "(4,6)");
        assert_eq!(pair(&e, "Cardinality").system_value, "Tetrad");
        // Eigenvalue ↔ Coherence (Activity Field).
        assert_eq!(pair(&e, "Eigenvalue").system_value, "Activity Field");
        assert_eq!(pair(&e, "Eigenvalue").topology_value, "0 (×1), 4 (×3)");
        assert_eq!(pair(&e, "Size").topology_value, "6");
        assert_eq!(pair(&e, "Size").system_value, "Interplays");
        // six edges ↔ connective1..connective6.
        assert_eq!(pair(&e, "EdgeOrdinality").system, "ConnectiveOrdinality");
        assert_eq!(pair(&e, "EdgeOrdinality").topology_value, "1..6");
        assert_eq!(pair(&e, "EdgeOrdinality").system_value, "connective1..connective6");
    }

    #[test]
    fn hexads_book_match_for_every_order() {
        // The two faces are derived from the one n, so the archetype is always consistent.
        for n in 1..=8u8 {
            assert!(
                EquivalenceHexad::for_order(n).validate().is_ok(),
                "hexads should book-match at order {n}"
            );
        }
    }

    #[test]
    fn valid_triad_instance_passes() {
        assert!(validate_instance(3, "Dynamism", "Impulses", "Acts", &[1, 2, 3], &[1, 2, 3]).is_ok());
    }

    #[test]
    fn wrong_coherence_fails() {
        let errs = validate_instance(3, "Complementarity", "Impulses", "Acts", &[1, 2, 3], &[1, 2, 3])
            .unwrap_err();
        assert!(errs.iter().any(|e| e.contains("Eigenvalue↔Coherence")));
    }

    #[test]
    fn misanchored_ordinalities_fail() {
        // Only two term ordinalities for a triad ⇒ terms don't anchor to the three vertices.
        let errs = validate_instance(3, "Dynamism", "Impulses", "Acts", &[1, 2], &[1, 2, 3])
            .unwrap_err();
        assert!(errs.iter().any(|e| e.contains("VertexOrdinality↔TermOrdinality")));
    }
}
