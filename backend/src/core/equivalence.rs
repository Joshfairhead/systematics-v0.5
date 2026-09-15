//! **Archetype equivalence** — the mapping between a system's **topology** face and its
//! **vocabulary** (system) face. Both faces are determined by the single cardinality `n`;
//! the equivalence names the six paired dimensions and lets us validate that a concrete
//! instance's two faces agree. See `docs/v0.6-rebuild/validation.md`.
//!
//! | topology          | vocabulary (system)     | example (n = 3) |
//! |-------------------|-------------------------|-----------------|
//! | Graph             | System                  | K3 = Triad      |
//! | Cardinality       | Coherence               | (3,3) = Dynamism|
//! | Order             | Term designation        | 3 = Impulses    |
//! | Size              | Connective designation  | 3 = Acts        |
//! | Vertex ordinality | Term position           | 1 = term1       |
//! | Edge seriality    | Connective position     | 1 = connective1 |
//!
//! Topology and vocabulary are two faces of one archetype: validating an instance is
//! checking they agree (all derived from the one cardinality) and that terms anchor to
//! vertices and connectives to edges (position = ordinality/seriality).

use super::hexadicsystems::systematics_hexad;

/// One paired dimension of the archetype, with the value each face takes at order `n`.
#[derive(Debug, Clone, PartialEq)]
pub struct EquivalencePair {
    /// The topology-face name: Graph, Cardinality, Order, Size, VertexOrdinality, EdgeSeriality.
    pub topology: &'static str,
    /// The vocabulary/system-face name: System, Coherence, TermDesignation, …
    pub system: &'static str,
    /// The topology value at this order (e.g. "K3", "(3,3)", "3", "1..3").
    pub topology_value: String,
    /// The system value at this order (e.g. "Triad", "Dynamism", "Impulses", "term1..term3").
    pub system_value: String,
}

/// `|E| = C(n, 2)` — the edge (connective) cardinality.
pub fn edge_cardinality(order: u8) -> u8 {
    let n = order as usize;
    (n * n.saturating_sub(1) / 2) as u8
}

fn serial(n: u8) -> String {
    if n == 0 { "—".to_string() } else { format!("1..{n}") }
}
fn labelled(base: &str, n: u8) -> String {
    if n == 0 { "—".to_string() } else { format!("{base}1..{base}{n}") }
}

/// The six equivalence pairs at order `n` — the archetype at that cardinality. This *is*
/// the topology ↔ vocabulary mapping, made first-class.
pub fn equivalence(order: u8) -> Vec<EquivalencePair> {
    let h = systematics_hexad(order);
    let size = edge_cardinality(order);
    vec![
        EquivalencePair {
            topology: "Graph",
            system: "System",
            topology_value: format!("K{order}"),
            system_value: h.name,
        },
        EquivalencePair {
            topology: "Cardinality",
            system: "Coherence",
            topology_value: format!("({order},{size})"),
            system_value: h.coherence,
        },
        EquivalencePair {
            topology: "Order",
            system: "TermDesignation",
            topology_value: order.to_string(),
            system_value: h.term_designation,
        },
        EquivalencePair {
            topology: "Size",
            system: "ConnectiveDesignation",
            topology_value: size.to_string(),
            system_value: h.connective_designation,
        },
        EquivalencePair {
            topology: "VertexOrdinality",
            system: "TermPosition",
            topology_value: serial(order),
            system_value: labelled("term", order),
        },
        EquivalencePair {
            topology: "EdgeSeriality",
            system: "ConnectivePosition",
            topology_value: serial(size),
            system_value: labelled("connective", size),
        },
    ]
}

/// Validate a system **instance** against the archetype for its order — the topology ↔
/// vocabulary equivalence, checked:
/// - Cardinality ↔ Coherence, Order ↔ TermDesignation, Size ↔ ConnectiveDesignation
///   (the vocabulary face must match the canonical value for `order`);
/// - VertexOrdinality ↔ TermPosition, EdgeSeriality ↔ ConnectivePosition (the term positions
///   must serialise the vertex ordinalities `1..n`, the connective positions the edge
///   serialities `1..C(n,2)` — i.e. terms anchor to vertices and connectives to edges).
///
/// Returns the mismatches (empty ⇒ the instance conforms to its archetype).
pub fn validate_instance(
    order: u8,
    coherence: &str,
    term_designation: &str,
    connective_designation: &str,
    term_positions: &[i32],
    connective_positions: &[i32],
) -> Result<(), Vec<String>> {
    let h = systematics_hexad(order);
    let size = edge_cardinality(order);
    let mut errs = Vec::new();

    if !coherence.eq_ignore_ascii_case(&h.coherence) {
        errs.push(format!(
            "Cardinality↔Coherence: '{coherence}' ≠ '{}' for K{order}",
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
    if term_positions != want_terms.as_slice() {
        errs.push(format!(
            "VertexOrdinality↔TermPosition: term positions {term_positions:?} ≠ vertex ordinalities {want_terms:?}"
        ));
    }
    let want_conns: Vec<i32> = (1..=size as i32).collect();
    if connective_positions != want_conns.as_slice() {
        errs.push(format!(
            "EdgeSeriality↔ConnectivePosition: connective positions {connective_positions:?} ≠ edge serialities {want_conns:?}"
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
        let g = pair(&e, "Graph");
        assert_eq!((g.system, g.topology_value.as_str(), g.system_value.as_str()), ("System", "K3", "Triad"));
        let c = pair(&e, "Cardinality");
        assert_eq!((c.system, c.topology_value.as_str(), c.system_value.as_str()), ("Coherence", "(3,3)", "Dynamism"));
        assert_eq!(pair(&e, "Order").system_value, "Impulses");       // 3 → Impulses
        assert_eq!(pair(&e, "Size").system_value, "Acts");            // 3 → Acts
    }

    #[test]
    fn tetrad_cardinality_is_four_six() {
        let e = equivalence(4);
        assert_eq!(pair(&e, "Cardinality").topology_value, "(4,6)");
        assert_eq!(pair(&e, "Cardinality").system_value, "Activity Field");
        assert_eq!(pair(&e, "Size").topology_value, "6");
        assert_eq!(pair(&e, "Size").system_value, "Interplays");
    }

    #[test]
    fn valid_triad_instance_passes() {
        assert!(validate_instance(3, "Dynamism", "Impulses", "Acts", &[1, 2, 3], &[1, 2, 3]).is_ok());
    }

    #[test]
    fn wrong_coherence_fails() {
        let errs = validate_instance(3, "Complementarity", "Impulses", "Acts", &[1, 2, 3], &[1, 2, 3])
            .unwrap_err();
        assert!(errs.iter().any(|e| e.contains("Cardinality↔Coherence")));
    }

    #[test]
    fn misanchored_positions_fail() {
        // Only two term positions for a triad ⇒ terms don't anchor to the three vertices.
        let errs = validate_instance(3, "Dynamism", "Impulses", "Acts", &[1, 2], &[1, 2, 3])
            .unwrap_err();
        assert!(errs.iter().any(|e| e.contains("VertexOrdinality↔TermPosition")));
    }
}
