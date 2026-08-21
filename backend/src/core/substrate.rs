//! The substrate — Holochain-style **elements + links** (placeholders until a real
//! Holochain backend). Each element and each link is a **function-monad**: a
//! *container with a boundary + a set of actions* (functional composition). **NB:
//! this is the FP monad, NOT our systematics `Monad` (the order-1 system) — do not
//! conflate them.**
//!
//! **Generate, don't read (user, 2026-08-20).** Systems are *composed then stored*,
//! so the substrate is built by **generating** structure, not by materialising an
//! already-composed system. Step 1 (here): from the **graph rules** — order + size
//! (the `Template`) and its adjacency/incidence — **generate a topology**: the K_n
//! shape as bare elements (vertices) + undirected links (edges). We work
//! **UNDIRECTED**: a triad's six directed links simplify to **3 bidirectional
//! 'orbits'**, so a `Link` holds an *unordered* endpoint pair.
//!
//! Later: **store** the topology in the DHT, then **anchor** semantics onto it —
//! term/connective characters, geometry, colour — each as a *separate* mapping
//! (a functor of morphisms over the semantic pentad; the anchoring links are the
//! orthogonal `vertex-element ──▶ term-character-element`). See `docs/design-intent.md`
//! → *CORRECTED — materialisation is a Functor…*. (Retires the AD4M 'perspective'
//! language.)

use serde::{Deserialize, Serialize};

use super::grammar::Template;

/// A vertex of the topology — a Holochain-style **element** (a function-monad:
/// container + actions). Bare: it holds only its topological anchor `(order,
/// position)`. Data (term character, geometry, colour) is **linked on** later, not
/// embedded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub order: u8,
    /// The topological anchor: position `1..=order`.
    pub position: u8,
}

/// An edge of the topology — a Holochain-style **link** (also a function-monad).
/// One **undirected 'orbit'** (the two opposite directed links treated as one):
/// an unordered vertex-position pair `{a, b}` with `a < b`. The connective character
/// is **linked on** later, not embedded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub endpoints: (u8, u8),
}

/// A generated topology (a K_n) as a hypergraph of bare elements + undirected links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologyGraph {
    pub order: u8,
    pub elements: Vec<Element>,
    pub links: Vec<Link>,
}

/// **Generate the topology** for an order from the graph rules — the `Template`
/// (order + size) and its edges. Pure: the K_n shape as elements (vertices `1..=n`)
/// + `C(n,2)` undirected links (edges), in canonical `(1,2),(1,3),…,(n-1,n)` order.
/// No semantics/geometry/colour — those are anchored on afterwards as separate maps.
pub fn generate_topology(order: u8) -> TopologyGraph {
    let template = Template::for_order(order);
    let elements = (1..=order)
        .map(|position| Element {
            id: format!("el_{}_{}", order, position),
            order,
            position,
        })
        .collect();
    let links = template
        .edges()
        .into_iter()
        .map(|(a, b)| Link {
            id: format!("lk_{}_{}_{}", order, a, b),
            endpoints: (a, b),
        })
        .collect();
    TopologyGraph { order, elements, links }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triad_topology_is_3_vertices_3_orbits() {
        let t = generate_topology(3);
        assert_eq!(t.elements.len(), 3);
        assert_eq!(t.links.len(), 3); // C(3,2) undirected orbits (not 6)
        assert_eq!(t.elements[0].id, "el_3_1");
        assert_eq!(t.elements[0].position, 1);
        // edges in canonical order, undirected (a < b).
        assert_eq!(t.links[0].endpoints, (1, 2));
        assert_eq!(t.links[1].endpoints, (1, 3));
        assert_eq!(t.links[2].endpoints, (2, 3));
        assert_eq!(t.links[2].id, "lk_3_2_3");
    }

    #[test]
    fn edge_count_matches_the_template_size_for_any_order() {
        for (order, size) in [(1u8, 0usize), (2, 1), (4, 6), (6, 15), (8, 28)] {
            let t = generate_topology(order);
            assert_eq!(t.elements.len(), order as usize);
            assert_eq!(t.links.len(), size);
            assert_eq!(t.links.len(), Template::for_order(order).size());
        }
    }

    #[test]
    fn links_are_undirected_low_high() {
        for l in &generate_topology(5).links {
            assert!(l.endpoints.0 < l.endpoints.1, "undirected: a < b");
        }
    }
}
