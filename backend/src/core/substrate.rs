//! The substrate — Holochain-style **elements + links** (placeholders until a real
//! Holochain backend). Each element and each link is a **function-monad**: a
//! *container with a boundary + a set of actions* (functional composition). **NB:
//! this is the FP monad, NOT our systematics `Monad` (the order-1 system) — do not
//! conflate them.**
//!
//! A system materialises as a **hypergraph**: topological **vertex → element**,
//! topological **edge → link**; each links to its data — term / connective
//! characters, geometry, colour — which may link to further data. We work
//! **UNDIRECTED**: a triad's six directed links simplify to **3 bidirectional
//! 'orbits'** (an orbit = the two opposite directed links treated as one), so a link
//! here holds an *unordered* endpoint pair. Materialisation is **derived** now;
//! storable in the **DHT** later. (Retires the AD4M 'perspective'/'Link' language.)
//!
//! SUPERSEDED-IN-SHAPE (user, 2026-08-20): `materialize` here embeds `term`/
//! `coordinate`/`colour` and takes coords/colours as inputs — the corrected model
//! makes those SEPARATE mappings and has elements **link to** their data elements
//! (vertex-element ──link──▶ term-character-element), with the mapping a **functor of
//! morphisms over the semantic pentad** (coherence · term-designation ·
//! connective-designation · terms · connectives), generated topology-first into the
//! DHT. See `docs/design-intent.md` → *CORRECTED — materialisation is a Functor…*.
//! Kept as a placeholder for the derived hypergraph until that rework.

use serde::{Deserialize, Serialize};

/// A materialised vertex — a Holochain-style **element** (a function-monad:
/// container + actions). Holds its topological anchor `(order, position)` and links
/// to its data: the term character, its geometry (coordinate) and colour.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub order: u8,
    /// The topological anchor: position `1..=order`.
    pub position: u8,
    /// Link to the term character (its value / ref).
    pub term: String,
    /// Geometry — the coordinate, for graph views.
    pub coordinate: [f64; 3],
    /// Colour, for graph views.
    pub colour: String,
}

/// A materialised edge — a Holochain-style **link** (also a function-monad). Stored
/// as one **undirected 'orbit'** (the two opposite directed links treated as one):
/// an unordered vertex-position pair, plus a link to the connective character.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    /// Unordered vertex-position pair `{a, b}` with `a < b` (undirected).
    pub endpoints: (u8, u8),
    /// Link to the connective character (its value / ref).
    pub connective: String,
}

/// A system materialised as a hypergraph of elements + (undirected) links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Hypergraph {
    pub order: u8,
    pub elements: Vec<Element>,
    pub links: Vec<Link>,
}

/// Materialise a system into its hypergraph — **derived** (pure). Maps the semantic
/// characters onto the topological anchor + geometry + colour.
///
/// - `terms` — term characters (value / ref), in position order `1..=order`;
/// - `connectives` — connective characters, in canonical edge order
///   `(1,2), (1,3), …, (n-1,n)` (matching `Template::edges`);
/// - `coordinates` / `colours` — per-position geometry and colour.
///
/// Missing inputs fall back to empty/origin, so a bare topology still materialises.
pub fn materialize(
    order: u8,
    terms: &[String],
    connectives: &[String],
    coordinates: &[[f64; 3]],
    colours: &[String],
) -> Hypergraph {
    let elements = (0..order as usize)
        .map(|i| Element {
            id: format!("el_{}_{}", order, i + 1),
            order,
            position: (i + 1) as u8,
            term: terms.get(i).cloned().unwrap_or_default(),
            coordinate: coordinates.get(i).copied().unwrap_or([0.0, 0.0, 0.0]),
            colour: colours.get(i).cloned().unwrap_or_default(),
        })
        .collect();

    let mut links = Vec::new();
    let mut e = 0usize;
    for a in 1..=order {
        for b in (a + 1)..=order {
            links.push(Link {
                id: format!("lk_{}_{}_{}", order, a, b),
                endpoints: (a, b),
                connective: connectives.get(e).cloned().unwrap_or_default(),
            });
            e += 1;
        }
    }
    Hypergraph { order, elements, links }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slugs(xs: &[&str]) -> Vec<String> {
        xs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn materialize_triad_gives_3_elements_3_orbits() {
        let hg = materialize(
            3,
            &slugs(&["Will", "Function", "Being"]),
            &slugs(&["Generation", "Decision", "Consent"]), // (1,2),(1,3),(2,3)
            &[[0.0, 1.0, 0.0], [-1.0, -1.0, 0.0], [1.0, -1.0, 0.0]],
            &slugs(&["red", "green", "blue"]),
        );
        assert_eq!(hg.elements.len(), 3);
        assert_eq!(hg.links.len(), 3); // C(3,2) = 3 undirected orbits (not 6)

        // vertices carry the term + its geometry + colour at the right anchor.
        assert_eq!(hg.elements[0].position, 1);
        assert_eq!(hg.elements[0].term, "Will");
        assert_eq!(hg.elements[0].coordinate, [0.0, 1.0, 0.0]);
        assert_eq!(hg.elements[2].colour, "blue");

        // edges are undirected orbits in canonical order, typed by the connective.
        assert_eq!(hg.links[0].endpoints, (1, 2));
        assert_eq!(hg.links[0].connective, "Generation");
        assert_eq!(hg.links[1].endpoints, (1, 3));
        assert_eq!(hg.links[1].connective, "Decision");
        assert_eq!(hg.links[2].endpoints, (2, 3));
        assert_eq!(hg.links[2].connective, "Consent");
    }

    #[test]
    fn edge_count_is_c_n_2_for_any_order() {
        for (order, edges) in [(1u8, 0usize), (2, 1), (4, 6), (6, 15), (8, 28)] {
            let hg = materialize(order, &[], &[], &[], &[]);
            assert_eq!(hg.elements.len(), order as usize);
            assert_eq!(hg.links.len(), edges);
        }
    }

    #[test]
    fn endpoints_are_ordered_low_high_undirected() {
        let hg = materialize(4, &[], &[], &[], &[]);
        for l in &hg.links {
            assert!(l.endpoints.0 < l.endpoints.1, "undirected: a < b");
        }
    }
}
