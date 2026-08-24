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

impl TopologyGraph {
    /// **Validate against the graph rules** (the `Template`): order (vertex count),
    /// size (orbit count), and shape (every orbit is a legal, undirected template
    /// edge). This is the Controller's guarantee — the Graph Template holds the
    /// validation rules, so a correct topology is *enforced*, not hoped for.
    pub fn validate_against(&self, template: &Template) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        let n = template.order();
        if self.elements.len() != n as usize {
            errs.push(format!(
                "order: {} vertices, expected {n}",
                self.elements.len()
            ));
        }
        if self.links.len() != template.size() {
            errs.push(format!(
                "size: {} orbits, expected {}",
                self.links.len(),
                template.size()
            ));
        }
        let legal: std::collections::HashSet<(u8, u8)> =
            template.edges().into_iter().collect();
        for l in &self.links {
            if l.endpoints.0 >= l.endpoints.1 {
                errs.push(format!("orbit {} is not undirected (a < b)", l.id));
            } else if !legal.contains(&l.endpoints) {
                errs.push(format!("orbit {:?} is not a legal template edge", l.endpoints));
            }
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

// ---- The Controller's morphisms: micro-monads that compose a system ----

/// The **kind** of a morphism — the taxonomy. Only `Monomorphism` is used so far
/// (topology assignment); the rest are placeholders for the composition to come.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphismKind {
    Monomorphism,
    Bimorphism,
    Homomorphism,
}

/// A **morphism** — a **micro-monad** (a container with a *single function point*,
/// `apply`). Concrete morphisms (structs) implement this and declare their `kind`.
/// A system is composed by **bundling** morphisms and applying them to build up the
/// substrate — "the model is a bundled set of monomorphisms".
pub trait Morphism {
    /// Which kind of morphism this is (the enum taxonomy).
    fn kind(&self) -> MorphismKind;
    /// The single function point: apply this morphism to the topology under
    /// construction (its one action).
    fn apply(&self, topology: &mut TopologyGraph);
}

/// `position ↦ vertex-element` — a monomorphism (one vertex per position).
pub struct PositionToVertex {
    pub order: u8,
    pub position: u8,
}
impl Morphism for PositionToVertex {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn apply(&self, topology: &mut TopologyGraph) {
        topology.elements.push(Element {
            id: format!("el_{}_{}", self.order, self.position),
            order: self.order,
            position: self.position,
        });
    }
}

/// `edge ↦ orbit-link` — a monomorphism (one undirected orbit per edge).
pub struct EdgeToOrbit {
    pub order: u8,
    pub a: u8,
    pub b: u8,
}
impl Morphism for EdgeToOrbit {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn apply(&self, topology: &mut TopologyGraph) {
        let (a, b) = if self.a < self.b { (self.a, self.b) } else { (self.b, self.a) };
        topology.links.push(Link {
            id: format!("lk_{}_{}_{}", self.order, a, b),
            endpoints: (a, b),
        });
    }
}

/// The **bundle** of monomorphisms that composes a K_n topology from its graph rules
/// (the `Template`): a `Position→Vertex` per vertex, an `Edge→Orbit` per edge.
pub fn topology_morphisms(order: u8) -> Vec<Box<dyn Morphism>> {
    let mut bundle: Vec<Box<dyn Morphism>> = Vec::new();
    for position in 1..=order {
        bundle.push(Box::new(PositionToVertex { order, position }));
    }
    for (a, b) in Template::for_order(order).edges() {
        bundle.push(Box::new(EdgeToOrbit { order, a, b }));
    }
    bundle
}

/// **Generate the topology** for an order by **composing the monomorphism bundle**
/// (applying each micro-monad to build up the graph), then **validating** the result
/// against the graph rules (the `Template`). Correct topology is guaranteed by the
/// constraints. No semantics/geometry/colour — those are anchored on afterwards.
pub fn generate_topology(order: u8) -> TopologyGraph {
    let mut topology = TopologyGraph {
        order,
        elements: Vec::new(),
        links: Vec::new(),
    };
    for morphism in topology_morphisms(order) {
        morphism.apply(&mut topology);
    }
    debug_assert!(
        topology
            .validate_against(&Template::for_order(order))
            .is_ok(),
        "generated topology must satisfy the template"
    );
    topology
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

    #[test]
    fn topology_morphisms_are_monomorphisms_one_per_vertex_and_edge() {
        let bundle = topology_morphisms(4);
        // 4 vertices + C(4,2)=6 edges = 10 micro-monads, all monomorphisms.
        assert_eq!(bundle.len(), 10);
        for m in &bundle {
            assert_eq!(m.kind(), MorphismKind::Monomorphism);
        }
    }

    #[test]
    fn generated_topology_validates_and_broken_one_is_rejected() {
        let template = Template::for_order(3);
        let good = generate_topology(3);
        assert!(good.validate_against(&template).is_ok());

        // Break the shape: an orbit (1,1) is not a legal undirected template edge.
        let mut bad = good.clone();
        bad.links[0].endpoints = (1, 1);
        assert!(bad.validate_against(&template).is_err());

        // Break the size: drop a vertex → wrong order.
        let mut short = good;
        short.elements.pop();
        assert!(short.validate_against(&template).is_err());
    }
}
