//! **Hexadic systematics** — the **shape / outline of a system**, keyed by its
//! **cardinality**. The six facets are *mutually determining*: given the cardinality
//! (or any one facet), the rest follow. This is the reusable hexadic module — the
//! Controller's **validation + derivation** template (and, in time, the generative
//! instruction-set that composes/renders systematics). A **triad** MUST be
//! `{Triad, Dynamism, Impulses, Acts, 3, 3}`; a wrong coherence, or cardinality 4,
//! fails validation. It is the single source of truth for the canonical metadata
//! (`data::mod` delegates here).

use serde::{Deserialize, Serialize};

const ORDER_NAMES: [&str; 12] = [
    "Monad", "Dyad", "Triad", "Tetrad", "Pentad", "Hexad", "Heptad", "Octad", "Ennead",
    "Decad", "Undecad", "Dodecad",
];
const COHERENCE: [&str; 12] = [
    "Universality", "Complementarity", "Dynamism", "Activity Field",
    "Significance and Potential", "Coalescence", "Generation", "Self-Sufficiency",
    "Transformation", "Intrinsic Harmony", "Articulate Symmetry", "Perfection",
];
const TERM_DESIGNATION: [&str; 12] = [
    "Totality", "Poles", "Impulses", "Sources", "Limits", "Laws", "States", "Elements",
    "Needs Research", "Needs Research", "Needs Research", "Needs Research",
];
const CONNECTIVE_DESIGNATION: [&str; 12] = [
    "Unity", "Force", "Acts", "Interplays", "Mutualities", "Steps", "Intervals",
    "Components", "Needs Research", "Needs Research", "Needs Research", "Needs Research",
];

fn lookup(table: &[&'static str; 12], order: u8) -> &'static str {
    table
        .get(order.wrapping_sub(1) as usize)
        .copied()
        .unwrap_or("Unknown")
}

pub fn order_name(order: u8) -> &'static str {
    lookup(&ORDER_NAMES, order)
}
pub fn coherence(order: u8) -> &'static str {
    lookup(&COHERENCE, order)
}
pub fn term_designation(order: u8) -> &'static str {
    lookup(&TERM_DESIGNATION, order)
}
pub fn connective_designation(order: u8) -> &'static str {
    lookup(&CONNECTIVE_DESIGNATION, order)
}

/// `|E| = C(n, 2)` — the connective (edge) cardinality for order `n`.
fn edge_cardinality(order: u8) -> u8 {
    let n = order as usize;
    (n * n.saturating_sub(1) / 2) as u8
}

/// The **systematics hexad** — a system's six mutually-determining metadata facets.
/// Term cardinality = `|V|` = order; connective cardinality = `|E|` = `C(order, 2)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystematicsHexad {
    pub name: String,
    pub coherence: String,
    pub term_designation: String,
    pub connective_designation: String,
    pub term_cardinality: u8,
    pub connective_cardinality: u8,
}

/// The canonical hexad row for a cardinality (order `n`): all six facets derived from
/// the one number — e.g. `3 → {Triad, Dynamism, Impulses, Acts, 3, 3}`.
pub fn systematics_hexad(cardinality: u8) -> SystematicsHexad {
    SystematicsHexad {
        name: order_name(cardinality).to_string(),
        coherence: coherence(cardinality).to_string(),
        term_designation: term_designation(cardinality).to_string(),
        connective_designation: connective_designation(cardinality).to_string(),
        term_cardinality: cardinality,
        connective_cardinality: edge_cardinality(cardinality),
    }
}

/// **Derive the cardinality** from any single facet value (reverse lookup) — e.g.
/// `("connective_designation", "Acts") → Some(3)`, `("name", "Tetrad") → Some(4)`.
/// Returns the first matching cardinality (facets 9–12 designations are ambiguous —
/// all "Needs Research" — so those won't resolve uniquely).
pub fn cardinality_from(facet: &str, value: &str) -> Option<u8> {
    (1..=12u8).find(|&n| {
        let h = systematics_hexad(n);
        let got = match facet {
            "name" => h.name,
            "coherence" => h.coherence,
            "term_designation" => h.term_designation,
            "connective_designation" => h.connective_designation,
            "term_cardinality" => h.term_cardinality.to_string(),
            "connective_cardinality" => h.connective_cardinality.to_string(),
            _ => return false,
        };
        got.eq_ignore_ascii_case(value)
    })
}

/// **Validate** a candidate system's metadata against the canonical hexad for its
/// cardinality: every facet must match exactly. A triad whose coherence ≠ Dynamism,
/// or whose connective cardinality ≠ 3, fails. Returns the mismatches.
pub fn validate_metadata(candidate: &SystematicsHexad) -> Result<(), Vec<String>> {
    let n = candidate.term_cardinality;
    let canonical = systematics_hexad(n);
    let mut errs = Vec::new();
    let mut check = |field: &str, got: &str, want: &str| {
        if !got.eq_ignore_ascii_case(want) {
            errs.push(format!("{field}: '{got}' ≠ canonical '{want}' (cardinality {n})"));
        }
    };
    check("name", &candidate.name, &canonical.name);
    check("coherence", &candidate.coherence, &canonical.coherence);
    check("term_designation", &candidate.term_designation, &canonical.term_designation);
    check(
        "connective_designation",
        &candidate.connective_designation,
        &canonical.connective_designation,
    );
    if candidate.connective_cardinality != canonical.connective_cardinality {
        errs.push(format!(
            "connective_cardinality: {} ≠ canonical {} (cardinality {n})",
            candidate.connective_cardinality, canonical.connective_cardinality
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

    #[test]
    fn triad_hexad_is_the_canonical_row() {
        let h = systematics_hexad(3);
        assert_eq!(h.name, "Triad");
        assert_eq!(h.coherence, "Dynamism");
        assert_eq!(h.term_designation, "Impulses");
        assert_eq!(h.connective_designation, "Acts");
        assert_eq!(h.term_cardinality, 3);
        assert_eq!(h.connective_cardinality, 3); // C(3,2)
        // tetrad: |V|=4, |E|=6.
        let t = systematics_hexad(4);
        assert_eq!((t.name.as_str(), t.term_cardinality, t.connective_cardinality), ("Tetrad", 4, 6));
    }

    #[test]
    fn derivation_is_reciprocal() {
        assert_eq!(cardinality_from("connective_designation", "Acts"), Some(3));
        assert_eq!(cardinality_from("name", "Tetrad"), Some(4));
        assert_eq!(cardinality_from("coherence", "dynamism"), Some(3)); // case-insensitive
        assert_eq!(cardinality_from("term_cardinality", "5"), Some(5));
        assert_eq!(cardinality_from("name", "Nonad"), None);
    }

    #[test]
    fn validation_is_exact() {
        // the correct triad passes.
        assert!(validate_metadata(&systematics_hexad(3)).is_ok());
        // wrong coherence for a triad fails.
        let mut bad = systematics_hexad(3);
        bad.coherence = "Complementarity".to_string();
        assert!(validate_metadata(&bad).is_err());
        // a "triad" claiming tetrad edge-cardinality fails.
        let mut bad2 = systematics_hexad(3);
        bad2.connective_cardinality = 6;
        assert!(validate_metadata(&bad2).is_err());
        // mismatched name vs cardinality fails (name Triad but cardinality 4).
        let mut bad3 = systematics_hexad(4);
        bad3.name = "Triad".to_string();
        assert!(validate_metadata(&bad3).is_err());
    }
}
