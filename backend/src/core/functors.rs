//! Functor: a stored mapping from a source vocabulary to targets in the graph.
//!
//! A `Functor` is data (a table of `FunctorMapping`s) plus a small piece of code
//! (`apply`) that materialises `Term`s from mappings whose target is a Location.
//! Higher-level targets (e.g. an Order or whole system) are recorded but not
//! directly materialised — they represent structural intent for later expansion.
//!
//! The taxonomy of functor kinds (monomorphism, homomorphism, endomorphism,
//! holomorphism, …) is *emergent* from the shape of the mapping set; we don't
//! encode it in the type.

use serde::{Deserialize, Serialize};

use super::entries::{Entry, Term};
use super::language::Language;

/// A single (base → target) pair inside a Functor.
///
/// `base` and `target` are opaque strings that reference existing entries by ID:
/// - `base` is typically a Character ID (`"char_canonical_will"`) or a dotted
///   path (`"aspectsofexperiencetriad.triad"`) that resolves to a Character.
/// - `target` is any anchor ID: a Location (`"loc_3_1"`, `"loc_3_1_2"`), an
///   Order (`"order_3"`), a SystemName, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctorMapping {
    pub base: String,
    pub target: String,
}

impl FunctorMapping {
    pub fn new(base: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            target: target.into(),
        }
    }
}

/// A named mapping table with optional source-language tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Functor {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional vocabulary tag for the source side (informational; the graph
    /// still resolves `base` by ID).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_language: Option<Language>,
    pub mappings: Vec<FunctorMapping>,
}

impl Functor {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        source_language: Option<Language>,
        mappings: Vec<FunctorMapping>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            source_language,
            mappings,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Materialise `Term`s from each mapping whose target is a Location ID.
    ///
    /// Mappings whose target is a higher-level anchor (e.g. `order_3`) are
    /// skipped by this transform — they describe structural intent that a
    /// future expansion step will handle. Returns the produced Terms in order.
    pub fn apply(&self) -> Vec<Term> {
        self.mappings
            .iter()
            .filter_map(|m| location_to_term(&m.base, &m.target))
            .collect()
    }
}

fn location_to_term(base: &str, target: &str) -> Option<Term> {
    let rest = target.strip_prefix("loc_")?;
    let mut parts = rest.split('_');
    let order: u8 = parts.next()?.parse().ok()?;
    let p1: u8 = parts.next()?.parse().ok()?;
    match parts.next() {
        Some(p2_str) => {
            let p2: u8 = p2_str.parse().ok()?;
            if parts.next().is_some() {
                return None;
            }
            Some(Term::for_link(order, p1, p2, base))
        }
        None => Some(Term::with_auto_id(order, p1, base)),
    }
}

/// True when a `target` string references a Location (entry- or link-shaped).
pub fn target_is_location(target: &str) -> bool {
    target.starts_with("loc_")
}

/// True when a target string references an Order anchor.
pub fn target_is_order(target: &str) -> bool {
    target.starts_with("order_")
}

/// Convenience: check whether a Term-producing mapping matches a graph entry.
/// Not currently used by `apply` (which produces Terms without a graph
/// dependency) but useful for callers validating targets before storage.
pub fn target_exists_in(target: &str, entries: &[Entry]) -> bool {
    entries.iter().any(|e| e.id() == target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functor_apply_entry_shaped_target() {
        let f = Functor::new(
            "f_triad",
            "canonical-triad",
            Some(Language::Canonical),
            vec![
                FunctorMapping::new("char_canonical_will", "loc_3_1"),
                FunctorMapping::new("char_canonical_function", "loc_3_2"),
                FunctorMapping::new("char_canonical_being", "loc_3_3"),
            ],
        );
        let terms = f.apply();
        assert_eq!(terms.len(), 3);
        assert_eq!(terms[0].id, "term_3_1");
        assert_eq!(terms[0].location, "loc_3_1");
        assert_eq!(terms[0].character, "char_canonical_will");
        assert_eq!(terms[2].id, "term_3_3");
    }

    #[test]
    fn test_functor_apply_link_shaped_target() {
        let f = Functor::new(
            "f_triad_acts",
            "canonical-triad-acts",
            Some(Language::Canonical),
            vec![FunctorMapping::new(
                "char_canonical_generation",
                "loc_3_1_2",
            )],
        );
        let terms = f.apply();
        assert_eq!(terms.len(), 1);
        assert_eq!(terms[0].id, "term_3_1_2");
        assert_eq!(terms[0].location, "loc_3_1_2");
        assert!(terms[0].is_link_term());
    }

    #[test]
    fn test_functor_apply_skips_non_location_targets() {
        let f = Functor::new(
            "f_mixed",
            "mixed",
            None,
            vec![
                FunctorMapping::new("char_canonical_will", "loc_3_1"),
                FunctorMapping::new("aspectsofexperiencetriad.triad", "order_3"),
            ],
        );
        let terms = f.apply();
        assert_eq!(terms.len(), 1);
        assert_eq!(terms[0].location, "loc_3_1");
    }

    #[test]
    fn test_target_predicates() {
        assert!(target_is_location("loc_3_1"));
        assert!(target_is_location("loc_3_1_2"));
        assert!(!target_is_location("order_3"));
        assert!(target_is_order("order_3"));
        assert!(!target_is_order("loc_3_1"));
    }
}
