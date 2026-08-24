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

/// A **data element** — a Holochain entry holding a character (a **term** or
/// **connective**). It is *linked onto* the topology, **not embedded** (the
/// fully-linked form: `vertex-element ──▶ term-character-element`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataElement {
    pub id: String,
    pub kind: String,      // "term" | "connective"
    pub character: String, // the character value / ref
}

/// A **hyperlink** — the general Holochain link `base ──type──▶ target` between two
/// element ids. Used for the **orthogonal anchors** (topology element ▶ data element)
/// and the **lateral** system links (`term ──connective── term`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperLink {
    pub id: String,
    pub base: String,
    pub target: String,
    pub link_type: String, // "anchor" | a connective element id (lateral)
}

/// The **hypergraph** the Controller composes: the bare `topology` (the *lateral
/// topology* domain, vertex–edge–vertex), plus the anchored **semantics** — `data`
/// (term/connective character elements = the *lateral system* domain) and `links`
/// (the **orthogonal anchors** + the **lateral** `term–connective–term`). The
/// semantics stay **decoupled**: drop the anchors and a standalone semantic triad
/// remains; re-adding them is a **dyadic remap** (term→vertex, connective→edge).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Hypergraph {
    pub topology: TopologyGraph,
    pub data: Vec<DataElement>,
    pub links: Vec<HyperLink>,
}

// ---- The Controller's morphisms: micro-monads that compose a system ----

/// The **kind** of a morphism — the taxonomy. Topology + semantic anchoring use
/// `Monomorphism`; a decoupled-triad remap is a **`Bimorphism`** (two orthogonal
/// maps); `Homomorphism` is a placeholder for the composition to come.
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
/// hypergraph — "the model is a bundled set of monomorphisms".
pub trait Morphism {
    /// Which kind of morphism this is (the enum taxonomy).
    fn kind(&self) -> MorphismKind;
    /// The single function point: apply this morphism to the hypergraph under
    /// construction (its one action).
    fn apply(&self, hg: &mut Hypergraph);
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
    fn apply(&self, hg: &mut Hypergraph) {
        hg.topology.elements.push(Element {
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
    fn apply(&self, hg: &mut Hypergraph) {
        let (a, b) = if self.a < self.b { (self.a, self.b) } else { (self.b, self.a) };
        hg.topology.links.push(Link {
            id: format!("lk_{}_{}_{}", self.order, a, b),
            endpoints: (a, b),
        });
    }
}

/// `term character ↦ vertex` — a monomorphism: adds the term as a data element and an
/// **orthogonal anchor** from its canonical vertex to it (the position must not be
/// missed — the topology is the anchor).
pub struct TermToVertex {
    pub order: u8,
    pub position: u8,
    pub character: String,
}
impl Morphism for TermToVertex {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let term_id = format!("term_{}_{}", self.order, self.position);
        hg.data.push(DataElement {
            id: term_id.clone(),
            kind: "term".to_string(),
            character: self.character.clone(),
        });
        hg.links.push(HyperLink {
            id: format!("anchor_v_{}_{}", self.order, self.position),
            base: format!("el_{}_{}", self.order, self.position),
            target: term_id,
            link_type: "anchor".to_string(),
        });
    }
}

/// `connective character ↦ orbit` — a monomorphism: adds the connective as a data
/// element, an **orthogonal anchor** from its orbit to it, and the **lateral**
/// `term ──connective── term` link between the orbit's two endpoint terms.
pub struct ConnectiveToOrbit {
    pub order: u8,
    pub a: u8,
    pub b: u8,
    pub character: String,
}
impl Morphism for ConnectiveToOrbit {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let (a, b) = if self.a < self.b { (self.a, self.b) } else { (self.b, self.a) };
        let conn_id = format!("conn_{}_{}_{}", self.order, a, b);
        hg.data.push(DataElement {
            id: conn_id.clone(),
            kind: "connective".to_string(),
            character: self.character.clone(),
        });
        // orthogonal anchor: orbit ▶ connective character.
        hg.links.push(HyperLink {
            id: format!("anchor_e_{}_{}_{}", self.order, a, b),
            base: format!("lk_{}_{}_{}", self.order, a, b),
            target: conn_id.clone(),
            link_type: "anchor".to_string(),
        });
        // lateral system link: term_a ──connective── term_b (the link type is the
        // connective element, so the triad stands alone even without the topology).
        hg.links.push(HyperLink {
            id: format!("lat_{}_{}_{}", self.order, a, b),
            base: format!("term_{}_{}", self.order, a),
            target: format!("term_{}_{}", self.order, b),
            link_type: conn_id,
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
/// constraints. No semantics — those are anchored on by `compose_system`.
pub fn generate_topology(order: u8) -> TopologyGraph {
    let mut hg = Hypergraph::default();
    hg.topology.order = order;
    for morphism in topology_morphisms(order) {
        morphism.apply(&mut hg);
    }
    debug_assert!(
        hg.topology
            .validate_against(&Template::for_order(order))
            .is_ok(),
        "generated topology must satisfy the template"
    );
    hg.topology
}

/// **Compose a full system hypergraph** for an order: the topology bundle, then the
/// semantic bundle — a `TermToVertex` per position, a `ConnectiveToOrbit` per edge.
/// `terms` are in position order (`1..=order`); `connectives` in canonical edge order
/// (`(1,2),(1,3),…`). Topology is composed first so the semantic anchors reference
/// existing vertex/orbit elements.
pub fn compose_system(order: u8, terms: &[String], connectives: &[String]) -> Hypergraph {
    let mut hg = Hypergraph::default();
    hg.topology.order = order;
    for morphism in topology_morphisms(order) {
        morphism.apply(&mut hg);
    }
    for position in 1..=order {
        if let Some(character) = terms.get((position - 1) as usize) {
            TermToVertex {
                order,
                position,
                character: character.clone(),
            }
            .apply(&mut hg);
        }
    }
    for ((a, b), character) in Template::for_order(order)
        .edges()
        .into_iter()
        .zip(connectives.iter())
    {
        ConnectiveToOrbit {
            order,
            a,
            b,
            character: character.clone(),
        }
        .apply(&mut hg);
    }
    hg
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

    #[test]
    fn compose_system_anchors_terms_and_connectives_at_canonical_positions() {
        let hg = compose_system(
            3,
            &["Will".into(), "Function".into(), "Being".into()],
            &["Generation".into(), "Decision".into(), "Consent".into()], // (1,2)(1,3)(2,3)
        );
        // topology intact.
        assert_eq!(hg.topology.elements.len(), 3);
        assert_eq!(hg.topology.links.len(), 3);
        // 3 term + 3 connective data elements, LINKED on (not embedded).
        assert_eq!(hg.data.iter().filter(|d| d.kind == "term").count(), 3);
        assert_eq!(hg.data.iter().filter(|d| d.kind == "connective").count(), 3);
        // canonical position: term_3_1 = Will, anchored orthogonally to vertex el_3_1.
        assert_eq!(hg.data.iter().find(|d| d.id == "term_3_1").unwrap().character, "Will");
        let anchor = hg.links.iter().find(|l| l.id == "anchor_v_3_1").unwrap();
        assert_eq!(anchor.base, "el_3_1");
        assert_eq!(anchor.target, "term_3_1");
        // connective on the (1,2) orbit = Generation.
        assert_eq!(hg.data.iter().find(|d| d.id == "conn_3_1_2").unwrap().character, "Generation");
        // lateral term–connective–term: term_3_1 ──conn_3_1_2── term_3_2.
        let lat = hg.links.iter().find(|l| l.id == "lat_3_1_2").unwrap();
        assert_eq!(lat.base, "term_3_1");
        assert_eq!(lat.target, "term_3_2");
        assert_eq!(lat.link_type, "conn_3_1_2");
    }

    #[test]
    fn semantic_triad_is_decoupled_from_topology() {
        // Dropping the orthogonal anchors leaves a standalone semantic triad
        // (term–connective–term) — the decoupled projection.
        let hg = compose_system(
            3,
            &["a".into(), "b".into(), "c".into()],
            &["x".into(), "y".into(), "z".into()],
        );
        let lateral: Vec<_> = hg.links.iter().filter(|l| l.id.starts_with("lat_")).collect();
        assert_eq!(lateral.len(), 3); // the triad's internal term–connective–term structure
        for l in lateral {
            assert!(l.base.starts_with("term_") && l.target.starts_with("term_"));
            assert!(l.link_type.starts_with("conn_")); // linked BY a connective character
        }
    }
}
